/* SPDX-License-Identifier: BSD-3-Clause */

pub(crate) mod token;

use chumsky::{
	IterParser, Parser,
	extra::ParserExtra,
	input::{SliceInput, StrInput},
	label::LabelError,
	prelude::*,
	text::{Char, TextExpected},
};
use vermilion_lang::parser::{LexErr, LexResult, Spanned};

use crate::{
	VerilogVariant,
	lang::lexer::token::{Keyword, NumericBase, Token, Whitespace},
};

pub(crate) fn lexer<'src>(
	_std: VerilogVariant,
) -> impl Parser<'src, &'src str, LexResult<Token<'src>>, LexErr<'src>> {
	let whitespace = lex_whitespace();
	let comment = lex_comment();
	let string = lex_string();
	let compiler_directive = lex_compiler_directive();
	let kw_or_ident = lex_keyword_or_ident();
	let numeric_base = lex_numeric_base();
	let numeric_literal = lex_numeric_literal();

	let ctrl = one_of("@#()[]{}:;,.$").map(Token::Ctrl);
	let op = one_of("+-!~&|^*%=<>")
		.repeated()
		.at_least(1)
		.at_most(3)
		.to_slice()
		.map(Token::Op);

	let token = whitespace
		.or(comment)
		.or(compiler_directive)
		.or(ctrl)
		.or(op)
		.or(string)
		.or(kw_or_ident)
		.or(numeric_base)
		.or(numeric_literal);

	token
		.map_with(|tok, e| Spanned::new(tok, Some(e.span())))
		.recover_with(skip_then_retry_until(any().ignored(), end()))
		.repeated()
		.collect()
}

fn lex_whitespace<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	just("\r\n")
		.or(just("\n"))
		.map(|_| Token::Whitespace(Whitespace::Newline))
		.or(one_of("\x20\t")
			.repeated()
			.at_least(1)
			.collect::<String>()
			.map(|ws| Token::Whitespace(Whitespace::Indentation(ws))))
}

fn lex_comment<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	let line_comment = just("//")
		.ignore_then(any().and_is(just("\n").not()).repeated().to_slice())
		.map(Token::Comment);

	let block_comment = just("/*")
		.ignore_then(
			any()
				.and_is(just("/*").or(just("*/")).not())
				.repeated()
				.to_slice(),
		)
		.then_ignore(just("*/"))
		.map(Token::Comment);

	line_comment.or(block_comment)
}

fn lex_string<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	just('"')
		.ignore_then(none_of('"').repeated().to_slice())
		.then_ignore(just('"'))
		.map(Token::Str)
}

fn lex_compiler_directive<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	just("`")
		// BUG(aki): This is not correct for verilog
		.ignore_then(text::ascii::ident())
		.map(Token::CompilerDirective)
}

fn lex_keyword_or_ident<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	text::ascii::ident().map(|kw| match kw {
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
		_ => Token::Ident(kw),
	})
}

fn lex_numeric_base<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	let bin = just("'")
		.ignore_then(just("b").or(just("B")))
		.map(|_| Token::NumericBase(NumericBase::Binary));
	let oct = just("'")
		.ignore_then(just("o").or(just("O")))
		.map(|_| Token::NumericBase(NumericBase::Octal));
	let dec = just("'")
		.ignore_then(just("d").or(just("D")))
		.map(|_| Token::NumericBase(NumericBase::Dec));
	let hex = just("'")
		.ignore_then(just("h").or(just("H")))
		.map(|_| Token::NumericBase(NumericBase::Hex));

	bin.or(oct).or(dec).or(hex)
}

fn lex_numeric_literal<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	lex_number_helper(16).map(Token::NumericLiteral)
}

fn lex_number_helper<'src, I, E>(
	radix: u32,
) -> impl Parser<'src, I, <I as SliceInput<'src>>::Slice, E> + Copy
where
	I: StrInput<'src>,
	I::Token: Char + 'src,
	E: ParserExtra<'src, I>,
	E::Error: LabelError<'src, I, TextExpected<'src, I>>,
{
	any()
		.filter(move |c: &I::Token| {
			c.is_digit(radix)
			// || c.to_ascii()
			// 	.is_some_and(|c| c == b'_' || c == b'x' || c == b'X' || c == b'z' || c == b'Z')
		})
		.map_err(move |mut err: E::Error| {
			err.label_with(TextExpected::Digit(0..radix));
			err
		})
		.repeated()
		.ignored()
		.or(just(I::Token::digit_zero()).ignored())
		.to_slice()
}

#[cfg(test)]
mod tests {
	use crate::VerilogStd;

	use super::*;
	use test_log::test;

	#[test]
	fn test_lex_whitespace() {
		let ws = lex_whitespace();

		assert_eq!(
			ws.parse("\r\n").into_output(),
			Some(Token::Whitespace(Whitespace::Newline))
		);

		assert_eq!(
			ws.parse("\n").into_output(),
			Some(Token::Whitespace(Whitespace::Newline))
		);

		assert_eq!(
			ws.parse(" ").into_output(),
			Some(Token::Whitespace(Whitespace::Indentation(" ".into())))
		);

		assert_eq!(
			ws.parse("	").into_output(),
			Some(Token::Whitespace(Whitespace::Indentation("\t".into())))
		);

		assert_eq!(
			ws.parse("	 ").into_output(),
			Some(Token::Whitespace(Whitespace::Indentation("\t\x20".into())))
		);
	}

