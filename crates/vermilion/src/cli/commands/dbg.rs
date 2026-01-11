// SPDX-License-Identifier: BSD-3-Clause

use std::{fs::File, io::Read};

use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::{OptionExt, eyre};

use crate::lang::Language;

pub(crate) const COMMAND_NAME: &str = "dbg";

fn subcommands() -> Vec<Command> {
	vec![
		Command::new("dump-tokens")
			.about("Dump the token stream from the given file to stdout")
			.arg(
				Arg::new("files")
					.help("HDL files")
					.action(ArgAction::Append)
					.value_name("FILE")
					.value_delimiter(',')
					.required(true),
			),
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

pub(crate) fn exec(args: &ArgMatches) -> eyre::Result<()> {
	let lang_id = crate::lang::get_langid(args).ok_or_eyre("Unable to get language id")?;

	match args.subcommand() {
		Some((cmd, cmd_args)) => match cmd {
			"dump-ast" => dump_ast(cmd_args, lang_id),
			"dump-tokens" => dump_tokens(cmd_args, lang_id),
			_ => unreachable!(),
		},
		_ => Err(eyre!("No subcommand")),
	}
}

fn dump_ast(_args: &ArgMatches, _language: Language) -> eyre::Result<()> {
	Ok(())
}

fn dump_tokens(args: &ArgMatches, language: Language) -> eyre::Result<()> {
	fn dump_iter<I: Iterator>(itr: I)
	where
		<I as std::iter::Iterator>::Item: std::fmt::Display,
	{
		for tok in itr {
			println!("{tok}");
		}
	}

	if let Some(files) = args.try_get_many::<String>("files")? {
		for file in files {
			let mut data = Vec::new();
			let mut hdl_file = File::open(file)?;
			hdl_file.read_to_end(&mut data)?;

			match language.tokenizer(data.as_slice().into()) {
				crate::lang::Tokenizer::Verilog(tokenizer) => dump_iter(tokenizer),
				crate::lang::Tokenizer::Vhdl(tokenizer) => dump_iter(tokenizer),
			}
		}
	}

	Ok(())
}
