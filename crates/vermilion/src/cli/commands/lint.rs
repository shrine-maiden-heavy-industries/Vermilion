// SPDX-License-Identifier: BSD-3-Clause

use clap::{ArgMatches, Command};

use crate::settings::Config;

pub(crate) const COMMAND_NAME: &str = "lint";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Lint the provided HDL source files"))
}

pub(crate) fn exec(args: &ArgMatches, _cfg: Config) -> eyre::Result<()> {
	let _lang_id = crate::lang::get_langid(args);

	Ok(())
}
