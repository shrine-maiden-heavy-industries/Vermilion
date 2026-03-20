// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	system_verilog05,
	line_text_macro,
	"`__LINE__",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"__LINE__".as_bytes().into()
		))),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	file_text_macro,
	"`__FILE__",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"__FILE__".as_bytes().into()
		))),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	triple_quote_string,
	r#""""meow meow""""#,
	spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageStd::Sv23),
		0u32..15u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	operator_equivalence,
	"<->",
	spanned_token!(
		Token::ContextuallyInvalid(
			"<->".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	operator_followed_by_nonoverlapped,
	"#=#",
	spanned_token!(
		Token::ContextuallyInvalid(
			"#=#".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	operator_followed_by_overlapped,
	"#-#",
	spanned_token!(
		Token::ContextuallyInvalid(
			"#-#".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	operator_wildcard_export,
	"*::*",
	spanned_token!(
		Token::ContextuallyInvalid(
			"*::*".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	operator_abs_tolerance,
	"+/-",
	spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageStd::Sv23),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog05,
	operator_rel_tolerance,
	"+%-",
	spanned_token!(
		Token::ContextuallyInvalid("+%-".as_bytes().into(), LanguageStd::Sv23),
		0u32..3u32,
		Position::new(0, 0)
	)
);
