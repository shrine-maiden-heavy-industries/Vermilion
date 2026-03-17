// SPDX-License-Identifier: BSD-3-Clause
use vermilion_loc::{position::Position, span::Span};

pub trait Diagnostic {
	fn message(&self) -> &str;
	fn span(&self) -> Option<&Span<usize, Position>>;
	fn position(&self) -> Option<&Position>;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StringDiagnostic {
	span:    Option<Span<usize, Position>>,
	message: String,
}

impl StringDiagnostic {
	pub fn new<Str>(message: Str, location: Option<Span<usize, Position>>) -> Self
	where
		Str: Into<String>,
	{
		Self { message: message.into(), span: location }
	}
}

impl Diagnostic for StringDiagnostic {
	fn message(&self) -> &str {
		self.message.as_str()
	}

	fn span(&self) -> Option<&Span<usize, Position>> {
		self.span.as_ref()
	}

	fn position(&self) -> Option<&Position> {
		self.span.as_ref().map(|f| f.context())
	}
}
