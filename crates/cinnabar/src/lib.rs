// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod hooks;

/// Main entrypoint for `Cinnabar`
pub fn main() -> eyre::Result<()> {
	hooks::install()?;

	Ok(())
}
