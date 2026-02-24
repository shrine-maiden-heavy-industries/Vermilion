// SPDX-License-Identifier: BSD-3-Clause

system_verilog17_tokenizer_test!(
	line_text_macro,
	"`__LINE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::DunderLine),
		0..9,
		Position::new(0, 0)
	),]
);

system_verilog17_tokenizer_test!(
	file_text_macro,
	"`__FILE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::DunderFile),
		0..9,
		Position::new(0, 0)
	),]
);

system_verilog17_tokenizer_test!(
	triple_quote_string,
	r#""""meow meow""""#,
	vec![spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageStd::Sv23),
		0..15,
		Position::new(0, 0)
	),]
);

system_verilog17_tokenizer_test!(
	operator_abs_tolerance,
	"+/-",
	vec![spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageStd::Sv23),
		0..3,
		Position::new(0, 0)
	),]
);

system_verilog17_tokenizer_test!(
	operator_rel_tolerance,
	"+%-",
	vec![spanned_token!(
		Token::ContextuallyInvalid("+%-".as_bytes().into(), LanguageStd::Sv23),
		0..3,
		Position::new(0, 0)
	),]
);
