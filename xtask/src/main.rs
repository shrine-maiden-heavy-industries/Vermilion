// SPDX-License-Identifier: BSD-3-Clause

use clap::{Arg, ArgAction, Command};
use eyre::OptionExt;
use tracing_subscriber::{
	self, Layer,
	filter::{EnvFilter, LevelFilter},
	fmt,
	layer::SubscriberExt,
	util::SubscriberInitExt,
};

mod commands;
mod git;
mod paths;

const LOG_LEVEL_ENV: &str = "VERMILION_LOG_LEVEL";

fn initialize_tracing(level: LevelFilter) -> eyre::Result<()> {
	Ok(tracing_subscriber::registry()
		.with(
			fmt::layer().with_filter(
				EnvFilter::builder()
					.with_default_directive(level.into())
					.with_env_var(LOG_LEVEL_ENV)
					.from_env_lossy(),
			),
		)
		.try_init()?)
}

fn cli() -> eyre::Result<Command> {
	Ok(Command::new("xtask")
		.arg(
			Arg::new("verbose")
				.short('v')
				.action(ArgAction::Count)
				.help("Enable verbose logging")
				.long_help(format!(
					"Enable verbose logging\n\n Repeat this option multiple times for \
					 increasingly verbose output.\n\n This setting is overridden by the {} \
					 environment variable and also yields to the -q flag for quiet operation.",
					LOG_LEVEL_ENV
				)),
		)
		.arg(
			Arg::new("quiet")
				.short('q')
				.action(ArgAction::SetTrue)
				.help("Disable logging")
				.long_help(format!(
					"Disable logging\n\nDisable all output {}",
					LOG_LEVEL_ENV,
				)),
		)
		.subcommands(commands::init()?))
}

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut cli = cli()?;
	let args = cli.clone().get_matches();

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

	let cmd = match args.subcommand() {
		Some((cmd, args)) => (cmd, args),
		_ => {
			cli.print_help()?;
			return Ok(());
		},
	};

	commands::exec(cmd.0).ok_or_eyre(format!("Unable to find command executor for {}", cmd.0))?(
		cmd.1,
	)
}
