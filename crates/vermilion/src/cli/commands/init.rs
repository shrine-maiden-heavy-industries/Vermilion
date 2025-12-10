/* SPDX-License-Identifier: BSD-3-Clause */

use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::settings::Config;

pub(crate) const COMMAND_NAME: &str = "init";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Initialize a new Vermilion project or config"))
}

pub(crate) fn exec(_args: &ArgMatches, _cfg: Config) -> eyre::Result<()> {
	Ok(())
}
