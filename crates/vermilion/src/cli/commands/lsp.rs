/* SPDX-License-Identifier: BSD-3-Clause */

use std::{
	path::PathBuf,
	sync::atomic::{AtomicUsize, Ordering},
	time::Duration,
};

use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint};

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

pub(crate) fn exec(_args: &ArgMatches, _cfg: Config) -> eyre::Result<()> {
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
			debug!("Starting tokio worker thread");
		})
		.build()?;

	debug!("Shutting down runtime...");
	rt.shutdown_timeout(Duration::from_secs(10));

	Ok(())
}
