/* SPDX-License-Identifier: BSD-3-Clause */

use std::{
	fmt::{Debug, Display},
	str::FromStr,
};

use chumsky::input::{ExactSizeInput, Input, SliceInput, ValueInput};

use crate::parser::Span;

#[derive(Clone)]
pub struct Tendril(tendril::Tendril<tendril::fmt::UTF8>);

impl Tendril {
	pub const fn new(src: tendril::Tendril<tendril::fmt::UTF8>) -> Self {
		Self(src)
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn slice(&self, range: std::ops::Range<&usize>) -> Self {
		let start: u32 = (*range.start).try_into().unwrap();
		let len: u32 = (*range.end - *range.start).try_into().unwrap();
		Self(self.0.subtendril(start, len))
	}

	pub fn slice_from(&self, range: std::ops::RangeFrom<&usize>) -> Self {
		let start: u32 = (*range.start).try_into().unwrap();
		Self(self.0.subtendril(start, self.len().try_into().unwrap()))
	}

	pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
	where
		F: FromStr,
	{
		self.0.parse::<F>()
	}
}

impl Debug for Tendril {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Display for Tendril {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl PartialEq for Tendril {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl PartialEq<char> for Tendril {
	fn eq(&self, other: &char) -> bool {
		self.len() == other.len_utf8() && self.0.starts_with(*other)
	}
}

// impl<'a> PartialEq<&'a str> for Tendril {
// 	fn eq(&self, other: &&'a str) -> bool {
// 		self.0.as_bytes().cmp(other)
// 	}
// }

impl<'a> From<&'a str> for Tendril {
	fn from(value: &'a str) -> Self {
		Self(tendril::Tendril::from_slice(value))
	}
}

impl<'src> Input<'src> for Tendril {
	type Cache = Self;
	type Cursor = usize;
	type MaybeToken = Self::Token;
	type Span = Span;
	type Token = char;

	#[inline]
	fn begin(self) -> (Self::Cursor, Self::Cache) {
		(0, self)
	}

	#[inline]
	fn cursor_location(cursor: &Self::Cursor) -> usize {
		*cursor
	}

	unsafe fn span(_cache: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Span {
		(*range.start..*range.end).into()
	}

	unsafe fn next_maybe(
		cache: &mut Self::Cache,
		cursor: &mut Self::Cursor,
	) -> Option<Self::MaybeToken> {
		if *cursor < cache.0.len() {
			unsafe {
				let chr = cache
					.0
					.get_unchecked(*cursor..)
					.chars()
					.next()
					.unwrap_unchecked();

				*cursor += chr.len_utf8();
				Some(chr)
			}
		} else {
			None
		}
	}
}

impl<'src> ExactSizeInput<'src> for Tendril {
	unsafe fn span_from(
		cache: &mut Self::Cache,
		range: std::ops::RangeFrom<&Self::Cursor>,
	) -> Self::Span {
		(*range.start..cache.0.len()).into()
	}
}

impl<'src> ValueInput<'src> for Tendril {
	unsafe fn next(cache: &mut Self::Cache, cursor: &mut Self::Cursor) -> Option<Self::Token> {
		unsafe { Self::next_maybe(cache, cursor) }
	}
}

impl<'src> SliceInput<'src> for Tendril {
	type Slice = Self;

	fn full_slice(cache: &mut Self::Cache) -> Self::Slice {
		cache.clone()
	}

	unsafe fn slice(cache: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Slice {
		cache.slice(range)
	}

	unsafe fn slice_from(
		cache: &mut Self::Cache,
		from: std::ops::RangeFrom<&Self::Cursor>,
	) -> Self::Slice {
		cache.slice_from(from)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_log::test;
}
