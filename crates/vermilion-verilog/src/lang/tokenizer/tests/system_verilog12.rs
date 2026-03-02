// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	system_verilog12,
	line_text_macro,
	"`__LINE__",
	spanned_token!(
		Token::TextMacro(TextMacro::DunderLine),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog12,
	file_text_macro,
	"`__FILE__",
	spanned_token!(
		Token::TextMacro(TextMacro::DunderFile),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog12,
	triple_quote_string,
	r#""""meow meow""""#,
	spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageStd::Sv23),
		0..15,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog12,
	operator_abs_tolerance,
	"+/-",
	spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageStd::Sv23),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog12,
	operator_rel_tolerance,
	"+%-",
	spanned_token!(
		Token::ContextuallyInvalid("+%-".as_bytes().into(), LanguageStd::Sv23),
		0..3,
		Position::new(0, 0)
	)
);
