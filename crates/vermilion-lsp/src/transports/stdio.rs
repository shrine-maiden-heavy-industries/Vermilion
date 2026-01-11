// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

use eyre::Result;
use tokio::{
	io::{self, AsyncReadExt, AsyncWriteExt},
	select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::error;

use super::LSPTransport;
use crate::{
	message::Message,
	trace::Trace,
	transports::{ReadPhase, parse_message, setup_trace},
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
	trace_sender: Option<UnboundedSender<Trace>>,
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
						match parse_message(
							read,
							&buf,
							&mut content,
							&mut phase,
							&sender,
							&shutdown_channel,
							&trace_sender
						) {
							Ok(_) => { continue; }
							Err(_) => { break; }
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
	trace_sender: Option<UnboundedSender<Trace>>,
) -> Result<()> {
	let mut stdout = io::stdout();
	let mut msg_buffer = Vec::new();
	let mut send_buffer = Vec::new();

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(message) = receiver.recv() => {
				if let Some(ref trace_sender) = trace_sender {
					// We don't want to abort the task if the send to the trace writer failed
					let _ = trace_sender.send(Trace::new(crate::trace::Origin::Server, &message));
				}

				message.serialize(&mut msg_buffer)?;

				send_buffer.extend_from_slice(format!("Content-Length: {}\r\n\r\n", msg_buffer.len()).as_bytes());
				send_buffer.extend_from_slice(&msg_buffer);

				stdout.write_all(&send_buffer).await?;
				stdout.flush().await?;

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
		trace_file: Option<PathBuf>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)> {
		let mut tasks = JoinSet::new();

		let (read_tx, read_rx) = mpsc::unbounded_channel::<Message>();
		let (write_tx, write_rx) = mpsc::unbounded_channel::<Message>();

		let trace_sender = setup_trace(trace_file, &mut tasks, &cancellation_token);

		tasks
			.build_task()
			.name("stdio-lsp-reader")
			.spawn(stdio_reader(
				read_tx,
				cancellation_token.clone(),
				shutdown_channel.clone(),
				trace_sender.clone(),
			))?;

		tasks
			.build_task()
			.name("stdio-lsp-writer")
			.spawn(stdio_writer(
				write_rx,
				cancellation_token,
				shutdown_channel,
				trace_sender,
			))?;

		Ok((read_rx, write_tx, tasks))
	}
}
