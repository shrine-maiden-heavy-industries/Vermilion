// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::{
	LanguageStd,
	lpf::lang::tokenizer::{LpfTokenizer, token::Token},
};

pub struct LpfParser {
	_std:           LanguageStd,
	_tokenizer:     LpfTokenizer,
	_current_token: Option<Spanned<Token, Position>>,
}

impl LpfParser {
	pub fn new(std: LanguageStd, content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self {
			_std:           std,
			_tokenizer:     LpfTokenizer::new(std, content)?,
			_current_token: None,
		})
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
