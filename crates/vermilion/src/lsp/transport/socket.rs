/* SPDX-License-Identifier: BSD-3-Clause */

use eyre::{Result, eyre};
use tokio::{
	net::{
		TcpStream,
		tcp::{OwnedReadHalf, OwnedWriteHalf},
	},
	select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use vermilion_lsp::message::Message;

use super::LSPTransport;

#[derive(Debug)]
pub(crate) struct SocketTransport {
	port: u16,
}

impl SocketTransport {
	pub const fn new(port: u16) -> Self {
		Self { port }
	}
}

async fn socket_reader(
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

async fn socket_writer(
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

impl LSPTransport for SocketTransport {
	async fn create(
		self,
		cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)> {
		unimplemented!("LSP socket transport not implemented");

		let mut tasks = JoinSet::new();

		let (read_tx, read_rx) = mpsc::unbounded_channel::<Message>();
		let (write_tx, write_rx) = mpsc::unbounded_channel::<Message>();

		// Connect to the given PIPE and then split it into halves
		let stream = TcpStream::connect(format!("127.0.0.1:{}", self.port)).await?;
		let (read, write) = stream.into_split();

		tasks
			.build_task()
			.name("socket-lsp-reader")
			.spawn(socket_reader(
				read,
				read_tx,
				cancellation_token.clone(),
				shutdown_channel.clone(),
			))?;

		tasks
			.build_task()
			.name("socket-lsp-writer")
			.spawn(socket_writer(
				write,
				write_rx,
				cancellation_token,
				shutdown_channel,
			))?;

		Ok((read_rx, write_tx, tasks))
	}
}
