// SPDX-License-Identifier: BSD-3-Clause

use clap::{ArgMatches, Command};

use crate::settings::Config;

pub(crate) const COMMAND_NAME: &str = "fmt";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Format the given HDL source files"))
}

pub(crate) fn exec(args: &ArgMatches, _cfg: Config) -> eyre::Result<()> {
	let _lang_id = crate::lang::get_langid(args);

	Ok(())
}
