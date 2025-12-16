/* SPDX-License-Identifier: BSD-3-Clause */

use std::{collections::VecDeque, ops::Range};

use tendril::ByteTendril;

use self::token::{BaseSpecifier, Comment, Control, Keyword, Operator, Token};
use crate::VerilogVariant;

use vermilion_lang::{Position, Span, Spanned};

pub(crate) mod token;

pub(crate) struct Tokenizer {
	standard: VerilogVariant,
	file: ByteTendril,
	current_char: u8,
	position: usize,
	context: Position,
	eof: bool,
	token: Spanned<Token, Position>,
	token_stream: VecDeque<Spanned<Token, Position>>,
}

impl Tokenizer {
	pub fn new(standard: VerilogVariant, file: ByteTendril) -> Tokenizer {
		let mut tokenizer = Self {
			standard,
			file,
			current_char: 0,
			position: 0,
			context: Position::sof(),
			eof: false,
			token: Spanned::new(Token::default(), None),
			token_stream: VecDeque::new(),
		};
		tokenizer.current_char = tokenizer.file[tokenizer.position];

		tokenizer
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
		self.current_char == b' ' || self.current_char == b'\t'
	}

	fn read_token(&mut self) {
		if !self.token_stream.is_empty() {
			// SAFETY:
			// This has to work or the stream is empty, thus the if condition fails.
			#[allow(clippy::expect_used)]
			let tok = self
				.token_stream
				.pop_front()
				.expect("Unable to pop token from token stream");

			self.token = tok;
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
			b'(' => self.token = Token::Control(Control::ParenOpen).into(),
			b')' => self.token = Token::Control(Control::ParenClose).into(),
			b'[' => self.token = Token::Control(Control::BracketOpen).into(),
			b']' => self.token = Token::Control(Control::BracketClose).into(),
			b'{' => self.token = Token::Control(Control::BraceOpen).into(),
			b'}' => self.token = Token::Control(Control::BraceClose).into(),
			b':' => self.token = Token::Control(Control::Colon).into(),
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
			b'%' => self.token = Token::Operator(Operator::Percent).into(),
			b'*' => self.token = Token::Operator(Operator::Asterisk).into(),
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
				self.read_compiler_directive_token();
				return;
			},
			b'"' => {
				self.read_string_token();
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

	fn read_whitespace(&mut self) {
		let context = self.context;
		let begin = self.position;
		while matches!(self.current_char, b' ' | b'\t') {
			self.next_char();
		}
		let end = self.position;
		self.token = Spanned::new(
			Token::Whitespace(self.file.subtendril(begin as u32, (end - begin) as u32)),
			Some(Span::new(begin..end, context)),
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
			Some(Span::new(begin..end, context)),
		)
	}

	fn read_exclame_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'=' {
			self.next_char();
			let end = self.position;

			if self.current_char == b'=' {
				self.next_char();
				let end = self.position;

				self.token = Spanned::new(
					Token::Operator(Operator::CaseInequality),
					Some(Span::new(begin..end, context)),
				)
			} else {
				self.token = Spanned::new(
					Token::Operator(Operator::LogicalInequality),
					Some(Span::new(begin..end, context)),
				)
			}
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Exclamation),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_equals_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'=' {
			self.next_char();
			let end = self.position;

			if self.current_char == b'=' {
				self.next_char();
				let end = self.position;

				self.token = Spanned::new(
					Token::Operator(Operator::CaseEquality),
					Some(Span::new(begin..end, context)),
				)
			} else {
				self.token = Spanned::new(
					Token::Operator(Operator::LogicalEquality),
					Some(Span::new(begin..end, context)),
				)
			}
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Equals),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_ampersand_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'&' {
			self.next_char();
			let end = self.position;

			self.token = Spanned::new(
				Token::Operator(Operator::LogicalAnd),
				Some(Span::new(begin..end, context)),
			)
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Ampersand),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_tilde_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'^' {
			self.next_char();
			let end = self.position;
			self.token = Spanned::new(
				Token::Operator(Operator::TildeCircumflex(false)),
				Some(Span::new(begin..end, context)),
			)
		} else if self.current_char == b'&' {
			self.next_char();
			let end = self.position;
			self.token = Spanned::new(
				Token::Operator(Operator::ReductionNand),
				Some(Span::new(begin..end, context)),
			)
		} else if self.current_char == b'|' {
			self.next_char();
			let end = self.position;

			self.token = Spanned::new(
				Token::Operator(Operator::ReductionNor),
				Some(Span::new(begin..end, context)),
			)
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Tilde),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_circumflex_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'~' {
			self.next_char();
			let end = self.position;
			self.token = Spanned::new(
				Token::Operator(Operator::TildeCircumflex(true)),
				Some(Span::new(begin..end, context)),
			)
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Circumflex),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_pipe_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'|' {
			self.next_char();
			let end = self.position;
			self.token = Spanned::new(
				Token::Operator(Operator::LogicalOr),
				Some(Span::new(begin..end, context)),
			)
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Pipe),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_less_than_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'=' {
			self.next_char();
			let end = self.position;

			self.token = Spanned::new(
				Token::Operator(Operator::LessThanEqual),
				Some(Span::new(begin..end, context)),
			)
		} else if self.current_char == b'>' {
			self.next_char();
			let end = self.position;

			self.token = Spanned::new(
				Token::Operator(Operator::ShiftRight),
				Some(Span::new(begin..end, context)),
			)
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::LessThan),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_greater_than_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char == b'=' {
			self.next_char();
			let end = self.position;

			self.token = Spanned::new(
				Token::Operator(Operator::GreaterThanEqual),
				Some(Span::new(begin..end, context)),
			)
		} else if self.current_char == b'<' {
			self.next_char();
			let end = self.position;

			self.token = Spanned::new(
				Token::Operator(Operator::ShiftLeft),
				Some(Span::new(begin..end, context)),
			)
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::GreaterThan),
				Some(Span::new(begin..end, context)),
			)
		}
	}

	fn read_string_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		todo!("String tokenization")
	}

	fn read_minus_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if self.current_char.is_ascii_digit() {
			self.token_stream.push_back(Spanned::new(
				Token::Sign(token::Sign::Negative),
				Some(Span::new(begin..end, context)),
			));
			self.read_number_token();
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Minus),
				Some(Span::new(begin..end, context)),
			);
		}
	}

	fn read_solidus_token(&mut self) {
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		if matches!(self.current_char, b'*' | b'/') {
			let multiline = self.current_char == b'*';
			self.next_char();

			todo!("Comment tokenization")
		} else {
			self.token = Spanned::new(
				Token::Operator(Operator::Solidus),
				Some(Span::new(begin..end, context)),
			);
		}
	}

	fn read_compiler_directive_token(&mut self) {
		let context = self.context;
		let begin = self.position;

		todo!("Compiler Directive tokenization")
	}

	fn read_extended_token(&mut self) {
		let context = self.context;
		if self.current_char.is_ascii_alphabetic() || self.current_char == b'_' {
			let range = self.read_normal_ident();
			// We've already validated via the above read, that the entire token is valid UTF-8. Just make it a string.
			let ident = unsafe { str::from_utf8_unchecked(&self.file[range.clone()]) };
			let token = match ident {
				"always" => Token::Keyword(Keyword::Always),
				"and" => Token::Keyword(Keyword::And),
				"assign" => Token::Keyword(Keyword::Assign),
				"begin" => Token::Keyword(Keyword::Begin),
				"buf" => Token::Keyword(Keyword::Buf),
				"bufif0" => Token::Keyword(Keyword::BufIf0),
				"bufif1" => Token::Keyword(Keyword::BufIf1),
				"case" => Token::Keyword(Keyword::Case),
				"casex" => Token::Keyword(Keyword::CaseX),
				"casez" => Token::Keyword(Keyword::CaseZ),
				"cmos" => Token::Keyword(Keyword::Cmos),
				"deassign" => Token::Keyword(Keyword::Deassign),
				"default" => Token::Keyword(Keyword::Default),
				"defparam" => Token::Keyword(Keyword::DefParam),
				"disable" => Token::Keyword(Keyword::Disable),
				"edge" => Token::Keyword(Keyword::Edge),
				"else" => Token::Keyword(Keyword::Else),
				"end" => Token::Keyword(Keyword::End),
				"endcase" => Token::Keyword(Keyword::EndCase),
				"endfunction" => Token::Keyword(Keyword::EndFunction),
				"endmodule" => Token::Keyword(Keyword::EndModule),
				"endprimitive" => Token::Keyword(Keyword::EndPrimitive),
				"endspecify" => Token::Keyword(Keyword::EndSpecify),
				"endtable" => Token::Keyword(Keyword::EndTable),
				"endtask" => Token::Keyword(Keyword::EndTask),
				"event" => Token::Keyword(Keyword::Event),
				"for" => Token::Keyword(Keyword::For),
				"force" => Token::Keyword(Keyword::Force),
				"forever" => Token::Keyword(Keyword::Forever),
				"fork" => Token::Keyword(Keyword::Fork),
				"function" => Token::Keyword(Keyword::Function),
				"highz0" => Token::Keyword(Keyword::HighZ0),
				"highz1" => Token::Keyword(Keyword::HighZ1),
				"if" => Token::Keyword(Keyword::If),
				"ifnone" => Token::Keyword(Keyword::IfNone),
				"initial" => Token::Keyword(Keyword::Initial),
				"inout" => Token::Keyword(Keyword::InOut),
				"input" => Token::Keyword(Keyword::Input),
				"integer" => Token::Keyword(Keyword::Integer),
				"join" => Token::Keyword(Keyword::Join),
				"large" => Token::Keyword(Keyword::Large),
				"macromodule" => Token::Keyword(Keyword::MacroModule),
				"medium" => Token::Keyword(Keyword::Medium),
				"module" => Token::Keyword(Keyword::Module),
				"nand" => Token::Keyword(Keyword::Nand),
				"negedge" => Token::Keyword(Keyword::NegEdge),
				"nmos" => Token::Keyword(Keyword::Nmos),
				"nor" => Token::Keyword(Keyword::Nor),
				"not" => Token::Keyword(Keyword::Not),
				"notif0" => Token::Keyword(Keyword::NotIf0),
				"notif1" => Token::Keyword(Keyword::NotIf1),
				"or" => Token::Keyword(Keyword::Or),
				"output" => Token::Keyword(Keyword::Output),
				"parameter" => Token::Keyword(Keyword::Parameter),
				"pmos" => Token::Keyword(Keyword::Pmos),
				"posedge" => Token::Keyword(Keyword::PosEdge),
				"primitive" => Token::Keyword(Keyword::Primitive),
				"pull0" => Token::Keyword(Keyword::Pull0),
				"pull1" => Token::Keyword(Keyword::Pull1),
				"pulldown" => Token::Keyword(Keyword::Pulldown),
				"pullup" => Token::Keyword(Keyword::Pullup),
				"rcmos" => Token::Keyword(Keyword::Rcmos),
				"real" => Token::Keyword(Keyword::Real),
				"realtime" => Token::Keyword(Keyword::Realtime),
				"reg" => Token::Keyword(Keyword::Reg),
				"release" => Token::Keyword(Keyword::Release),
				"repeat" => Token::Keyword(Keyword::Repeat),
				"rnmos" => Token::Keyword(Keyword::Rnmos),
				"rpmos" => Token::Keyword(Keyword::Rpmos),
				"rtran" => Token::Keyword(Keyword::Rtran),
				"rtranif0" => Token::Keyword(Keyword::RtranIf0),
				"rtranif1" => Token::Keyword(Keyword::RtranIf1),
				"scalared" => Token::Keyword(Keyword::Scalared),
				"small" => Token::Keyword(Keyword::Small),
				"specify" => Token::Keyword(Keyword::Specify),
				"specparam" => Token::Keyword(Keyword::SpecParam),
				"strong0" => Token::Keyword(Keyword::Strong0),
				"strong1" => Token::Keyword(Keyword::Strong1),
				"supply0" => Token::Keyword(Keyword::Supply0),
				"supply1" => Token::Keyword(Keyword::Supply1),
				"table" => Token::Keyword(Keyword::Table),
				"task" => Token::Keyword(Keyword::Task),
				"time" => Token::Keyword(Keyword::Time),
				"tran" => Token::Keyword(Keyword::Tran),
				"tranif0" => Token::Keyword(Keyword::TranIf0),
				"tranif1" => Token::Keyword(Keyword::TranIf1),
				"tri" => Token::Keyword(Keyword::Tri),
				"tri0" => Token::Keyword(Keyword::Tri0),
				"tri1" => Token::Keyword(Keyword::Tri1),
				"triand" => Token::Keyword(Keyword::Triand),
				"trior" => Token::Keyword(Keyword::Trior),
				"trireg" => Token::Keyword(Keyword::Trireg),
				"vectored" => Token::Keyword(Keyword::Vectored),
				"wait" => Token::Keyword(Keyword::Wait),
				"wand" => Token::Keyword(Keyword::Wand),
				"weak0" => Token::Keyword(Keyword::Weak0),
				"weak1" => Token::Keyword(Keyword::Weak1),
				"while" => Token::Keyword(Keyword::While),
				"wire" => Token::Keyword(Keyword::Wire),
				"wor" => Token::Keyword(Keyword::Wor),
				"xnor" => Token::Keyword(Keyword::Xnor),
				"xor" => Token::Keyword(Keyword::Xor),
				_ => {
					Token::Identifier(self.file.subtendril(range.start as u32, range.len() as u32))
				},
			};
			// Turn the result into the final token to return
			self.token = Spanned::new(token, Some(Span::new(range, context)));
		} else if self.current_char == b'\\' {
			let range = self.read_escaped_ident();
			self.token = Spanned::new(
				Token::Identifier(self.file.subtendril(range.start as u32, range.len() as u32)),
				Some(Span::new(range, context)),
			);
		} else if self.current_char.is_ascii_digit() || self.current_char == b'\'' {
			self.read_number_token();
		}
	}

	fn read_plus_token(&mut self) {
		// Grab the context for and consume the `+` character
		let context = self.context;
		let begin = self.position;
		self.next_char();
		let end = self.position;

		// If that got us to a number, this was a sign token
		if self.current_char.is_ascii_digit() {
			self.token_stream.push_back(Spanned::new(
				Token::Sign(token::Sign::Positive),
				Some(Span::new(begin..end, context)),
			));
			// Shunt into the number token subparser to handle number things
			self.read_number_token();
		} else {
			// Otherwise we got a simple unary or binary `+` operator
			self.token = Spanned::new(
				Token::Operator(Operator::Plus),
				Some(Span::new(begin..end, context)),
			);
		}
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
			self.token_stream.push_back(Spanned::new(
				Token::UnsignedNumber(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				),
				Some(Span::new(begin..self.position, context)),
			));
		};
		// Deal with any whitespace that comes after the size
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push_back(self.token.clone())
		}
		// TODO(aki): We need to think about `Real` numbers here too, at this point,
		// If we get here and we don't yet have a `'`,  we've consumed a decimal number - we're done.
		if self.current_char != b'\'' {
			// SAFETY:
			// If we're here, we have to have pushed stuff to the token stream, so this is always okay.
			#[allow(clippy::expect_used)]
			let tok = self
				.token_stream
				.pop_front()
				.expect("Unable to pop token from token stream");

			self.token = tok;
			return;
		}

		// Grab the position for and start matching a base specifier
		let context = self.context;
		let begin = self.position;
		self.next_char();
		// Having consumed the `'` we should now be left with a base specifier, and if not then
		// this is an invalid token.
		match self.current_char {
			b'b' | b'B' => self.read_based_token(context, begin, BaseSpecifier::Binary, |c| {
				matches!(c, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0' | b'1')
			}),
			b'o' | b'O' => self.read_based_token(context, begin, BaseSpecifier::Octal, |c| {
				matches!(c, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0'..=b'7')
			}),
			b'd' | b'D' => self.read_based_token(context, begin, BaseSpecifier::Decimal, |c| {
				c.is_ascii_digit()
			}),
			b'h' | b'H' => self.read_based_token(
				context,
				begin,
				BaseSpecifier::Hexadecimal,
				|c| matches!(c, b'x' | b'X' | b'z' | b'Z' | b'?' | b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'),
			),
			_ => {
				self.token_stream.push_back(Spanned::new(
					Token::Invalid(Some(
						self.file
							.subtendril(begin as u32, (self.position - begin) as u32),
					)),
					Some(Span::new(begin..self.position, context)),
				));
			},
		};
		// Having processed all of the number tokens, pop the first as the result token.

		// SAFETY:
		// The only way we get here is having pushed at least something onto the token stream.
		#[allow(clippy::expect_used)]
		let tok = self
			.token_stream
			.pop_front()
			.expect("Unable to pop token from token stream");

		self.token = tok;
	}

	fn read_based_token(
		&mut self,
		context: Position,
		begin: usize,
		spec: BaseSpecifier,
		digit_filter: fn(u8) -> bool,
	) {
		let upper_case = self.next_char().is_ascii_uppercase();
		self.token_stream.push_back(Spanned::new(
			Token::BaseSpecifier(spec, upper_case),
			Some(Span::new(begin..self.position, context)),
		));
		// Deal with any whitespace that comes after the base specifier
		if self.current_is_whitespace() {
			self.read_whitespace();
			self.token_stream.push_back(self.token.clone())
		}

		let context = self.context;
		let begin = self.position;
		// Make sure we've got a valid number - if we have not, eat the content and turn it into an Invalid token.
		if !digit_filter(self.current_char) {
			// Keep monching until we find whitespace.
			while !self.current_is_whitespace() {
				self.next_char();
			}

			// Turn whatever we got into an invalid token
			self.token_stream.push_back(Spanned::new(
				Token::Invalid(Some(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				)),
				Some(Span::new(begin..self.position, context)),
			));
		} else {
			// Monch digits till we don't find no more.
			while digit_filter(self.current_char) || self.current_char == b'_' {
				self.next_char();
			}

			// Turn the consumed number into a Number token
			self.token_stream.push_back(Spanned::new(
				Token::Number(
					self.file
						.subtendril(begin as u32, (self.position - begin) as u32),
				),
				Some(Span::new(begin..self.position, context)),
			));
		}
	}

	#[inline]
	fn read_normal_ident(&mut self) -> Range<usize> {
		let begin = self.position;
		// Scan through till we get something that's not a-zA-Z0-9_$
		while self.current_char.is_ascii_alphanumeric()
			|| self.current_char == b'_'
			|| self.current_char == b'$'
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

impl Iterator for Tokenizer {
	type Item = Spanned<Token, Position>;

	fn next(&mut self) -> Option<Self::Item> {
		// If we hit the end of the file, we've nothing more to give
		if self.eof {
			return None;
		}
		self.read_token();
		Some(self.token.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::token::Token;
	use super::*;
	use crate::VerilogStd;

	macro_rules! tokenizer_test {
		($test_name:ident, $input:literal, $tokens:expr) => {
			#[test]
			fn $test_name() {
				let tokenizer = Tokenizer::new(
					VerilogVariant::Verilog(VerilogStd::Vl95),
					$input.as_bytes().into(),
				);

				let parsed = tokenizer.collect::<Vec<_>>();
				let expected = $tokens;

				assert_eq!(
					parsed, expected,
					"Parsed: {parsed:#?}\nExpected: {expected:#?}"
				);
			}
		};
	}

	tokenizer_test!(
		test_tokenize_whitespace,
		" \r\n \t\n\t\n\r",
		vec![
			Spanned::new(
				Token::Whitespace(" ".as_bytes().into()),
				Some(Span::new(0..1, Position::new(0, 0))),
			),
			Spanned::new(
				Token::Newline("\r\n".as_bytes().into()),
				Some(Span::new(1..3, Position::new(0, 1))),
			),
			Spanned::new(
				Token::Whitespace(" \t".as_bytes().into()),
				Some(Span::new(3..5, Position::new(1, 0))),
			),
			Spanned::new(
				Token::Newline("\n".as_bytes().into()),
				Some(Span::new(5..6, Position::new(1, 2))),
			),
			Spanned::new(
				Token::Whitespace("\t".as_bytes().into()),
				Some(Span::new(6..7, Position::new(2, 0))),
			),
			Spanned::new(
				Token::Newline("\n".as_bytes().into()),
				Some(Span::new(7..8, Position::new(2, 1))),
			),
			Spanned::new(
				Token::Newline("\r".as_bytes().into()),
				Some(Span::new(8..9, Position::new(3, 0))),
			),
		]
	);

	tokenizer_test!(
		test_tokenize_tilde,
		"~",
		vec![Spanned::new(
			Token::Operator(Operator::Tilde),
			Some(Span::new(0..1, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_tilde_circumflex_right,
		"~^",
		vec![Spanned::new(
			Token::Operator(Operator::TildeCircumflex(false)),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_reduction_nand,
		"~&",
		vec![Spanned::new(
			Token::Operator(Operator::ReductionNand),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_reduction_nor,
		"~|",
		vec![Spanned::new(
			Token::Operator(Operator::ReductionNor),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_circumflex,
		"^",
		vec![Spanned::new(
			Token::Operator(Operator::Circumflex),
			Some(Span::new(0..1, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_tilde_circumflex_left,
		"^~",
		vec![Spanned::new(
			Token::Operator(Operator::TildeCircumflex(true)),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_pipe,
		"|",
		vec![Spanned::new(
			Token::Operator(Operator::Pipe),
			Some(Span::new(0..1, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_logical_or,
		"||",
		vec![Spanned::new(
			Token::Operator(Operator::LogicalOr),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_ampersand,
		"&",
		vec![Spanned::new(
			Token::Operator(Operator::Ampersand),
			Some(Span::new(0..1, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_logical_and,
		"&&",
		vec![Spanned::new(
			Token::Operator(Operator::LogicalAnd),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_less_than,
		">",
		vec![Spanned::new(
			Token::Operator(Operator::LessThan),
			Some(Span::new(0..1, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_less_than_equal,
		">=",
		vec![Spanned::new(
			Token::Operator(Operator::LessThanEqual),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_shift_right,
		">>",
		vec![Spanned::new(
			Token::Operator(Operator::ShiftRight),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_greater_than,
		"<",
		vec![Spanned::new(
			Token::Operator(Operator::GreaterThan),
			Some(Span::new(0..1, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_greater_than_equal,
		"<=",
		vec![Spanned::new(
			Token::Operator(Operator::GreaterThanEqual),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_shift_left,
		"<<",
		vec![Spanned::new(
			Token::Operator(Operator::ShiftLeft),
			Some(Span::new(0..2, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_solidus,
		"/",
		vec![Spanned::new(
			Token::Operator(Operator::Solidus),
			Some(Span::new(0..1, Position::new(0, 0)))
		)]
	);

	tokenizer_test!(
		test_tokenize_equal,
		"=",
		vec![Spanned::new(
			Token::Operator(Operator::Equals),
			Some(Span::new(0..1, Position::new(0, 0))),
		)]
	);

	tokenizer_test!(
		test_tokenize_logical_equality,
		"==",
		vec![Spanned::new(
			Token::Operator(Operator::LogicalEquality),
			Some(Span::new(0..2, Position::new(0, 0))),
		)]
	);

	tokenizer_test!(
		test_tokenize_case_equality,
		"===",
		vec![Spanned::new(
			Token::Operator(Operator::CaseEquality),
			Some(Span::new(0..3, Position::new(0, 0))),
		)]
	);

	tokenizer_test!(
		test_tokenize_exclamation,
		"!",
		vec![Spanned::new(
			Token::Operator(Operator::Exclamation),
			Some(Span::new(0..1, Position::new(0, 0))),
		)]
	);

	tokenizer_test!(
		test_tokenize_logical_inequality,
		"!=",
		vec![Spanned::new(
			Token::Operator(Operator::LogicalInequality),
			Some(Span::new(0..2, Position::new(0, 0))),
		)]
	);

	tokenizer_test!(
		test_tokenize_case_inequality,
		"!==",
		vec![Spanned::new(
			Token::Operator(Operator::CaseInequality),
			Some(Span::new(0..3, Position::new(0, 0))),
		)]
	);

	tokenizer_test!(
		test_tokenize_binary_lump,
		"4 'b01zx\n+1'B ?\n2'bZX\n",
		vec![
			Spanned::new(
				Token::UnsignedNumber("4".as_bytes().into()),
				Some(Span::new(0..1, Position::new(0, 0)))
			),
			Spanned::new(
				Token::Whitespace(" ".as_bytes().into()),
				Some(Span::new(1..2, Position::new(0, 1)))
			),
			Spanned::new(
				Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
				Some(Span::new(2..4, Position::new(0, 2)))
			),
			Spanned::new(
				Token::Number("01zx".as_bytes().into()),
				Some(Span::new(4..8, Position::new(0, 4)))
			),
			Spanned::new(
				Token::Newline("\n".as_bytes().into()),
				Some(Span::new(8..9, Position::new(0, 8)))
			),
			Spanned::new(
				Token::Sign(token::Sign::Positive),
				Some(Span::new(9..10, Position::new(1, 0)))
			),
			Spanned::new(
				Token::UnsignedNumber("1".as_bytes().into()),
				Some(Span::new(10..11, Position::new(1, 1)))
			),
			Spanned::new(
				Token::BaseSpecifier(token::BaseSpecifier::Binary, true),
				Some(Span::new(11..13, Position::new(1, 2)))
			),
			Spanned::new(
				Token::Whitespace(" ".as_bytes().into()),
				Some(Span::new(13..14, Position::new(1, 4)))
			),
			Spanned::new(
				Token::Number("?".as_bytes().into()),
				Some(Span::new(14..15, Position::new(1, 5)))
			),
			Spanned::new(
				Token::Newline("\n".as_bytes().into()),
				Some(Span::new(15..16, Position::new(1, 6)))
			),
			Spanned::new(
				Token::UnsignedNumber("2".as_bytes().into()),
				Some(Span::new(16..17, Position::new(2, 0)))
			),
			Spanned::new(
				Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
				Some(Span::new(17..19, Position::new(2, 1)))
			),
			Spanned::new(
				Token::Number("ZX".as_bytes().into()),
				Some(Span::new(19..21, Position::new(2, 3)))
			),
			Spanned::new(
				Token::Newline("\n".as_bytes().into()),
				Some(Span::new(21..22, Position::new(2, 5)))
			),
		]
	);
}
