// SPDX-License-Identifier: BSD-3-Clause

use crate::{Position, span::Span};

pub type ParseErr<'src, T> = chumsky::error::Rich<'src, T, Span<usize, Position>>;

#[cfg(test)]
mod tests {
	use test_log::test;

	use super::*;
}
