// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

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

fn setup_cli(exec: &dyn Executable) -> eyre::Result<clap::Command> {
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

	let cli = setup_cli(exec)?;

	// XXX(aki): We need to clone the Command here because we need it still
	let args = cli.clone().get_matches();

	// TODO(aki): Subcommand parsing

	Ok(())
}

pub fn _dump_completions(
	exec: &dyn Executable,
	shell: clap_complete::Shell,
) -> eyre::Result<String> {
	let mut completion_buffer = Vec::new();
	let mut cli = setup_cli(exec)?;

	clap_complete::generate(shell, &mut cli, exec.name(), &mut completion_buffer);

	Ok(String::from_utf8(completion_buffer)?)
}
