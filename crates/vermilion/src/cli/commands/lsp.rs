/* SPDX-License-Identifier: BSD-3-Clause */

use std::{
	path::PathBuf,
	sync::atomic::{AtomicUsize, Ordering},
	time::Duration,
};

use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint};
use tokio::{
	select, signal,
	sync::mpsc::{self, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, trace, warn};

use crate::settings::Config;

pub(crate) const COMMAND_NAME: &str = "lang-server";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME)
		.about("Start the Vermillion LSP server")
		.arg(
			Arg::new("pipe")
				.long("pipe")
				.help("Path to the Unix socket or named pipe for LSP communication")
				.value_hint(ValueHint::FilePath)
				.value_name("PIPE")
				.value_parser(clap::value_parser!(PathBuf))
				.action(ArgAction::Set)
				// .conflicts_with_all(["socket", "stdio"]),
		)
		// XXX(aki): For now we're only supporting LSP over PIPE,
		// .arg(
		// 	Arg::new("socket")
		// 		.long("socket")
		// 		.help("The port to connect to for the LSP server to talk to")
		// 		.value_name("PORT")
		// 		.value_parser(clap::value_parser!(u16))
		// 		.action(ArgAction::Set)
		// 		.conflicts_with_all(["pipe", "stdio"]),
		// )
		// .arg(
		// 	Arg::new("stdio")
		// 		.long("stdio")
		// 		.help("Use STDIO for LSP communication")
		// 		.action(ArgAction::SetTrue)
		// 		.conflicts_with_all(["pipe", "socket"]),
		// )
		.arg(
			Arg::new("client-pid")
				.long("clientProcessId")
				.help("The LSP client's PID")
				.value_name("PID")
				.value_parser(clap::value_parser!(usize))
				.action(ArgAction::Set),
		))
}

async fn watch_pid(
	pid: usize,
	cancellation_token: CancellationToken,
	shutdown_channel: UnboundedSender<()>,
) {
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
}

pub(crate) fn exec(args: &ArgMatches, _cfg: Config) -> eyre::Result<()> {
	// If we get passed `--clientProcessId` we want to watch for that PID to die
	let client_pid = args.try_get_one::<usize>("client-pid")?.copied();

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

		select! {
			_ = signal::ctrl_c() => {},
			_ = shutdown_recv.recv() => {},
		}

		info!("Caught shutdown signal, stopping language server");
		cancel_token.cancel();

		let _ = tasks.join_all().await;

		Ok::<(), eyre::Report>(())
	})?;

	debug!("Shutting down runtime...");
	rt.shutdown_timeout(Duration::from_secs(10));

	Ok(())
}
