/* SPDX-License-Identifier: BSD-3-Clause */

use std::{fmt::Display, ops::Range};

use tendril::ByteTendril;

use crate::VerilogVariant;
use self::token::Token;

use vermilion_lang::parser::{Span, Spanned};

pub(crate) mod token;

pub(crate) struct Tokenizer {
	standard: VerilogVariant,
	file: ByteTendril,
	current_char: u8,
	position: usize,
	context: Position,
	eof: bool,
	token: Spanned<Token, Position>,
	token_stream: Vec<Spanned<Token, Position>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	line: usize,
	character: usize,
}

impl Tokenizer {
	pub fn new(standard: VerilogVariant, file: ByteTendril) -> Tokenizer {
		let mut tokenizer = Self {
			standard,
			file,
			current_char: 0,
			position: 0,
			context: Position { line: 0, character: 0 },
			eof: false,
			token: Spanned::new(Token::default(), None),
			token_stream: Vec::new(),
		};
		tokenizer.current_char = tokenizer.file[tokenizer.position];

		tokenizer
	}

	fn next_char(&mut self) -> u8 {
		// Copy the current character value to return at the end
		let value = self.current_char;
		// Check for EOF and handle
		if self.position + 1 == self.file.len() {
			self.eof = true;
			self.current_char = 0;
			return value;
		}

		// If we're not at the end of the file, get the next character from Tendril and step forward
		// in the file - both position and contextually.
		self.position += 1;
		self.context.next_char();
		self.current_char = self.file[self.position];
		value
	}

	#[inline]
	fn current_is_whitespace(&self) -> bool {
		self.current_char == b' ' || self.current_char == b'\t'
	}

	fn read_token(&mut self) {
		if !self.token_stream.is_empty() {
			// This has to work or the stream is empty, thus the if condition fails.
			self.token = self.token_stream.pop().unwrap();
			return;
		}
		let context = self.context;
		let begin = self.position;
		match self.current_char {
			b' ' | b'\t' => {
				self.read_whitespace();
				return;
			},
			b'\r' | b'\n' => {
				self.read_newline();
				return;
			},
			b'@' => self.token = Token::Control(token::Control::At).into(),
			b'#' => self.token = Token::Control(token::Control::Octothorp).into(),
			b'(' => self.token = Token::Control(token::Control::ParenOpen).into(),
			b')' => self.token = Token::Control(token::Control::ParenClose).into(),
			b'[' => self.token = Token::Control(token::Control::BracketOpen).into(),
			b']' => self.token = Token::Control(token::Control::BracketClose).into(),
			b'{' => self.token = Token::Control(token::Control::BraceOpen).into(),
			b'}' => self.token = Token::Control(token::Control::BraceClose).into(),
			b':' => self.token = Token::Control(token::Control::Colon).into(),
			b';' => self.token = Token::Control(token::Control::Semicolon).into(),
			b',' => self.token = Token::Control(token::Control::Comma).into(),
			b'.' => self.token = Token::Control(token::Control::Dot).into(),
			b'$' => self.token = Token::Control(token::Control::Dollar).into(),
			_ => {
				self.read_extended_token();
				return;
			}
		}
		self.next_char();
		let end = self.position;
		self.token.attach_span(Span::new(begin..end, context));
	}

	fn read_extended_token(&mut self) {
		let context = self.context;
		if self.current_char.is_ascii_alphabetic() || self.current_char == b'_' {
			let range = self.read_normal_ident();
			// We've already validated via the above read, that the entire token is valid UTF-8. Just make it a string.
			let ident = unsafe { str::from_utf8_unchecked(&self.file[range.clone()]) };
			let token = match ident {
				"always" => Token::Keyword(token::Keyword::Always),
				"assign" => Token::Keyword(token::Keyword::Assign),
				_ => Token::Identifier(self.file.subtendril(range.start as u32, range.len() as u32)),
			};
			// Turn the result into the final token to return
			self.token = Spanned::new(token, Some(Span::new(range, context)));
		} else if self.current_char == b'\\' {
			let range = self.read_escaped_ident();
			self.token = Spanned::new(
				Token::Identifier(self.file.subtendril(range.start as u32, range.len() as u32)),
				Some(Span::new(range, context))
			);
		} else if self.current_char.is_ascii_digit() || self.current_char == b'\'' {
			self.read_number_token();
		}
	}

	fn read_whitespace(&mut self) {
		let context = self.context;
		let begin = self.position;
		while match self.current_char { b' ' | b'\t' => true, _ => false } {
			self.next_char();
		}
		let end = self.position;
		self.token = Spanned::new(
			Token::Whitespace(self.file.subtendril(begin as u32, (end - begin) as u32)),
			Some(Span::new(begin..end, context))
		)
	}

