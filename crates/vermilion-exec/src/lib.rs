// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

use std::io;

use tracing_subscriber::{
	Layer,
	filter::{EnvFilter, LevelFilter},
	fmt,
	layer::SubscriberExt,
	util::SubscriberInitExt,
};

pub mod cli;
mod hooks;

/// Create a new [`Executable`] with the given name and log-level
#[macro_export]
macro_rules! new {
	($name:ident, $env:literal) => {
		pub(crate) struct $name;

		impl $crate::Executable for $name {
			#[inline]
			fn description(&self) -> &'static str {
				env!("CARGO_PKG_DESCRIPTION")
			}

			#[inline]
			fn log_level_var(&self) -> &'static str {
				$env
			}

			#[inline]
			fn name(&self) -> &'static str {
				env!("CARGO_PKG_NAME")
			}

			#[inline]
			fn version(&self) -> &'static str {
				env!("CARGO_PKG_VERSION")
			}
		}

		impl $name {
			#[inline]
			pub fn new() -> Self {
				Self
			}

			/// Run this executable
			#[inline]
			pub fn run(&self) -> eyre::Result<()> {
				$crate::_run(self)
			}

			/// Dump shell completions for this executable
			#[inline]
			pub fn dump_completions(&self, shell: clap_complete::Shell) -> eyre::Result<String> {
				$crate::_dump_completions(self, shell)
			}
		}

		#[inline]
		pub fn main() -> eyre::Result<()> {
			let exec = $name::new();
			exec.run()
		}
	};
}

pub trait Executable {
	/// The executable description
	///
	/// Should be set to `env!("CARGO_PKG_DESCRIPTION")`
	fn description(&self) -> &'static str;
	/// The environment variable used to set/override the log level
	fn log_level_var(&self) -> &'static str;
	/// The executable name
	///
	/// Should be set to `env!("CARGO_PKG_NAME")`
	fn name(&self) -> &'static str;
	/// The executable version
	///
	/// Should be set to `env!("CARGO_PKG_VERSION")`
	fn version(&self) -> &'static str;

	/// Additional root-level CLI arguments
	fn args(&self) -> eyre::Result<Vec<clap::Arg>> {
		Ok(vec![])
	}
}

fn fmt_color() -> bool {
	use crossterm::tty::IsTty;

	io::stderr().is_tty() && colorchoice::ColorChoice::global() != colorchoice::ColorChoice::Never
}

fn initialize_tracing(level: LevelFilter, env_var: &'static str) -> eyre::Result<()> {
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
						.with_env_var(env_var)
						.from_env_lossy()
						.add_directive("tokio=error".parse().unwrap())
						.add_directive("runtime=error".parse().unwrap())
						.add_directive("mio=error".parse().unwrap()),
				),
		)
		.try_init()?)
}

pub fn _setup_cli(exec: &dyn Executable) -> eyre::Result<clap::Command> {
	let cli = cli::init(
		exec.name(),
		exec.version(),
		exec.description(),
		exec.log_level_var(),
	)?;

	let additional_args = exec.args()?;
	let cli = cli.args(additional_args);

	// TODO(aki): Subcommands

	Ok(cli)
}

/// Setup and run the given [`Executable`]
pub fn _run(exec: &dyn Executable) -> eyre::Result<()> {
	// Set-up the eyre and panic hooks
	hooks::install(exec.name(), exec.version())?;

	let cli = _setup_cli(exec)?;

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
	initialize_tracing(
		{
			if args.get_flag("quiet") {
				LevelFilter::WARN
			} else {
				match args.get_count("verbose") {
					0 => LevelFilter::INFO,
					1 => LevelFilter::DEBUG,
					_ => LevelFilter::TRACE,
				}
			}
		},
		exec.log_level_var(),
	)?;

	// TODO(aki): Subcommand parsing

	Ok(())
}

pub fn _dump_completions(
	exec: &dyn Executable,
	shell: clap_complete::Shell,
) -> eyre::Result<String> {
	let mut completion_buffer = Vec::new();
	let mut cli = _setup_cli(exec)?;

	clap_complete::generate(shell, &mut cli, exec.name(), &mut completion_buffer);

	Ok(String::from_utf8(completion_buffer)?)
}
