// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

#[cfg(feature = "trace-server")]
use tokio::{
	fs::OpenOptions,
	io::AsyncWriteExt,
	select,
	sync::mpsc::{self, UnboundedReceiver},
};
use tokio::{sync::mpsc::UnboundedSender, task::JoinSet};
use tokio_util::sync::CancellationToken;
#[cfg(feature = "trace-server")]
use tracing::debug;

use crate::trace::Trace;

#[cfg(feature = "trace-server")]
pub(crate) async fn trace_writer(
	mut trace_channel: UnboundedReceiver<Trace>,
	cancellation_token: CancellationToken,
	trace_file: PathBuf,
) -> eyre::Result<()> {
	let mut file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(trace_file)
		.await?;

	debug!("Starting LSP message logging");

	loop {
		select! {
			_ = cancellation_token.cancelled() => { break; },
			Some(trace_message) = trace_channel.recv() => {
				let msg = serde_json::to_string(&trace_message)?;
				file.write_all(msg.as_bytes()).await?;
				file.write_u8(b'\n').await?;
			},
		}
	}

	Ok(())
}

#[cfg(feature = "trace-server")]
pub(crate) fn setup_trace(
	trace_file: Option<PathBuf>,
	tasks: &mut JoinSet<eyre::Result<()>>,
	cancellation_token: &CancellationToken,
) -> Option<UnboundedSender<Trace>> {
	if let Some(trace_file) = trace_file {
		debug!("LSP Trace file specified: {:?}", trace_file);

		let (trace_tx, trace_rx) = mpsc::unbounded_channel::<Trace>();
		tasks
			.build_task()
			.name("lsp-tracer")
			.spawn(trace_writer(
				trace_rx,
				cancellation_token.clone(),
				trace_file,
			))
			.ok()?;
		Some(trace_tx)
	} else {
		None
	}
}

#[cfg(not(feature = "trace-server"))]
pub(crate) fn setup_trace(
	_trace_file: Option<PathBuf>,
	_tasks: &mut JoinSet<eyre::Result<()>>,
	_cancellation_token: &CancellationToken,
) -> Option<UnboundedSender<Trace>> {
	None
}
