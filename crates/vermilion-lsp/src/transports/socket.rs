// SPDX-License-Identifier: BSD-3-Clause

use std::{io, time::Duration};

use eyre::Result;
use tokio::{
	io::Interest,
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
use tracing::{debug, error};

use super::LSPTransport;
use crate::{
	message::Message,
	transports::{ReadPhase, parse_message},
};
#[cfg(feature = "trace-server")]
use crate::{
	trace::Trace,
	transports::trace::{TraceTransport, setup_trace},
};

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
	sender: UnboundedSender<Message>,
	cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
	#[cfg(feature = "trace-server")] trace_sender: Option<UnboundedSender<Trace>>,
) -> Result<()> {
	let mut buf = vec![0u8; 4096].into_boxed_slice();
	let mut content = Vec::new();
	let mut phase = ReadPhase::Header;

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			ready_state = stream.ready(Interest::READABLE | Interest::ERROR) => {
				let ready_state = ready_state?;
				if ready_state.is_empty() {
					// Need to wait again here..
					continue;
				} else if ready_state.is_error() {
					error!("Socket entered error state, aborting");
					break;
				}
				match stream.try_read(&mut buf) {
					Ok(read) => {
						match parse_message(
							read,
							&buf,
							&mut content,
							&mut phase,
							&sender,
							&shutdown_channel,
							#[cfg(feature = "trace-server")]
							&trace_sender
						) {
							Ok(_) => { continue; }
							Err(_) => { break; }
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

async fn socket_writer(
	stream: OwnedWriteHalf,
	mut receiver: UnboundedReceiver<Message>,
	cancellation_token: CancellationToken,
	_shutdown_channel: UnboundedSender<()>,
	#[cfg(feature = "trace-server")] trace_sender: Option<UnboundedSender<Trace>>,
) -> Result<()> {
	let mut msg_buffer = Vec::new();
	let mut send_buffer = Vec::new();

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(message) = receiver.recv() => {
				#[cfg(feature = "trace-server")]
				if let Some(ref trace_sender) = trace_sender {
					// We don't want to abort the task if the send to the trace writer failed
					let _ = trace_sender.send(Trace::new(crate::trace::Origin::Server, &message));
				}

				message.serialize(&mut msg_buffer)?;

				send_buffer.extend_from_slice(format!("Content-Length: {}\r\n\r\n", msg_buffer.len()).as_bytes());
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

impl LSPTransport for SocketTransport {
	async fn create(
		self,
		cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
		#[cfg(feature = "trace-server")] trace_transport: Option<TraceTransport>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)> {
		let mut tasks = JoinSet::new();

		let (read_tx, read_rx) = mpsc::unbounded_channel::<Message>();
		let (write_tx, write_rx) = mpsc::unbounded_channel::<Message>();

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

		#[cfg(feature = "trace-server")]
		let trace_sender = setup_trace(trace_transport, &mut tasks, &cancellation_token);

		tasks
			.build_task()
			.name("socket-lsp-reader")
			.spawn(socket_reader(
				read,
				read_tx,
				cancellation_token.clone(),
				shutdown_channel.clone(),
				#[cfg(feature = "trace-server")]
				trace_sender.clone(),
			))?;

		tasks
			.build_task()
			.name("socket-lsp-writer")
			.spawn(socket_writer(
				write,
				write_rx,
				cancellation_token,
				shutdown_channel,
				#[cfg(feature = "trace-server")]
				trace_sender,
			))?;

		Ok((read_rx, write_tx, tasks))
	}
}
