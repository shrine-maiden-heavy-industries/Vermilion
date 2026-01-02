/* SPDX-License-Identifier: BSD-3-Clause */

use eyre::Result;
use tokio::{
	io, select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::error;
use vermilion_lsp::message::Message;

use super::LSPTransport;

#[derive(Debug)]
pub(crate) struct StdioTransport {}

impl StdioTransport {
	pub const fn new() -> Self {
		Self {}
	}
}

async fn stdio_reader(
	_sender: UnboundedSender<Message>,
	_cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
) -> Result<()> {
	let _stdin = io::stdin();

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
	let _stdout = io::stdout();
	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(_message) = receiver.recv() => {},
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
