/* SPDX-License-Identifier: BSD-3-Clause */

pub(crate) mod token;

use chumsky::{IterParser, Parser, prelude::*};
use vermilion_lang::parser::{LexErr, LexResult, Spanned};

use crate::{
	VerilogVariant,
	lang::lexer::token::{Token, Whitespace},
};

pub(crate) fn lexer<'src>(
	_std: VerilogVariant,
) -> impl Parser<'src, &'src str, LexResult<Token<'src>>, LexErr<'src>> {
	let whitespace = lex_whitespace();
	let comment = lex_comment();
	let string = lex_string();
	// let compiler_directive =
	// let ident =
	// let number =

	let ctrl = one_of("@#()[]{}:;,.$").map(Token::Ctrl);
	let op = one_of("+-!~&|^*%=<>")
		.repeated()
		.at_least(1)
		.at_most(3)
		.to_slice()
		.map(Token::Op);

	let token = whitespace.or(comment).or(ctrl).or(op).or(string);

	token
		.map_with(|tok, e| Spanned::new(tok, Some(e.span())))
		.repeated()
		.collect()
}

fn lex_whitespace<'src>() -> impl Parser<'src, &'src str, Token<'src>, LexErr<'src>> {
	just("\r\n")
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
	fn text_lex_string() {
		let string = lex_string();

		assert_eq!(
			string.parse("\"Meow\"").into_output(),
			Some(Token::Str("Meow"))
		);
		assert!(string.parse("\"Meow").has_errors());
	}

	#[test]
	fn test_lexer() {
		let lexer = lexer(VerilogVariant::Verilog(VerilogStd::Vl95));
	}
}
