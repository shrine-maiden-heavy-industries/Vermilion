// SPDX-License-Identifier: BSD-3-Clause

verilog95_tokenizer_test!(
	operator_arithmetic_shift_right,
	">>>",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			">>>".as_bytes().into(),
			VerilogVariant::Verilog(VerilogStd::Vl01)
		),
		0..3,
		Position::new(0, 0)
	),]
);

verilog95_tokenizer_test!(
	operator_arithmetic_shift_left,
	"<<<",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"<<<".as_bytes().into(),
			VerilogVariant::Verilog(VerilogStd::Vl01)
		),
		0..3,
		Position::new(0, 0)
	),]
);

verilog95_tokenizer_test!(
	operator_indexed_part_neg,
	"-:",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"-:".as_bytes().into(),
			VerilogVariant::Verilog(VerilogStd::Vl01)
		),
		0..2,
		Position::new(0, 0)
	),]
);

verilog95_tokenizer_test!(
	operator_indexed_part_pos,
	"+:",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"+:".as_bytes().into(),
			VerilogVariant::Verilog(VerilogStd::Vl01)
		),
		0..2,
		Position::new(0, 0)
	),]
);

verilog95_tokenizer_test!(
	operator_attribute_open,
	"(*",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"(*".as_bytes().into(),
			VerilogVariant::Verilog(VerilogStd::Vl01)
		),
		0..2,
		Position::new(0, 0)
	),]
);

verilog95_tokenizer_test!(
	operator_attribute_close,
	"*)",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"*)".as_bytes().into(),
			VerilogVariant::Verilog(VerilogStd::Vl01)
		),
		0..2,
		Position::new(0, 0)
	),]
);
