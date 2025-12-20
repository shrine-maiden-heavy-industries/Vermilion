/* SPDX-License-Identifier: BSD-3-Clause */

use eyre::Result;

use super::LSPTransport;

pub(crate) struct PipeTransport {}

impl LSPTransport for PipeTransport {
	async fn connect(&mut self) -> Result<()> {
		todo!()
	}

	async fn ready(&mut self) -> Result<()> {
		todo!()
	}

	async fn close(&mut self) -> Result<()> {
		todo!()
	}

	async fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		todo!()
	}

	async fn write(&mut self, buffer: &mut [u8]) -> Result<usize> {
		todo!()
	}
}
