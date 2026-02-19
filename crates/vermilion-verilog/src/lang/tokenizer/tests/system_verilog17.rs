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
