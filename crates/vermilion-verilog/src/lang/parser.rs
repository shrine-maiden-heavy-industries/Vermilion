/* SPDX-License-Identifier: BSD-3-Clause */

use crate::{VerilogVariant, lang::tokenizer::Tokenizer};

pub struct VerilogParser {
	std: VerilogVariant,
	tokenizer: Tokenizer,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: &str) -> Self {
		Self {
			std,
			tokenizer: Tokenizer::new(std, content.as_bytes().into()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_log::test;
}