	#[test]
	fn test_lex_comment() {
		let comment = lex_comment();

		assert_eq!(
			comment.parse("// This Is A Test").into_output(),
			Some(Token::Comment(" This Is A Test"))
		);
		assert_eq!(
			comment
				.parse(
					r#"/*
						This Is a Multi-line comment,

						It should be fine,

						// Also allow this.
					*/"#
				)
				.into_output(),
			Some(Token::Comment(
				r#"
						This Is a Multi-line comment,

						It should be fine,

						// Also allow this.
					"#
			))
		);

		assert!(
			comment
				.parse(
					r#"/*
						This is a test comment where /* this */ should not be allowed
					*/"#
				)
				.has_errors()
		);
	}

	#[test]
	fn test_lex_string() {
		let string = lex_string();

		assert_eq!(
			string.parse("\"Meow\"").into_output(),
			Some(Token::Str("Meow"))
		);
		assert!(string.parse("\"Meow").has_errors());
	}

	#[test]
	fn test_lex_compiler_directive() {
		let compiler_directive = lex_compiler_directive();

		assert_eq!(
			compiler_directive.parse("`define").into_output(),
			Some(Token::CompilerDirective("define"))
		);

		assert!(compiler_directive.parse("``meow").has_errors());
	}

	#[test]
	fn test_lex_kw_or_ident() {
		let kw_or_ident = lex_keyword_or_ident();

		assert_eq!(
			kw_or_ident.parse("always").into_output(),
			Some(Token::Keyword(Keyword::Always))
		);
		assert_eq!(
			kw_or_ident.parse("nya").into_output(),
			Some(Token::Ident("nya"))
		);
	}

	#[test]
	fn test_lex_numeric_base() {
		let numeric_base = lex_numeric_base();

		assert_eq!(
			numeric_base.parse("'b").into_output(),
			Some(Token::NumericBase(NumericBase::Binary))
		);

		assert_eq!(
			numeric_base.parse("'B").into_output(),
			Some(Token::NumericBase(NumericBase::Binary))
		);

		assert_eq!(
			numeric_base.parse("'o").into_output(),
			Some(Token::NumericBase(NumericBase::Octal))
		);

		assert_eq!(
			numeric_base.parse("'O").into_output(),
			Some(Token::NumericBase(NumericBase::Octal))
		);

		assert_eq!(
			numeric_base.parse("'d").into_output(),
			Some(Token::NumericBase(NumericBase::Dec))
		);

		assert_eq!(
			numeric_base.parse("'D").into_output(),
			Some(Token::NumericBase(NumericBase::Dec))
		);

		assert_eq!(
			numeric_base.parse("'h").into_output(),
			Some(Token::NumericBase(NumericBase::Hex))
		);

		assert_eq!(
			numeric_base.parse("'H").into_output(),
			Some(Token::NumericBase(NumericBase::Hex))
		);
	}

	#[test]
	fn test_lex_numeric_literal() {
		let numeric_literal = lex_numeric_literal();

		assert_eq!(
			numeric_literal.parse("659").into_result(),
			Ok(Token::NumericLiteral("659"))
		);

		assert_eq!(
			numeric_literal.parse("27_195_000").into_result(),
			Ok(Token::NumericLiteral("27_195_000"))
		);
		assert_eq!(
			numeric_literal.parse("837FF").into_result(),
			Ok(Token::NumericLiteral("837FF"))
		);
		assert_eq!(
			numeric_literal.parse("7460").into_result(),
			Ok(Token::NumericLiteral("7460"))
		);
		assert_eq!(
			numeric_literal.parse("1001").into_result(),
			Ok(Token::NumericLiteral("1001"))
		);
		assert_eq!(
			numeric_literal.parse("0011_0101_0001_1111").into_result(),
			Ok(Token::NumericLiteral("0011_0101_0001_1111"))
		);

		assert_eq!(
			numeric_literal.parse("01x").into_result(),
			Ok(Token::NumericLiteral("01x"))
		);
		assert_eq!(
			numeric_literal.parse("X").into_result(),
			Ok(Token::NumericLiteral("X"))
		);
		assert_eq!(
			numeric_literal.parse("z").into_result(),
			Ok(Token::NumericLiteral("z"))
		);
		assert_eq!(
			numeric_literal.parse("12ab_f001").into_result(),
			Ok(Token::NumericLiteral("12ab_f001"))
		);
	}

	#[test]
	fn test_lexer() {
		let lexer = lexer(VerilogVariant::Verilog(VerilogStd::Vl95));

		// assert_eq!(lexer.parse("").into_output(), Some(Vec::new()));

		assert_eq!(
			lexer.parse("// Test").into_output().unwrap(),
			vec![Token::Comment(" Test")]
		);

		// assert_eq!(
		// 	lexer.parse("`default_nettype wire").into_output().unwrap(),
		// 	vec![
		// 		Token::CompilerDirective("default_nettype"),
		// 		Token::Whitespace(Whitespace::Indentation(" ".into())),
		// 		Token::Keyword(Keyword::Wire)
		// 	]
		// );

		// 		assert_eq!(
		// 			lexer
		// 				.parse(
		// 					r#"
		// module foo();
		//
		// endmodule
		// "#,
		// 				)
		// 				.into_output()
		// 				.unwrap(),
		// 			vec![
		// 				Token::Whitespace(Whitespace::Newline),
		// 				Token::Keyword(Keyword::Module),
		// 				Token::Whitespace(Whitespace::Indentation(" ".into())),
		// 				Token::Ident("foo"),
		// 				Token::Ctrl('('),
		// 				Token::Ctrl(')'),
		// 				Token::Ctrl(';'),
		// 				Token::Whitespace(Whitespace::Newline),
		// 				Token::Whitespace(Whitespace::Newline),
		// 				Token::Keyword(Keyword::EndModule),
		// 				Token::Whitespace(Whitespace::Newline),
		// 			]
		// 		);
	}
}
