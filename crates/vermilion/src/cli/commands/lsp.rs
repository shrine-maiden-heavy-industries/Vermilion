/* SPDX-License-Identifier: BSD-3-Clause */

use clap::{ArgMatches, Command};

use crate::settings::Config;

pub(crate) const COMMAND_NAME: &str = "lang-server";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Start the Vermillion LSP server"))
}

pub(crate) fn exec(_args: &ArgMatches, _cfg: Config) -> eyre::Result<()> {
	Ok(())
}
