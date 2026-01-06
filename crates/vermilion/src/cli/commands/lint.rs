// SPDX-License-Identifier: BSD-3-Clause

use clap::{ArgMatches, Command};

pub(crate) const COMMAND_NAME: &str = "lint";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Lint the provided HDL source files"))
}

pub(crate) fn exec(args: &ArgMatches) -> eyre::Result<()> {
	let _lang_id = crate::lang::get_langid(args);

	Ok(())
}