	fn read_newline(&mut self) {
		let context = self.context;
		let begin = self.position;
		// Handle \r\n sequences
		if self.next_char() == b'\r' && self.current_char == b'\n' {
			self.next_char();
		}
		let end = self.position;
		// Newlines reset the position context for the next token
		self.context.next_line();
		self.token = Spanned::new(
			Token::Newline(self.file.subtendril(begin as u32, (end - begin) as u32)),
			Some(Span::new(begin..end, context))
		)
	}

	fn read_number_token(&mut self) {
		// Pop out a size if available
		if self.current_char.is_ascii_digit() {
			let context = self.context;
			let begin = self.position;
			// Consume digits while we find valid unsigned number digits
			while self.current_char.is_ascii_digit() || self.current_char == b'_' {
				self.next_char();
			}
			// Stuff the unsigned number token onto the token stack
			self.token_stream.push(
				Spanned::new(
					Token::UnsignedNumber(
						self.file.subtendril(begin as u32, (self.position - begin) as u32)
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		};
		// Deal with any whitespace that comes after the size
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push(self.token.clone())
		}
		// If we get here and we don't yet have a `'`,  we've consumed a decimal number - we're done.
		if self.current_char != b'\'' {
			// If we're here, we have to have pushed stuff to the token stream, so this is always okay.
			self.token = self.token_stream.pop().unwrap();
			return;
		}

		// Grab the position for and start matching a base specifier
		let context = self.context;
		let begin = self.position;
		self.next_char();
		// Having consumed the `'` we should now be left with a base specifier, and if not then
		// this is an invalid token.
		match self.current_char {
			b'b' | b'B' => self.read_binary_token(context, begin),
			b'o' | b'O' => self.read_octal_token(context, begin),
			b'd' | b'D' => self.read_decimal_token(context, begin),
			b'h' | b'H' => self.read_hexadecimal_token(context, begin),
			_ => {
				self.token_stream.push(
					Spanned::new(
						Token::Invalid(
							Some(self.file.subtendril(begin as u32, (self.position - begin) as u32))
						),
						Some(Span::new(begin..self.position, context))
					)
				);
			},
		};
		// Having processed all of the number tokens, pop the first as the result token.
		// The only way we get here is having pushed at least something onto the token stream.
		self.token = self.token_stream.pop().unwrap();
	}

	fn read_binary_token(&mut self, context: Position, begin: usize) {
		let upper_case = self.next_char() == b'B';
		self.token_stream.push(
			Spanned::new(
				Token::BaseSpecifier(token::BaseSpecifier::Binary, upper_case),
				Some(Span::new(begin..self.position, context))
			)
		);
		// Deal with any whitespace that comes after the base specifier
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push(self.token.clone())
		}

		fn is_binary_digit(current_char: u8) -> bool {
			match current_char {
				b'x' | b'X' |
				b'z' | b'Z' | b'?' |
				b'0' | b'1' => true,
				_ => false,
			}
		}

		let context = self.context;
		let begin = self.position;
		// Make sure we've got a valid number - if we have not, eat the content and turn it into an Invalid token.
		if !is_binary_digit(self.current_char) {
			// Keep monching until we find whitespace.
			while !self.current_is_whitespace() {
				self.next_char();
			}

			// Turn whatever we got into an invalid token
			self.token_stream.push(
				Spanned::new(
					Token::Invalid(
						Some(self.file.subtendril(begin as u32, (self.position - begin) as u32))
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		} else {
			// Monch digits till we don't find no more.
			while is_binary_digit(self.current_char) || self.current_char == b'_' {
				self.next_char();
			}

			// Turn the consumed number into a Number token
			self.token_stream.push(
				Spanned::new(
					Token::Number(
						self.file.subtendril(begin as u32, (self.position - begin) as u32)
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		}
	}

	fn read_octal_token(&mut self, context: Position, begin: usize) {
		let upper_case = self.next_char() == b'O';
		self.token_stream.push(
			Spanned::new(
				Token::BaseSpecifier(token::BaseSpecifier::Octal, upper_case),
				Some(Span::new(begin..self.position, context))
			)
		);
		// Deal with any whitespace that comes after the base specifier
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push(self.token.clone())
		}

		fn is_octal_digit(current_char: u8) -> bool {
			match current_char {
				b'x' | b'X' |
				b'z' | b'Z' | b'?' |
				b'0'..=b'7' => true,
				_ => false,
			}
		}

		let context = self.context;
		let begin = self.position;
		// Make sure we've got a valid number - if we have not, eat the content and turn it into an Invalid token.
		if !is_octal_digit(self.current_char) {
			// Keep monching until we find whitespace.
			while !self.current_is_whitespace() {
				self.next_char();
			}

			// Turn whatever we got into an invalid token
			self.token_stream.push(
				Spanned::new(
					Token::Invalid(
						Some(self.file.subtendril(begin as u32, (self.position - begin) as u32))
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		} else {
			// Monch digits till we don't find no more.
			while is_octal_digit(self.current_char) || self.current_char == b'_' {
				self.next_char();
			}

			// Turn the consumed number into a Number token
			self.token_stream.push(
				Spanned::new(
					Token::Number(
						self.file.subtendril(begin as u32, (self.position - begin) as u32)
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		}
	}

	fn read_decimal_token(&mut self, context: Position, begin: usize) {
		let upper_case = self.next_char() == b'D';
		self.token_stream.push(
			Spanned::new(
				Token::BaseSpecifier(token::BaseSpecifier::Decimal, upper_case),
				Some(Span::new(begin..self.position, context))
			)
		);
		// Deal with any whitespace that comes after the base specifier
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push(self.token.clone())
		}

		fn is_decimal_digit(current_char: u8) -> bool {
			match current_char {
				b'0'..=b'9' => true,
				_ => false,
			}
		}

		let context = self.context;
		let begin = self.position;
		// Make sure we've got a valid number - if we have not, eat the content and turn it into an Invalid token.
		if !is_decimal_digit(self.current_char) {
			// Keep monching until we find whitespace.
			while !self.current_is_whitespace() {
				self.next_char();
			}

			// Turn whatever we got into an invalid token
			self.token_stream.push(
				Spanned::new(
					Token::Invalid(
						Some(self.file.subtendril(begin as u32, (self.position - begin) as u32))
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		} else {
			// Monch digits till we don't find no more.
			while is_decimal_digit(self.current_char) || self.current_char == b'_' {
				self.next_char();
			}

			// Turn the consumed number into a Number token
			self.token_stream.push(
				Spanned::new(
					Token::Number(
						self.file.subtendril(begin as u32, (self.position - begin) as u32)
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		}
	}

	fn read_hexadecimal_token(&mut self, context: Position, begin: usize) {
		let upper_case = self.next_char() == b'H';
		self.token_stream.push(
			Spanned::new(
				Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, upper_case),
				Some(Span::new(begin..self.position, context))
			)
		);
		// Deal with any whitespace that comes after the base specifier
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push(self.token.clone())
		}

		fn is_hexadecimal_digit(current_char: u8) -> bool {
			match current_char {
				b'x' | b'X' |
				b'z' | b'Z' | b'?' |
				b'0'..=b'9' |
				b'a'..=b'f' |
				b'A'..=b'F'  => true,
				_ => false,
			}
		}

		let context = self.context;
		let begin = self.position;
		// Make sure we've got a valid number - if we have not, eat the content and turn it into an Invalid token.
		if !is_hexadecimal_digit(self.current_char) {
			// Keep monching until we find whitespace.
			while !self.current_is_whitespace() {
				self.next_char();
			}

			// Turn whatever we got into an invalid token
			self.token_stream.push(
				Spanned::new(
					Token::Invalid(
						Some(self.file.subtendril(begin as u32, (self.position - begin) as u32))
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		} else {
			// Monch digits till we don't find no more.
			while is_hexadecimal_digit(self.current_char) || self.current_char == b'_' {
				self.next_char();
			}

			// Turn the consumed number into a Number token
			self.token_stream.push(
				Spanned::new(
					Token::Number(
						self.file.subtendril(begin as u32, (self.position - begin) as u32)
					),
					Some(Span::new(begin..self.position, context))
				)
			);
		}
	}

	#[inline]
	fn read_normal_ident(&mut self) -> Range<usize> {
		let begin = self.position;
		// Scan through till we get something that's not a-zA-Z0-9_$
		while self.current_char.is_ascii_alphanumeric() || self.current_char == b'_' || self.current_char == b'$' {
			self.next_char();
		}
		// Return the range consumed
		begin..self.position
	}

	#[inline]
	fn read_escaped_ident(&mut self) -> Range<usize> {
		let begin = self.position;
		// Scan through till we get something that's not ASCII printable
		while match self.current_char as char {
			'a'..='z' | 'A'..='Z' | '0'..='9' | '!' | '"' | '#' | '$' | '%' | '&' | '\'' |
			'(' | ')' | '*' | '+' | ',' | '-' | '.' | '/' | '@' | '[' | ']' | '^' | '_' |
			'`' | '\\' => true,
			_ => false,
		} {
			self.next_char();
		}
		// Return the range consumed
		begin..self.position
	}
}

impl Iterator for Tokenizer {
	type Item = Spanned<Token, Position>;

	fn next(&mut self) -> Option<Self::Item> {
		// If we hit the end of the file, we've nothing more to give
		if self.eof {
			return None
		}
		self.read_token();
		Some(self.token.clone())
	}
}

impl Position {
	pub fn next_line(&mut self) {
		self.character = 0;
		self.line += 1;
	}

	pub fn next_char(&mut self) {
		self.character += 1;
	}
}

impl Display for Position {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "line {}, character {}", self.line, self.character)
	}
}
