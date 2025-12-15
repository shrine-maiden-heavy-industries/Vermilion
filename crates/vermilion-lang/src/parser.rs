/* SPDX-License-Identifier: BSD-3-Clause */

use std::{fmt::{Debug, Display}, ops::Range};

use chumsky::span::SimpleSpan;

pub type ParseErr<'src> = chumsky::error::Rich<'src, String, Span<()>>;
pub type LexErr<'src> = chumsky::extra::Err<chumsky::error::Rich<'src, char, Span<()>>>;
pub type ParserResult<'src, T> = Result<T, Vec<ParseErr<'src>>>;
pub type LexResult<T> = Vec<Spanned<T, ()>>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span<C>(chumsky::span::SimpleSpan<usize, C>);

#[derive(Clone, Copy)]
pub struct Spanned<T, C>(T, Option<Span<C>>);

impl<C> Span<C> {
	pub fn new(range: Range<usize>, context: C) -> Self {
		Self(SimpleSpan { start: range.start, end: range.end, context })
	}
}

impl<C> Display for Span<C>
where
	C: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let SimpleSpan { start: begin, end, context } = &self.0;
		write!(f, "[{begin}, {end}) => {context}")
	}
}

impl<C> Debug for Span<C>
where
	C: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let SimpleSpan { start: begin, end, context } = &self.0;
		write!(f, "[{begin}, {end}) => {context:?}")
	}
}

impl<T, C> Spanned<T, C> {
	pub const fn new(token: T, span: Option<Span<C>>) -> Self {
		Self(token, span)
	}

	pub fn token(&self) -> &T {
		&self.0
	}

	pub fn span(&self) -> Option<&Span<C>> {
		self.1.as_ref()
	}

	pub fn attach_span(&mut self, span: Span<C>) {
		self.1 = Some(span)
	}
}

impl<T, C> PartialEq for Spanned<T, C>
where
	T: PartialEq,
	C: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.token() == other.token() && self.span() == other.span()
	}
}

impl<T, C> PartialEq<T> for Spanned<T, C>
where
	T: PartialEq,
{
	fn eq(&self, other: &T) -> bool {
		self.token() == other
	}
}

impl<T, C> Debug for Spanned<T, C>
where
	T: Debug,
	C: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?} @ {:?}", self.0, self.1)
	}
}

impl<T, C> Display for Spanned<T, C>
where
	T: Display,
	C: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.1 {
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
