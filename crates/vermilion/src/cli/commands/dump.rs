// SPDX-License-Identifier: BSD-3-Clause

use std::{fs, io::Write};

use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint, value_parser};
use eyre::eyre;
use schemars::schema_for;

pub(crate) const COMMAND_NAME: &str = "dump";

fn subcommands() -> Vec<Command> {
	vec![
		Command::new("schema")
			.about("Dump the Vermilion workspace schema to stdout or the specified file"),
		Command::new("config").about(
			"Dump the default Vermilion workspace configuration to stdout or the specified file",
		),
		Command::new("completions")
			.about(
				"Dump Vermilion shell completion file for the given shell to stdout or the \
				 specified file",
			)
			.arg(
				Arg::new("shell")
					.short('s')
					.action(ArgAction::Set)
					.help("The shell in which to dump completions for")
					.value_parser(value_parser!(clap_complete::Shell))
					.value_name("SHELL")
					.required(true),
			),
	]
}

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME)
		.about("Dump ancillary support files")
		.subcommands(subcommands())
		.subcommand_required(true)
		.hide(true)
		.arg(
			Arg::new("output")
				.long("output")
				.short('o')
				.help("The file to write the generated output to")
				.action(ArgAction::Set)
				.value_hint(ValueHint::FilePath),
		))
}

fn dump_schema(file_path: Option<String>) -> eyre::Result<()> {
	if let Some(file_path) = file_path {
		let mut file = fs::File::create(file_path)?;

		write!(
			&mut file,
			"{}",
			serde_json::to_string_pretty(&schema_for!(crate::workspace::WorkspaceConfig))?
		)?;
	} else {
		println!(
			"{}",
			serde_json::to_string_pretty(&schema_for!(crate::workspace::WorkspaceConfig))?
		);
	}

	Ok(())
}

fn dump_config(file_path: Option<String>) -> eyre::Result<()> {
	if let Some(file_path) = file_path {
		let mut file = fs::File::create(file_path)?;

		write!(
			&mut file,
			"{}",
			toml::to_string(&crate::workspace::WorkspaceConfig::default())?
		)?;
	} else {
		println!(
			"{}",
			toml::to_string(&crate::workspace::WorkspaceConfig::default())?
		);
	}

	Ok(())
}

fn dump_completions(
	cmd: &Command,
	args: &ArgMatches,
	file_path: Option<String>,
) -> eyre::Result<()> {
	// If the user wants us to print completions, then do so
	if let Some(shell) = args.get_one::<clap_complete::Shell>("shell").copied() {
		let mut cmd = cmd.clone();
		if let Some(file_path) = file_path {
			let mut file = fs::File::create(file_path)?;

			clap_complete::generate(shell, &mut cmd, env!("CARGO_PKG_NAME"), &mut file);
		} else {
			clap_complete::generate(
				shell,
				&mut cmd,
				env!("CARGO_PKG_NAME"),
				&mut std::io::stdout(),
			);
		}
	}
	Ok(())
}

pub(crate) fn exec(cmd: &mut Command, args: &ArgMatches) -> eyre::Result<()> {
	let file_path = args.try_get_one::<String>("output")?.cloned();

	match args.subcommand() {
		Some((sub_cmd, cmd_args)) => match sub_cmd {
			"schema" => dump_schema(file_path),
			"config" => dump_config(file_path),
			"completions" => dump_completions(cmd, cmd_args, file_path),
			_ => unreachable!(),
		},
		_ => Err(eyre!("No subcommand specified")),
	}
}
