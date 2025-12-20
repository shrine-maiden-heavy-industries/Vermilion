/* SPDX-License-Identifier: BSD-3-Clause */

use std::path::PathBuf;

use eyre::{Result, eyre};
use tokio::{io::Interest, net::UnixStream};

use super::LSPTransport;

pub(crate) struct PipeTransport {
	path: PathBuf,
	stream: Option<UnixStream>,
}

impl PipeTransport {
	pub fn new(path: PathBuf) -> Self {
		Self { path, stream: None }
	}
}

impl LSPTransport for PipeTransport {
	async fn connect(&mut self) -> Result<()> {
		self.stream = Some(UnixStream::connect(self.path.clone()).await?);
		Ok(())
	}

	async fn ready(&mut self) -> Result<()> {
		if let Some(stream) = self.stream.as_ref() {
			stream
				.ready(Interest::READABLE | Interest::WRITABLE)
				.await?;

			Ok(())
		} else {
			Err(eyre!("No Socket!"))
		}
	}

	async fn close(&mut self) -> Result<()> {
		todo!()
	}

	async fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		if let Some(stream) = self.stream.as_ref() {
			return Ok(stream.try_read(buffer)?);
		}
		Err(eyre!(""))
	}

	async fn write(&mut self, buffer: &mut [u8]) -> Result<usize> {
		if let Some(stream) = self.stream.as_ref() {
			return Ok(stream.try_write(buffer)?);
		}
		Err(eyre!(""))
	}
}
