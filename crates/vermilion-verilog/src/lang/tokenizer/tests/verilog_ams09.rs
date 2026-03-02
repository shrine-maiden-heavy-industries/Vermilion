// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	verilog_ams09,
	line_text_macro,
	"`__LINE__",
	spanned_token!(
		Token::TextMacro(TextMacro::Other("__LINE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_ams09,
	file_text_macro,
	"`__FILE__",
	spanned_token!(
		Token::TextMacro(TextMacro::Other("__FILE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	)
);
