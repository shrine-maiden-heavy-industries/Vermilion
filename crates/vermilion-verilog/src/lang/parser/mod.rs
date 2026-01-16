// SPDX-License-Identifier: BSD-3-Clause

use chumsky::{
	Parser,
	error::Rich,
	extra,
	input::{Input, Stream},
	prelude::just,
};
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

	pub fn parse(self) -> Result<(Ast, Vec<Diagnostic>), Vec<Diagnostic>> {
		// Find out how long the document being parsed is
		let doc_len = self.tokenizer.document_length();
		// Turn our tokenizer into something Chumksy can consume
		let token_stream = Stream::from_iter(self.tokenizer)
			// Transform the iteration items into tuples for Chumsky
			.map(
				Span::new(doc_len..doc_len, Position::sof()),
				|token| (
						token.inner().clone(),
						token
							.span()
							.copied()
							.unwrap_or_else(|| Span::new(0..0, Position::sof()))
					)
			);

		// Get the parser combinator for the selected Verilog version
		let parser = Self::parser_for(self.std);

		let (ast, diagnostics) = parser.parse(token_stream).into_output_errors();

		let diagnostics = diagnostics
			.into_iter()
			.map(|_diagnostic| Diagnostic {})
			.collect();

		match ast {
			Some(ast) => Ok((ast, diagnostics)),
			None => Err(diagnostics),
		}
	}

	fn parser_for<'src, I>(
		_std: VerilogVariant,
	) -> impl Parser<'src, I, Ast, extra::Err<Rich<'src, Token, Span<usize, Position>>>>
	where
		I: Input<'src, Span = Span<usize, Position>, Token = Token>,
	{
		just(Token::Keyword(Keyword::Module)).map(|_| Ast {})
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
