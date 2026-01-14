// SPDX-License-Identifier: BSD-3-Clause

use chumsky::input::{Input, Stream};
use eyre::eyre;
use vermilion_lang::{AtomicByteTendril, Position, Span};

use crate::{
	VerilogVariant,
	lang::{ast::Ast, tokenizer::VerilogTokenizer},
};

pub struct VerilogParser {
	_std:      VerilogVariant,
	tokenizer: VerilogTokenizer,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: AtomicByteTendril) -> Self {
		Self {
			_std:      std,
			tokenizer: VerilogTokenizer::new(std, content),
		}
	}

	pub fn parse(self) -> eyre::Result<Ast> {
		// Find out how long the document being parsed is
		let doc_len = self.tokenizer.document_length();
		// Turn our tokenizer into something Chumksy can consume
		let _token_stream = Stream::from_iter(self.tokenizer)
			// Transform the iteration items into tuples for Chumsky
			.map(
				doc_len..doc_len,
				|token| (
						token.inner().clone(),
						token
							.span()
							.copied()
							.unwrap_or_else(|| Span::new(0..0, Position::sof()))
							.into()
					)
			);

		Err(eyre!("blah"))
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
