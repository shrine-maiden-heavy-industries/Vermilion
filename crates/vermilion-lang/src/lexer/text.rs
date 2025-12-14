/* SPDX-License-Identifier: BSD-3-Clause */
//! This is a re-implementation of some/most of the [`chumsky::text`]
//! helpers as they make assumptions that are not helpful to us, such
//! as ignoring whitespace.
//!
//! As such we need to roll our own.

#[cfg(test)]
mod tests {
	use super::*;
	use test_log::test;
}
