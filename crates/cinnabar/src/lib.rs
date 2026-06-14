// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use eyre::{Context, OptionExt};
use vermilion_macros::cfg_build_tooling;

mod cli;
mod env;
mod hooks;

/// Main entrypoint for `Cinnabar`
pub fn main() -> eyre::Result<()> {
	hooks::install()?;

	let mut cli = cli::init()?;

	// XXX(aki): We need to clone the Command here because we need it still
	let args = cli.clone().get_matches();

	// Try to invoke the command
	match args.subcommand() {
		Some((sub_cmd, args)) => cli::exec_command(sub_cmd).ok_or_eyre(format!(
			"Unable to find command entry point for `{}`!",
			sub_cmd
		))?(&mut cli, args),
		_ => cli.print_help().wrap_err("Unable to write CLI help?"),
	}
}

// XXX(aki): The methods here are only used with other Cinnabar internal build tooling
cfg_build_tooling! {
	/// Dump Cinnabar CLI completions for the given [`clap_complete::Shell`] to a [`String`]
	pub fn dump_completions(shell: clap_complete::Shell) -> eyre::Result<String> {
		let cli = cli::init()?;
		cli::dump_completions(&cli, shell)
	}

	/// Get a constructed copy of the Cinnabar CLI
	///
	/// ## NOTE
	///
	/// This should only be used to generate help pages and other metadata.
	pub fn get_cli() -> eyre::Result<clap::Command> {
		crate::cli::init()
	}
}
