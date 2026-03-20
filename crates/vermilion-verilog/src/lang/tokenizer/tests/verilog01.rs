// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	verilog01+,
	operator_arithmetic_shift_right,
	">>>",
	spanned_token!(
		Token::Operator(Operator::ArithmeticShr),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog01+,
	operator_arithmetic_shift_left,
	"<<<",
	spanned_token!(
		Token::Operator(Operator::ArithmeticShl),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog01+,
	operator_indexed_part_neg,
	"-:",
	spanned_token!(
		Token::Operator(Operator::IndexedPartNeg),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog01+,
	operator_indexed_part_pos,
	"+:",
	spanned_token!(
		Token::Operator(Operator::IndexedPartPos),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog01+,
	operator_attribute_open,
	"(*",
	spanned_token!(
		Token::Control(Control::AttributeOpen),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog01+,
	operator_attribute_close,
	"*)",
	spanned_token!(
		Token::Control(Control::AttributeClose),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog01+,
	operator_pow,
	"**",
	spanned_token!(Token::Operator(Operator::Pow), 0u32..2u32, Position::new(0, 0))
);

tokenizer_test!(
	verilog01,
	naked_based_number_hex_signed_lowercase,
	"'sh1",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			true,
		)),
		0u32..3u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("1".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	verilog01,
	naked_based_number_hex_signed_uppercase,
	"'Sh1",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			true,
		)),
		0u32..3u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("1".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);
