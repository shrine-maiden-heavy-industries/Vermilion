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

pub(super) trait LSPTransport {}
