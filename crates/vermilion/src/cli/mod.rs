// SPDX-License-Identifier: BSD-3-Clause

use clap::{
	Arg, ArgAction, ArgMatches, ColorChoice, Command, ValueHint, builder::styling, value_parser,
};
use color_print::cformat;

use crate::{env::VERMILION_LOG_LEVEL, lang::Language};

mod commands;

pub(crate) type CmdExec = fn(&ArgMatches) -> eyre::Result<()>;

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.before_help(cformat!(
			"<blue>{}</> v{}",
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_VERSION")
		))
		.after_help(cformat!(
			"\
			<blue,bold>{0}</> is licensed under the <cyan,bold>{2}</> and is part of the \
			 <magenta>Vermilion</> project,\nwhich can be found at {1}\n\n<red,bold>For bug \
			 reporting instructions, please see</>: {1}/issues\n
			",
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_REPOSITORY"),
			env!("CARGO_PKG_LICENSE")
		))
		.color(ColorChoice::Auto)
		.styles(
			styling::Styles::styled()
				.header(styling::AnsiColor::Magenta.on_default().bold())
				.usage(styling::AnsiColor::Cyan.on_default().bold())
				.literal(styling::AnsiColor::Blue.on_default().bold())
				.placeholder(styling::AnsiColor::Green.on_default().bold())
				.error(styling::AnsiColor::Red.on_default().bold())
				.invalid(styling::AnsiColor::Red.on_default().bold()),
		)
		.arg(
			Arg::new("color")
				.long("color")
				.action(ArgAction::Set)
				.help("Control if color is enabled or disabled")
				.value_parser(value_parser!(ColorChoice))
				.value_name("WHEN")
				.default_value("auto"),
		)
		.arg(
			Arg::new("verbose")
				.short('v')
				.action(ArgAction::Count)
				.help("Enable verbose logging")
				.long_help(cformat!(
					"Enable verbose logging\n\nRepeat this option multiple times for increasingly \
					 verbose output.\n\nThis setting is overridden by the <magenta>{}</> \
					 environment variable and also yields to the <blue>-q</> flag for quiet \
					 operation.\n\nWhen this flag is set to the highest tracing level (e.g. any \
					 more than 2) the \nTokio console subscriber is also enabled for debugging \
					 the async runtime with the <green>tokio-consol</> command.",
					VERMILION_LOG_LEVEL
				)),
		)
		.arg(
			Arg::new("quiet")
				.short('q')
				.action(ArgAction::SetTrue)
				.help("Disable all non-critical logging")
				.long_help(cformat!(
					"Disable all non-critical logging\n\nDisable all output unless it is a \
					 warning or error.\n\nThis setting is overridden by the <magenta>{}</> \
					 environment variable and overrides the <blue>-v</> flag if set.
					",
					VERMILION_LOG_LEVEL,
				)),
		)
		.arg(
			Arg::new("workspace")
				.long("workspace")
				.help("Specify a workspace configuration file")
				.long_help("")
				.action(ArgAction::Set)
				.value_hint(ValueHint::FilePath)
				.value_name("WORKSPACE_FILE"),
		)
		.arg(
			Arg::new("dump-schema")
				.long("dump-schema")
				.action(ArgAction::SetTrue)
				.help("Dump the Vermilion configuration schema to stdout")
				.hide_short_help(true),
		)
		.arg(
			Arg::new("dump-config")
				.long("dump-config")
				.action(ArgAction::SetTrue)
				.help("Dump the default Vermilion configuration to stdout")
				.hide_short_help(true),
		)
		.arg(
			Arg::new("dump-completions")
				.long("dump-completions")
				.action(ArgAction::Set)
				.help("Dump shell completion file for the given shell to stdout")
				.hide_short_help(true)
				.value_parser(value_parser!(clap_complete::Shell))
				.value_name("SHELL"),
		)
		.subcommands(init_commands()?))
}

fn init_commands() -> eyre::Result<Vec<Command>> {
	Ok(vec![
		#[cfg(debug_assertions)]
		lang_common(commands::dbg::init()?, false),
		commands::explain::init()?,
		commands::init::init()?,
		lang_common(commands::fmt::init()?, true),
		lang_common(commands::lint::init()?, true),
		commands::lsp::init()?,
	])
}

pub(crate) fn exec_command(command: &str) -> Option<CmdExec> {
	match command {
		#[cfg(debug_assertions)]
		commands::dbg::COMMAND_NAME => Some(commands::dbg::exec),
		commands::explain::COMMAND_NAME => Some(commands::explain::exec),
		commands::fmt::COMMAND_NAME => Some(commands::fmt::exec),
		commands::init::COMMAND_NAME => Some(commands::init::exec),
		commands::lint::COMMAND_NAME => Some(commands::lint::exec),
		commands::lsp::COMMAND_NAME => Some(commands::lsp::exec),
		_ => None,
	}
}

fn lang_common(command: Command, with_files: bool) -> Command {
	let cmd = command
		.arg(
			Arg::new("lang-std")
				.long("std")
				.value_parser(value_parser!(Language))
				.help("Set the language standard to use")
				.value_name("STD"),
		)
		.arg(
			Arg::new("define")
				.short('D')
				.long("define")
				.help("Define a given compiler macro")
				.action(ArgAction::Append)
				.value_name("NAME"),
		)
		.arg(
			Arg::new("undefine")
				.short('U')
				.long("undefine")
				.help("Undefine the provided compiler macro")
				.action(ArgAction::Append)
				.value_name("NAME"),
		);

	if with_files {
		cmd.arg(
			Arg::new("files")
				.help("HDL files")
				.action(ArgAction::Append)
				.value_name("FILE")
				.value_delimiter(',')
				.value_hint(ValueHint::FilePath)
				.required(true),
		)
	} else {
		cmd
	}
}
