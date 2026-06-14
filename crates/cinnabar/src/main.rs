// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! # Cinnabar CLI
//!
//! See [cinnabar] for details

/// Main Cinnabar CLI entrypoint, see [`cinnabar::main`] for details
fn main() -> eyre::Result<()> {
	cinnabar::main()
}
