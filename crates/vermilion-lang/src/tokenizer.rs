// SPDX-License-Identifier: BSD-3-Clause

use std::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::{AtomicByteTendril, Position};

#[macro_export]
macro_rules! simple_token {
	($self:path, $token:expr) => {{
		let context = $self.tokenizer.position();
		let begin = $self.tokenizer.offset();
		$self.tokenizer.next_char();

		$self.token = spanned_token!($token, begin..$self.tokenizer.offset(), context);
	}};
}

#[macro_export]
macro_rules! versioned_token {
	($self:path, $range:expr, $token:expr, $($stds:path),+) => {
		if ($($stds)|+).contains($self.standard) {
			$token
		} else {
			Token::ContextuallyInvalid($self.tokenizer.subtendril($range), $($stds)|+)
		}
	};
}

/// The underlying tokenization machinery for all other Vermilion tokenizers
pub struct CoreTokenizer {
	text:     AtomicByteTendril,
	offset:   usize,
	eof:      bool,
	current:  u8,
	position: Position,
}

// TODO(aki): We need to deal with UTF-8/UTF-16/UTF-32/WTF-8 inputs not just ASCII
impl CoreTokenizer {
	/// Create a new [`CoreTokenizer`] with the given [`AtomicByteTendril`] as it's backing store
	pub fn new(text: AtomicByteTendril) -> Self {
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
			tokenizer.current = tokenizer.text[tokenizer.offset]
		}

		tokenizer
	}

	// XXX(aki): I'm not sure that we should return the previous character, feels, off
	/// Advances the position in the input, returning the previous character
	/// and setting [`Self::current`] to the next character in the stream
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
	#[inline(always)]
	pub fn advance_line(&mut self) {
		self.position.next_line();
	}

	/// Returns the current byte offset into the input the tokenizer currently is
	#[inline(always)]
	pub fn offset(&self) -> usize {
		self.offset
	}

	/// Returns the current [`Position`] of the tokenizer
	#[inline(always)]
	pub fn position(&self) -> Position {
		self.position
	}

	/// Returns if the tokenizer has hit the end of the input or not
	#[inline(always)]
	pub fn is_eof(&self) -> bool {
		self.eof
	}

	/// Returns if the tokenizer is at the start of the input or not
	#[inline(always)]
	pub fn is_sof(&self) -> bool {
		self.offset == 0
	}

	/// Get the current character the tokenizer is on
	#[inline(always)]
	pub fn current_byte(&self) -> u8 {
		self.current
	}

	/// Get the length of the tokenizers text buffer
	#[inline(always)]
	pub fn len(&self) -> usize {
		self.text.len()
	}

	/// Check to see if the tokenizer text buffer is empty or not
	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.text.is_empty()
	}

	/// Get a subslice of [`Self::text`]
	///
	/// Panics on bounds or validity check failure.
	#[inline(always)]
	pub fn subtendril(&self, range: Range<usize>) -> AtomicByteTendril {
		self.text
			.subtendril(range.start as u32, (range.end - range.start) as u32)
	}
}

impl Index<usize> for CoreTokenizer {
	type Output = u8;

	#[inline(always)]
	fn index(&self, index: usize) -> &Self::Output {
		&self.text[index]
	}
}

impl Index<RangeFull> for CoreTokenizer {
	type Output = [u8];

	#[inline(always)]
	fn index(&self, index: RangeFull) -> &Self::Output {
		&self.text[index]
	}
}

impl Index<Range<usize>> for CoreTokenizer {
	type Output = [u8];

	#[inline(always)]
	fn index(&self, index: Range<usize>) -> &Self::Output {
		&self.text[index]
	}
}

impl Index<RangeFrom<usize>> for CoreTokenizer {
	type Output = [u8];

	#[inline(always)]
	fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
		&self.text[index]
	}
}

impl Index<RangeTo<usize>> for CoreTokenizer {
	type Output = [u8];

	#[inline(always)]
	fn index(&self, index: RangeTo<usize>) -> &Self::Output {
		&self.text[index]
	}
}

impl Index<RangeInclusive<usize>> for CoreTokenizer {
	type Output = [u8];

	#[inline(always)]
	fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
		&self.text[index]
	}
}

impl Index<RangeToInclusive<usize>> for CoreTokenizer {
	type Output = [u8];

	#[inline(always)]
	fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
		&self.text[index]
	}
}
