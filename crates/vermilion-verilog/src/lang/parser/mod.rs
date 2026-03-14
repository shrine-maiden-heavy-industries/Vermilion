// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::AtomicByteTendril;

use crate::{
	LanguageStd,
	lang::{ast::Ast, tokenizer::VerilogTokenizer},
};

pub struct VerilogParser {
	_std:       LanguageStd,
	_tokenizer: VerilogTokenizer,
}

impl VerilogParser {
	pub fn new(std: LanguageStd, content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self {
			_std:       std,
			_tokenizer: VerilogTokenizer::new(std, content)?,
		})
	}

	pub fn parse(mut self) -> eyre::Result<Ast> {
		Ok(Ast::default())
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
