// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::AtomicByteTendril;

use crate::{
	LanguageStd,
	lang::{
		keywords::Keyword,
		types::{
			BasedLiteralSpecifier, Comment, CompilerDirective, Control, Identifier, Operator,
			SingleQuotedString, SystemFunc, TextMacro, TripleQuotedString,
		},
	},
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Token {
	BasedLiteralSpecifier(BasedLiteralSpecifier),
	Comment(Comment),
	CompilerDirective(CompilerDirective),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(AtomicByteTendril, LanguageStd),
	Control(Control),
	Identifier(Identifier),
	Invalid(Option<AtomicByteTendril>),
	Keyword(Keyword),
	Newline(AtomicByteTendril),
	Number(AtomicByteTendril),
	Operator(Operator),
	Real(f64),
	SingleQuotedString(SingleQuotedString),
	SystemFunc(SystemFunc),
	TextMacro(TextMacro),
	TripleQuotedString(TripleQuotedString), // Added: IEEE 1800-2023
	UnsignedNumber(AtomicByteTendril),
	Whitespace(AtomicByteTendril),
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BasedLiteralSpecifier(specifier) => specifier.fmt(f),
			Self::Comment(comment) => comment.fmt(f),
			Self::CompilerDirective(compiler_directive) => compiler_directive.fmt(f),
			Self::ContextuallyInvalid(tendril, std) => {
				write!(
					f,
					"ContextuallyInvalid(\"{}\", {:?})",
					unsafe { str::from_utf8_unchecked(tendril) },
					std
				)
			},
			Self::Control(control) => control.fmt(f),
			Self::Identifier(identifier) => identifier.fmt(f),
			Self::Invalid(_tendril) => todo!(),
			Self::Keyword(keyword) => keyword.fmt(f),
			Self::Newline(tendril) => write!(f, "Newline({})", tendril.len()),
			Self::Number(tendril) => write!(f, "Number({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Operator(operator) => operator.fmt(f),
			Self::Real(value) => write!(f, "Real({})", value),
			Self::SingleQuotedString(string) => string.fmt(f),
			Self::SystemFunc(sysfunc) => sysfunc.fmt(f),
			Self::TextMacro(text_macro) => text_macro.fmt(f),
			Self::TripleQuotedString(string) => string.fmt(f),
			Self::UnsignedNumber(tendril) => write!(f, "UnsignedNumber({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Whitespace(tendril) => write!(f, "Whitespace({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Default for Token {
	fn default() -> Self {
		Self::Invalid(None)
	}
}
