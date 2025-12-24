/* SPDX-License-Identifier: BSD-3-Clause */

use std::path::PathBuf;

use eyre::Result;
use tokio::{
	sync::mpsc::{UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use vermilion_lsp::message::Message;

#[cfg_attr(unix, path = "pipe_unix.rs")]
#[cfg_attr(windows, path = "pipe_win.rs")]
pub(crate) mod pipe;
pub(crate) mod socket;
pub(crate) mod stdio;

pub(crate) enum TransportType {
	Stdio,
	Socket(u16),
	Pipe(PathBuf),
}

pub(super) trait LSPTransport: Sized {
	async fn create(
		self,
		cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)>;
}
