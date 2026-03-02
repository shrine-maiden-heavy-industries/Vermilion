// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::{
	LanguageStd,
	pdc::lang::tokenizer::{PdcTokenizer, token::Token},
};

pub struct PdcParser {
	_std:           LanguageStd,
	_tokenizer:     PdcTokenizer,
	_current_token: Option<Spanned<Token, Position>>,
}

impl PdcParser {
	pub fn new(std: LanguageStd, content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self {
			_std:           std,
			_tokenizer:     PdcTokenizer::new(std, content)?,
			_current_token: None,
		})
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
