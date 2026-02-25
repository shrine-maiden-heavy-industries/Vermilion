// SPDX-License-Identifier: BSD-3-Clause

use std::collections::VecDeque;

use eyre::eyre;
use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use self::token::Token;
use crate::LanguageStd;

pub mod token;

pub struct VhdlTokenizer {
	_standard:    LanguageStd,
	file:         AtomicByteTendril,
	current_char: u8,
	position:     usize,
	context:      Position,
	eof:          bool,
	token:        Spanned<Token, Position>,
	token_stream: VecDeque<Spanned<Token, Position>>,
}

impl VhdlTokenizer {
	pub fn new(standard: LanguageStd, file: AtomicByteTendril) -> eyre::Result<VhdlTokenizer> {
		// Check to see if more than one language standard is set
		if !standard.has_single_std() {
			return Err(eyre!("More than one language standard set"));
		}

		let mut tokenizer = Self {
			_standard: standard,
			file,
			current_char: 0,
			position: 0,
			context: Position::sof(),
			eof: false,
			token: Spanned::new(Token::default(), None),
			token_stream: VecDeque::new(),
		};

		// If we somehow created a tokenizer on an empty input, make sure we set EOF right away
		if tokenizer.file.is_empty() {
			tokenizer.eof = true;
		} else {
			tokenizer.current_char = tokenizer.file[tokenizer.position];
		}

		Ok(tokenizer)
	}

	fn read_token(&mut self) {}
}

impl Iterator for VhdlTokenizer {
	type Item = Spanned<Token, Position>;

	fn next(&mut self) -> Option<Self::Item> {
		// If we hit the end of the file, we've nothing more to give
		if self.eof && self.token_stream.is_empty() {
			return None;
		}
		self.read_token();
		Some(self.token.clone())
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
