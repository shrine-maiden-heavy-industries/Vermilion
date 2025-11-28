/* SPDX-License-Identifier: BSD-3-Clause */

use std::{
	sync::atomic::{AtomicUsize, Ordering},
	time::Duration,
};

use clap::{ArgMatches, Command};
use tracing::debug;

use crate::settings::Config;

pub(crate) const COMMAND_NAME: &str = "lang-server";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Start the Vermillion LSP server"))
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
