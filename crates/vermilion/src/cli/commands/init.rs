// SPDX-License-Identifier: BSD-3-Clause

use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint};

pub(crate) const COMMAND_NAME: &str = "init";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME)
		.about("Initialize vermilion in an existing HDL project or create a new project")
		// .arg(
		// 	Arg::new("name")
		// 		.long("name")
		// 		.help("The name of the project")
		// 		.value_name("NAME")
		// 		.action(ArgAction::Set),
		// )
		.arg(
			Arg::new("path")
				.help("The path of the project")
				.action(ArgAction::Set)
				.value_name("PATH")
				.value_hint(ValueHint::DirPath)
		))
}

pub(crate) fn exec(_args: &ArgMatches) -> eyre::Result<()> {
	todo!("TODO(aki): Actually figure out how we want this to work")
}
