// SPDX-License-Identifier: BSD-3-Clause

use tendril::ByteTendril;

pub trait Tokenizer: Iterator {
	type Context;
	type CurrentChar;
	type Token;
}

/// The underlying tokenization machinery for all other Vermilion tokenizers
pub struct CoreTokenizer {
	text:     ByteTendril,
	position: usize,
	eof:      bool,
	current:  u8,
}

impl CoreTokenizer {
	/// Create a new [`CoreTokenizer`] with the given [`ByteTendril`] as it's backing store
	pub fn new(text: ByteTendril) -> Self {
		let mut tokenizer = Self { text, position: 0, eof: false, current: 0 };

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
		if self.position + 1 >= self.text.len() {
			self.position = self.text.len();
			self.eof = true;
			self.current = 0;
			return prev;
		}

		// Increment the position and set the current character
		self.position += 1;
		self.current = self.text[self.position];

		prev
	}

	/// Returns the current character position in the input
	pub fn position(&self) -> usize {
		self.position
	}

	/// Returns if the tokenizer has hit the end of the input or not
	pub fn is_eof(&self) -> bool {
		self.eof
	}

	/// Get the current character the tokenizer is on
	pub fn current_char(&self) -> u8 {
		self.current
	}
}
