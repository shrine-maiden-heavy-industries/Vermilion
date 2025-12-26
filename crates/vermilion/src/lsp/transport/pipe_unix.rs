/* SPDX-License-Identifier: BSD-3-Clause */

use std::{io, path::PathBuf};

use eyre::Result;
use tokio::{
	net::{
		UnixStream,
		unix::{OwnedReadHalf, OwnedWriteHalf},
	},
	select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, trace};
use vermilion_lsp::message::Message;

use super::LSPTransport;

#[derive(Debug)]
pub(crate) struct PipeTransport {
	path: PathBuf,
}

impl PipeTransport {
	pub fn new(path: PathBuf) -> Self {
		Self { path }
	}
}

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

async fn pipe_reader(
	stream: OwnedReadHalf,
	sender: UnboundedSender<Message>,
	cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
) -> Result<()> {
	let mut buf = vec![0u8; 4096].into_boxed_slice();
	let mut content = Vec::new();
	let mut phase = ReadPhase::Header;

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Ok(_) = stream.readable() => {
				match stream.try_read(&mut buf) {
					Ok(read) => {
						match phase {
							ReadPhase::Header => {
								let res = &buf[0..read];
								// Check to see if what we read in starts with the content length header
								if res.starts_with(b"Content-Length: ") {
									// If the data read ends with an `\r\n` then we can kinda assume it's just the header
									if res.ends_with(b"\r\n") {
										let size = str::from_utf8(&res[16..])?.trim().parse::<usize>()?;
										trace!("Got content header of len: {}", size);
										phase = ReadPhase::Content(size);
										continue;
									} else {
										// We read way more than just the header, so we need to deal with it specially
										let pos = get_split_index(res);
										if let Some(pos) = pos {
											// Split out the header and body
											let header = &res[0..pos];
											let body = &res[pos..];

											// Like above, extract the content length from the header
											let size = str::from_utf8(header)?.trim().parse::<usize>()?;
											trace!("Got content header of len: {}", size);

											// Find out how many bytes we got vs how many left to read
											let current_content = read - pos;
											let remaining = size - current_content;

											// Append the bytes we do have,
											content.extend_from_slice(body);

											// Only move on if we have more bytes to read
											if remaining > 0 {
												trace!("Have {} bytes of body already, {} remaining", current_content, remaining);
												phase = ReadPhase::Content(remaining);
											} else {
												// Otherwise deserialize message and clear the buffer
												match Message::deserialize(&content) {
													Ok(msg) => sender.send(msg)?,
													Err(e) => {
														error!("Unable to deserialize LSP message:");
														error!("{}", e);
														error!("Message contents:");
														error!("{}", str::from_utf8(&content)?);
													}
												}
												content.clear();

											}
										} else {
											error!("Unable to separate header from content!");
											break;
										}
									}
								} else {
									error!("Invalid JSON-RPC header {:?}", str::from_utf8(res)?);
									break;
								}
							}
							ReadPhase::Content(length) => {
								let remaining = length - read;
								content.extend_from_slice(&buf[0..read]);

								trace!("{} bytes remaining", remaining);
								if remaining > 0 {
									phase = ReadPhase::Content(remaining);
								} else {
									phase = ReadPhase::Header;
									// Deserialize message
									match Message::deserialize(&content) {
										Ok(msg) => sender.send(msg)?,
										Err(e) => {
											error!("Unable to deserialize LSP message:");
											error!("{}", e);
											error!("Message contents:");
											error!("{}", str::from_utf8(&content)?);
										}
									}
									// Clear the contents buffer
									content.clear();
								}
							}
						}
					}
					Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
						continue;
					}
					Err(err) => {
						shutdown_channel.send(())?;
						return Err(err.into());
					}
				}
			},
		}
	}

	debug!("LSP Reader exited, shutting down");
	shutdown_channel.send(())?;

	Ok(())
}

async fn pipe_writer(
	stream: OwnedWriteHalf,
	mut receiver: UnboundedReceiver<Message>,
	cancellation_token: CancellationToken,
	_shutdown_channel: UnboundedSender<()>,
) -> Result<()> {
	let mut msg_buffer = Vec::new();
	let mut send_buffer = Vec::new();

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(message) = receiver.recv() => {
				let msg_size = message.serialize(msg_buffer.as_mut_slice())?;

				send_buffer.extend_from_slice(format!("Content-Length: {}\r\n\r\n", msg_size).as_bytes());
				send_buffer.extend_from_slice(&msg_buffer);

				stream.writable().await?;
				stream.try_write(&send_buffer)?;

				msg_buffer.clear();
				send_buffer.clear();
			},
		}
	}

	Ok(())
}

impl LSPTransport for PipeTransport {
	async fn create(
		self,
		cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)> {
		let mut tasks = JoinSet::new();

		let (read_tx, read_rx) = mpsc::unbounded_channel::<Message>();
		let (write_tx, write_rx) = mpsc::unbounded_channel::<Message>();

		// Connect to the given PIPE and then split it into halves
		let stream = UnixStream::connect(self.path).await?;
		let (read, write) = stream.into_split();

		tasks
			.build_task()
			.name("pipe-lsp-reader")
			.spawn(pipe_reader(
				read,
				read_tx,
				cancellation_token.clone(),
				shutdown_channel.clone(),
			))?;

		tasks
			.build_task()
			.name("pipe-lsp-writer")
			.spawn(pipe_writer(
				write,
				write_rx,
				cancellation_token,
				shutdown_channel,
			))?;

		Ok((read_rx, write_tx, tasks))
	}
}
