// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	system_verilog_all,
	operator_branch_contribution,
	"<+",
	spanned_token!(
		Token::ContextuallyInvalid("<+".as_bytes().into(), LanguageStd::VERILOG_AMS_STDS),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	control_apostrophe,
	"'",
	spanned_token!(
		Token::Control(Control::Apostrophe),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_add_equals,
	"+=",
	spanned_token!(
		Token::Operator(Operator::AddEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_sub_equals,
	"-=",
	spanned_token!(
		Token::Operator(Operator::SubEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_and_equals,
	"&=",
	spanned_token!(
		Token::Operator(Operator::AndEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_arithmetic_shr_equals,
	">>>=",
	spanned_token!(
		Token::Operator(Operator::ArithmeticShrEquals),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_arithmetic_shl_equals,
	"<<<=",
	spanned_token!(
		Token::Operator(Operator::ArithmeticShlEquals),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_or_equals,
	"|=",
	spanned_token!(
		Token::Operator(Operator::OrEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_mul_equals,
	"*=",
	spanned_token!(
		Token::Operator(Operator::MulEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_rem_equals,
	"%=",
	spanned_token!(
		Token::Operator(Operator::RemEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_div_equals,
	"/=",
	spanned_token!(
		Token::Operator(Operator::DivEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_inc,
	"++",
	spanned_token!(
		Token::Operator(Operator::Increment),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_dec,
	"--",
	spanned_token!(
		Token::Operator(Operator::Decrement),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_shl_equals,
	"<<=",
	spanned_token!(
		Token::Operator(Operator::ShiftLeftEquals),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_shr_equals,
	">>=",
	spanned_token!(
		Token::Operator(Operator::ShiftRightEquals),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_class_scope_resolution,
	"::",
	spanned_token!(
		Token::Operator(Operator::ClassScopeResolution),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_weight_assign_dist,
	":/",
	spanned_token!(
		Token::Operator(Operator::WeightAssignDist),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_weight_assign_unit,
	":=",
	spanned_token!(
		Token::Operator(Operator::WeightAssignUnit),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_xor_equals,
	"^=",
	spanned_token!(
		Token::Operator(Operator::XorEquals),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_wildcard_equal,
	"==?",
	spanned_token!(
		Token::Operator(Operator::WildcardEqual),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_wildcard_not_equal,
	"!=?",
	spanned_token!(
		Token::Operator(Operator::WildcardNotEqual),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_wildcard,
	".*",
	spanned_token!(
		Token::Operator(Operator::Wildcard),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_cycle_delay,
	"##",
	spanned_token!(
		Token::Operator(Operator::CycleDelay),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_event_trigger_nonblocking,
	"->>",
	spanned_token!(
		Token::Operator(Operator::EventTriggerNb),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_property_implies_non_overlap,
	"|=>",
	spanned_token!(
		Token::Operator(Operator::PropImplNonOverlap),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_property_implies_overlap,
	"|->",
	spanned_token!(
		Token::Operator(Operator::PropImplOverlap),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	system_verilog_all,
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
	system_verilog_all,
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
