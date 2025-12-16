/* SPDX-License-Identifier: BSD-3-Clause */

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
	test_tokenize_whitespace_space,
	" ",
	vec![Spanned::new(
		Token::Whitespace(" ".as_bytes().into()),
		Some(Span::new(0..1, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_whitespace_tab,
	"\t",
	vec![Spanned::new(
		Token::Whitespace("\t".as_bytes().into()),
		Some(Span::new(0..1, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_whitespace_mixed,
	"\t ",
	vec![Spanned::new(
		Token::Whitespace("\t ".as_bytes().into()),
		Some(Span::new(0..2, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_newline_unix,
	"\n",
	vec![Spanned::new(
		Token::Newline("\n".as_bytes().into()),
		Some(Span::new(0..1, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_newline_win32,
	"\r\n",
	vec![Spanned::new(
		Token::Newline("\r\n".as_bytes().into()),
		Some(Span::new(0..2, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_newline_mixed,
	"\r\n\n\n",
	vec![
		Spanned::new(
			Token::Newline("\r\n".as_bytes().into()),
			Some(Span::new(0..2, Position::new(0, 0))),
		),
		Spanned::new(
			Token::Newline("\n".as_bytes().into()),
			Some(Span::new(2..3, Position::new(1, 0))),
		),
		Spanned::new(
			Token::Newline("\n".as_bytes().into()),
			Some(Span::new(3..4, Position::new(2, 0))),
		),
	]
);

tokenizer_test!(
	test_tokenize_whitespace_newlines_mixed,
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
	test_tokenize_control_at,
	"@",
	vec![Spanned::new(
		Token::Control(Control::At),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_octothorp,
	"#",
	vec![Spanned::new(
		Token::Control(Control::Octothorp),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_paren_open,
	"(",
	vec![Spanned::new(
		Token::Control(Control::ParenOpen),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_paren_close,
	")",
	vec![Spanned::new(
		Token::Control(Control::ParenClose),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_bracket_open,
	"[",
	vec![Spanned::new(
		Token::Control(Control::BracketOpen),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_bracket_close,
	"]",
	vec![Spanned::new(
		Token::Control(Control::BracketClose),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_brace_open,
	"{",
	vec![Spanned::new(
		Token::Control(Control::BraceOpen),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_brace_close,
	"}",
	vec![Spanned::new(
		Token::Control(Control::BraceClose),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_colon,
	":",
	vec![Spanned::new(
		Token::Control(Control::Colon),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_semicolon,
	";",
	vec![Spanned::new(
		Token::Control(Control::Semicolon),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_comma,
	",",
	vec![Spanned::new(
		Token::Control(Control::Comma),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_dot,
	".",
	vec![Spanned::new(
		Token::Control(Control::Dot),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_dollar,
	"$",
	vec![Spanned::new(
		Token::Control(Control::Dollar),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_control_question,
	"?",
	vec![Spanned::new(
		Token::Control(Control::Question),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_exclame,
	"!",
	vec![Spanned::new(
		Token::Operator(Operator::Exclamation),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_logical_inequality,
	"!=",
	vec![Spanned::new(
		Token::Operator(Operator::LogicalInequality),
		Some(Span::new(0..2, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_operator_case_inequality,
	"!==",
	vec![Spanned::new(
		Token::Operator(Operator::CaseInequality),
		Some(Span::new(0..3, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_operator_equal,
	"=",
	vec![Spanned::new(
		Token::Operator(Operator::Equals),
		Some(Span::new(0..1, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_operator_logical_equality,
	"==",
	vec![Spanned::new(
		Token::Operator(Operator::LogicalEquality),
		Some(Span::new(0..2, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_operator_case_equality,
	"===",
	vec![Spanned::new(
		Token::Operator(Operator::CaseEquality),
		Some(Span::new(0..3, Position::new(0, 0))),
	),]
);

tokenizer_test!(
	test_tokenize_operator_ampersand,
	"&",
	vec![Spanned::new(
		Token::Operator(Operator::Ampersand),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_logical_and,
	"&&",
	vec![Spanned::new(
		Token::Operator(Operator::LogicalAnd),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_tilde,
	"~",
	vec![Spanned::new(
		Token::Operator(Operator::Tilde),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_tilde_circumflex_right,
	"~^",
	vec![Spanned::new(
		Token::Operator(Operator::TildeCircumflex(false)),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_reduction_nand,
	"~&",
	vec![Spanned::new(
		Token::Operator(Operator::ReductionNand),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_reduction_nor,
	"~|",
	vec![Spanned::new(
		Token::Operator(Operator::ReductionNor),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_circumflex,
	"^",
	vec![Spanned::new(
		Token::Operator(Operator::Circumflex),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_tilde_circumflex_left,
	"^~",
	vec![Spanned::new(
		Token::Operator(Operator::TildeCircumflex(true)),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_pipe,
	"|",
	vec![Spanned::new(
		Token::Operator(Operator::Pipe),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_logical_or,
	"||",
	vec![Spanned::new(
		Token::Operator(Operator::LogicalOr),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_less_than,
	">",
	vec![Spanned::new(
		Token::Operator(Operator::LessThan),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_less_than_equal,
	">=",
	vec![Spanned::new(
		Token::Operator(Operator::LessThanEqual),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_shift_right,
	">>",
	vec![Spanned::new(
		Token::Operator(Operator::ShiftRight),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_greater_than,
	"<",
	vec![Spanned::new(
		Token::Operator(Operator::GreaterThan),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_greater_than_equal,
	"<=",
	vec![Spanned::new(
		Token::Operator(Operator::GreaterThanEqual),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_shift_left,
	"<<",
	vec![Spanned::new(
		Token::Operator(Operator::ShiftLeft),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_percent,
	"%",
	vec![Spanned::new(
		Token::Operator(Operator::Percent),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_asterisk,
	"*",
	vec![Spanned::new(
		Token::Operator(Operator::Asterisk),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_plus,
	"+",
	vec![Spanned::new(
		Token::Operator(Operator::Plus),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_minus,
	"-",
	vec![Spanned::new(
		Token::Operator(Operator::Minus),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_operator_solidus,
	"/",
	vec![Spanned::new(
		Token::Operator(Operator::Solidus),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_comment_single_line,
	"// This Is A Test",
	vec![Spanned::new(
		Token::Comment(Comment::SingleLine(" This Is A Test".as_bytes().into())),
		Some(Span::new(0..17, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_comment_multi_line,
	"/*\nThis Is A\n\tMulti Line Comment\n*/",
	vec![Spanned::new(
		Token::Comment(Comment::MultiLine(
			"/*\nThis Is A\n\tMulti Line Comment\n*/".as_bytes().into()
		)),
		Some(Span::new(0..35, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_comment_multi_line_invalid,
	"/*\nThis Is A\n/*Multi Line Comment\n*/",
	vec![Spanned::new(
		Token::Comment(Comment::Invalid(
			"/*\nThis Is A\n/*Multi Line Comment\n*/".as_bytes().into()
		)),
		Some(Span::new(0..36, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_comment_multi_line_truncated,
	"/*\nThis Is A\n",
	vec![Spanned::new(
		Token::Comment(Comment::Invalid("/*\nThis Is A\n".as_bytes().into())),
		Some(Span::new(0..13, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_compiler_directive_standalone,
	"`meow",
	vec![Spanned::new(
		Token::CompilerDirective(CompilerDirective::Name("meow".as_bytes().into())),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_compiler_directive_arg,
	"`define meow",
	vec![
		Spanned::new(
			Token::CompilerDirective(CompilerDirective::Name("define".as_bytes().into())),
			Some(Span::new(0..7, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(7..8, Position::new(0, 7)))
		),
		Spanned::new(
			Token::CompilerDirective(CompilerDirective::Arg("meow".as_bytes().into())),
			Some(Span::new(8..12, Position::new(0, 8)))
		)
	]
);

tokenizer_test!(
	test_tokenize_compiler_directive_multi_arg,
	"`define nya 8",
	vec![
		Spanned::new(
			Token::CompilerDirective(CompilerDirective::Name("define".as_bytes().into())),
			Some(Span::new(0..7, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(7..8, Position::new(0, 7)))
		),
		Spanned::new(
			Token::CompilerDirective(CompilerDirective::Arg("nya".as_bytes().into())),
			Some(Span::new(8..11, Position::new(0, 8)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(11..12, Position::new(0, 11)))
		),
		Spanned::new(
			Token::CompilerDirective(CompilerDirective::Arg("8".as_bytes().into())),
			Some(Span::new(12..13, Position::new(0, 12)))
		),
	]
);

tokenizer_test!(
	test_tokenize_string,
	r#""This Is A Simple String :3""#,
	vec![Spanned::new(
		Token::String("This Is A Simple String :3".as_bytes().into()),
		Some(Span::new(0..28, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_string_eof,
	r#""This Is A Simple String :3"#,
	vec![Spanned::new(
		Token::String("This Is A Simple String :3".as_bytes().into()),
		Some(Span::new(0..27, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_ident,
	"meow",
	vec![Spanned::new(
		Token::Identifier("meow".as_bytes().into()),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_ident_adv,
	"m3ow_me0w",
	vec![Spanned::new(
		Token::Identifier("m3ow_me0w".as_bytes().into()),
		Some(Span::new(0..9, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_always,
	"always",
	vec![Spanned::new(
		Token::Keyword(Keyword::Always),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_and,
	"and",
	vec![Spanned::new(
		Token::Keyword(Keyword::And),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_assign,
	"assign",
	vec![Spanned::new(
		Token::Keyword(Keyword::Assign),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_begin,
	"begin",
	vec![Spanned::new(
		Token::Keyword(Keyword::Begin),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_buf,
	"buf",
	vec![Spanned::new(
		Token::Keyword(Keyword::Buf),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_bufif0,
	"bufif0",
	vec![Spanned::new(
		Token::Keyword(Keyword::BufIf0),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_bufif1,
	"bufif1",
	vec![Spanned::new(
		Token::Keyword(Keyword::BufIf1),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_case,
	"case",
	vec![Spanned::new(
		Token::Keyword(Keyword::Case),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_casex,
	"casex",
	vec![Spanned::new(
		Token::Keyword(Keyword::CaseX),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_casez,
	"casez",
	vec![Spanned::new(
		Token::Keyword(Keyword::CaseZ),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_cmos,
	"cmos",
	vec![Spanned::new(
		Token::Keyword(Keyword::Cmos),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_deassign,
	"deassign",
	vec![Spanned::new(
		Token::Keyword(Keyword::Deassign),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_default,
	"default",
	vec![Spanned::new(
		Token::Keyword(Keyword::Default),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_defparam,
	"defparam",
	vec![Spanned::new(
		Token::Keyword(Keyword::DefParam),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_disable,
	"disable",
	vec![Spanned::new(
		Token::Keyword(Keyword::Disable),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_edge,
	"edge",
	vec![Spanned::new(
		Token::Keyword(Keyword::Edge),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_else,
	"else",
	vec![Spanned::new(
		Token::Keyword(Keyword::Else),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_end,
	"end",
	vec![Spanned::new(
		Token::Keyword(Keyword::End),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_endcase,
	"endcase",
	vec![Spanned::new(
		Token::Keyword(Keyword::EndCase),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_endfunction,
	"endfunction",
	vec![Spanned::new(
		Token::Keyword(Keyword::EndFunction),
		Some(Span::new(0..11, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_endmodule,
	"endmodule",
	vec![Spanned::new(
		Token::Keyword(Keyword::EndModule),
		Some(Span::new(0..9, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_endprimitive,
	"endprimitive",
	vec![Spanned::new(
		Token::Keyword(Keyword::EndPrimitive),
		Some(Span::new(0..12, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_endspecify,
	"endspecify",
	vec![Spanned::new(
		Token::Keyword(Keyword::EndSpecify),
		Some(Span::new(0..10, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_endtable,
	"endtable",
	vec![Spanned::new(
		Token::Keyword(Keyword::EndTable),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_endtask,
	"endtask",
	vec![Spanned::new(
		Token::Keyword(Keyword::EndTask),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_event,
	"event",
	vec![Spanned::new(
		Token::Keyword(Keyword::Event),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_for,
	"for",
	vec![Spanned::new(
		Token::Keyword(Keyword::For),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_force,
	"force",
	vec![Spanned::new(
		Token::Keyword(Keyword::Force),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_forever,
	"forever",
	vec![Spanned::new(
		Token::Keyword(Keyword::Forever),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_fork,
	"fork",
	vec![Spanned::new(
		Token::Keyword(Keyword::Fork),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_function,
	"function",
	vec![Spanned::new(
		Token::Keyword(Keyword::Function),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_highz0,
	"highz0",
	vec![Spanned::new(
		Token::Keyword(Keyword::HighZ0),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_highz1,
	"highz1",
	vec![Spanned::new(
		Token::Keyword(Keyword::HighZ1),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_if,
	"if",
	vec![Spanned::new(
		Token::Keyword(Keyword::If),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_ifnone,
	"ifnone",
	vec![Spanned::new(
		Token::Keyword(Keyword::IfNone),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_initial,
	"initial",
	vec![Spanned::new(
		Token::Keyword(Keyword::Initial),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_inout,
	"inout",
	vec![Spanned::new(
		Token::Keyword(Keyword::InOut),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_input,
	"input",
	vec![Spanned::new(
		Token::Keyword(Keyword::Input),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_integer,
	"integer",
	vec![Spanned::new(
		Token::Keyword(Keyword::Integer),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_join,
	"join",
	vec![Spanned::new(
		Token::Keyword(Keyword::Join),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_large,
	"large",
	vec![Spanned::new(
		Token::Keyword(Keyword::Large),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_macromodule,
	"macromodule",
	vec![Spanned::new(
		Token::Keyword(Keyword::MacroModule),
		Some(Span::new(0..11, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_medium,
	"medium",
	vec![Spanned::new(
		Token::Keyword(Keyword::Medium),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_module,
	"module",
	vec![Spanned::new(
		Token::Keyword(Keyword::Module),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_nand,
	"nand",
	vec![Spanned::new(
		Token::Keyword(Keyword::Nand),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_negedge,
	"negedge",
	vec![Spanned::new(
		Token::Keyword(Keyword::NegEdge),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_nmos,
	"nmos",
	vec![Spanned::new(
		Token::Keyword(Keyword::Nmos),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_nor,
	"nor",
	vec![Spanned::new(
		Token::Keyword(Keyword::Nor),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_not,
	"not",
	vec![Spanned::new(
		Token::Keyword(Keyword::Not),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_notif0,
	"notif0",
	vec![Spanned::new(
		Token::Keyword(Keyword::NotIf0),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_notif1,
	"notif1",
	vec![Spanned::new(
		Token::Keyword(Keyword::NotIf1),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_or,
	"or",
	vec![Spanned::new(
		Token::Keyword(Keyword::Or),
		Some(Span::new(0..2, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_output,
	"output",
	vec![Spanned::new(
		Token::Keyword(Keyword::Output),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_parameter,
	"parameter",
	vec![Spanned::new(
		Token::Keyword(Keyword::Parameter),
		Some(Span::new(0..9, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_pmos,
	"pmos",
	vec![Spanned::new(
		Token::Keyword(Keyword::Pmos),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_posedge,
	"posedge",
	vec![Spanned::new(
		Token::Keyword(Keyword::PosEdge),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_primitive,
	"primitive",
	vec![Spanned::new(
		Token::Keyword(Keyword::Primitive),
		Some(Span::new(0..9, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_pull0,
	"pull0",
	vec![Spanned::new(
		Token::Keyword(Keyword::Pull0),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_pull1,
	"pull1",
	vec![Spanned::new(
		Token::Keyword(Keyword::Pull1),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_pulldown,
	"pulldown",
	vec![Spanned::new(
		Token::Keyword(Keyword::Pulldown),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_pullup,
	"pullup",
	vec![Spanned::new(
		Token::Keyword(Keyword::Pullup),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_rcmos,
	"rcmos",
	vec![Spanned::new(
		Token::Keyword(Keyword::Rcmos),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_real,
	"real",
	vec![Spanned::new(
		Token::Keyword(Keyword::Real),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_realtime,
	"realtime",
	vec![Spanned::new(
		Token::Keyword(Keyword::Realtime),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_reg,
	"reg",
	vec![Spanned::new(
		Token::Keyword(Keyword::Reg),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_release,
	"release",
	vec![Spanned::new(
		Token::Keyword(Keyword::Release),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_repeat,
	"repeat",
	vec![Spanned::new(
		Token::Keyword(Keyword::Repeat),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_rnmos,
	"rnmos",
	vec![Spanned::new(
		Token::Keyword(Keyword::Rnmos),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_rpmos,
	"rpmos",
	vec![Spanned::new(
		Token::Keyword(Keyword::Rpmos),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_rtran,
	"rtran",
	vec![Spanned::new(
		Token::Keyword(Keyword::Rtran),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_rtranif0,
	"rtranif0",
	vec![Spanned::new(
		Token::Keyword(Keyword::RtranIf0),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_rtranif1,
	"rtranif1",
	vec![Spanned::new(
		Token::Keyword(Keyword::RtranIf1),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_scalared,
	"scalared",
	vec![Spanned::new(
		Token::Keyword(Keyword::Scalared),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_small,
	"small",
	vec![Spanned::new(
		Token::Keyword(Keyword::Small),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_specify,
	"specify",
	vec![Spanned::new(
		Token::Keyword(Keyword::Specify),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_specparam,
	"specparam",
	vec![Spanned::new(
		Token::Keyword(Keyword::SpecParam),
		Some(Span::new(0..9, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_strong0,
	"strong0",
	vec![Spanned::new(
		Token::Keyword(Keyword::Strong0),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_strong1,
	"strong1",
	vec![Spanned::new(
		Token::Keyword(Keyword::Strong1),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_supply0,
	"supply0",
	vec![Spanned::new(
		Token::Keyword(Keyword::Supply0),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_supply1,
	"supply1",
	vec![Spanned::new(
		Token::Keyword(Keyword::Supply1),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_table,
	"table",
	vec![Spanned::new(
		Token::Keyword(Keyword::Table),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_task,
	"task",
	vec![Spanned::new(
		Token::Keyword(Keyword::Task),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_time,
	"time",
	vec![Spanned::new(
		Token::Keyword(Keyword::Time),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_tran,
	"tran",
	vec![Spanned::new(
		Token::Keyword(Keyword::Tran),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_tranif0,
	"tranif0",
	vec![Spanned::new(
		Token::Keyword(Keyword::TranIf0),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_tranif1,
	"tranif1",
	vec![Spanned::new(
		Token::Keyword(Keyword::TranIf1),
		Some(Span::new(0..7, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_tri,
	"tri",
	vec![Spanned::new(
		Token::Keyword(Keyword::Tri),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_tri0,
	"tri0",
	vec![Spanned::new(
		Token::Keyword(Keyword::Tri0),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_tri1,
	"tri1",
	vec![Spanned::new(
		Token::Keyword(Keyword::Tri1),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_triand,
	"triand",
	vec![Spanned::new(
		Token::Keyword(Keyword::Triand),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_trior,
	"trior",
	vec![Spanned::new(
		Token::Keyword(Keyword::Trior),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_trireg,
	"trireg",
	vec![Spanned::new(
		Token::Keyword(Keyword::Trireg),
		Some(Span::new(0..6, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_vectored,
	"vectored",
	vec![Spanned::new(
		Token::Keyword(Keyword::Vectored),
		Some(Span::new(0..8, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_wait,
	"wait",
	vec![Spanned::new(
		Token::Keyword(Keyword::Wait),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_wand,
	"wand",
	vec![Spanned::new(
		Token::Keyword(Keyword::Wand),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_weak0,
	"weak0",
	vec![Spanned::new(
		Token::Keyword(Keyword::Weak0),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_weak1,
	"weak1",
	vec![Spanned::new(
		Token::Keyword(Keyword::Weak1),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_while,
	"while",
	vec![Spanned::new(
		Token::Keyword(Keyword::While),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_wire,
	"wire",
	vec![Spanned::new(
		Token::Keyword(Keyword::Wire),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_wor,
	"wor",
	vec![Spanned::new(
		Token::Keyword(Keyword::Wor),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_xnor,
	"xnor",
	vec![Spanned::new(
		Token::Keyword(Keyword::Xnor),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_keyword_xor,
	"xor",
	vec![Spanned::new(
		Token::Keyword(Keyword::Xor),
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_extended_ident,
	r"\meow",
	vec![Spanned::new(
		Token::Identifier(r"\meow".as_bytes().into()),
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_extended_ident_adv,
	r"\nya$owo@uwu",
	vec![Spanned::new(
		Token::Identifier(r"\nya$owo@uwu".as_bytes().into()),
		Some(Span::new(0..12, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_unsigned_number,
	"1234",
	vec![Spanned::new(
		Token::UnsignedNumber("1234".as_bytes().into()),
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_real_number_simple,
	"1.23",
	vec![Spanned::new(
		Token::Real { value: 1.23, exponent: None },
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_real_number_simple_pos,
	"+1.23",
	vec![
		Spanned::new(
			Token::Operator(Operator::Plus),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Real { value: 1.23, exponent: None },
			Some(Span::new(1..5, Position::new(0, 1)))
		),
	]
);

tokenizer_test!(
	test_tokenize_real_number_simple_neg,
	"-1.23",
	vec![
		Spanned::new(
			Token::Operator(Operator::Minus),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Real { value: 1.23, exponent: None },
			Some(Span::new(1..5, Position::new(0, 1)))
		),
	]
);

tokenizer_test!(
	test_tokenize_real_number_exponent,
	"1e7",
	vec![Spanned::new(
		Token::Real { value: 1e7, exponent: Some("e7".as_bytes().into()) },
		Some(Span::new(0..3, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_real_number_dec_exponent,
	"1.2e6",
	vec![Spanned::new(
		Token::Real {
			value: 1.2e6,
			exponent: Some("e6".as_bytes().into())
		},
		Some(Span::new(0..5, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_real_number_exponent_pos,
	"1e+6",
	vec![Spanned::new(
		Token::Real {
			value: 1e6,
			exponent: Some("e+6".as_bytes().into())
		},
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_real_number_exponent_neg,
	"1e-6",
	vec![Spanned::new(
		Token::Real {
			value: 1e-6,
			exponent: Some("e-6".as_bytes().into())
		},
		Some(Span::new(0..4, Position::new(0, 0)))
	),]
);

tokenizer_test!(
	test_tokenize_binary_prefixed,
	"4'b1001",
	vec![
		Spanned::new(
			Token::UnsignedNumber("4".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("1001".as_bytes().into()),
			Some(Span::new(3..7, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_prefixed_dont_care,
	"4'bxx1x",
	vec![
		Spanned::new(
			Token::UnsignedNumber("4".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("xx1x".as_bytes().into()),
			Some(Span::new(3..7, Position::new(0, 3)))
		),
	]
);
tokenizer_test!(
	test_tokenize_binary_prefixed_all_dont_care,
	"1'bx",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(3..4, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_prefixed_high_z,
	"4'bzz11",
	vec![
		Spanned::new(
			Token::UnsignedNumber("4".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("zz11".as_bytes().into()),
			Some(Span::new(3..7, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_prefixed_all_high_z,
	"1'bz",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(3..4, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_prefixed_padded,
	"4 'b1001",
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
			Token::Number("1001".as_bytes().into()),
			Some(Span::new(4..8, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_prefixed_padded_dont_care,
	"4 'bxx1x",
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
			Token::Number("xx1x".as_bytes().into()),
			Some(Span::new(4..8, Position::new(0, 4)))
		),
	]
);
tokenizer_test!(
	test_tokenize_binary_prefixed_padded_all_dont_care,
	"1 'bx",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
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
			Token::Number("x".as_bytes().into()),
			Some(Span::new(4..5, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_prefixed_padded_high_z,
	"4 'bzz11",
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
			Token::Number("zz11".as_bytes().into()),
			Some(Span::new(4..8, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_prefixed_padded_all_high_z,
	"1 'bz",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
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
			Token::Number("z".as_bytes().into()),
			Some(Span::new(4..5, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_naked,
	"'b10",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("10".as_bytes().into()),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_naked_dont_care,
	"'bxx1x",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("xx1x".as_bytes().into()),
			Some(Span::new(2..6, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_naked_all_dont_care,
	"'bx",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(2..3, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_naked_high_z,
	"'bzz11",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("zz11".as_bytes().into()),
			Some(Span::new(2..6, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_binary_naked_all_high_z,
	"'bz",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(2..3, Position::new(0, 2)))
		),
	]
);

// TODO(aki): Invalid Digits

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
			Token::BaseSpecifier(BaseSpecifier::Binary, false),
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
			Token::Operator(Operator::Plus),
			Some(Span::new(9..10, Position::new(1, 0)))
		),
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(10..11, Position::new(1, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(BaseSpecifier::Binary, true),
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
			Token::BaseSpecifier(BaseSpecifier::Binary, false),
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

tokenizer_test!(
	test_tokenize_octal_prefixed,
	"3'o666",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("666".as_bytes().into()),
			Some(Span::new(3..6, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_prefixed_dont_care,
	"3'ox3x",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("x3x".as_bytes().into()),
			Some(Span::new(3..6, Position::new(0, 3)))
		),
	]
);
tokenizer_test!(
	test_tokenize_octal_prefixed_all_dont_care,
	"1'ox",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(3..4, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_prefixed_high_z,
	"3'ozz4",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("zz4".as_bytes().into()),
			Some(Span::new(3..6, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_prefixed_all_high_z,
	"1'oz",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(3..4, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_prefixed_padded,
	"3 'o464",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("464".as_bytes().into()),
			Some(Span::new(4..7, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_prefixed_padded_dont_care,
	"3 'ox2x",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("x2x".as_bytes().into()),
			Some(Span::new(4..7, Position::new(0, 4)))
		),
	]
);
tokenizer_test!(
	test_tokenize_octal_prefixed_padded_all_dont_care,
	"1 'ox",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(4..5, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_prefixed_padded_high_z,
	"3 'ozz1",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("zz1".as_bytes().into()),
			Some(Span::new(4..7, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_prefixed_padded_all_high_z,
	"1 'oz",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(4..5, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_naked,
	"'o66",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("66".as_bytes().into()),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_naked_dont_care,
	"'ox3x",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("x3x".as_bytes().into()),
			Some(Span::new(2..5, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_naked_all_dont_care,
	"'ox",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(2..3, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_naked_high_z,
	"'ozz7",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("zz7".as_bytes().into()),
			Some(Span::new(2..5, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_octal_naked_all_high_z,
	"'oz",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(2..3, Position::new(0, 2)))
		),
	]
);

// TODO(aki): Invalid Digits

tokenizer_test!(
	test_tokenize_octal_lump,
	"4 'o06zx\n+1'O ?\n2'oZX\n",
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
			Token::BaseSpecifier(BaseSpecifier::Octal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("06zx".as_bytes().into()),
			Some(Span::new(4..8, Position::new(0, 4)))
		),
		Spanned::new(
			Token::Newline("\n".as_bytes().into()),
			Some(Span::new(8..9, Position::new(0, 8)))
		),
		Spanned::new(
			Token::Operator(Operator::Plus),
			Some(Span::new(9..10, Position::new(1, 0)))
		),
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(10..11, Position::new(1, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(BaseSpecifier::Octal, true),
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
			Token::BaseSpecifier(BaseSpecifier::Octal, false),
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

tokenizer_test!(
	test_tokenize_decimal_prefixed,
	"2'd69",
	vec![
		Spanned::new(
			Token::UnsignedNumber("2".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("69".as_bytes().into()),
			Some(Span::new(3..5, Position::new(0, 3)))
		),
	]
);

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_dont_care,
// 	"3'dx3x",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("3".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(1..3, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::Number("x3x".as_bytes().into()),
// 			Some(Span::new(3..6, Position::new(0, 3)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_all_dont_care,
// 	"1'dx",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("1".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(1..3, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::Number("x".as_bytes().into()),
// 			Some(Span::new(3..4, Position::new(0, 3)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_high_z,
// 	"3'dzz4",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("3".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(1..3, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::Number("zz4".as_bytes().into()),
// 			Some(Span::new(3..6, Position::new(0, 3)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_all_high_z,
// 	"1'dz",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("1".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(1..3, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::Number("z".as_bytes().into()),
// 			Some(Span::new(3..4, Position::new(0, 3)))
// 		),
// 	]
// );

tokenizer_test!(
	test_tokenize_decimal_prefixed_padded,
	"2 'd99",
	vec![
		Spanned::new(
			Token::UnsignedNumber("2".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("99".as_bytes().into()),
			Some(Span::new(4..6, Position::new(0, 4)))
		),
	]
);

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_padded_dont_care,
// 	"3 'dx9x",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("3".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Whitespace(" ".as_bytes().into()),
// 			Some(Span::new(1..2, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(2..4, Position::new(0, 2)))
// 		),
// 		Spanned::new(
// 			Token::Number("x9x".as_bytes().into()),
// 			Some(Span::new(4..7, Position::new(0, 4)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_padded_all_dont_care,
// 	"1 'dx",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("1".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Whitespace(" ".as_bytes().into()),
// 			Some(Span::new(1..2, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(2..4, Position::new(0, 2)))
// 		),
// 		Spanned::new(
// 			Token::Number("x".as_bytes().into()),
// 			Some(Span::new(4..5, Position::new(0, 4)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_padded_high_z,
// 	"3 'dzz7",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("3".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Whitespace(" ".as_bytes().into()),
// 			Some(Span::new(1..2, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(2..4, Position::new(0, 2)))
// 		),
// 		Spanned::new(
// 			Token::Number("zz7".as_bytes().into()),
// 			Some(Span::new(4..7, Position::new(0, 4)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_prefixed_padded_all_high_z,
// 	"1 'dz",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("1".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Whitespace(" ".as_bytes().into()),
// 			Some(Span::new(1..2, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(2..4, Position::new(0, 2)))
// 		),
// 		Spanned::new(
// 			Token::Number("z".as_bytes().into()),
// 			Some(Span::new(4..5, Position::new(0, 4)))
// 		),
// 	]
// );

tokenizer_test!(
	test_tokenize_decimal_naked,
	"'d90",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("90".as_bytes().into()),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
	]
);

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_naked_dont_care,
// 	"'dx9x",
// 	vec![
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(0..2, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Number("x3x".as_bytes().into()),
// 			Some(Span::new(2..5, Position::new(0, 2)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_naked_all_dont_care,
// 	"'dx",
// 	vec![
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(0..2, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Number("x".as_bytes().into()),
// 			Some(Span::new(2..3, Position::new(0, 2)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_naked_high_z,
// 	"'dzz9",
// 	vec![
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(0..2, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Number("zz9".as_bytes().into()),
// 			Some(Span::new(2..5, Position::new(0, 2)))
// 		),
// 	]
// );

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_naked_all_high_z,
// 	"'dz",
// 	vec![
// 		Spanned::new(
// 			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
// 			Some(Span::new(0..2, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Number("z".as_bytes().into()),
// 			Some(Span::new(2..3, Position::new(0, 2)))
// 		),
// 	]
// );

// TODO(aki): Invalid Digits

// BUG(aki): Spins
// tokenizer_test!(
// 	test_tokenize_decimal_lump,
// 	"4 'd06zx\n+1'D ?\n2'dZX\n",
// 	vec![
// 		Spanned::new(
// 			Token::UnsignedNumber("4".as_bytes().into()),
// 			Some(Span::new(0..1, Position::new(0, 0)))
// 		),
// 		Spanned::new(
// 			Token::Whitespace(" ".as_bytes().into()),
// 			Some(Span::new(1..2, Position::new(0, 1)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(BaseSpecifier::Decimal, false),
// 			Some(Span::new(2..4, Position::new(0, 2)))
// 		),
// 		Spanned::new(
// 			Token::Number("06zx".as_bytes().into()),
// 			Some(Span::new(4..8, Position::new(0, 4)))
// 		),
// 		Spanned::new(
// 			Token::Newline("\n".as_bytes().into()),
// 			Some(Span::new(8..9, Position::new(0, 8)))
// 		),
// 		Spanned::new(
// 			Token::Operator(Operator::Plus),
// 			Some(Span::new(9..10, Position::new(1, 0)))
// 		),
// 		Spanned::new(
// 			Token::UnsignedNumber("1".as_bytes().into()),
// 			Some(Span::new(10..11, Position::new(1, 1)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(BaseSpecifier::Decimal, true),
// 			Some(Span::new(11..13, Position::new(1, 2)))
// 		),
// 		Spanned::new(
// 			Token::Whitespace(" ".as_bytes().into()),
// 			Some(Span::new(13..14, Position::new(1, 4)))
// 		),
// 		Spanned::new(
// 			Token::Number("?".as_bytes().into()),
// 			Some(Span::new(14..15, Position::new(1, 5)))
// 		),
// 		Spanned::new(
// 			Token::Newline("\n".as_bytes().into()),
// 			Some(Span::new(15..16, Position::new(1, 6)))
// 		),
// 		Spanned::new(
// 			Token::UnsignedNumber("2".as_bytes().into()),
// 			Some(Span::new(16..17, Position::new(2, 0)))
// 		),
// 		Spanned::new(
// 			Token::BaseSpecifier(BaseSpecifier::Decimal, false),
// 			Some(Span::new(17..19, Position::new(2, 1)))
// 		),
// 		Spanned::new(
// 			Token::Number("ZX".as_bytes().into()),
// 			Some(Span::new(19..21, Position::new(2, 3)))
// 		),
// 		Spanned::new(
// 			Token::Newline("\n".as_bytes().into()),
// 			Some(Span::new(21..22, Position::new(2, 5)))
// 		),
// 	]
// );

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed,
	"4'h1a4F",
	vec![
		Spanned::new(
			Token::UnsignedNumber("4".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("1a4F".as_bytes().into()),
			Some(Span::new(3..7, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_dont_care,
	"3'hxDx",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("xDx".as_bytes().into()),
			Some(Span::new(3..6, Position::new(0, 3)))
		),
	]
);
tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_all_dont_care,
	"1'hx",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(3..4, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_high_z,
	"3'hzzF",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("zzF".as_bytes().into()),
			Some(Span::new(3..6, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_all_high_z,
	"1'hz",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(1..3, Position::new(0, 1)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(3..4, Position::new(0, 3)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_padded,
	"3 'ha7d",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("a7d".as_bytes().into()),
			Some(Span::new(4..7, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_padded_dont_care,
	"3 'hxEx",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("xEx".as_bytes().into()),
			Some(Span::new(4..7, Position::new(0, 4)))
		),
	]
);
tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_padded_all_dont_care,
	"1 'hx",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(4..5, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_padded_high_z,
	"3 'hzzA",
	vec![
		Spanned::new(
			Token::UnsignedNumber("3".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("zzA".as_bytes().into()),
			Some(Span::new(4..7, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_prefixed_padded_all_high_z,
	"1 'hz",
	vec![
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(0..1, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Whitespace(" ".as_bytes().into()),
			Some(Span::new(1..2, Position::new(0, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(4..5, Position::new(0, 4)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_naked,
	"'h7F",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("7F".as_bytes().into()),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_naked_dont_care,
	"'hxAx",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("xAx".as_bytes().into()),
			Some(Span::new(2..5, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_naked_all_dont_care,
	"'hx",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("x".as_bytes().into()),
			Some(Span::new(2..3, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_naked_high_z,
	"'hzz7",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("zz7".as_bytes().into()),
			Some(Span::new(2..5, Position::new(0, 2)))
		),
	]
);

tokenizer_test!(
	test_tokenize_hexadecimal_naked_all_high_z,
	"'hz",
	vec![
		Spanned::new(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			Some(Span::new(0..2, Position::new(0, 0)))
		),
		Spanned::new(
			Token::Number("z".as_bytes().into()),
			Some(Span::new(2..3, Position::new(0, 2)))
		),
	]
);

// TODO(aki): Invalid Digits

tokenizer_test!(
	test_tokenize_hexadecimal_lump,
	"4 'h7Fzx\n+1'H ?\n2'hZX\n",
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
			Token::BaseSpecifier(BaseSpecifier::Hexadecimal, false),
			Some(Span::new(2..4, Position::new(0, 2)))
		),
		Spanned::new(
			Token::Number("7Fzx".as_bytes().into()),
			Some(Span::new(4..8, Position::new(0, 4)))
		),
		Spanned::new(
			Token::Newline("\n".as_bytes().into()),
			Some(Span::new(8..9, Position::new(0, 8)))
		),
		Spanned::new(
			Token::Operator(Operator::Plus),
			Some(Span::new(9..10, Position::new(1, 0)))
		),
		Spanned::new(
			Token::UnsignedNumber("1".as_bytes().into()),
			Some(Span::new(10..11, Position::new(1, 1)))
		),
		Spanned::new(
			Token::BaseSpecifier(BaseSpecifier::Hexadecimal, true),
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
			Token::BaseSpecifier(BaseSpecifier::Hexadecimal, false),
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
