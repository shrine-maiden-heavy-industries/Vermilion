// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	verilog95,
	operator_arithmetic_shift_right,
	">>>",
	spanned_token!(
		Token::ContextuallyInvalid(
			">>>".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
		),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog95,
	operator_arithmetic_shift_left,
	"<<<",
	spanned_token!(
		Token::ContextuallyInvalid(
			"<<<".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
		),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog95,
	operator_indexed_part_neg,
	"-:",
	spanned_token!(
		Token::ContextuallyInvalid(
			"-:".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
		),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog95,
	operator_indexed_part_pos,
	"+:",
	spanned_token!(
		Token::ContextuallyInvalid(
			"+:".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
		),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog95,
	operator_attribute_open,
	"(*",
	spanned_token!(
		Token::ContextuallyInvalid(
			"(*".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
		),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog95,
	operator_attribute_close,
	"*)",
	spanned_token!(
		Token::ContextuallyInvalid(
			"*)".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
		),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog95,
	operator_pow,
	"**",
	spanned_token!(
		Token::ContextuallyInvalid(
			"**".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
		),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog95,
	naked_based_number_hex_signed_lowercase,
	"'sh1",
	spanned_token!(
		Token::ContextuallyInvalid(
			"s".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
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
	)
);

tokenizer_test!(
	verilog95,
	naked_based_number_hex_signed_uppercase,
	"'Sh1",
	spanned_token!(
		Token::ContextuallyInvalid(
			"S".as_bytes().into(),
			LanguageStd::all_flags() & !LanguageStd::Vl95
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
	)
);
