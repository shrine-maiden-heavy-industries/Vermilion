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
