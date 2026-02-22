// SPDX-License-Identifier: BSD-3-Clause

verilog95_tokenizer_test!(
	operator_arithmetic_shift_right,
	">>>",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			">>>".as_bytes().into(),
			LanguageSet::all_flags() & !LanguageSet::Vl95
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
			LanguageSet::all_flags() & !LanguageSet::Vl95
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
			LanguageSet::all_flags() & !LanguageSet::Vl95
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
			LanguageSet::all_flags() & !LanguageSet::Vl95
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
			LanguageSet::all_flags() & !LanguageSet::Vl95
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
			LanguageSet::all_flags() & !LanguageSet::Vl95
		),
		0..2,
		Position::new(0, 0)
	),]
);

verilog95_tokenizer_test!(
	operator_pow,
	"**",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"**".as_bytes().into(),
			LanguageSet::all_flags() & !LanguageSet::Vl95
		),
		0..2,
		Position::new(0, 0)
	),]
);

verilog95_tokenizer_test!(
	naked_based_number_hex_signed_lowercase,
	"'sh1",
	vec![
		spanned_token!(
			Token::ContextuallyInvalid(
				"s".as_bytes().into(),
				LanguageSet::all_flags() & !LanguageSet::Vl95
			),
			1..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier {
				specifier: BaseSpecifier::Hexadecimal,
				uppercase: false,
				signed:    false,
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

verilog95_tokenizer_test!(
	naked_based_number_hex_signed_uppercase,
	"'Sh1",
	vec![
		spanned_token!(
			Token::ContextuallyInvalid(
				"S".as_bytes().into(),
				LanguageSet::all_flags() & !LanguageSet::Vl95
			),
			1..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier {
				specifier: BaseSpecifier::Hexadecimal,
				uppercase: false,
				signed:    false,
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
