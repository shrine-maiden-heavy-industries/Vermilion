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
	test_tokenize_tilde,
	"~",
	vec![Spanned::new(
		Token::Operator(Operator::Tilde),
		Some(Span::new(0..1, Position::new(0, 0)))
	),]
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
	test_tokenize_comment_single_line,
	"// This Is A Test",
	vec![Spanned::new(
		Token::Comment(Comment::SingleLine(" This Is A Test".as_bytes().into())),
		Some(Span::new(0..17, Position::new(0, 0)))
	)]
);

tokenizer_test!(
	test_tokenize_comment_multi_line,
	"/*\nThis Is A\n\tMulti Line Comment\n*/",
	vec![Spanned::new(
		Token::Comment(Comment::MultiLine(
			"/*\nThis Is A\n\tMulti Line Comment\n*/".as_bytes().into()
		)),
		Some(Span::new(0..35, Position::new(0, 0)))
	)]
);

tokenizer_test!(
	test_tokenize_comment_multi_line_invalid,
	"/*\nThis Is A\n/*Multi Line Comment\n*/",
	vec![Spanned::new(
		Token::Comment(Comment::Invalid(
			"/*\nThis Is A\n/*Multi Line Comment\n*/".as_bytes().into()
		)),
		Some(Span::new(0..36, Position::new(0, 0)))
	)]
);

tokenizer_test!(
	test_tokenize_comment_multi_line_truncated,
	"/*\nThis Is A\n",
	vec![Spanned::new(
		Token::Comment(Comment::Invalid("/*\nThis Is A\n".as_bytes().into())),
		Some(Span::new(0..13, Position::new(0, 0)))
	)]
);

tokenizer_test!(
	test_tokenize_compiler_directive_standalone,
	"`meow",
	vec![Spanned::new(
		Token::CompilerDirective(CompilerDirective::Name("meow".as_bytes().into())),
		Some(Span::new(0..5, Position::new(0, 0)))
	)]
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
	)]
);

tokenizer_test!(
	test_tokenize_string_eof,
	r#""This Is A Simple String :3"#,
	vec![Spanned::new(
		Token::String("This Is A Simple String :3".as_bytes().into()),
		Some(Span::new(0..27, Position::new(0, 0)))
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
	)]
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
	)]
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
	)]
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
	)]
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
