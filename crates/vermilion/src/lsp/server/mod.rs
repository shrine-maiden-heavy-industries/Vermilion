/* SPDX-License-Identifier: BSD-3-Clause */

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "win.rs")]
mod pipe;
