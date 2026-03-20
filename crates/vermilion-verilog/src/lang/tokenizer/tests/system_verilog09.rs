// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	system_verilog09,
	line_text_macro,
	"`__LINE__",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Builtin(
			BuiltinTextMacro::DunderLine
		))),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09,
	file_text_macro,
	"`__FILE__",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Builtin(
			BuiltinTextMacro::DunderFile
		))),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09,
	triple_quote_string,
	r#""""meow meow""""#,
	spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageStd::Sv23),
		0u32..15u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_equivalence,
	"<->",
	spanned_token!(
		Token::Operator(Operator::Equivalence),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_followed_by_nonoverlapped,
	"#=#",
	spanned_token!(
		Token::Operator(Operator::FollowedByNonOverlapped),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_followed_by_overlapped,
	"#-#",
	spanned_token!(
		Token::Operator(Operator::FollowedByOverlapped),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_wildcard_export,
	"*::*",
	spanned_token!(
		Token::Operator(Operator::WildcardExport),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09,
	operator_abs_tolerance,
	"+/-",
	spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageStd::Sv23),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog09,
	operator_rel_tolerance,
	"+%-",
	spanned_token!(
		Token::ContextuallyInvalid("+%-".as_bytes().into(), LanguageStd::Sv23),
		0u32..3u32,
		Position::new(0, 0)
	)
);
