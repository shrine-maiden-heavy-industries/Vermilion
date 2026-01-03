/* SPDX-License-Identifier: BSD-3-Clause */

use std::time::Duration;

use eyre::Result;
use tokio::{
	net::{
		TcpStream,
		tcp::{OwnedReadHalf, OwnedWriteHalf},
	},
	select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
	time,
};
use tokio_util::sync::CancellationToken;
use tracing::error;
use crate::message::Message;

use super::LSPTransport;

#[derive(Debug)]
pub struct SocketTransport {
	port: u16,
}

impl SocketTransport {
	pub const fn new(port: u16) -> Self {
		Self { port }
	}
}

async fn socket_reader(
	stream: OwnedReadHalf,
	_sender: UnboundedSender<Message>,
	cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
) -> Result<()> {
	shutdown_channel.send(())?;
	error!("LSP socket transport not implemented, exiting");

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Ok(_res) = stream.readable() => {},
		}
	}

	Ok(())
}

async fn socket_writer(
	_stream: OwnedWriteHalf,
	mut receiver: UnboundedReceiver<Message>,
	cancellation_token: CancellationToken,
	_shutdown_channel: UnboundedSender<()>,
) -> Result<()> {
	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(_message) = receiver.recv() => {},
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
		let mut tasks = JoinSet::new();

		let (read_tx, read_rx) = mpsc::unbounded_channel::<Message>();
		let (write_tx, write_rx) = mpsc::unbounded_channel::<Message>();

		// Connect to the given PIPE and then split it into halves

		let stream = match time::timeout(
			Duration::from_secs(15),
			TcpStream::connect(format!("127.0.0.1:{}", self.port)),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				error!("Connection timeout for LSP socket");
				shutdown_channel.send(())?;
				return Err(e.into());
			},
		}?;
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
