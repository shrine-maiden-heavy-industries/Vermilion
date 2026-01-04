// SPDX-License-Identifier: BSD-3-Clause

use std::{collections::VecDeque, ops::Range};

use vermilion_lang::{AtomicByteTendril, Position, Span, Spanned, tokenizer};

use self::token::Token;
use crate::VhdlStd;

pub mod token;

pub struct Tokenizer {
	_standard:    VhdlStd,
	file:         AtomicByteTendril,
	current_char: u8,
	position:     usize,
	context:      Position,
	eof:          bool,
	token:        Spanned<Token, Position>,
	token_stream: VecDeque<Spanned<Token, Position>>,
}

impl Tokenizer {
	pub fn new(standard: VhdlStd, file: AtomicByteTendril) -> Tokenizer {
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

		tokenizer
	}

	fn read_token(&mut self) {}
}

impl Iterator for Tokenizer {
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
