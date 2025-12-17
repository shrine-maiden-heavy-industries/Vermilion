/* SPDX-License-Identifier: BSD-3-Clause */

use std::path::PathBuf;

pub(crate) mod pipe;
pub(crate) mod socket;
pub(crate) mod stdio;

pub(crate) enum TransportType {
	Stdio,
	Socket(u16),
	Pipe(PathBuf),
}
