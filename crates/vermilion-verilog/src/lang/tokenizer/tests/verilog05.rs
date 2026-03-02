// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	verilog05,
	naked_based_number_hex_signed_lowercase,
	"'sh1",
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
	)
);

tokenizer_test!(
	verilog05,
	naked_based_number_hex_signed_uppercase,
	"'Sh1",
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
	)
);
