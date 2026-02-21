// SPDX-License-Identifier: BSD-3-Clause

all_verilog_tokenizer_test!(
	operator_branch_contribution,
	"<+",
	vec![spanned_token!(
		Token::ContextuallyInvalid("<+".as_bytes().into(), LanguageSet::VERILOG_AMS_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	line_text_macro,
	"`__LINE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__LINE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	file_text_macro,
	"`__FILE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__FILE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	triple_quote_string,
	r#""""meow meow""""#,
	vec![spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageSet::Sv23),
		0..15,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	control_apostrophe,
	"'",
	vec![spanned_token!(
		Token::ContextuallyInvalid("'".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..1,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_add_equals,
	"+=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("+=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_sub_equals,
	"-=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("-=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);
