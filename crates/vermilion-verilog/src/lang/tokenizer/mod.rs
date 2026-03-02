// SPDX-License-Identifier: BSD-3-Clause

use std::{collections::VecDeque, ops::Range};

use eyre::eyre;
use vermilion_lang::{
	AtomicByteTendril, Position, Span, Spanned, spanned_token, tokenizer::CoreTokenizer,
};

use self::token::{BaseSpecifier, Comment, CompilerDirective, Control, Operator, TextMacro, Token};
use crate::LanguageStd;

mod directives;
mod keywords;
pub mod token;

macro_rules! versioned_token {
	($self:path, $begin:path, $token:expr,at_least_vl01) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::Vl01 |
				crate::LanguageStd::Vl05 |
				crate::LanguageStd::SYSTEM_VERILOG_STDS |
				crate::LanguageStd::VERILOG_AMS_STDS
		)
	};
	($self:path, $begin:path, $token:expr,at_least_vl05) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::Vl05 |
				crate::LanguageStd::SYSTEM_VERILOG_STDS |
				crate::LanguageStd::VERILOG_AMS_STDS
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv05) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::SYSTEM_VERILOG_STDS
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv09) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::Sv09 |
				crate::LanguageStd::Sv12 |
				crate::LanguageStd::Sv17 |
				crate::LanguageStd::Sv23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv12) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::Sv12 | crate::LanguageStd::Sv17 | crate::LanguageStd::Sv23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv17) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::Sv17 | crate::LanguageStd::Sv23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_sv23) => {
		versioned_token!($self, $begin, $token, crate::LanguageStd::Sv23)
	};
	($self:path, $begin:path, $token:expr,at_least_vams09) => {
		versioned_token!($self, $begin, $token, crate::LanguageStd::VERILOG_AMS_STDS)
	};
	($self:path, $begin:path, $token:expr,at_least_vams14) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::Vams14 | crate::LanguageStd::Vams23
		)
	};
	($self:path, $begin:path, $token:expr,at_least_vams23) => {
		versioned_token!($self, $begin, $token, crate::LanguageStd::Vams23)
	};
	($self:path, $begin:path, $token:expr,only_verilog) => {
		versioned_token!($self, $begin, $token, crate::LanguageStd::VERILOG_STDS)
	};
	($self:path, $begin:path, $token:expr,only_system_verilog) => {
		versioned_token!(
			$self,
			$begin,
			$token,
			crate::LanguageStd::SYSTEM_VERILOG_STDS
		)
	};
	($self:path, $begin:path, $token:expr,only_verilog_ams) => {
		versioned_token!($self, $begin, $token, crate::LanguageStd::VERILOG_AMS_STDS)
	};
	($self:path, $begin:path, $token:expr, $stds:expr) => {
		if $stds.contains($self.standard.into()) {
			$token
		} else {
			Token::ContextuallyInvalid(
				$self.tokenizer.subtendril($begin..$self.tokenizer.offset()),
				$stds,
			)
		}
	};
}

pub struct VerilogTokenizer {
	standard:     LanguageStd,
	tokenizer:    CoreTokenizer,
	token:        Spanned<Token, Position>,
	token_stream: VecDeque<Spanned<Token, Position>>,
}

