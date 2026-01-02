/* SPDX-License-Identifier: BSD-3-Clause */

use tendril::ByteTendril;
use vermilion_lang::{Position, Spanned};

use crate::VhdlStd;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Token {
	Invalid(Option<ByteTendril>),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(ByteTendril, VhdlStd),
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
