// SPDX-License-Identifier: BSD-3-Clause

use std::{
	net::{SocketAddr, ToSocketAddrs},
	path::PathBuf,
	time::Duration,
};

use eyre::OptionExt;
use tokio::{
	fs::OpenOptions,
	io::AsyncWriteExt,
	net::TcpStream,
	select,
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinSet,
	time,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

use crate::trace::Trace;

pub enum TraceTransport {
	File(PathBuf),
	Socket(SocketAddr),
}

impl TraceTransport {
	pub fn socket<A: ToSocketAddrs>(addr: A) -> eyre::Result<Self> {
		Ok(Self::Socket(
			addr.to_socket_addrs()?
				.next()
				.ok_or_eyre("No Socket Address")?,
		))
	}
}

async fn trace_file_writer(
	mut trace_channel: UnboundedReceiver<Trace>,
	cancellation_token: CancellationToken,
	trace_file: PathBuf,
) -> eyre::Result<()> {
	debug!("Logging LSP messages to {:?}", trace_file);

	let mut file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(trace_file)
		.await?;

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

	// Drain the trace channel having taken a cancellation of some sort
	while !trace_channel.is_empty() {
		if let Some(trace_message) = trace_channel.recv().await {
			let msg = serde_json::to_string(&trace_message)?;
			file.write_all(msg.as_bytes()).await?;
			file.write_u8(b',').await?;
			file.write_u8(b'\n').await?;
		}
	}
	file.write_u8(b']').await?;

	Ok(())
}

async fn trace_socket_writer(
	mut trace_channel: UnboundedReceiver<Trace>,
	cancellation_token: CancellationToken,
	trace_addr: SocketAddr,
) -> eyre::Result<()> {
	debug!("Logging LSP messages to {:?}", trace_addr);
	let mut stream =
		match time::timeout(Duration::from_secs(15), TcpStream::connect(trace_addr)).await {
			Ok(res) => res,
			Err(e) => {
				error!("Connection timeout for LSP socket");
				return Err(e.into());
			},
		}?;

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(trace_message) = trace_channel.recv() => {
				let msg = serde_json::to_string(&trace_message)?;
				let msg = msg.as_bytes();
				stream.write_u8(b'\x7F').await?;
				stream.write_u64(msg.len() as u64).await?;
				stream.write_all(msg).await?;
				stream.write_u8(b'\xFF').await?;
			}
		}
	}

	Ok(())
}

pub(crate) fn setup_trace(
	trace_transport: Option<TraceTransport>,
	tasks: &mut JoinSet<eyre::Result<()>>,
	cancellation_token: &CancellationToken,
) -> Option<UnboundedSender<Trace>> {
	if let Some(trace_transport) = trace_transport {
		let (trace_tx, trace_rx) = mpsc::unbounded_channel::<Trace>();
		match trace_transport {
			TraceTransport::File(path) => {
				tasks
					.build_task()
					.name("lsp-tracer")
					.spawn(trace_file_writer(
						trace_rx,
						cancellation_token.clone(),
						path,
					))
			},
			TraceTransport::Socket(addr) => {
				tasks
					.build_task()
					.name("lsp-tracer")
					.spawn(trace_socket_writer(
						trace_rx,
						cancellation_token.clone(),
						addr,
					))
			},
		}
		.ok()?;
		Some(trace_tx)
	} else {
		None
	}
}
