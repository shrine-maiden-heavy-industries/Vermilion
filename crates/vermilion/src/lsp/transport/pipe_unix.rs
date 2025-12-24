/* SPDX-License-Identifier: BSD-3-Clause */

use std::path::PathBuf;

use eyre::{Result, eyre};
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
use tracing::trace;
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

async fn pipe_reader(
	stream: OwnedReadHalf,
	mut sender: UnboundedSender<Message>,
	cancellation_token: CancellationToken,
	shutdown_tx: UnboundedSender<()>,
) -> Result<()> {
	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			res = stream.readable() => {},
		}
	}

	Ok(())
}

async fn pipe_writer(
	stream: OwnedWriteHalf,
	mut receiver: UnboundedReceiver<Message>,
	cancellation_token: CancellationToken,
	shutdown_tx: UnboundedSender<()>,
) -> Result<()> {
	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			message = receiver.recv() => {},
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
