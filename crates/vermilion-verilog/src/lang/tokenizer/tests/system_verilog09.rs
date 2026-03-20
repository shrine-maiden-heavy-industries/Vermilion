// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	system_verilog09,
	line_text_macro,
	"`__LINE__",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Builtin(
			BuiltinTextMacro::DunderLine
		))),
		Span::from_position(0, 9, Position::new(0, 0))
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
		Span::from_position(0, 9, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog09,
	triple_quote_string,
	r#""""meow meow""""#,
	spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageStd::Sv23),
		Span::from_position(0, 15, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_equivalence,
	"<->",
	spanned_token!(
		Token::Operator(Operator::Equivalence),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_followed_by_nonoverlapped,
	"#=#",
	spanned_token!(
		Token::Operator(Operator::FollowedByNonOverlapped),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_followed_by_overlapped,
	"#-#",
	spanned_token!(
		Token::Operator(Operator::FollowedByOverlapped),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog09+,
	operator_wildcard_export,
	"*::*",
	spanned_token!(
		Token::Operator(Operator::WildcardExport),
		Span::from_position(0, 4, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog09,
	operator_abs_tolerance,
	"+/-",
	spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageStd::Sv23),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog09,
	operator_rel_tolerance,
	"+%-",
	spanned_token!(
		Token::ContextuallyInvalid("+%-".as_bytes().into(), LanguageStd::Sv23),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);
