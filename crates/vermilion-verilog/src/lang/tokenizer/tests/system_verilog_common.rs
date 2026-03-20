// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	system_verilog_all,
	operator_branch_contribution,
	"<+",
	spanned_token!(
		Token::ContextuallyInvalid("<+".as_bytes().into(), LanguageStd::VERILOG_AMS_STDS),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	control_apostrophe,
	"'",
	spanned_token!(
		Token::Control(Control::Apostrophe),
		Span::from_position(0, 1, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_add_equals,
	"+=",
	spanned_token!(
		Token::Operator(Operator::AddEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_sub_equals,
	"-=",
	spanned_token!(
		Token::Operator(Operator::SubEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_and_equals,
	"&=",
	spanned_token!(
		Token::Operator(Operator::AndEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_arithmetic_shr_equals,
	">>>=",
	spanned_token!(
		Token::Operator(Operator::ArithmeticShrEquals),
		Span::from_position(0, 4, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_arithmetic_shl_equals,
	"<<<=",
	spanned_token!(
		Token::Operator(Operator::ArithmeticShlEquals),
		Span::from_position(0, 4, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_or_equals,
	"|=",
	spanned_token!(
		Token::Operator(Operator::OrEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_mul_equals,
	"*=",
	spanned_token!(
		Token::Operator(Operator::MulEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_rem_equals,
	"%=",
	spanned_token!(
		Token::Operator(Operator::RemEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_div_equals,
	"/=",
	spanned_token!(
		Token::Operator(Operator::DivEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_inc,
	"++",
	spanned_token!(
		Token::Operator(Operator::Increment),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_dec,
	"--",
	spanned_token!(
		Token::Operator(Operator::Decrement),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_shl_equals,
	"<<=",
	spanned_token!(
		Token::Operator(Operator::ShiftLeftEquals),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_shr_equals,
	">>=",
	spanned_token!(
		Token::Operator(Operator::ShiftRightEquals),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_class_scope_resolution,
	"::",
	spanned_token!(
		Token::Operator(Operator::ClassScopeResolution),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_weight_assign_dist,
	":/",
	spanned_token!(
		Token::Operator(Operator::WeightAssignDist),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_weight_assign_unit,
	":=",
	spanned_token!(
		Token::Operator(Operator::WeightAssignUnit),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_xor_equals,
	"^=",
	spanned_token!(
		Token::Operator(Operator::XorEquals),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_wildcard_equal,
	"==?",
	spanned_token!(
		Token::Operator(Operator::WildcardEqual),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_wildcard_not_equal,
	"!=?",
	spanned_token!(
		Token::Operator(Operator::WildcardNotEqual),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_wildcard,
	".*",
	spanned_token!(
		Token::Operator(Operator::Wildcard),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_cycle_delay,
	"##",
	spanned_token!(
		Token::Operator(Operator::CycleDelay),
		Span::from_position(0, 2, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_event_trigger_nonblocking,
	"->>",
	spanned_token!(
		Token::Operator(Operator::EventTriggerNb),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_property_implies_non_overlap,
	"|=>",
	spanned_token!(
		Token::Operator(Operator::PropImplNonOverlap),
		Span::from_position(0, 3, Position::new(0, 0))
	)
);

tokenizer_test!(
	system_verilog_all,
	operator_property_implies_overlap,
	"|->",
	spanned_token!(
		Token::Operator(Operator::PropImplOverlap),
		Span::from_position(0, 3, Position::new(0, 0))
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
		Span::from_position(0, 3, Position::new(0, 0))
	),
	spanned_token!(
		Token::Number("1".as_bytes().into()),
		Span::from_position(3, 4, Position::new(0, 3))
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
		Span::from_position(0, 3, Position::new(0, 0))
	),
	spanned_token!(
		Token::Number("1".as_bytes().into()),
		Span::from_position(3, 4, Position::new(0, 3))
	)
);
