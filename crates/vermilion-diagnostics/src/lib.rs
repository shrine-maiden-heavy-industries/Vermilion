// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod workspace;

/// The creature-readable contextual position within a document
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
	line:      usize,
	character: usize,
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span<T = usize, C = ()> {
	begin:   T,
	end:     T,
	context: C,
}

impl<T, C> Span<T, C> {
	pub fn new(begin: T, end: T, context: C) -> Self {
		Self { begin, end, context }
	}
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
	position: Position,
	message:  String,
}

impl Position {
	pub const fn sof() -> Self {
		Self { line: 0, character: 0 }
	}

	pub const fn new(line: usize, character: usize) -> Self {
		Self { line, character }
	}
}

impl Diagnostic {
	// XXX: This fails to properly convert location spans into diagnostic spans
	pub fn new<LocationSpan, Str>(location: Option<LocationSpan>, message: Str) -> Self
	where
		Span<usize, Position>: From<LocationSpan>,
		Str: Into<String>,
		Option<Span<usize, Position>>: From<Option<LocationSpan>>,
	{
		let location: Option<Span<usize, Position>> = location.into();
		let position = if let Some(span) = location {
			span.context
		} else {
			Position::sof()
		};

		Self { position, message: message.into() }
	}
}
