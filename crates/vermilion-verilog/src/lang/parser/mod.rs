/* SPDX-License-Identifier: BSD-3-Clause */

use vermilion_lang::AtomicByteTendril;

use crate::{VerilogVariant, lang::tokenizer::Tokenizer};

pub struct VerilogParser {
	_std: VerilogVariant,
	_tokenizer: Tokenizer,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: AtomicByteTendril) -> Self {
		Self {
			_std: std,
			_tokenizer: Tokenizer::new(std, content),
		}
	}

	// pub fn parse(&self)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
