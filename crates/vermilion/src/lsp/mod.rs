/* SPDX-License-Identifier: BSD-3-Clause */

use std::{
	sync::atomic::{AtomicUsize, Ordering},
	time::Duration,
};

use vermilion_lsp::prelude::{Message, Notification, Request, Response};

use tokio::{
	select, signal,
	sync::mpsc::{self, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, trace, warn};

pub(crate) mod transport;
pub(crate) use transport::{TransportType, pipe::PipeTransport};

use crate::lsp::transport::{LSPTransport, socket::SocketTransport, stdio::StdioTransport};

pub(crate) fn start(transport: TransportType, client_pid: Option<usize>) -> eyre::Result<()> {
	debug!("Starting runtime...");
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(4)
		.thread_name_fn(|| {
			static WORKER_POOL_ID: AtomicUsize = AtomicUsize::new(0);
			let pool_id = WORKER_POOL_ID.fetch_add(1, Ordering::SeqCst);
			format!("vermilion-worker-{pool_id}")
		})
		.on_thread_start(|| {
			trace!("Starting tokio worker thread");
		})
		.build()?;

	info!("Starting Vermilion Language Server");
	rt.block_on(async {
		let mut tasks = JoinSet::new();
		let cancel_token = CancellationToken::new();
		let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<()>();

		if let Some(client_pid) = client_pid {
			debug!("Client gave us their PID, spawning watcher task");
			tasks.build_task().name("client-watcher").spawn(watch_pid(
				client_pid,
				cancel_token.clone(),
				shutdown_send.clone(),
			))?;
		}

		debug!("Starting LSP server task");
		tasks.build_task().name("lsp-server").spawn(lsp_server(
			transport,
			cancel_token.clone(),
			shutdown_send.clone(),
		))?;

		select! {
			_ = signal::ctrl_c() => {},
			_ = shutdown_recv.recv() => {},
		}

		info!("Caught shutdown signal, stopping language server");
		cancel_token.cancel();

		select! {
			_ = tasks.join_all() => {},
			_ = tokio::time::sleep(Duration::from_secs(15)) => {
				warn!("Tasks did not all join! Forcing shutdown");
			}
		}

		Ok::<(), eyre::Report>(())
	})?;

	debug!("Shutting down runtime...");
	rt.shutdown_timeout(Duration::from_secs(10));

	Ok(())
}

pub fn process_lsp_request(
	request: Box<Request>,
	response_channel: &UnboundedSender<Message>,
	shutdown_channel: &UnboundedSender<()>,
) -> eyre::Result<()> {
	match request.req {
		RequestType::Initialize(params) => {
		},
		_ => unimplemented!(),
	}

	Ok(())
}

pub fn process_lsp_response(
	response: Response,
	response_channel: &UnboundedSender<Message>,
	shutdown_channel: &UnboundedSender<()>,
) -> eyre::Result<()> {
	Ok(())
}

pub fn process_lsp_notification(
	notification: Notification,
	response_channel: &UnboundedSender<Message>,
	shutdown_channel: &UnboundedSender<()>,
) -> eyre::Result<()> {
	match notification {
		Notification::Exit => {
			info!("Shutting down LSP");
			shutdown_channel.send(())?;
		},
		Notification::Initialized(_) => {
			debug!("LSP Initialized");
			LSP_INITIALIZED.store(true, Ordering::Release);
		},
		_ => unimplemented!(),
	}
	Ok(())
}

fn process_lsp_message(
	message: Message,
	response_channel: &UnboundedSender<Message>,
	shutdown_channel: &UnboundedSender<()>,
) -> eyre::Result<()> {
	// Check to see if we have been initialized yet, if not we need to error out
	let initialized = LSP_INITIALIZED.load(Ordering::Acquire);
	if !initialized && !message.is_initialize().unwrap_or(false) {
		warn!("LSP is not Initialized but a request other than `Initialize` was received");
		return Ok(());
	}

	match message {
		Message::Request(request) => {
			process_lsp_request(request, response_channel, shutdown_channel)
		},
		Message::Response(response) => {
			process_lsp_response(response, response_channel, shutdown_channel)
		},
		Message::Notification(notification) => {
			process_lsp_notification(notification, response_channel, shutdown_channel)
		},
	}
}

async fn lsp_server(
	transport: TransportType,
	cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
) -> eyre::Result<()> {
	let (mut reader, writer, tasks) = match transport {
		TransportType::Stdio => {
			StdioTransport::new()
				.create(cancellation_token.clone(), shutdown_channel.clone())
				.await?
		},
		TransportType::Socket(port) => {
			SocketTransport::new(port)
				.create(cancellation_token.clone(), shutdown_channel.clone())
				.await?
		},
		TransportType::Pipe(path) => {
			PipeTransport::new(path)
				.create(cancellation_token.clone(), shutdown_channel.clone())
				.await?
		},
	};

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(message) = reader.recv() => {
				process_lsp_message(message, &writer, &shutdown_channel)?;
			},
		}
	}

	debug!("LSP Server shutting down");
	// If we're not explicitly cancelled and we hit this point,
	// we need to tell everything else to shutdown
	if !cancellation_token.is_cancelled() {
		let _ = shutdown_channel.send(());
	}

	select! {
		_ = tasks.join_all() => {},
		_ = tokio::time::sleep(Duration::from_secs(15)) => {
			warn!("Tasks did not all join! Forcing shutdown");
		}
	}

	Ok(())
}

async fn watch_pid(
	pid: usize,
	cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
) -> eyre::Result<()> {
	let pid = sysinfo::Pid::from(pid);
	let mut sys = sysinfo::System::new_with_specifics(
		sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing()),
	);

	debug!("Watching PID {}", pid);
	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			// TODO(aki): should we let the user tune this?
			_ = tokio::time::sleep(Duration::from_secs(15)) => {
				sys.refresh_all();

				if !sys.processes().contains_key(&pid) {
					warn!("Client process died, exiting...");
					let _ = shutdown_channel.send(());
					break;
				}
			}
		}
	}

	Ok(())
}
