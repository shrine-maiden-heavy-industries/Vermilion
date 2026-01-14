// SPDX-License-Identifier: BSD-3-Clause

use clap::{ArgMatches, Command};
use eyre::OptionExt;

use crate::lang::Language;

pub(crate) const COMMAND_NAME: &str = "fmt";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME).about("Format the given HDL source files"))
}

pub(crate) fn exec(args: &ArgMatches) -> eyre::Result<()> {
	let _workspace_config = crate::workspace::load_workspace_config(args)?;
	let _lang = args
		.try_get_one::<Language>("lang-std")?
		.cloned()
		.ok_or_eyre("Language standard not specified")?;

	Ok(())
}
