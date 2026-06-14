// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! # Alembic CLI
//!
//! See [alembic] for details

/// Main Alembic CLI entrypoint, see [`alembic::main`] for details
fn main() -> eyre::Result<()> {
	alembic::main()
}
