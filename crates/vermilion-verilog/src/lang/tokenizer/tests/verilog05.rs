// SPDX-License-Identifier: BSD-3-Clause

verilog05_tokenizer_test!(
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

verilog05_tokenizer_test!(
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
