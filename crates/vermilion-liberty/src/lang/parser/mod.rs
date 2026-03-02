// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::{
	LanguageStd,
	lang::tokenizer::{LibertyTokenizer, token::Token},
};

pub struct LibertyParser {
	_std:           LanguageStd,
	_tokenizer:     LibertyTokenizer,
	_current_token: Option<Spanned<Token, Position>>,
}

impl LibertyParser {
	pub fn new(std: LanguageStd, content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self {
			_std:           std,
			_tokenizer:     LibertyTokenizer::new(std, content)?,
			_current_token: None,
		})
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
