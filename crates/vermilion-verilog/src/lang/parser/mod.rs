// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::{AtomicByteTendril, tokenizer::TokenizerIter};

use crate::{
	LanguageStd,
	lang::{
		ast::Ast,
		tokenizer::{VerilogTokenizer, token::Token},
	},
};

pub struct VerilogParser {
	_std:      LanguageStd,
	tokenizer: VerilogTokenizer,
}

impl VerilogParser {
	pub fn new(std: LanguageStd, content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self {
			_std:      std,
			tokenizer: VerilogTokenizer::new(std, content)?,
		})
	}

	pub fn parse(mut self) -> eyre::Result<Ast> {
		// TODO(aki):
		// Skip over whitespace for now, just so we can deal with getting the parser working,
		// once it is, figure out how to merge in whitespace to the AST.
		let mut iter = TokenizerIter::new(&mut self.tokenizer)
			.filter(|token| !matches!(token.inner(), Token::Whitespace(_) | Token::Newline(_)));

		for token in &mut iter {}

		Ok(Ast::default())
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
