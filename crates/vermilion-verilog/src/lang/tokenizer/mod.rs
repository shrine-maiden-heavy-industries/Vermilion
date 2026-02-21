// SPDX-License-Identifier: BSD-3-Clause

use std::{collections::VecDeque, ops::Range};

use vermilion_lang::{AtomicByteTendril, Position, Span, Spanned, spanned_token};

use self::token::{BaseSpecifier, Comment, CompilerDirective, Control, Operator, TextMacro, Token};
use crate::{LanguageSet, VerilogVariant};

mod directives;
mod keywords;
pub mod token;

macro_rules! versioned_token {
	($self:path, $begin:path, $token:expr,at_least_vl01) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::Vl01 |
				crate::LanguageSet::Vl05 |
				crate::LanguageSet::SYSTEM_VERILOG_STDS |
				crate::LanguageSet::VERILOG_AMS_STDS
		)
	};
	($self:path, $begin:path, $token:expr,at_least_vl05) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::Vl05 |
				crate::LanguageSet::SYSTEM_VERILOG_STDS |
				crate::LanguageSet::VERILOG_AMS_STDS
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv05) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::SYSTEM_VERILOG_STDS
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv09) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::Sv09 |
				crate::LanguageSet::Sv12 |
				crate::LanguageSet::Sv17 |
				crate::LanguageSet::Sv23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv12) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::Sv12 | crate::LanguageSet::Sv17 | crate::LanguageSet::Sv23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv17) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::Sv17 | crate::LanguageSet::Sv23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv23) => {
		versioned_token!($self, $begin, $token, crate::LanguageSet::Sv23)
	};
	($self:path, $begin:path, $token:expr,at_least_vams09) => {
		versioned_token!($self, $begin, $token, crate::LanguageSet::VERILOG_AMS_STDS)
	};
	($self:path, $begin:path, $token:expr,at_least_vams14) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::Vams14 | crate::LanguageSet::Vams23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_vams23) => {
		versioned_token!($self, $begin, $token, crate::LanguageSet::Vams23)
	};
	($self:path, $begin:path, $token:expr,only_verilog) => {
		versioned_token!($self, $begin, $token, crate::LanguageSet::VERILOG_STDS)
	};
	($self:path, $begin:path, $token:expr,only_system_verilog) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageSet::SYSTEM_VERILOG_STDS
		)
	};
	($self:path, $begin:path, $token:expr,only_verilog_ams) => {
		versioned_token!($self, $begin, $token, crate::LanguageSet::VERILOG_AMS_STDS)
	};
	($self:path, $begin:path, $token:expr, $stds:expr) => {
		if $stds.contains($self.standard.into()) {
			$token
		} else {
			Token::ContextuallyInvalid(
				$self
					.file
					.subtendril($begin as u32, ($self.position - $begin) as u32),
				$stds,
			)
		}
	};
}

pub struct VerilogTokenizer {
	standard:     VerilogVariant,
	file:         AtomicByteTendril,
	current_char: u8,
	position:     usize,
	context:      Position,
	eof:          bool,
	token:        Spanned<Token, Position>,
	token_stream: VecDeque<Spanned<Token, Position>>,
}

