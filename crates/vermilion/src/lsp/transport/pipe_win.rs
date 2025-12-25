/* SPDX-License-Identifier: BSD-3-Clause */

use eyre::{Result, eyre};
use tokio::{
	sync::mpsc::{UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use vermilion_lsp::message::Message;

use super::LSPTransport;

#[derive(Debug)]
pub(crate) struct PipeTransport {}

impl LSPTransport for PipeTransport {
	async fn create(
		self,
		cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)> {
		shutdown_channel.send(())?;

		unimplemented!("LSP pipe transport for windows not implemented");

		Err(eyre!("あああああああああああ"))
	}
}
