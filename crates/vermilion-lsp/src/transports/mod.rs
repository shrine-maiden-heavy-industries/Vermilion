// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

use eyre::Result;
use tokio::{
	sync::mpsc::{UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;

use super::message::Message;

#[cfg_attr(unix, path = "pipe_unix.rs")]
#[cfg_attr(windows, path = "pipe_win.rs")]
pub mod pipe;
pub mod socket;
pub mod stdio;

enum ReadPhase {
	Header,
	Content(usize),
}

fn get_split_index(buffer: &[u8]) -> Option<usize> {
	buffer
		.windows(4)
		.enumerate()
		.find(|&(_, w)| matches!(w, b"\r\n\r\n"))
		.map(|(idx, _)| idx + 4)
}

pub enum TransportType {
	Stdio,
	Socket(u16),
	Pipe(PathBuf),
}

pub trait LSPTransport: Sized {
	#[allow(async_fn_in_trait)]
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
