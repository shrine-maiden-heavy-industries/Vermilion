// SPDX-License-Identifier: BSD-3-Clause

use clap::{ArgMatches, Command};

pub(crate) const COMMAND_NAME: &str = "fmt";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Format the given HDL source files"))
}

pub(crate) fn exec(args: &ArgMatches) -> eyre::Result<()> {
	let _lang_id = crate::lang::get_langid(args);

	Ok(())
}
