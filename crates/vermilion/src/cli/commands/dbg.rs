/* SPDX-License-Identifier: BSD-3-Clause */

use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::settings::Config;

pub(crate) const COMMAND_NAME: &str = "dbg";

fn subcommands() -> Vec<Command> {
	vec![
		Command::new("dump-ast")
			.about("Dump a pretty-printed version of the AST to the command line")
			.arg(
				Arg::new("files")
					.help("HDL files")
					.action(ArgAction::Append)
					.value_name("FILE")
					.value_delimiter(',')
					.required(true),
			),
	]
}

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME)
		.about("Vermilion debugging commands")
		.subcommands(subcommands()))
}

pub(crate) fn exec(args: &ArgMatches, _cfg: Config) -> eyre::Result<()> {
	let _lang_id = crate::lang::get_langid(args);

	Ok(())
}
