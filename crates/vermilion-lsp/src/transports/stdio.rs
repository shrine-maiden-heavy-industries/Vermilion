// SPDX-License-Identifier: BSD-3-Clause

use eyre::Result;
use tokio::{
	io::{self, AsyncReadExt, AsyncWriteExt},
	select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, trace};

use super::LSPTransport;
use crate::{
	message::Message,
	transports::{ReadPhase, get_split_index},
};

#[derive(Debug)]
pub struct StdioTransport {}

impl StdioTransport {
	#[allow(clippy::new_without_default)]
	pub const fn new() -> Self {
		Self {}
	}
}

async fn stdio_reader(
	sender: UnboundedSender<Message>,
	cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
) -> Result<()> {
	let mut stdin = io::stdin();
	let mut buf = vec![0u8; 4096].into_boxed_slice();
	let mut content = Vec::new();
	let mut phase = ReadPhase::Header;

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			read_result = stdin.read(&mut buf) => {
				match read_result {
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
											let header = &res[16..pos];
											let body = &res[pos..];

											// Like above, extract the content length from the header
											let size = match str::from_utf8(header) {
												Ok(size) => {
													match size.trim().parse::<usize>() {
														Ok(size) => size,
														Err(err) => {
															error!("{}", err);
															shutdown_channel.send(())?;
															return Err(err.into());
														}
													}
												},
												Err(err) => {
													error!("{}", err);
													shutdown_channel.send(())?;
													return Err(err.into());
												}
											};
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
							},
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
							},
						}
					},
					Err(e) => {
						error!("stdin read error: {}", e)
					}
				}
			}
		}
	}

	shutdown_channel.send(())?;
	error!("LSP stdio transport not implemented, exiting");

	// loop {}

	Ok(())
}

async fn stdio_writer(
	mut receiver: UnboundedReceiver<Message>,
	cancellation_token: CancellationToken,
	_shutdown_channel: UnboundedSender<()>,
) -> Result<()> {
	let mut stdout = io::stdout();
	let mut msg_buffer = Vec::new();
	let mut send_buffer = Vec::new();

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(message) = receiver.recv() => {
				message.serialize(&mut msg_buffer)?;

				send_buffer.extend_from_slice(format!("Content-Length: {}\r\n\r\n", msg_buffer.len()).as_bytes());
				send_buffer.extend_from_slice(&msg_buffer);

				stdout.write_all(&send_buffer).await?;

				msg_buffer.clear();
				send_buffer.clear();
			},
		}
	}

	Ok(())
}

impl LSPTransport for StdioTransport {
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

		tasks
			.build_task()
			.name("stdio-lsp-reader")
			.spawn(stdio_reader(
				read_tx,
				cancellation_token.clone(),
				shutdown_channel.clone(),
			))?;

		tasks
			.build_task()
			.name("stdio-lsp-writer")
			.spawn(stdio_writer(write_rx, cancellation_token, shutdown_channel))?;

		Ok((read_rx, write_tx, tasks))
	}
}
