// SPDX-License-Identifier: BSD-3-Clause

use crate::span::{Span, Spanned};

pub type ParseErr<'src> = chumsky::error::Rich<'src, String, Span<()>>;
pub type LexErr<'src> = chumsky::extra::Err<chumsky::error::Rich<'src, char, Span<()>>>;
pub type ParserResult<'src, T> = Result<T, Vec<ParseErr<'src>>>;
pub type LexResult<T> = Vec<Spanned<T, ()>>;

#[cfg(test)]
mod tests {
	use test_log::test;

	use super::*;
}
