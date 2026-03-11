// SPDX-License-Identifier: BSD-3-Clause
#![deny(clippy::unwrap_used, clippy::expect_used)]

// TODO(aki):
// - Message/Diagnostic i18n/l10n
// - PGO builds
// - Cross-compilation
// - Release engineering/builds (cargo-dist?)
// - Constraint file support (NOTE: Most of these are just TCL, which is a little problematic)
//   - SDC: Synopsys (Used by other vendor tools as well)
//   - Others (Lattice, Xilinx, etc)
// - LSP Bits
//   - Lots of things,
// - IEEE 1735 Support?
//   - Needed so we don't die on encrypted modules
// - IEEE 1685 Support?
//   - Would be nice so we can offer LSP integration for vendor IP stuff

use std::io;

use eyre::{Context, OptionExt};
use tracing_subscriber::{
	Layer,
	filter::{EnvFilter, LevelFilter},
	fmt,
	layer::SubscriberExt,
	util::SubscriberInitExt,
};

use crate::env::VERMILION_LOG_LEVEL;

mod cli;
mod env;
mod eyre_hook;
mod fs;
mod lang;
mod lsp;
mod paths;
mod workspace;

fn fmt_color() -> bool {
	use crossterm::tty::IsTty;

	io::stderr().is_tty() && colorchoice::ColorChoice::global() != colorchoice::ColorChoice::Never
}

fn initialize_tracing(level: LevelFilter) -> eyre::Result<()> {
	Ok(tracing_subscriber::registry()
		.with((level == LevelFilter::TRACE).then(|| {
			console_subscriber::spawn().with_filter(
				#[allow(
					clippy::unwrap_used,
					reason = "These `Directive` strings are hard-coded and as correct as we can \
					          ensure, and there is no way to construct them in a more-safe manner \
					          other than `.parse()`"
				)]
				EnvFilter::builder()
					.with_default_directive(LevelFilter::OFF.into())
					.from_env_lossy()
					.add_directive("tokio=trace".parse().unwrap())
					.add_directive("runtime=trace".parse().unwrap()),
			)
		}))
		.with(
			fmt::layer()
				.with_writer(io::stderr)
				.with_ansi(fmt_color())
				.with_filter(
					#[allow(
						clippy::unwrap_used,
						reason = "These `Directive` strings are hard-coded and as correct as we \
						          can ensure, and there is no way to construct them in a \
						          more-safe manner other than `.parse()`"
					)]
					EnvFilter::builder()
						.with_default_directive(level.into())
						.with_env_var(VERMILION_LOG_LEVEL)
						.from_env_lossy()
						.add_directive("tokio=error".parse().unwrap())
						.add_directive("runtime=error".parse().unwrap())
						.add_directive("mio=error".parse().unwrap()),
				),
		)
		.try_init()?)
}

fn main() -> eyre::Result<()> {
	eyre_hook::install()?;

	let mut cli = cli::init()?;

	// XXX(aki): We need to clone the Command here because we need it still
	let args = cli.clone().get_matches();

	// Make sure we propagate the color choice
	colorchoice::ColorChoice::write_global(match args.get_one::<clap::ColorChoice>("color") {
		Some(choice) => match choice {
			clap::ColorChoice::Auto => colorchoice::ColorChoice::Auto,
			clap::ColorChoice::Always => colorchoice::ColorChoice::Always,
			clap::ColorChoice::Never => colorchoice::ColorChoice::Never,
		},
		None => colorchoice::ColorChoice::Auto,
	});

	// Initialize tracing with the appropriate log level
	initialize_tracing({
		if args.get_flag("quiet") {
			LevelFilter::WARN
		} else {
			match args.get_count("verbose") {
				0 => LevelFilter::INFO,
				1 => LevelFilter::DEBUG,
				_ => LevelFilter::TRACE,
			}
		}
	})?;

	// Try to invoke the command
	match args.subcommand() {
		Some((sub_cmd, args)) => cli::exec_command(sub_cmd).ok_or_eyre(format!(
			"Unable to find command entry point for `{}`!",
			sub_cmd
		))?(&mut cli, args),
		_ => cli.print_help().wrap_err("Unable to write CLI help?"),
	}
}
