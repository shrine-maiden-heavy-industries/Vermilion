// SPDX-License-Identifier: BSD-3-Clause
use vermilion_loc::{position::Position, span::Span};

use crate::Code;

pub trait Diagnostic {
	fn code(&self) -> Code;
	fn message(&self) -> &str;
	fn span(&self) -> Option<&Span<usize, Position>>;
	fn position(&self) -> Option<&Position>;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StrDiagnostic<'a> {
	code:    Code,
	message: &'a str,
	span:    Option<Span<usize, Position>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StringDiagnostic {
	code:    Code,
	message: String,
	span:    Option<Span<usize, Position>>,
}

impl<'a> StrDiagnostic<'a> {
	pub const fn init(code: Code, message: &'a str) -> Self {
		Self { code, message, span: None }
	}

	pub fn new(&self, location: Option<Span<usize, Position>>) -> Self {
		Self {
			code:    self.code,
			message: self.message,
			span:    location,
		}
	}
}

impl StringDiagnostic {
	pub fn new<Str>(code: Code, message: Str, location: Option<Span<usize, Position>>) -> Self
	where
		Str: Into<String>,
	{
		Self { code, message: message.into(), span: location }
	}
}

impl<'a> Diagnostic for StrDiagnostic<'a> {
	fn code(&self) -> Code {
		self.code
	}

	fn message(&self) -> &str {
		self.message
	}

	fn span(&self) -> Option<&Span<usize, Position>> {
		self.span.as_ref()
	}

	fn position(&self) -> Option<&Position> {
		self.span.as_ref().map(|f| f.context())
	}
}

impl Diagnostic for StringDiagnostic {
	fn code(&self) -> Code {
		self.code
	}

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