impl VerilogTokenizer {
	pub fn new(standard: LanguageStd, file: AtomicByteTendril) -> eyre::Result<VerilogTokenizer> {
		// Check to see if more than one language standard is set
		if !standard.has_single_std() {
			return Err(eyre!("More than one language standard set: {:?}", standard));
		}

		Ok(Self {
			standard,
			tokenizer: CoreTokenizer::new(file),
			token: spanned_token!(Token::default()),
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

	#[inline]
	fn current_is_whitespace(&self) -> bool {
		matches!(self.tokenizer.current_byte(), b' ' | b'\t')
	}

	#[inline]
	fn current_is_newline(&self) -> bool {
		matches!(self.tokenizer.current_byte(), b'\r' | b'\n')
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
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		match self.tokenizer.current_byte() {
			b' ' | b'\t' => {
				self.read_whitespace();
				return;
			},
			b'\r' | b'\n' => {
				self.read_newline();
				return;
			},
			b'@' => self.token = Token::Control(Control::At).into(),
			b'#' => {
				self.read_octothorp_token();
				return;
			},
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
			b'.' => {
				self.read_dot_token();
				return;
			},
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
		self.tokenizer.next_char();
		let end = self.tokenizer.offset();
		self.token.attach_span(Span::new(begin..end, context));
	}

	fn read_extended_token(&mut self) {
		let context = self.tokenizer.position();
		if self.tokenizer.current_byte().is_ascii_alphabetic() ||
			self.tokenizer.current_byte() == b'_'
		{
			let range = self.read_normal_ident();
			// We've already validated via the above read, that the entire token is valid UTF-8.
			// Just make it a string.
			let ident = unsafe { str::from_utf8_unchecked(&self.tokenizer[range.clone()]) };

			// Check to see if the given identifier is a keyword or not
			let keyword = keywords::get_keyword(ident, self.standard);
			let token = match keyword {
				Some(keyword) => Token::Keyword(keyword),
				None => Token::Identifier(self.tokenizer.subtendril(range.clone())),
			};
			// Turn the result into the final token to return
			self.token = spanned_token!(token, range, context);
		} else if self.tokenizer.current_byte() == b'\\' {
			let range = self.read_escaped_ident();
			self.token = spanned_token!(
				Token::Identifier(self.tokenizer.subtendril(range.clone())),
				range,
				context
			);
		} else if self.tokenizer.current_byte().is_ascii_digit() {
			self.read_number_token(false);
		}
	}

	fn read_whitespace(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();

		while matches!(self.tokenizer.current_byte(), b' ' | b'\t') {
			self.tokenizer.next_char();
		}

		let token_range = begin..self.tokenizer.offset();
		self.token = spanned_token!(
			Token::Whitespace(self.tokenizer.subtendril(token_range.clone())),
			token_range,
			context
		)
	}

	fn read_newline(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();

		// Handle \r\n sequences
		if self.tokenizer.next_char() == b'\r' && self.tokenizer.current_byte() == b'\n' {
			self.tokenizer.next_char();
		}

		// Newlines reset the position context for the next token
		self.tokenizer.advance_line();
		let token_range = begin..self.tokenizer.offset();
		self.token = spanned_token!(
			Token::Newline(self.tokenizer.subtendril(token_range.clone())),
			token_range,
			context
		)
	}

	fn read_octothorp_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'#' => {
					self.tokenizer.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::CycleDelay),
						at_least_sv05
					)
				},
				b'=' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'#' {
						self.tokenizer.next_char();

						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::FollowedByNonOverlapped),
							at_least_sv09
						)
					} else {
						return;
					}
				},
				b'-' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'#' {
						self.tokenizer.next_char();

						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::FollowedByOverlapped),
							at_least_sv09
						)
					} else {
						return;
					}
				},
				_ => Token::Control(Control::Octothorp),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_paren_open_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'*' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Control(Control::AttributeOpen),
						at_least_vl01
					)
				},
				_ => Token::Control(Control::ParenOpen),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_colon_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b':' => {
					self.tokenizer.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::ClassScopeResolution),
						at_least_sv05
					)
				},
				b'/' => {
					self.tokenizer.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::WeightAssignDist),
						at_least_sv05
					)
				},
				b'=' => {
					self.tokenizer.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::WeightAssignUnit),
						at_least_sv05
					)
				},
				_ => Token::Control(Control::Colon),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_dot_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'*' => {
					self.tokenizer.next_char();
					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::Wildcard),
						at_least_sv05
					)
				},
				_ => Token::Control(Control::Dot),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_exclame_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'=' => {
					self.tokenizer.next_char();

					match self.tokenizer.current_byte() {
						b'=' => {
							self.tokenizer.next_char();
							Token::Operator(Operator::CaseInequality)
						},
						b'?' => {
							self.tokenizer.next_char();
							versioned_token!(
								self,
								begin,
								Token::Operator(Operator::WildcardNotEqual),
								at_least_sv05
							)
						},
						_ => Token::Operator(Operator::LogicalInequality),
					}
				},
				_ => Token::Operator(Operator::Exclamation),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_equals_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'=' => {
					self.tokenizer.next_char();

					match self.tokenizer.current_byte() {
						b'=' => {
							self.tokenizer.next_char();

							Token::Operator(Operator::CaseEquality)
						},
						b'?' => {
							self.tokenizer.next_char();

							versioned_token!(
								self,
								begin,
								Token::Operator(Operator::WildcardEqual),
								at_least_sv05
							)
						},
						_ => Token::Operator(Operator::LogicalEquality),
					}
				},
				b'>' => {
					self.tokenizer.next_char();

					Token::Operator(Operator::ParallelConnection)
				},
				_ => Token::Operator(Operator::Equals),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_ampersand_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'&' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'&' {
						self.tokenizer.next_char();

						Token::Operator(Operator::TripleAnd)
					} else {
						Token::Operator(Operator::LogicalAnd)
					}
				},
				b'=' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::AndEquals),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Ampersand),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_tilde_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'^' => {
					self.tokenizer.next_char();

					Token::Operator(Operator::TildeCircumflex(false))
				},
				b'&' => {
					self.tokenizer.next_char();

					Token::Operator(Operator::ReductionNand)
				},
				b'|' => {
					self.tokenizer.next_char();

					Token::Operator(Operator::ReductionNor)
				},
				_ => Token::Operator(Operator::Tilde),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_circumflex_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'~' => {
					self.tokenizer.next_char();
					Token::Operator(Operator::TildeCircumflex(true))
				},
				b'=' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::XorEquals),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Circumflex),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_pipe_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'|' => {
					self.tokenizer.next_char();

					Token::Operator(Operator::LogicalOr)
				},
				b'=' => {
					self.tokenizer.next_char();
					if self.tokenizer.current_byte() == b'>' {
						self.tokenizer.next_char();
						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::PropImplNonOverlap),
							at_least_sv05
						)
					} else {
						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::OrEquals),
							at_least_sv05
						)
					}
				},
				b'-' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'>' {
						self.tokenizer.next_char();

						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::PropImplOverlap),
							at_least_sv05
						)
					} else {
						// Don't over-consume, the `-` might be valid in another context
						return;
					}
				},
				_ => Token::Operator(Operator::Pipe),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_less_than_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'=' => {
					self.tokenizer.next_char();
					Token::Operator(Operator::LessThanEqual)
				},
				b'>' => {
					self.tokenizer.next_char();

					match self.tokenizer.current_byte() {
						b'>' => {
							self.tokenizer.next_char();

							if self.tokenizer.current_byte() == b'=' {
								self.tokenizer.next_char();

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
							self.tokenizer.next_char();

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
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_greater_than_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'=' => {
					self.tokenizer.next_char();

					Token::Operator(Operator::GreaterThanEqual)
				},
				b'<' => {
					self.tokenizer.next_char();
					match self.tokenizer.current_byte() {
						b'<' => {
							self.tokenizer.next_char();

							if self.tokenizer.current_byte() == b'=' {
								self.tokenizer.next_char();

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
							self.tokenizer.next_char();

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
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::BranchContribution),
						only_verilog_ams
					)
				},
				b'-' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'>' {
						self.tokenizer.next_char();

						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::Equivalence),
							at_least_sv09
						)
					} else {
						return;
					}
				},
				_ => Token::Operator(Operator::GreaterThan),
			},
			begin..self.tokenizer.offset(),
			context
		)
	}

	fn read_percent_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'=' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::RemEquals),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Percent),
			},
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_asterisk_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b')' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Control(Control::AttributeClose),
						at_least_vl01
					)
				},
				b'*' => {
					self.tokenizer.next_char();

					versioned_token!(self, begin, Token::Operator(Operator::Pow), at_least_vl01)
				},
				b'=' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::MulEquals),
						at_least_sv05
					)
				},
				b'>' => {
					self.tokenizer.next_char();

					Token::Operator(Operator::FullConnection)
				},
				b':' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b':' {
						self.tokenizer.next_char();

						if self.tokenizer.current_byte() == b'*' {
							self.tokenizer.next_char();

							versioned_token!(
								self,
								begin,
								Token::Operator(Operator::WildcardExport),
								at_least_sv09
							)
						} else {
							// There should be no cases in which `*::` is valid
							Token::Invalid(Some(
								self.tokenizer.subtendril(begin..self.tokenizer.offset()),
							))
						}
					} else {
						// There should be no cases in which `*:` is valid
						Token::Invalid(Some(
							self.tokenizer.subtendril(begin..self.tokenizer.offset()),
						))
					}
				},
				_ => Token::Operator(Operator::Asterisk),
			},
			begin..self.tokenizer.offset(),
			context
		)
	}

	fn read_plus_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b':' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::IndexedPartPos),
						at_least_vl01
					)
				},
				b'=' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::AddEquals),
						at_least_sv05
					)
				},
				b'+' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::Increment),
						at_least_sv05
					)
				},
				b'/' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'-' {
						self.tokenizer.next_char();

						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::AbsTolerance),
							at_least_sv23
						)
					} else {
						Token::Invalid(Some(
							self.tokenizer.subtendril(begin..self.tokenizer.offset()),
						))
					}
				},
				b'%' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'-' {
						self.tokenizer.next_char();

						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::RelTolerance),
							at_least_sv23
						)
					} else {
						Token::Invalid(Some(
							self.tokenizer.subtendril(begin..self.tokenizer.offset()),
						))
					}
				},
				_ => Token::Operator(Operator::Plus),
			},
			begin..self.tokenizer.offset(),
			context
		)
	}

	fn read_minus_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		self.token = spanned_token!(
			match self.tokenizer.current_byte() {
				b'>' => {
					self.tokenizer.next_char();

					if self.tokenizer.current_byte() == b'>' {
						self.tokenizer.next_char();

						versioned_token!(
							self,
							begin,
							Token::Operator(Operator::EventTriggerNb),
							at_least_sv05
						)
					} else {
						Token::Operator(Operator::EventTrigger)
					}
				},
				b':' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::IndexedPartNeg),
						at_least_vl01
					)
				},
				b'=' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::SubEquals),
						at_least_sv05
					)
				},
				b'-' => {
					self.tokenizer.next_char();

					versioned_token!(
						self,
						begin,
						Token::Operator(Operator::Decrement),
						at_least_sv05
					)
				},
				_ => Token::Operator(Operator::Minus),
			},
			begin..self.tokenizer.offset(),
			context
		)
	}

	fn read_solidus_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char(); // Consume the opening `/`

		if matches!(self.tokenizer.current_byte(), b'*' | b'/') {
			if self.tokenizer.next_char() == b'*' {
				self.read_multiline_comment(context, begin);
			} else {
				self.read_singleline_comment(context, begin);
			}
		} else if self.tokenizer.current_byte() == b'=' {
			self.tokenizer.next_char();

			self.token = spanned_token!(
				versioned_token!(
					self,
					begin,
					Token::Operator(Operator::DivEquals),
					at_least_sv05
				),
				begin..self.tokenizer.offset(),
				context
			);
		} else {
			self.token = spanned_token!(
				Token::Operator(Operator::Solidus),
				begin..self.tokenizer.offset(),
				context
			);
		}
	}

	fn read_multiline_comment(&mut self, context: Position, begin: usize) {
		let mut invalid_comment = false;

		// Consume the multiline comment
		loop {
			if self.tokenizer.is_eof() {
				invalid_comment = true;
				break;
			}

			match self.tokenizer.current_byte() {
				b'*' => {
					self.tokenizer.next_char();
					// Check if we are ending the comment
					if self.tokenizer.current_byte() == b'/' {
						self.tokenizer.next_char();
						break;
					}
				},
				b'/' => {
					self.tokenizer.next_char();

					// Check to see if it's an illegal start of a new multiline comment
					if self.tokenizer.current_byte() == b'*' {
						invalid_comment = true;
					}
				},
				_ => {
					self.tokenizer.next_char();
				},
			}
		}

		let token_range = begin..self.tokenizer.offset();
		if invalid_comment {
			self.token = spanned_token!(
				Token::Comment(Comment::Invalid(
					self.tokenizer.subtendril(token_range.clone())
				)),
				token_range,
				context
			);
		} else {
			self.token = spanned_token!(
				Token::Comment(Comment::MultiLine(
					self.tokenizer.subtendril(token_range.clone())
				)),
				token_range,
				context
			);
		}
	}

	fn read_singleline_comment(&mut self, context: Position, begin: usize) {
		while !matches!(self.tokenizer.current_byte(), b'\r' | b'\n') && !self.tokenizer.is_eof() {
			self.tokenizer.next_char();
		}

		self.token = spanned_token!(
			Token::Comment(Comment::SingleLine(
				self.tokenizer
					.subtendril((begin + 2)..self.tokenizer.offset()),
			)),
			begin..self.tokenizer.offset(),
			context
		);
	}

	fn read_grave_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char(); // Skip the '`'

		// Check to make sure the character right after the ` is valid
		if !matches!(self.tokenizer.current_byte(), b'a'..=b'z' | b'A'..=b'Z' | b'_') {
			let token_range = begin..self.tokenizer.offset();
			self.token = spanned_token!(
				Token::Invalid(Some(self.tokenizer.subtendril(token_range.clone()))),
				token_range,
				context
			);

			return;
		}

		// We validated the first char, consume it and ingest the rest of the identifier
		self.tokenizer.next_char();

		while (self.tokenizer.current_byte().is_ascii_alphanumeric() ||
			self.tokenizer.current_byte() == b'_') &&
			!self.tokenizer.is_eof()
		{
			self.tokenizer.next_char();
		}

		let ident_range = begin + 1..=(self.tokenizer.offset() - (begin + 1));
		// We've already validated via the above read, that the entire token is valid UTF-8.
		// Just make it a string.
		let ident = unsafe { str::from_utf8_unchecked(&self.tokenizer[ident_range.clone()]) };

		self.token = if let Some(directive) = directives::get_directive(ident, self.standard) {
			self.token_stream.push_back(spanned_token!(
				Token::CompilerDirective(CompilerDirective::Name(directive)),
				begin..self.tokenizer.offset(),
				context
			));

			if self.current_is_whitespace() {
				self.read_whitespace();
				self.token_stream.push_back(self.token.clone())
			}

			// Consume arguments up until we get to a newline
			while !matches!(self.tokenizer.current_byte(), b'\r' | b'\n') &&
				!self.tokenizer.is_eof()
			{
				// Deal with  whitespace
				if self.current_is_whitespace() {
					self.read_whitespace();
					self.token_stream.push_back(self.token.clone())
				}

				let begin = self.tokenizer.offset();
				let context = self.tokenizer.position();

				// Consume argument
				while self.is_ascii_printable() && !self.tokenizer.is_eof() {
					self.tokenizer.next_char();
				}

				let token_range = begin..self.tokenizer.offset();
				self.token_stream.push_back(spanned_token!(
					Token::CompilerDirective(CompilerDirective::Arg(
						self.tokenizer.subtendril(token_range.clone())
					)),
					token_range,
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
					if (crate::LanguageStd::SYSTEM_VERILOG_STDS & !crate::LanguageStd::Sv05 |
						crate::LanguageStd::Vams23)
						.contains(self.standard)
					{
						match ident {
							"__FILE__" => TextMacro::DunderFile,
							"__LINE__" => TextMacro::DunderLine,
							_ => TextMacro::Other(self.tokenizer.subtendril_inclusive(ident_range)),
						}
					} else {
						TextMacro::Other(self.tokenizer.subtendril_inclusive(ident_range))
					}
				),
				begin..self.tokenizer.offset(),
				context
			)
		};
	}

	// BUG(aki):
	// This is likely really fragile in some specific cases with miss-matching or many quotes in a
	// row. for instance, what would `""test""` tokenize as? How about `"test"""`, or `"""test""`,
	// etc. so we need to iron out this a bit eventually.
	fn read_quote_token(&mut self) {
		let context = self.tokenizer.position();
		let quote_begin = self.tokenizer.offset();
		self.tokenizer.next_char(); // Consume the first `"`

		// Check to see if we're a triple-quoted string
		let triple_quote = if self.tokenizer.current_byte() == b'"' {
			self.tokenizer.next_char();
			self.tokenizer.next_char() == b'"'
		} else {
			false
		};

		let str_begin = self.tokenizer.offset();

		while self.tokenizer.current_byte() != b'"' && !self.tokenizer.is_eof() {
			// If we hit `\"` then consume it and keep going, all other
			// escapes are checked after tokenization
			if self.tokenizer.next_char() == b'\\' && self.tokenizer.current_byte() == b'"' {
				self.tokenizer.next_char();
			}
		}

		// Consume the last `"`
		let str_end = self.tokenizer.offset();
		self.tokenizer.next_char();
		if triple_quote {
			self.tokenizer.next_char();
			self.tokenizer.next_char();
		}
		let quote_end = self.tokenizer.offset();

		let quote_range = quote_begin..quote_end;
		let str_range = str_begin..str_end;
		self.token = spanned_token!(
			if triple_quote {
				if LanguageStd::Sv23.contains(self.standard) {
					Token::TripleQuotedString(self.tokenizer.subtendril(str_range))
				} else {
					Token::ContextuallyInvalid(
						self.tokenizer.subtendril(quote_range.clone()),
						LanguageStd::Sv23,
					)
				}
			} else {
				Token::String(self.tokenizer.subtendril(str_range))
			},
			quote_range,
			context
		)
	}

	fn read_apostrophe_token(&mut self) {
		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		self.tokenizer.next_char();

		// If there is a base specifier right after us, then it's a number token
		if matches!(
			self.tokenizer.current_byte(),
			b'b' | b'B' | b'o' | b'O' | b'd' | b'D' | b'h' | b'H' | b's' | b'S'
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
				begin..self.tokenizer.offset(),
				context
			);
		}
	}

	fn read_number_token(&mut self, from_apostrophe: bool) {
		// Pop out a size if available
		if self.tokenizer.current_byte().is_ascii_digit() {
			let context = self.tokenizer.position();
			let begin = self.tokenizer.offset();
			// Consume digits while we find valid unsigned number digits
			while self.tokenizer.current_byte().is_ascii_digit() ||
				self.tokenizer.current_byte() == b'_'
			{
				self.tokenizer.next_char();
			}

			// If we next hit a `.` or an exponent indicator we need to treat this as
			// a Real number
			if matches!(self.tokenizer.current_byte(), b'.' | b'e' | b'E') {
				self.read_real_number_token(context, begin);
				return;
			}

			// Stuff the unsigned number token onto the token stack
			let token_range = begin..self.tokenizer.offset();
			self.token_stream.push_back(spanned_token!(
				Token::UnsignedNumber(self.tokenizer.subtendril(token_range.clone())),
				token_range,
				context
			));
		}

		// Deal with any whitespace that comes after the size
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push_back(self.token.clone())
		}

		// If we get here and we don't yet have a `'`,  we've consumed a decimal number - we're
		// done.
		if self.tokenizer.current_byte() != b'\'' && !from_apostrophe {
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
		let mut context = self.tokenizer.position();
		let mut begin = self.tokenizer.offset();
		// If we're not coming from the `parse_apostrophe_token` then we need to advance
		if !from_apostrophe {
			self.tokenizer.next_char();
		} else {
			// If we're coming from the `parse_apostrophe_token` then we need to roll back
			// `begin` and `context`
			context = Position::new(*context.line(), context.character() - 1);
			begin -= 1;
		}

		let signed = if matches!(self.tokenizer.current_byte(), b's' | b'S') {
			self.tokenizer.next_char();

			// If we are in Verilog 95, we don't have support for signed base specifiers
			if self.standard == LanguageStd::Vl95 {
				let token_range = (begin + 1)..self.tokenizer.offset();
				self.token_stream.push_back(spanned_token!(
					Token::ContextuallyInvalid(
						self.tokenizer.subtendril(token_range.clone()),
						LanguageStd::all_flags() & !LanguageStd::Vl95
					),
					token_range,
					context
				));
				false
			} else {
				true
			}
		} else {
			false
		};

		// Having consumed the `'` we should now be left with a base specifier, and if not then
		// this is an invalid token.
		match self.tokenizer.current_byte() {
			b'b' | b'B' => self.read_based_token(
				context,
				begin,
				BaseSpecifier::Binary,
				|char| matches!(char, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0' | b'1'),
				signed,
			),
			b'o' | b'O' => self.read_based_token(
				context,
				begin,
				BaseSpecifier::Octal,
				|char| matches!(char, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0'..=b'7'),
				signed,
			),
			b'd' | b'D' => self.read_based_token(
				context,
				begin,
				BaseSpecifier::Decimal,
				|char| char.is_ascii_digit(),
				signed,
			),
			b'h' | b'H' => self.read_based_token(
				context,
				begin,
				BaseSpecifier::Hexadecimal,
				|c| matches!(c, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'),
				signed,
			),
			_ => {
				self.token_stream.push_back(spanned_token!(
					Token::Invalid(Some(
						self.tokenizer.subtendril(begin..self.tokenizer.offset())
					)),
					begin..self.tokenizer.offset(),
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
		if self.tokenizer.current_byte() == b'.' {
			self.tokenizer.next_char();
			// Consume digits while we find valid unsigned number digits
			while self.tokenizer.current_byte().is_ascii_digit() ||
				self.tokenizer.current_byte() == b'_'
			{
				self.tokenizer.next_char();
			}
		}

		// If we have an exponent part, consume that
		let exponent = if matches!(self.tokenizer.current_byte(), b'e' | b'E') {
			let exp_begin = self.tokenizer.offset();
			self.tokenizer.next_char();

			// Check to make sure we have a valid exponent
			if !self.tokenizer.current_byte().is_ascii_digit() &&
				!matches!(self.tokenizer.current_byte(), b'+' | b'-')
			{
				let token_range = begin..self.tokenizer.offset();
				self.token = spanned_token!(
					Token::Invalid(Some(self.tokenizer.subtendril(token_range.clone()))),
					token_range,
					context
				);

				return;
			}
			// Consume the + or - (or first digit)
			self.tokenizer.next_char();

			// Consume digits while we find valid unsigned number digits
			while self.tokenizer.current_byte().is_ascii_digit() ||
				self.tokenizer.current_byte() == b'_'
			{
				self.tokenizer.next_char();
			}

			Some(
				self.tokenizer
					.subtendril(exp_begin..self.tokenizer.offset()),
			)
		} else {
			None
		};

		// SAFETY:
		// If we got to this point, then we have already ensured the contents are
		// ASCII, which is a valid UTF-8 subset
		let value =
			unsafe { str::from_utf8_unchecked(&self.tokenizer[begin..self.tokenizer.offset()]) };

		// If we got a valid f64, then we use that, otherwise emit an invalid token
		let token_range = begin..self.tokenizer.offset();
		self.token = if let Ok(value) = value.parse() {
			spanned_token!(Token::Real { value, exponent }, token_range, context)
		} else {
			spanned_token!(
				Token::Invalid(Some(value.as_bytes().into())),
				token_range,
				context
			)
		}
	}

	fn read_based_token(
		&mut self,
		context: Position,
		begin: usize,
		specifier: BaseSpecifier,
		digit_filter: fn(u8) -> bool,
		signed: bool,
	) {
		let uppercase = self.tokenizer.next_char().is_ascii_uppercase();
		self.token_stream.push_back(spanned_token!(
			Token::BaseSpecifier { specifier, uppercase, signed },
			begin..self.tokenizer.offset(),
			context
		));
		// Deal with any whitespace that comes after the base specifier
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push_back(self.token.clone())
		}

		let context = self.tokenizer.position();
		let begin = self.tokenizer.offset();
		// Make sure we've got a valid number - if we have not, eat the content and turn it into an
		// Invalid token.
		if !digit_filter(self.tokenizer.current_byte()) {
			// keep consuming input until we hit whitespace or EOF
			while !self.current_is_whitespace() &&
				!self.current_is_newline() &&
				!self.tokenizer.is_eof()
			{
				if self.current_is_newline() {
					self.read_newline();
				} else {
					self.tokenizer.next_char();
				}
			}

			// Turn whatever we got into an invalid token
			// TODO(aki):
			// In IEEE 1364-2001 (Verilog 2001) decimal bases ('d|'D) are allowed to
			// have [xz?] as the first digit, followed by `_` exclusively if it's not
			// a base-10 digit, so this should be turned into a Token::ContextuallyInvalid
			// in the case we hit one of those and are in Verilog 95.
			let token_range = begin..self.tokenizer.offset();
			self.token_stream.push_back(spanned_token!(
				Token::Invalid(Some(self.tokenizer.subtendril(token_range.clone()))),
				token_range,
				context
			));
		} else {
			// Run the filter to the end consuming all the digits we can
			while digit_filter(self.tokenizer.current_byte()) ||
				self.tokenizer.current_byte() == b'_'
			{
				self.tokenizer.next_char();
			}

			// If we're consuming a decimal number, we need to ensure we also consume trailing
			// [xz?]'s to generate an invalid token if there are any
			if specifier == BaseSpecifier::Decimal &&
				matches!(
					self.tokenizer.current_byte(),
					b'x' | b'X' | b'z' | b'Z' | b'?'
				) {
				while matches!(
					self.tokenizer.current_byte(),
					b'x' | b'X' | b'z' | b'Z' | b'?'
				) && !self.current_is_whitespace() &&
					!self.current_is_newline() &&
					!self.tokenizer.is_eof()
				{
					if self.current_is_newline() {
						self.read_newline();
					} else {
						self.tokenizer.next_char();
					}
				}

				let token_range = begin..self.tokenizer.offset();
				self.token_stream.push_back(spanned_token!(
					Token::Invalid(Some(self.tokenizer.subtendril(token_range.clone()))),
					token_range,
					context
				));
			} else {
				// Turn the consumed number into a Number token
				let token_range = begin..self.tokenizer.offset();
				self.token_stream.push_back(spanned_token!(
					Token::Number(self.tokenizer.subtendril(token_range.clone())),
					token_range,
					context
				));
			}
		}
	}

	#[inline]
	fn read_normal_ident(&mut self) -> Range<usize> {
		let begin = self.tokenizer.offset();
		// Scan through till we get something that's not a-zA-Z0-9_$
		while self.tokenizer.current_byte().is_ascii_alphanumeric() ||
			self.tokenizer.current_byte() == b'_' ||
			self.tokenizer.current_byte() == b'$'
		{
			self.tokenizer.next_char();
		}
		// Return the range consumed
		begin..self.tokenizer.offset()
	}

	#[inline(always)]
	fn is_ascii_printable(&self) -> bool {
		matches!(self.tokenizer.current_byte() as char, 'a'..='z'
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
		let begin = self.tokenizer.offset();
		// Scan through till we get something that's not ASCII printable
		while self.is_ascii_printable() {
			self.tokenizer.next_char();
		}
		// Return the range consumed
		begin..self.tokenizer.offset()
	}
}

impl Iterator for VerilogTokenizer {
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
