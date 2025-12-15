/* SPDX-License-Identifier: BSD-3-Clause */

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
		}
		self.next_char();
		let end = self.position;
		self.token.attach_span(Span::new(begin..end, context));
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
