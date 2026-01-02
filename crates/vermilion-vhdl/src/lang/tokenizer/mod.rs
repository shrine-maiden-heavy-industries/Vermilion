/* SPDX-License-Identifier: BSD-3-Clause */

use std::{collections::VecDeque, ops::Range};

use tendril::ByteTendril;

use self::token::Token;
use crate::VhdlStd;

use vermilion_lang::{Position, Span, Spanned};

pub(crate) mod token;

pub(crate) struct Tokenizer {
	_standard: VhdlStd,
	file: ByteTendril,
	current_char: u8,
	position: usize,
	context: Position,
	eof: bool,
	token: Spanned<Token, Position>,
	token_stream: VecDeque<Spanned<Token, Position>>,
}

impl Tokenizer {
	pub fn new(standard: VhdlStd, file: ByteTendril) -> Tokenizer {
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
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
