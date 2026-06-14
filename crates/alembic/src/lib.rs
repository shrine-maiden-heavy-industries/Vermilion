// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use eyre::{Context, OptionExt};

mod cli;
mod env;
mod hooks;

/// Main entrypoint for `Alembic`
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
