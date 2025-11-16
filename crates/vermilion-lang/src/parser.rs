/* SPDX-License-Identifier: BSD-3-Clause */

use std::fmt::{Debug, Display};

pub type Span = chumsky::span::SimpleSpan;
pub type ParseErr<'src> = chumsky::error::Rich<'src, String, Span>;
pub type LexErr<'src> = chumsky::extra::Err<chumsky::error::Rich<'src, char, Span>>;
pub type ParserResult<'src, T> = Result<T, Vec<ParseErr<'src>>>;
pub type LexResult<T> = Vec<Spanned<T>>;

pub struct Spanned<T>(T, Option<Span>);

impl<T> Spanned<T> {
	pub const fn new(token: T, span: Option<Span>) -> Self {
		Self(token, span)
	}

	pub fn token(&self) -> &T {
		&self.0
	}

	pub fn span(&self) -> Option<&Span> {
		self.1.as_ref()
	}
}

impl<T> PartialEq for Spanned<T>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.token() == other.token() && self.span() == other.span()
	}
}

impl<T> PartialEq<T> for Spanned<T>
where
	T: PartialEq,
{
	fn eq(&self, other: &T) -> bool {
		self.token() == other
	}
}

impl<T> Debug for Spanned<T>
where
	T: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?} @ {:?}", self.0, self.1)
	}
}

impl<T> Display for Spanned<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.1 {
			Some(span) => write!(f, "{} @ {}", self.0, span),
			None => write!(f, "{}", self.0),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_log::test;
}
