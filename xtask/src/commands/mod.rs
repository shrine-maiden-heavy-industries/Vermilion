// SPDX-License-Identifier: BSD-3-Clause

use clap::{ArgMatches, Command};
use eyre::Result;

pub type CmdExec = fn(&ArgMatches) -> Result<()>;

pub(crate) fn init() -> Result<Vec<Command>> {
	Ok(vec![])
}

pub(crate) fn exec(command: &str) -> Option<CmdExec> {
	match command {
		_ => None,
	}
}
