// SPDX-License-Identifier: BSD-3-Clause

use tendril::ByteTendril;

use crate::Position;

/// The underlying tokenization machinery for all other Vermilion tokenizers
pub struct CoreTokenizer {
	text:     ByteTendril,
	offset:   usize,
	eof:      bool,
	current:  u8,
	position: Position,
}

// TODO(aki): We need to deal with UTF-8/UTF-16/UTF-32/WTF-8 inputs not just ASCII
impl CoreTokenizer {
	/// Create a new [`CoreTokenizer`] with the given [`ByteTendril`] as it's backing store
	pub fn new(text: ByteTendril) -> Self {
		let mut tokenizer = Self {
			text,
			offset: 0,
			eof: false,
			current: 0,
			position: Position::sof(),
		};

		// If there was a case where somehow a tokenizer was made with an empty file
		// then ensure we're at the EOF right away to prevent any issues
		if tokenizer.text.is_empty() {
			tokenizer.eof = true;
		} else {
			tokenizer.next_char();
		}

		tokenizer
	}

	// XXX(aki): I'm not sure that we should return the previous character, feels, off
	/// Advances the position in the input, returning the previous character
	/// and setting [`Self::current_char`] to the next character in the stream
	///
	/// This method also sets [`Self::is_eof`] appropriately if we reach the end
	/// of the input stream,
	pub fn next_char(&mut self) -> u8 {
		let prev = self.current;

		// Check to see if advancing will run off the end of the input
		if self.offset + 1 >= self.text.len() {
			self.offset = self.text.len();
			self.eof = true;
			self.current = 0;
			return prev;
		}

		// Increment the position and set the current character
		self.offset += 1;
		self.position.next_char();
		self.current = self.text[self.offset];

		prev
	}

	/// Advance the line counter and reset the character position to 0 of the context
	pub fn advance_line(&mut self) {
		self.position.next_line();
	}

	/// Returns the current byte offset into the input the tokenizer currently is
	pub fn offset(&self) -> usize {
		self.offset
	}

	/// Returns the current [`Position`] of the tokenizer
	pub fn position(&self) -> &Position {
		&self.position
	}

	/// Returns if the tokenizer has hit the end of the input or not
	pub fn is_eof(&self) -> bool {
		self.eof
	}

	/// Returns if the tokenizer is at the start of the input or not
	pub fn is_sof(&self) -> bool {
		self.offset == 0
	}

	/// Get the current character the tokenizer is on
	pub fn current_byte(&self) -> u8 {
		self.current
	}
}
