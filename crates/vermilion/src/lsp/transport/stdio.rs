/* SPDX-License-Identifier: BSD-3-Clause */

use eyre::Result;
use tokio::{
	sync::mpsc::{UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use vermilion_lsp::message::Message;

use super::LSPTransport;

#[derive(Debug)]
pub(crate) struct StdioTransport {}

impl StdioTransport {
	pub const fn new() -> Self {
		Self {}
	}
}

impl LSPTransport for StdioTransport {
	async fn create(
		self,
		_cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)> {
		shutdown_channel.send(())?;
		unimplemented!("LSP stdio transport not implemented");
	}
}
