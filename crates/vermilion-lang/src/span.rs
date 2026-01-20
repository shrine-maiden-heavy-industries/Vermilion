// SPDX-License-Identifier: BSD-3-Clause

use std::{
	fmt::{Debug, Display},
	ops::Range,
};

#[macro_export]
macro_rules! spanned_token {
	($token:expr) => {
		vermilion_lang::span::Spanned::new($token, None)
	};
	($token:expr, $span_range:expr, $context:expr) => {
		vermilion_lang::span::Spanned::new(
			$token,
			Some(vermilion_lang::span::Span::new($span_range, $context)),
		)
	};
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span<T = usize, C = ()> {
	begin:   T,
	end:     T,
	context: C,
}

#[derive(Copy, Clone)]
pub struct Spanned<T, C> {
	inner: T,
	span:  Option<Span<usize, C>>,
}

impl<T, C> Span<T, C> {
	pub fn new(range: Range<T>, context: C) -> Self {
		Self { begin: range.start, end: range.end, context }
	}

	pub fn begin(&self) -> &T {
		&self.begin
	}

	pub fn end(&self) -> &T {
		&self.end
	}

	pub fn context(&self) -> &C {
		&self.context
	}
}

impl<T> From<Range<T>> for Span<T> {
	fn from(value: Range<T>) -> Self {
		Self {
			begin:   value.start,
			end:     value.end,
			context: (),
		}
	}
}

impl<T, C> From<Span<T, C>> for Range<T> {
	fn from(value: Span<T, C>) -> Self {
		Range { start: value.begin, end: value.end }
	}
}

impl<T, C> Display for Span<T, C>
where
	T: Display,
	C: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Span { begin, end, context } = &self;

		write!(f, "[{begin},{end}) => {context}")
	}
}

impl<T, C> Debug for Span<T, C>
where
	T: Debug,
	C: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Span")
			.field("begin", &self.begin)
			.field("end", &self.end)
			.field("context", &self.context)
			.finish()
	}
}

impl<T, C> Spanned<T, C> {
	pub fn new(inner: T, span: Option<Span<usize, C>>) -> Self {
		Self { inner, span }
	}

	pub fn inner(&self) -> &T {
		&self.inner
	}

	pub fn span(&self) -> Option<&Span<usize, C>> {
		self.span.as_ref()
	}

	pub fn attach_span(&mut self, span: Span<usize, C>) {
		self.span = Some(span)
	}
}

impl<T, C> PartialEq for Spanned<T, C>
where
	T: PartialEq,
	C: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.inner() == other.inner() && self.span() == other.span()
	}
}

impl<T, C> PartialEq<T> for Spanned<T, C>
where
	T: PartialEq,
{
	fn eq(&self, other: &T) -> bool {
		self.inner() == other
	}
}

impl<T, C> Display for Spanned<T, C>
where
	T: Display,
	C: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.span {
			Some(span) => write!(f, "{} @ {}", self.inner, span),
			None => write!(f, "{}", self.inner),
		}
	}
}

impl<T, C> Debug for Spanned<T, C>
where
	T: Debug,
	C: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Spanned")
			.field("inner", &self.inner)
			.field("span", &self.span)
			.finish()
	}
}
