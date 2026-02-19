// SPDX-License-Identifier: BSD-3-Clause

verilog_ams09_tokenizer_test!(
	line_text_macro,
	"`__LINE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__LINE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);

verilog_ams09_tokenizer_test!(
	file_text_macro,
	"`__FILE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__FILE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);
