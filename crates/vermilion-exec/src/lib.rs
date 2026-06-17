// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

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
}

/// Setup and run the given [`Executable`]
pub fn _run(exec: &dyn Executable) -> eyre::Result<()> {
	Ok(())
}
