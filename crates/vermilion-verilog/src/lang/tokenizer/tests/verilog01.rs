// SPDX-License-Identifier: BSD-3-Clause

verilog01_and_up_tokenizer_test!(
	operator_arithmetic_shift_right,
	">>>",
	vec![spanned_token!(
		Token::Operator(Operator::ArithmeticShr),
		0..3,
		Position::new(0, 0)
	),]
);

verilog01_and_up_tokenizer_test!(
	operator_arithmetic_shift_left,
	"<<<",
	vec![spanned_token!(
		Token::Operator(Operator::ArithmeticShl),
		0..3,
		Position::new(0, 0)
	),]
);

verilog01_and_up_tokenizer_test!(
	operator_indexed_part_neg,
	"-:",
	vec![spanned_token!(
		Token::Operator(Operator::IndexedPartNeg),
		0..2,
		Position::new(0, 0)
	),]
);

verilog01_and_up_tokenizer_test!(
	operator_indexed_part_pos,
	"+:",
	vec![spanned_token!(
		Token::Operator(Operator::IndexedPartPos),
		0..2,
		Position::new(0, 0)
	),]
);

verilog01_and_up_tokenizer_test!(
	operator_attribute_open,
	"(*",
	vec![spanned_token!(
		Token::Control(Control::AttributeOpen),
		0..2,
		Position::new(0, 0)
	),]
);

verilog01_and_up_tokenizer_test!(
	operator_attribute_close,
	"*)",
	vec![spanned_token!(
		Token::Control(Control::AttributeClose),
		0..2,
		Position::new(0, 0)
	),]
);

verilog01_and_up_tokenizer_test!(
	operator_pow,
	"**",
	vec![spanned_token!(Token::Operator(Operator::Pow), 0..2, Position::new(0, 0)),]
);

verilog01_tokenizer_test!(
	naked_based_number_hex_signed_lowercase,
	"'sh1",
	vec![
		spanned_token!(
			Token::BaseSpecifier {
				specifier: BaseSpecifier::Hexadecimal,
				uppercase: false,
				signed:    true,
			},
			0..3,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("1".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);

verilog01_tokenizer_test!(
	naked_based_number_hex_signed_uppercase,
	"'Sh1",
	vec![
		spanned_token!(
			Token::BaseSpecifier {
				specifier: BaseSpecifier::Hexadecimal,
				uppercase: false,
				signed:    true,
			},
			0..3,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("1".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);