impl VerilogTokenizer {
	pub fn new(standard: VerilogVariant, file: AtomicByteTendril) -> VerilogTokenizer {
		let mut tokenizer = Self {
			standard,
			file,
			current_char: 0,
			position: 0,
			context: Position::sof(),
			eof: false,
			token: spanned_token!(Token::default()),
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

	pub fn document_length(&self) -> usize {
		self.file.len()
	}

	fn next_char(&mut self) -> u8 {
		// Copy the current character value to return at the end
		let value = self.current_char;
		// Check for EOF and handle
		if self.position + 1 >= self.file.len() {
			self.position = self.file.len();
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
		matches!(self.current_char, b' ' | b'\t')
	}

	#[inline]
	fn current_is_newline(&self) -> bool {
		matches!(self.current_char, b'\r' | b'\n')
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
			b'@' => self.token = Token::Control(Control::At).into(),
			b'#' => self.token = Token::Control(Control::Octothorp).into(),
			b'(' => {
				self.read_paren_open_token();
				return;
			},
			b')' => self.token = Token::Control(Control::ParenClose).into(),
			b'[' => self.token = Token::Control(Control::BracketOpen).into(),
			b']' => self.token = Token::Control(Control::BracketClose).into(),
			b'{' => self.token = Token::Control(Control::BraceOpen).into(),
			b'}' => self.token = Token::Control(Control::BraceClose).into(),
			b':' => {
				self.read_colon_token();
				return;
			},
			b';' => self.token = Token::Control(Control::Semicolon).into(),
			b',' => self.token = Token::Control(Control::Comma).into(),
			b'.' => self.token = Token::Control(Control::Dot).into(),
			b'$' => self.token = Token::Control(Control::Dollar).into(),
			b'?' => self.token = Token::Control(Control::Question).into(),
			b'!' => {
				self.read_exclame_token();
				return;
			},
			b'=' => {
				self.read_equals_token();
				return;
			},
			b'&' => {
				self.read_ampersand_token();
				return;
			},
			b'~' => {
				self.read_tilde_token();
				return;
			},
			b'^' => {
				self.read_circumflex_token();
				return;
			},
			b'|' => {
				self.read_pipe_token();
				return;
			},
			b'>' => {
				self.read_less_than_token();
				return;
			},
			b'<' => {
				self.read_greater_than_token();
				return;
			},
			b'%' => {
				self.read_percent_token();
				return;
			},
			b'*' => {
				self.read_asterisk_token();
				return;
			},
			b'+' => {
				self.read_plus_token();
				return;
			},
			b'-' => {
				self.read_minus_token();
				return;
			},
			b'/' => {
				self.read_solidus_token();
				return;
			},
			b'`' => {
				self.read_grave_token();
				return;
			},
			b'"' => {
				self.read_quote_token();
				return;
			},
			b'\'' => {
				self.read_apostrophe_token();
				return;
			},
			_ => {
				self.read_extended_token();
				return;
			},
		}
		self.next_char();
		let end = self.position;
		self.token.attach_span(Span::new(begin..end, context));
	}

	fn read_extended_token(&mut self) {
		let context = self.context;
		if self.current_char.is_ascii_alphabetic() || self.current_char == b'_' {
			let range = self.read_normal_ident();
			// We've already validated via the above read, that the entire token is valid UTF-8.
			// Just make it a string.
			let ident = unsafe { str::from_utf8_unchecked(&self.file[range.clone()]) };

			// Check to see if the given identifier is a keyword or not
			let keyword = keywords::get_keyword(ident, self.standard);
			let token = match keyword {
				Some(keyword) => Token::Keyword(keyword),
				None => {
					Token::Identifier(self.file.subtendril(range.start as u32, range.len() as u32))
				},
			};
			// Turn the result into the final token to return
			self.token = spanned_token!(token, range, context);
		} else if self.current_char == b'\\' {
			let range = self.read_escaped_ident();
			self.token = spanned_token!(
				Token::Identifier(self.file.subtendril(range.start as u32, range.len() as u32)),
				range,
				context
			);
		} else if self.current_char.is_ascii_digit() {
			self.read_number_token(false);
		}
	}

	fn read_whitespace(&mut self) {
		let context = self.context;
		let begin = self.position;

		while matches!(self.current_char, b' ' | b'\t') {
			self.next_char();
		}

		self.token = spanned_token!(
			Token::Whitespace(
				self.file
					.subtendril(begin as u32, (self.position - begin) as u32)
			),
			begin..self.position,
			context
		)
	}

	fn read_newline(&mut self) {
		let context = self.context;
		let begin = self.position;

		// Handle \r\n sequences
		if self.next_char() == b'\r' && self.current_char == b'\n' {
			self.next_char();
		}

		// Newlines reset the position context for the next token
		self.context.next_line();
		self.token = spanned_token!(
			Token::Newline(
				self.file
					.subtendril(begin as u32, (self.position - begin) as u32)
			),
			begin..self.position,
			context
		)
	}

	fn read_paren_open_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'*' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Control(Control::AttributeOpen),
						at_least_vl01
					)
				},
				_ => Token::Control(Control::ParenOpen),
			},
			begin..self.position,
			context
		);
	}

	fn read_colon_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b':' => {
					self.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::ClassScopeResolution),
						at_least_sv05
					)
				},
				b'/' => {
					self.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::WeightAssignDist),
						at_least_sv05
					)
				},
				b'=' => {
					self.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::WeightAssignUnit),
						at_least_sv05
					)
				},
				_ => Token::Control(Control::Colon),
			},
			begin..self.position,
			context
		);
	}

	fn read_exclame_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'=' => {
					self.next_char();

					match self.current_char {
						b'=' => {
							self.next_char();
							Token::Operator(Operator::CaseInequality)
						},
						_ => Token::Operator(Operator::LogicalInequality),
					}
				},
				_ => Token::Operator(Operator::Exclamation),
			},
			begin..self.position,
			context
		);
	}

	fn read_equals_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'=' => {
					self.next_char();
					if self.current_char == b'=' {
						self.next_char();

						Token::Operator(Operator::CaseEquality)
					} else {
						Token::Operator(Operator::LogicalEquality)
					}
				},
				b'>' => {
					self.next_char();

					Token::Operator(Operator::ParallelConnection)
				},
				_ => Token::Operator(Operator::Equals),
			},
			begin..self.position,
			context
		);
	}

	fn read_ampersand_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'&' => {
					self.next_char();

					if self.current_char == b'&' {
						self.next_char();

						Token::Operator(Operator::TripleAnd)
					} else {
						Token::Operator(Operator::LogicalAnd)
					}
				},
				b'=' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::AndEquals),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Ampersand),
			},
			begin..self.position,
			context
		);
	}

	fn read_tilde_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'^' => {
					self.next_char();

					Token::Operator(Operator::TildeCircumflex(false))
				},
				b'&' => {
					self.next_char();

					Token::Operator(Operator::ReductionNand)
				},
				b'|' => {
					self.next_char();

					Token::Operator(Operator::ReductionNor)
				},
				_ => Token::Operator(Operator::Tilde),
			},
			begin..self.position,
			context
		);
	}

	fn read_circumflex_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'~' => {
					self.next_char();
					Token::Operator(Operator::TildeCircumflex(true))
				},
				_ => Token::Operator(Operator::Circumflex),
			},
			begin..self.position,
			context
		);
	}

	fn read_pipe_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'|' => {
					self.next_char();

					Token::Operator(Operator::LogicalOr)
				},
				b'=' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::OrEquals),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Pipe),
			},
			begin..self.position,
			context
		);
	}

	fn read_less_than_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'=' => {
					self.next_char();
					Token::Operator(Operator::LessThanEqual)
				},
				b'>' => {
					self.next_char();

					match self.current_char {
						b'>' => {
							self.next_char();

							if self.current_char == b'=' {
								self.next_char();

								versioned_token!(
									self,
									begin,
									Token::Operator(Operator::ArithmeticShrEquals),
									at_least_sv05
								)
							} else {
								versioned_token!(
									self,
									begin,
									Token::Operator(Operator::ArithmeticShr),
									at_least_vl01
								)
							}
						},
						b'=' => {
							self.next_char();

							versioned_token!(
								self,
								begin,
								Token::Operator(Operator::ShiftRightEquals),
								at_least_sv05
							)
						},
						_ => Token::Operator(Operator::ShiftRight),
					}
				},
				_ => Token::Operator(Operator::LessThan),
			},
			begin..self.position,
			context
		);
	}

	fn read_greater_than_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'=' => {
					self.next_char();

					Token::Operator(Operator::GreaterThanEqual)
				},
				b'<' => {
					self.next_char();
					match self.current_char {
						b'<' => {
							self.next_char();

							if self.current_char == b'=' {
								self.next_char();

								versioned_token!(
									self,
									begin,
									Token::Operator(Operator::ArithmeticShlEquals),
									at_least_sv05
								)
							} else {
								versioned_token!(
									self,
									begin,
									Token::Operator(Operator::ArithmeticShl),
									at_least_vl01
								)
							}
						},
						b'=' => {
							self.next_char();

							versioned_token!(
								self,
								begin,
								Token::Operator(Operator::ShiftLeftEquals),
								at_least_sv05
							)
						},
						_ => Token::Operator(Operator::ShiftLeft),
					}
				},
				b'+' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::BranchContribution),
						only_verilog_ams
					)
				},
				_ => Token::Operator(Operator::GreaterThan),
			},
			begin..self.position,
			context
		)
	}

	fn read_percent_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'=' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::RemEquals),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Percent),
			},
			begin..self.position,
			context
		);
	}

	fn read_asterisk_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b')' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Control(Control::AttributeClose),
						at_least_vl01
					)
				},
				b'*' => {
					self.next_char();

					versioned_token!(self, begin, Token::Operator(Operator::Pow), at_least_vl01)
				},
				b'=' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::MulEquals),
						at_least_sv05
					)
				},
				b'>' => {
					self.next_char();

					Token::Operator(Operator::FullConnection)
				},
				_ => Token::Operator(Operator::Asterisk),
			},
			begin..self.position,
			context
		)
	}

	fn read_plus_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b':' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::IndexedPartPos),
						at_least_vl01
					)
				},
				b'=' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::AddEquals),
						at_least_sv05
					)
				},
				b'+' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::Increment),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Plus),
			},
			begin..self.position,
			context
		)
	}

	fn read_minus_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		self.token = spanned_token!(
			match self.current_char {
				b'>' => {
					self.next_char();

					Token::Operator(Operator::EventTrigger)
				},
				b':' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::IndexedPartNeg),
						at_least_vl01
					)
				},
				b'=' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::SubEquals),
						at_least_sv05
					)
				},
				b'-' => {
					self.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::Decrement),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Minus),
			},
			begin..self.position,
			context
		)
	}

	fn read_solidus_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char(); // Consume the opening `/`

		if matches!(self.current_char, b'*' | b'/') {
			if self.next_char() == b'*' {
				self.read_multiline_comment(context, begin);
			} else {
				self.read_singleline_comment(context, begin);
			}
		} else if self.current_char == b'=' {
			self.next_char();

			self.token = spanned_token!(
				versioned_token!(
					self,
					begin,
					Token::Operator(Operator::DivEquals),
					at_least_sv05
				),
				begin..self.position,
				context
			);
		} else {
			self.token = spanned_token!(
				Token::Operator(Operator::Solidus),
				begin..self.position,
				context
			);
		}
	}

	fn read_multiline_comment(&mut self, context: Position, begin: usize) {
		let mut invalid_comment = false;

		// Consume the multiline comment
		loop {
			if self.eof {
				invalid_comment = true;
				break;
			}

			match self.current_char {
				b'*' => {
					self.next_char();
					// Check if we are ending the comment
					if self.current_char == b'/' {
						self.next_char();
						break;
					}
				},
				b'/' => {
					self.next_char();

					// Check to see if it's an illegal start of a new multiline comment
					if self.current_char == b'*' {
						invalid_comment = true;
					}
				},
				_ => {
					self.next_char();
				},
			}
		}

		if invalid_comment {
			self.token = spanned_token!(
				Token::Comment(Comment::Invalid(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				)),
				begin..self.position,
				context
			);
		} else {
			self.token = spanned_token!(
				Token::Comment(Comment::MultiLine(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				)),
				begin..self.position,
				context
			);
		}
	}

	fn read_singleline_comment(&mut self, context: Position, begin: usize) {
		while !matches!(self.current_char, b'\r' | b'\n') && !self.eof {
			self.next_char();
		}

		self.token = spanned_token!(
			Token::Comment(Comment::SingleLine(
				self.file
					.subtendril((begin + 2) as u32, (self.position - (begin + 2)) as u32),
			)),
			begin..self.position,
			context
		);
	}

	fn read_grave_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char(); // Skip the '`'

		// Check to make sure the character right after the ` is valid
		if !matches!(self.current_char, b'a'..=b'z' | b'A'..=b'Z' | b'_') {
			self.token = spanned_token!(
				Token::Invalid(Some(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				)),
				begin..self.position,
				context
			);

			return;
		}

		// We validated the first char, consume it and ingest the rest of the identifier
		self.next_char();

		while (self.current_char.is_ascii_alphanumeric() || self.current_char == b'_') && !self.eof
		{
			self.next_char();
		}

		let ident_range = begin + 1..=(self.position - (begin + 1));
		// We've already validated via the above read, that the entire token is valid UTF-8.
		// Just make it a string.
		let ident = unsafe { str::from_utf8_unchecked(&self.file[ident_range.clone()]) };

		self.token = if let Some(directive) = directives::get_directive(ident, self.standard) {
			self.token_stream.push_back(spanned_token!(
				Token::CompilerDirective(CompilerDirective::Name(directive)),
				begin..self.position,
				context
			));

			if self.current_is_whitespace() {
				self.read_whitespace();
				self.token_stream.push_back(self.token.clone())
			}

			// Consume arguments up until we get to a newline
			while !matches!(self.current_char, b'\r' | b'\n') && !self.eof {
				// Deal with  whitespace
				if self.current_is_whitespace() {
					self.read_whitespace();
					self.token_stream.push_back(self.token.clone())
				}

				let begin = self.position;
				let context = self.context;

				// Consume argument
				while self.is_ascii_printable() && !self.eof {
					self.next_char();
				}

				self.token_stream.push_back(spanned_token!(
					Token::CompilerDirective(CompilerDirective::Arg(
						self.file
							.subtendril(begin as u32, (self.position - begin) as u32),
					)),
					begin..self.position,
					context
				));
			}

			// Stuff the compiler directive name back front and center
			// SAFETY:
			// If we're here, we have to have pushed stuff to the token stream, so this is always
			// okay.
			#[allow(clippy::expect_used)]
			let token = self
				.token_stream
				.pop_front()
				.expect("Unable to pop token from token stream");
			token
		} else {
			spanned_token!(
				Token::TextMacro(
					if (crate::LanguageSet::SYSTEM_VERILOG_STDS & !crate::LanguageSet::Sv05 |
						crate::LanguageSet::Vams23)
						.contains(self.standard.into())
					{
						match ident {
							"__FILE__" => TextMacro::DunderFile,
							"__LINE__" => TextMacro::DunderLine,
							_ => TextMacro::Other(self.file.subtendril(
								*ident_range.start() as u32,
								*ident_range.end() as u32,
							)),
						}
					} else {
						TextMacro::Other(
							self.file
								.subtendril(*ident_range.start() as u32, *ident_range.end() as u32),
						)
					}
				),
				begin..self.position,
				context
			)
		};
	}

	// BUG(aki):
	// This is likely really fragile in some specific cases with miss-matching or many quotes in a
	// row. for instance, what would `""test""` tokenize as? How about `"test"""`, or `"""test""`,
	// etc. so we need to iron out this a bit eventually.
	fn read_quote_token(&mut self) {
		let context = self.context;
		let quote_begin = self.position;
		self.next_char(); // Consume the first `"`

		// Check to see if we're a triple-quoted string
		let triple_quote = if self.current_char == b'"' {
			self.next_char();
			self.next_char() == b'"'
		} else {
			false
		};

		let str_begin = self.position;

		while self.current_char != b'"' && !self.eof {
			// If we hit `\"` then consume it and keep going, all other
			// escapes are checked after tokenization
			if self.next_char() == b'\\' && self.current_char == b'"' {
				self.next_char();
			}
		}

		// Consume the last `"`
		let str_end = self.position;
		self.next_char();
		if triple_quote {
			self.next_char();
			self.next_char();
		}
		let quote_end = self.position;

		self.token = spanned_token!(
			if triple_quote {
				if LanguageSet::Sv23.contains(self.standard.into()) {
					Token::TripleQuotedString(
						self.file
							.subtendril(str_begin as u32, (str_end - str_begin) as u32),
					)
				} else {
					Token::ContextuallyInvalid(
						self.file
							.subtendril(quote_begin as u32, (quote_end - quote_begin) as u32),
						LanguageSet::Sv23,
					)
				}
			} else {
				Token::String(
					self.file
						.subtendril(str_begin as u32, (str_end - str_begin) as u32),
				)
			},
			quote_begin..quote_end,
			context
		)
	}

	fn read_apostrophe_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();

		// If there is a base specifier right after us, then it's a number token
		if matches!(
			self.current_char,
			b'b' | b'B' | b'o' | b'O' | b'd' | b'D' | b'h' | b'H'
		) {
			self.read_number_token(true);
		} else {
			self.token = spanned_token!(
				versioned_token!(
					self,
					begin,
					Token::Control(Control::Apostrophe),
					at_least_sv05
				),
				begin..self.position,
				context
			);
		}
	}

	fn read_number_token(&mut self, from_apostrophe: bool) {
		// Pop out a size if available
		if self.current_char.is_ascii_digit() {
			let context = self.context;
			let begin = self.position;
			// Consume digits while we find valid unsigned number digits
			while self.current_char.is_ascii_digit() || self.current_char == b'_' {
				self.next_char();
			}

			// If we next hit a `.` or an exponent indicator we need to treat this as
			// a Real number
			if matches!(self.current_char, b'.' | b'e' | b'E') {
				self.read_real_number_token(context, begin);
				return;
			}

			// Stuff the unsigned number token onto the token stack
			self.token_stream.push_back(spanned_token!(
				Token::UnsignedNumber(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				),
				begin..self.position,
				context
			));
		};

		// Deal with any whitespace that comes after the size
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push_back(self.token.clone())
		}

		// If we get here and we don't yet have a `'`,  we've consumed a decimal number - we're
		// done.
		if self.current_char != b'\'' && !from_apostrophe {
			// SAFETY:
			// If we're here, we have to have pushed stuff to the token stream, so this is always
			// okay.
			#[allow(clippy::expect_used)]
			let token = self
				.token_stream
				.pop_front()
				.expect("Unable to pop token from token stream");

			self.token = token;
			return;
		}

		// Grab the position for and start matching a base specifier
		let mut context = self.context;
		let mut begin = self.position;
		// If we're not coming from the `parse_apostrophe_token` then we need to advance
		if !from_apostrophe {
			self.next_char();
		} else {
			// If we're coming from the `parse_apostrophe_token` then we need to roll back
			// `begin` and `context`
			context = Position::new(*context.line(), context.character() - 1);
			begin -= 1;
		}

		// Having consumed the `'` we should now be left with a base specifier, and if not then
		// this is an invalid token.
		match self.current_char {
			b'b' | b'B' => self.read_based_token(context, begin, BaseSpecifier::Binary, |char| {
				matches!(char, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0' | b'1')
			}),
			b'o' | b'O' => self.read_based_token(context, begin, BaseSpecifier::Octal, |char| {
				matches!(char, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0'..=b'7')
			}),
			b'd' | b'D' => self.read_based_token(context, begin, BaseSpecifier::Decimal, |char| {
				char.is_ascii_digit()
			}),
			b'h' | b'H' => self.read_based_token(
				context,
				begin,
				BaseSpecifier::Hexadecimal,
				|c| matches!(c, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'),
			),
			_ => {
				self.token_stream.push_back(spanned_token!(
					Token::Invalid(Some(
						self.file
							.subtendril(begin as u32, (self.position - begin) as u32),
					)),
					begin..self.position,
					context
				));
			},
		};
		// Having processed all of the number tokens, pop the first as the result token.

		// SAFETY:
		// The only way we get here is having pushed at least something onto the token stream.
		#[allow(clippy::expect_used)]
		let token = self
			.token_stream
			.pop_front()
			.expect("Unable to pop token from token stream");

		self.token = token;
	}

	fn read_real_number_token(&mut self, context: Position, begin: usize) {
		// If it's a decimal point, consume the next set of digits
		if self.current_char == b'.' {
			self.next_char();
			// Consume digits while we find valid unsigned number digits
			while self.current_char.is_ascii_digit() || self.current_char == b'_' {
				self.next_char();
			}
		}

		// If we have an exponent part, consume that
		let exponent = if matches!(self.current_char, b'e' | b'E') {
			let exp_begin = self.position;
			self.next_char();

			// Check to make sure we have a valid exponent
			if !self.current_char.is_ascii_digit() && !matches!(self.current_char, b'+' | b'-') {
				self.token = spanned_token!(
					Token::Invalid(Some(
						self.file
							.subtendril(begin as u32, (self.position - begin) as u32),
					)),
					begin..self.position,
					context
				);

				return;
			}
			// Consume the + or - (or first digit)
			self.next_char();

			// Consume digits while we find valid unsigned number digits
			while self.current_char.is_ascii_digit() || self.current_char == b'_' {
				self.next_char();
			}

			Some(
				self.file
					.subtendril(exp_begin as u32, (self.position - exp_begin) as u32),
			)
		} else {
			None
		};

		// SAFETY:
		// If we got to this point, then we have already ensured the contents are
		// ASCII, which is a valid UTF-8 subset
		let value = unsafe { str::from_utf8_unchecked(&self.file[begin..self.position]) };

		// If we got a valid f64, then we use that, otherwise emit an invalid token
		self.token = if let Ok(value) = value.parse() {
			spanned_token!(
				Token::Real { value, exponent },
				begin..self.position,
				context
			)
		} else {
			spanned_token!(
				Token::Invalid(Some(value.as_bytes().into())),
				begin..self.position,
				context
			)
		}
	}

	fn read_based_token(
		&mut self,
		context: Position,
		begin: usize,
		spec: BaseSpecifier,
		digit_filter: fn(u8) -> bool,
	) {
		let upper_case = self.next_char().is_ascii_uppercase();
		self.token_stream.push_back(spanned_token!(
			Token::BaseSpecifier(spec, upper_case),
			begin..self.position,
			context
		));
		// Deal with any whitespace that comes after the base specifier
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push_back(self.token.clone())
		}

		let context = self.context;
		let begin = self.position;
		// Make sure we've got a valid number - if we have not, eat the content and turn it into an
		// Invalid token.
		if !digit_filter(self.current_char) {
			// keep consuming input until we hit whitespace or EOF
			while !self.current_is_whitespace() && !self.current_is_newline() && !self.eof {
				if self.current_is_newline() {
					self.read_newline();
				} else {
					self.next_char();
				}
			}

			// Turn whatever we got into an invalid token
			// TODO(aki):
			// In IEEE 1364-2001 (Verilog 2001) decimal bases ('d|'D) are allowed to
			// have [xz?] as the first digit, followed by `_` exclusively if it's not
			// a base-10 digit, so this should be turned into a Token::ContextuallyInvalid
			// in the case we hit one of those and are in Verilog 95.
			self.token_stream.push_back(spanned_token!(
				Token::Invalid(Some(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				)),
				begin..self.position,
				context
			));
		} else {
			// Run the filter to the end consuming all the digits we can
			while digit_filter(self.current_char) || self.current_char == b'_' {
				self.next_char();
			}

			// If we're consuming a decimal number, we need to ensure we also consume trailing
			// [xz?]'s to generate an invalid token if there are any
			if spec == BaseSpecifier::Decimal &&
				matches!(self.current_char, b'x' | b'X' | b'z' | b'Z' | b'?')
			{
				while matches!(self.current_char, b'x' | b'X' | b'z' | b'Z' | b'?') &&
					!self.current_is_whitespace() &&
					!self.current_is_newline() &&
					!self.eof
				{
					if self.current_is_newline() {
						self.read_newline();
					} else {
						self.next_char();
					}
				}
				self.token_stream.push_back(spanned_token!(
					Token::Invalid(Some(
						self.file
							.subtendril(begin as u32, (self.position - begin) as u32),
					)),
					begin..self.position,
					context
				));
			} else {
				// Turn the consumed number into a Number token
				self.token_stream.push_back(spanned_token!(
					Token::Number(
						self.file
							.subtendril(begin as u32, (self.position - begin) as u32),
					),
					begin..self.position,
					context
				));
			}
		}
	}

	#[inline]
	fn read_normal_ident(&mut self) -> Range<usize> {
		let begin = self.position;
		// Scan through till we get something that's not a-zA-Z0-9_$
		while self.current_char.is_ascii_alphanumeric() ||
			self.current_char == b'_' ||
			self.current_char == b'$'
		{
			self.next_char();
		}
		// Return the range consumed
		begin..self.position
	}

	#[inline(always)]
	fn is_ascii_printable(&self) -> bool {
		matches!(self.current_char as char, 'a'..='z'
			| 'A'..='Z'
			| '0'..='9'
			| '!'
			| '"'
			| '#'
			| '$'
			| '%'
			| '&'
			| '\''
			| '('
			| ')'
			| '*'
			| '+'
			| ','
			| '-'
			| '.'
			| '/'
			| '@'
			| '['
			| ']'
			| '^'
			| '_'
			| '`'
			| '\\')
	}

	#[inline]
	fn read_escaped_ident(&mut self) -> Range<usize> {
		let begin = self.position;
		// Scan through till we get something that's not ASCII printable
		while self.is_ascii_printable() {
			self.next_char();
		}
		// Return the range consumed
		begin..self.position
	}
}

impl Iterator for VerilogTokenizer {
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
