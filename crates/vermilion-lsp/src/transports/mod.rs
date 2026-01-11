// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

use eyre::{Result, eyre};
use tokio::{
	fs::OpenOptions,
	io::AsyncWriteExt,
	select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, trace};

use super::message::Message;
use crate::trace::Trace;

#[cfg_attr(unix, path = "pipe_unix.rs")]
#[cfg_attr(windows, path = "pipe_win.rs")]
pub mod pipe;
pub mod socket;
pub mod stdio;

enum ReadPhase {
	Header,
	Content(usize),
}

fn get_split_index(buffer: &[u8]) -> Option<usize> {
	buffer
		.windows(4)
		.enumerate()
		.find(|&(_, w)| matches!(w, b"\r\n\r\n"))
		.map(|(idx, _)| idx + 4)
}

fn parse_message(
	read: usize,
	buf: &[u8],
	content: &mut Vec<u8>,
	phase: &mut ReadPhase,
	sender: &UnboundedSender<Message>,
	shutdown_channel: &UnboundedSender<()>,
	trace_sender: &Option<UnboundedSender<Trace>>,
) -> eyre::Result<()> {
	match phase {
		ReadPhase::Header => {
			let res = &buf[0..read];
			// Check to see if what we read in starts with the content length header
			if res.starts_with(b"Content-Length: ") {
				// If the data read ends with an `\r\n` then we can kinda assume it's just the
				// header
				if res.ends_with(b"\r\n") {
					let size = str::from_utf8(&res[16..])?.trim().parse::<usize>()?;
					trace!("Got content header of len: {}", size);
					*phase = ReadPhase::Content(size);
					return Ok(());
				} else {
					// We read way more than just the header, so we need to deal with it specially
					let pos = get_split_index(res);
					if let Some(pos) = pos {
						// Split out the header and body
						let header = &res[16..pos];
						let body = &res[pos..];

						// Like above, extract the content length from the header
						let size = match str::from_utf8(header) {
							Ok(size) => match size.trim().parse::<usize>() {
								Ok(size) => size,
								Err(err) => {
									error!("{}", err);
									shutdown_channel.send(())?;
									return Err(err.into());
								},
							},
							Err(err) => {
								error!("{}", err);
								shutdown_channel.send(())?;
								return Err(err.into());
							},
						};
						trace!("Got content header of len: {}", size);

						// Find out how many bytes we got vs how many left to read
						let current_content = read - pos;
						let remaining = size - current_content;

						// Append the bytes we do have,
						content.extend_from_slice(body);

						// Only move on if we have more bytes to read
						if remaining > 0 {
							trace!(
								"Have {} bytes of body already, {} remaining",
								current_content, remaining
							);
							*phase = ReadPhase::Content(remaining);
						} else {
							// Otherwise deserialize message and clear the buffer
							match Message::deserialize(content) {
								Ok(msg) => {
									if let Some(trace_sender) = trace_sender {
										// We don't want to abort the task if the send to the trace
										// writer failed
										let _ = trace_sender
											.send(Trace::new(crate::trace::Origin::Client, &msg));
									}
									sender.send(msg)?
								},
								Err(e) => {
									error!("Unable to deserialize LSP message:");
									error!("{}", e);
									error!("Message contents:");
									error!("{}", str::from_utf8(content)?);
								},
							}
							content.clear();
						}
					} else {
						error!("Unable to separate header from content!");
						return Err(eyre!("Unable to separate header from content!"));
					}
				}
			} else {
				error!("Invalid JSON-RPC header {:?}", str::from_utf8(res)?);
				return Err(eyre!("Invalid JSON-RPC header {:?}", str::from_utf8(res)?));
			}
		},
		ReadPhase::Content(length) => {
			let remaining = *length - read;
			content.extend_from_slice(&buf[0..read]);

			trace!("{} bytes remaining", remaining);
			if remaining > 0 {
				*phase = ReadPhase::Content(remaining);
			} else {
				*phase = ReadPhase::Header;
				// Deserialize message
				match Message::deserialize(content) {
					Ok(msg) => {
						if let Some(trace_sender) = trace_sender {
							// We don't want to abort the task if the send to the trace writer
							// failed
							let _ =
								trace_sender.send(Trace::new(crate::trace::Origin::Client, &msg));
						}
						sender.send(msg)?
					},
					Err(e) => {
						error!("Unable to deserialize LSP message:");
						error!("{}", e);
						error!("Message contents:");
						error!("{}", str::from_utf8(content)?);
					},
				}
				// Clear the contents buffer
				content.clear();
			}
		},
	}
	Ok(())
}

async fn trace_writer(
	mut trace_channel: UnboundedReceiver<Trace>,
	cancellation_token: CancellationToken,
	trace_file: PathBuf,
) -> eyre::Result<()> {
	let mut file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(trace_file)
		.await?;

	debug!("Starting LSP message logging");
	file.write_u8(b'[').await?;

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(trace_message) = trace_channel.recv() => {
				let msg = serde_json::to_string(&trace_message)?;
				file.write_all(msg.as_bytes()).await?;
				file.write_u8(b',').await?;
				file.write_u8(b'\n').await?;
			},
		}
	}

	file.write_u8(b']').await?;
	Ok(())
}

fn setup_trace(
	trace_file: Option<PathBuf>,
	tasks: &mut JoinSet<eyre::Result<()>>,
	cancellation_token: &CancellationToken,
) -> Option<UnboundedSender<Trace>> {
	if let Some(trace_file) = trace_file {
		debug!("LSP Trace file specified: {:?}", trace_file);

		let (trace_tx, trace_rx) = mpsc::unbounded_channel::<Trace>();
		tasks
			.build_task()
			.name("lsp-tracer")
			.spawn(trace_writer(
				trace_rx,
				cancellation_token.clone(),
				trace_file,
			))
			.ok()?;
		Some(trace_tx)
	} else {
		None
	}
}

pub enum TransportType {
	Stdio,
	Socket(u16),
	Pipe(PathBuf),
}

pub trait LSPTransport: Sized {
	#[allow(async_fn_in_trait)]
	async fn create(
		self,
		cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
		trace_file: Option<PathBuf>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)>;
}
