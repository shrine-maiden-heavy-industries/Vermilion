// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::AtomicByteTendril;
use vermilion_loc::{Position, Spanned};

use crate::{
	LanguageStd,
	lang::tokenizer::{VhdlTokenizer, token::Token},
};

pub mod error;

pub struct VhdlParser {
	_std:           LanguageStd,
	_tokenizer:     VhdlTokenizer,
	_current_token: Option<Spanned<Token, Position>>,
}

impl VhdlParser {
	pub fn new(std: LanguageStd, content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self {
			_std:           std,
			_tokenizer:     VhdlTokenizer::new(std, content)?,
			_current_token: None,
		})
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
