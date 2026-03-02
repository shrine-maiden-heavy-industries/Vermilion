// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::{
	LanguageStd,
	sdc::lang::tokenizer::{SdcTokenizer, token::Token},
};

pub struct SdcParser {
	_std:           LanguageStd,
	_tokenizer:     SdcTokenizer,
	_current_token: Option<Spanned<Token, Position>>,
}

impl SdcParser {
	pub fn new(std: LanguageStd, content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self {
			_std:           std,
			_tokenizer:     SdcTokenizer::new(std, content)?,
			_current_token: None,
		})
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
