/* SPDX-License-Identifier: BSD-3-Clause */

use std::path::PathBuf;

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

pub(super) trait LSPTransport {
	async fn connect(&mut self) -> eyre::Result<()>;
	async fn ready(&mut self) -> eyre::Result<()>;
	async fn close(&mut self) -> eyre::Result<()>;

	async fn read(&mut self, buffer: &mut [u8]) -> eyre::Result<usize>;
	async fn write(&mut self, buffer: &mut [u8]) -> eyre::Result<usize>;
}
