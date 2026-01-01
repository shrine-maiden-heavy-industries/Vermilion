/* SPDX-License-Identifier: BSD-3-Clause */

use crate::{VerilogVariant, lang::tokenizer::Tokenizer};

pub struct VerilogParser {
	_std: VerilogVariant,
	_tokenizer: Tokenizer,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: &str) -> Self {
		Self {
			_std: std,
			_tokenizer: Tokenizer::new(std, content.as_bytes().into()),
		}
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
	// use test_log::test;
}
