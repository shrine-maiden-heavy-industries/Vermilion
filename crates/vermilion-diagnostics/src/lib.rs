// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

use vermilion_loc::{position::Position, span::Span};

pub mod workspace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
	span:    Option<Span<usize, Position>>,
	message: String,
}

impl Diagnostic {
	pub fn new<Str>(message: Str, location: Option<Span<usize, Position>>) -> Self
	where
		Str: Into<String>,
	{
		Self { message: message.into(), span: location }
	}

	pub fn message(&self) -> &String {
		&self.message
	}

	pub fn span(&self) -> Option<&Span<usize, Position>> {
		self.span.as_ref()
	}

	pub fn position(&self) -> Option<&Position> {
		self.span.as_ref().map(|f| f.context())
	}
}
