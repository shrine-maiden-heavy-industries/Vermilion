/* SPDX-License-Identifier: BSD-3-Clause */

use eyre::Result;

use super::LSPTransport;

pub(crate) struct StdioTransport {}

impl StdioTransport {
	pub const fn new() -> Self {
		Self {}
	}
}

impl LSPTransport for StdioTransport {
	async fn connect(&mut self) -> Result<()> {
		// stdin/stdout *should* always be connected
		Ok(())
	}

	async fn ready(&mut self) -> Result<()> {
		// stdin/stdout *should* always be ready
		Ok(())
	}

	async fn close(&mut self) -> Result<()> {
		// stdin/stdout close when program exits
		Ok(())
	}

	async fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		todo!()
	}

	async fn write(&mut self, buffer: &mut [u8]) -> Result<usize> {
		todo!()
	}
}
