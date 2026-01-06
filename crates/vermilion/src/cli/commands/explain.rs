// SPDX-License-Identifier: BSD-3-Clause

use clap::{Arg, ArgAction, ArgMatches, Command};

pub(crate) const COMMAND_NAME: &str = "explain";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME)
		.about("Explain a diagnostics code")
		.arg(
			Arg::new("diagnostic")
				.help("Diagnostic codes to explain")
				.action(ArgAction::Append)
				.value_name("CODE")
				.value_delimiter(','),
		))
}

pub(crate) fn exec(_args: &ArgMatches) -> eyre::Result<()> {
	Ok(())
}
