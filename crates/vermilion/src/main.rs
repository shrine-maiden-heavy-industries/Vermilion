// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! # Vermilion CLI
//!
//! See [vermilion] for details

/// Main Vermilion CLI entrypoint, see [`vermilion::vermilion_main`] for details
fn main() -> eyre::Result<()> {
	vermilion::vermilion_main()
}
