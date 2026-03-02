// SPDX-License-Identifier: BSD-3-Clause
use std::collections::VecDeque;

use eyre::eyre;
use vermilion_lang::{AtomicByteTendril, Position, Spanned, tokenizer::CoreTokenizer};

use crate::{LanguageStd, lang::tokenizer::token::Token};

pub mod token;

pub struct VhdlTokenizer {
	_standard:    LanguageStd,
	tokenizer:    CoreTokenizer,
	token:        Spanned<Token, Position>,
	token_stream: VecDeque<Spanned<Token, Position>>,
}

impl VhdlTokenizer {
	pub fn new(standard: LanguageStd, file: AtomicByteTendril) -> eyre::Result<VhdlTokenizer> {
		// Check to see if more than one language standard is set
		if !standard.has_single_std() {
			return Err(eyre!("More than one language standard set"));
		}

		Ok(Self {
			_standard:    standard,
			tokenizer:    CoreTokenizer::new(file),
			token:        Spanned::new(Token::default(), None),
			token_stream: VecDeque::new(),
		})
	}

	#[inline(always)]
	pub fn len(&self) -> usize {
		self.tokenizer.len()
	}

	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.tokenizer.is_empty()
	}

	fn read_token(&mut self) {
		if !self.token_stream.is_empty() {
			// SAFETY:
			// This has to work or the stream is empty, thus the if condition fails.
			#[allow(clippy::expect_used)]
			let token = self
				.token_stream
				.pop_front()
				.expect("Unable to pop token from token stream");

			self.token = token;
			return;
		}

		// XXX(aki): Temporary until we start fleshing out the Tokenizer
		#[allow(clippy::match_single_binding)]
		match self.tokenizer.current_byte() {
			_ => self.read_extended_token(),
		}
	}

	fn read_extended_token(&mut self) {
		let _context = self.tokenizer.position();
	}
}

impl Iterator for VhdlTokenizer {
	type Item = Spanned<Token, Position>;

	fn next(&mut self) -> Option<Self::Item> {
		// If we hit the end of the file, we've nothing more to give
		if self.tokenizer.is_eof() && self.token_stream.is_empty() {
			return None;
		}
		self.read_token();
		Some(self.token.clone())
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
