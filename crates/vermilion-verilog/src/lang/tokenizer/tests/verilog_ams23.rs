// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	verilog_ams23,
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
	verilog_ams23,
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
