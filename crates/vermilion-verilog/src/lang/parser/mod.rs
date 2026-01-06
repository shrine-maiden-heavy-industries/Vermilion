// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::AtomicByteTendril;

use crate::{VerilogVariant, lang::tokenizer::VerilogTokenizer};

pub struct VerilogParser {
	_std:       VerilogVariant,
	_tokenizer: VerilogTokenizer,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: AtomicByteTendril) -> Self {
		Self {
			_std:       std,
			_tokenizer: VerilogTokenizer::new(std, content),
		}
	}

	// pub fn parse(&self)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
