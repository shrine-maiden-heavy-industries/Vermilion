/* SPDX-License-Identifier: BSD-3-Clause */

use super::LSPTransport;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win;

pub(super) struct PipeTransport {}

impl LSPTransport for PipeTransport {}
