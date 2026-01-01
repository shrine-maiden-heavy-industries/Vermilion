/* SPDX-License-Identifier: BSD-3-Clause */
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::io;

use eyre::{Context, OptionExt};
use schemars::schema_for;
use tracing_subscriber::{
	Layer,
	filter::{EnvFilter, LevelFilter},
	fmt,
	layer::SubscriberExt,
	util::SubscriberInitExt,
};

use vermilion_core::vars::VERMILION_LOG_LEVEL;

use crate::settings::Config;

mod cli;
mod eyre_hook;
pub(crate) mod lang;
pub(crate) mod lsp;
pub(crate) mod paths;
pub(crate) mod settings;

fn fmt_color() -> bool {
	use crossterm::tty::IsTty;
	use std::io::stdout;

	stdout().is_tty() && colorchoice::ColorChoice::global() != colorchoice::ColorChoice::Never
}

fn initialize_tracing(level: LevelFilter) -> eyre::Result<()> {
	Ok(tracing_subscriber::registry()
		.with((level == LevelFilter::TRACE).then(|| {
			console_subscriber::spawn().with_filter(
				// SAFETY:
				// These `Directive` strings are hard-coded and as correct as we can ensure,
				// and there is no way to construct them in a more-safe manner other than `.parse()`
				#[allow(clippy::unwrap_used)]
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
					// SAFETY:
					// These `Directive` strings are hard-coded and as correct as we can ensure,
					// and there is no way to construct them in a more-safe manner other than `.parse()`
					#[allow(clippy::unwrap_used)]
					EnvFilter::builder()
						.with_default_directive(level.into())
						.with_env_var(VERMILION_LOG_LEVEL)
						.from_env_lossy()
						.add_directive("tokio=error".parse().unwrap())
						.add_directive("runtime=error".parse().unwrap()),
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

	// If the user wants us to dump the configuration schema, do so and exit right away
	if args.get_flag("dump-schema") {
		println!(
			"{}",
			serde_json::to_string_pretty(&schema_for!(crate::settings::Config))?
		);

		return Ok(());
	}

	// Likewise, if we want to dump the default config, do that
	if args.get_flag("dump-config") {
		println!("{}", toml::to_string(&Config::default())?);

		return Ok(());
	}

	// If the user wants us to print completions, then do so
	if let Some(shell) = args
		.get_one::<clap_complete::Shell>("dump-completions")
		.copied()
	{
		let mut comp_cli = cli.clone();
		clap_complete::generate(
			shell,
			&mut comp_cli,
			env!("CARGO_PKG_NAME"),
			&mut std::io::stdout(),
		);

		return Ok(());
	}

	// Initialize tracing with the appropriate log level
	initialize_tracing({
		if args.get_flag("quiet") {
			LevelFilter::ERROR
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
		Some((cmd, args)) => {
			// TODO(aki): Not all commands need the config
			let cfg = settings::load_config(args)?;

			cli::exec_command(cmd)
				.ok_or_eyre(format!("Unable to find command entry point for `{}`!", cmd))?(args, cfg)
		},
		_ => cli.print_help().wrap_err("Unable to write CLI help?"),
	}
}
