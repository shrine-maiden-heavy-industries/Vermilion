// SPDX-License-Identifier: BSD-3-Clause

use vermilion_diagnostics::Diagnostic;
use vermilion_lang::{AtomicByteTendril, Position, Span};

use crate::{
	VerilogVariant,
	lang::{
		ast::Ast,
		tokenizer::{
			VerilogTokenizer,
			token::{Keyword, Token},
		},
	},
};

pub mod error;

pub struct VerilogParser {
	std:       VerilogVariant,
	tokenizer: VerilogTokenizer,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: AtomicByteTendril) -> Self {
		Self {
			std,
			tokenizer: VerilogTokenizer::new(std, content),
		}
	}

	pub fn parse(self) -> eyre::Result<(Option<Ast>, Vec<Diagnostic>)> {
		unimplemented!()
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
