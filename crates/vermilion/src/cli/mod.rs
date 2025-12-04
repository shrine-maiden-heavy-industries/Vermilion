/* SPDX-License-Identifier: BSD-3-Clause */

use clap::{
	Arg, ArgAction, ArgMatches, ColorChoice, Command, ValueHint, builder::styling, value_parser,
};
use color_print::{cformat, cstr};
use vermilion_core::vars::VERMILION_LOG_LEVEL;

use crate::settings::Config;

mod commands;

pub(crate) type CmdExec = fn(&ArgMatches, Config) -> eyre::Result<()>;

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
			<blue,bold>{0}</> is licensed under the <cyan,bold>{2}</> and is part of the <magenta>Vermilion</> \
			 project,\nwhich can be found at {1}\n\n<red,bold>For bug reporting instructions, \
			 please see</>: {1}/issues\n
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
			Arg::new("verbose")
				.short('v')
				.action(ArgAction::Count)
				.help("Enable verbose logging")
				.long_help(cformat!(
				"Enable verbose logging\n\n Repeat this option multiple times for increasingly \
				 verbose output.\n\n This setting is overridden by the <magenta>{}</> \
				 environment variable and also yields to the <blue>-q</> flag for quiet \
				 operation.",
				VERMILION_LOG_LEVEL
			)),
		)
		.arg(
			Arg::new("quiet")
				.short('q')
				.action(ArgAction::SetTrue)
				.help("Disable logging")
				.long_help(cformat!(
					"Disable logging\n\nDisable all output {}",
					VERMILION_LOG_LEVEL,
				)),
		).arg(
			Arg::new("config")
				.short('c')
				.long("config")
				.help("The specific configuration file to use")
				.action(ArgAction::Set)
				.value_hint(ValueHint::FilePath)
				.value_name("CONFIG_FILE"),
		).arg(
			Arg::new("dump-schema")
				.long("dump-schema")
				.action(ArgAction::SetTrue)
				.help("Dump the Vermilion configuration schema to stdout")
				.hide_short_help(true)
		).arg(
			Arg::new("dump-config")
				.long("dump-config")
				.action(ArgAction::SetTrue)
				.help("Dump the default Vermilion configuration to stdout")
				.hide_short_help(true)
		).arg(
			Arg::new("dump-completions")
				.long("dump-completions")
				.action(ArgAction::Set)
				.help("Dump shell completion file for the given shell to stdout")
				.hide_short_help(true)
				.value_parser(value_parser!(clap_complete::Shell))
				.value_name("SHELL")
		).subcommands(init_commands()?))
}

fn init_commands() -> eyre::Result<Vec<Command>> {
	Ok(vec![
		commands::explain::init()?,
		lang_common(commands::fmt::init()?, true),
		lang_common(commands::lint::init()?, true),
		#[cfg(debug_assertions)]
		lang_common(commands::dbg::init()?, false),
		commands::lsp::init()?,
	])
}

pub(crate) fn exec_command(command: &str) -> Option<CmdExec> {
	match command {
		commands::explain::COMMAND_NAME => Some(commands::explain::exec),
		commands::fmt::COMMAND_NAME => Some(commands::fmt::exec),
		commands::lint::COMMAND_NAME => Some(commands::lint::exec),
		commands::lsp::COMMAND_NAME => Some(commands::lsp::exec),
		#[cfg(debug_assertions)]
		commands::dbg::COMMAND_NAME => Some(commands::dbg::exec),
		_ => None,
	}
}

fn lang_common(command: Command, with_files: bool) -> Command {
	let cmd = command
		.arg(
			Arg::new("std")
				.long("std")
				.value_parser([
					"vl95", "vl01", "vl05",
					"sv05", "sv09", "sv12", "sv17", "sv23",
					"vams09", "vams14", "vams23",
					"vh87", "vh93", "vh2k", "vh02", "vh07", "vh08", "vh11", "vh19", "vh23",
				])
				.help("Set the language standard to use")
				.long_help(cstr!(
					"\n\
				 --std=vl95\n\
				 \tSet Language and standard to <magenta>Verilog</> 1995 (<blue>IEEE</> 1364-1995)\n\n\
				 --std=vl01\n\
				 \tSet Language and standard to <magenta>Verilog</> 2001 (<blue>IEEE</> 1364-2001)\n\n\
				 --std=vl05\n\
				 \tSet Language and standard to <magenta>Verilog</> 2005 (<blue>IEEE</> 1364-2005)\n\n\
				 --std=sv05\n\
				 \tSet Language and standard to <cyan>SystemVerilog</> 2005 (<blue>IEEE</> 1800-2005)\n\n\
				 --std=sv09\n\
				 \tSet Language and standard to <cyan>SystemVerilog</> 2009 (<blue>IEEE</> 1800-2009)\n\n\
				 --std=sv12\n\
				 \tSet Language and standard to <cyan>SystemVerilog</> 2012 (<blue>IEEE</> 1800-2012)\n\n\
				 --std=sv17\n\
				 \tSet Language and standard to <cyan>SystemVerilog</> 2017 (<blue>IEEE</> 1800-2017)\n\n\
				 --std=sv23\n\
				 \tSet Language and standard to <cyan>SystemVerilog</> 2023 (<blue>IEEE</> 1800-2023)\n\n\
				 --std=vams09\n\
				 \tSet Language and standard to <yellow>Verilog-AMS</> 2009 (<yellow>Accellera</> Verilog-AMS 2.3.1)\n\n\
				 --std=vams14\n\
				 \tSet Language and standard to <yellow>Verilog-AMS</> 2014 (<yellow>Accellera</> Verilog-AMS 2.4)\n\n\
				 --std=vams23\n\
				 \tSet Language and standard to <yellow>Verilog-AMS</> 2023 (<yellow>Accellera</> Verilog-AMS 2023)\n\n\
				 --std=vh87\n\
				 \tSet Language and standard to <green>VHDL</> 1987 (<blue>IEEE</> 1076-1987)\n\n\
				 --std=vh93\n\
				 \tSet Language and standard to <green>VHDL</> 1993 (<blue>IEEE</> 1076-1993)\n\n\
				 --std=vh2k\n\
				 \tSet Language and standard to <green>VHDL</> 2000 (<blue>IEEE</> 1076-2000)\n\n\
				 --std=vh02\n\
				 \tSet Language and standard to <green>VHDL</> 2002 (<blue>IEEE</> 1076-2002)\n\n\
				 --std=vh07\n\
				 \tSet Language and standard to <green>VHDL</> 2007 (<blue>IEEE</> 1076-2007)\n\n\
				 --std=vh08\n\
				 \tSet Language and standard to <green>VHDL</> 2008 (<blue>IEEE</> 1076-2008)\n\n\
				 --std=vh11\n\
				 \tSet Language and standard to <green>VHDL</> 2011 (<blue>IEEE</> 1076-2011)\n\n\
				 --std=vh19\n\
				 \tSet Language and standard to <green>VHDL</> 2019 (<blue>IEEE</> 1076-2019)\n\n\
				 --std=vh23\n\
				 \tSet Language and standard to <green>VHDL</> 2023 (<blue>IEEE</> 1076-2023)\n\
				"
				))
				.hide_possible_values(true)
				// .required(true)
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
