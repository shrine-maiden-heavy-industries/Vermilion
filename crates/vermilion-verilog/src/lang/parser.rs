/* SPDX-License-Identifier: BSD-3-Clause */

use tendril::ByteTendril;

use crate::{VerilogVariant, lang::tokenizer::Tokenizer};

pub struct VerilogParser {
	_std: VerilogVariant,
	_tokenizer: Tokenizer,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: ByteTendril) -> Self {
		Self {
			_std: std,
			_tokenizer: Tokenizer::new(std, content),
		}
	}

	// pub fn parse(&self)
}

#[cfg(test)]
mod tests {
	// use super::*;
	// use test_log::test;
}
