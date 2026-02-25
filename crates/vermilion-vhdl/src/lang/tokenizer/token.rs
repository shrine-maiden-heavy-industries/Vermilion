// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::LanguageStd;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
	Invalid(Option<AtomicByteTendril>),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(AtomicByteTendril, LanguageStd),
}

impl Display for Token {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Invalid(_tendril) => todo!(),
			Token::ContextuallyInvalid(_tendril, _vhdl_std) => todo!(),
		}
	}
}

impl Default for Token {
	fn default() -> Self {
		Self::Invalid(None)
	}
}

impl From<Token> for Spanned<Token, Position> {
	fn from(token: Token) -> Self {
		Self::new(token, None)
	}
}
