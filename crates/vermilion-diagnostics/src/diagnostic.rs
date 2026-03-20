// SPDX-License-Identifier: BSD-3-Clause
use vermilion_loc::{position::Position, span::Span};

use crate::Code;

pub trait Diagnostic {
	fn code(&self) -> Code;
	fn message(&self) -> &str;
	fn span(&self) -> Option<&Span>;
	fn position(&self) -> Option<Position>;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StrDiagnostic<'a> {
	code:    Code,
	message: &'a str,
	span:    Option<Span>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StringDiagnostic {
	code:    Code,
	message: String,
	span:    Option<Span>,
}

impl<'a> StrDiagnostic<'a> {
	pub const fn init(code: Code, message: &'a str) -> Self {
		Self { code, message, span: None }
	}

	pub fn new(&self, location: Option<Span>) -> Self {
		Self {
			code:    self.code,
			message: self.message,
			span:    location,
		}
	}
}

impl StringDiagnostic {
	pub fn new<Str>(code: Code, message: Str, location: Option<Span>) -> Self
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

	fn span(&self) -> Option<&Span> {
		self.span.as_ref()
	}

	fn position(&self) -> Option<Position> {
		self.span.as_ref().map(|f| f.get_position())
	}
}

impl Diagnostic for StringDiagnostic {
	fn code(&self) -> Code {
		self.code
	}

	fn message(&self) -> &str {
		self.message.as_str()
	}

	fn span(&self) -> Option<&Span> {
		self.span.as_ref()
	}

	fn position(&self) -> Option<Position> {
		self.span.as_ref().map(|f| f.get_position())
	}
}
