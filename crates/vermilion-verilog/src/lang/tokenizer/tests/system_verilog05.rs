// SPDX-License-Identifier: BSD-3-Clause

system_verilog05_tokenizer_test!(
	line_text_macro,
	"`__LINE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__LINE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);

system_verilog05_tokenizer_test!(
	file_text_macro,
	"`__FILE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__FILE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);

system_verilog05_tokenizer_test!(
	triple_quote_string,
	r#""""meow meow""""#,
	vec![spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageSet::Sv23),
		0..15,
		Position::new(0, 0)
	),]
);

system_verilog05_tokenizer_test!(
	operator_equivalence,
	"<->",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"<->".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..3,
		Position::new(0, 0)
	),]
);

system_verilog05_tokenizer_test!(
	operator_followed_by_nonoverlapped,
	"#=#",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"#=#".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..3,
		Position::new(0, 0)
	),]
);

system_verilog05_tokenizer_test!(
	operator_followed_by_overlapped,
	"#-#",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"#-#".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..3,
		Position::new(0, 0)
	),]
);

system_verilog05_tokenizer_test!(
	operator_wildcard_export,
	"*::*",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"*::*".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..4,
		Position::new(0, 0)
	),]
);

system_verilog05_tokenizer_test!(
	operator_abs_tolerance,
	"+/-",
	vec![spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageSet::Sv23),
		0..3,
		Position::new(0, 0)
	),]
);
