// SPDX-License-Identifier: BSD-3-Clause

all_system_verilog_tokenizer_test!(
	operator_branch_contribution,
	"<+",
	vec![spanned_token!(
		Token::ContextuallyInvalid("<+".as_bytes().into(), LanguageSet::VERILOG_AMS_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	control_apostrophe,
	"'",
	vec![spanned_token!(
		Token::Control(Control::Apostrophe),
		0..1,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_add_equals,
	"+=",
	vec![spanned_token!(
		Token::Operator(Operator::AddEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_sub_equals,
	"-=",
	vec![spanned_token!(
		Token::Operator(Operator::SubEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_and_equals,
	"&=",
	vec![spanned_token!(
		Token::Operator(Operator::AndEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_arithmetic_shr_equals,
	">>>=",
	vec![spanned_token!(
		Token::Operator(Operator::ArithmeticShrEquals),
		0..4,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_arithmetic_shl_equals,
	"<<<=",
	vec![spanned_token!(
		Token::Operator(Operator::ArithmeticShlEquals),
		0..4,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_or_equals,
	"|=",
	vec![spanned_token!(
		Token::Operator(Operator::OrEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_mul_equals,
	"*=",
	vec![spanned_token!(
		Token::Operator(Operator::MulEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_rem_equals,
	"%=",
	vec![spanned_token!(
		Token::Operator(Operator::RemEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_div_equals,
	"/=",
	vec![spanned_token!(
		Token::Operator(Operator::DivEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_inc,
	"++",
	vec![spanned_token!(
		Token::Operator(Operator::Increment),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_dec,
	"--",
	vec![spanned_token!(
		Token::Operator(Operator::Decrement),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_shl_equals,
	"<<=",
	vec![spanned_token!(
		Token::Operator(Operator::ShiftLeftEquals),
		0..3,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_shr_equals,
	">>=",
	vec![spanned_token!(
		Token::Operator(Operator::ShiftRightEquals),
		0..3,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_class_scope_resolution,
	"::",
	vec![spanned_token!(
		Token::Operator(Operator::ClassScopeResolution),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_weight_assign_dist,
	":/",
	vec![spanned_token!(
		Token::Operator(Operator::WeightAssignDist),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_weight_assign_unit,
	":=",
	vec![spanned_token!(
		Token::Operator(Operator::WeightAssignUnit),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_xor_equals,
	"^=",
	vec![spanned_token!(
		Token::Operator(Operator::XorEquals),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_wildcard_equal,
	"==?",
	vec![spanned_token!(
		Token::Operator(Operator::WildcardEqual),
		0..3,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_wildcard_not_equal,
	"!=?",
	vec![spanned_token!(
		Token::Operator(Operator::WildcardNotEqual),
		0..3,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_wildcard,
	".*",
	vec![spanned_token!(
		Token::Operator(Operator::Wildcard),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_cycle_delay,
	"##",
	vec![spanned_token!(
		Token::Operator(Operator::CycleDelay),
		0..2,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_event_trigger_nonblocking,
	"->>",
	vec![spanned_token!(
		Token::Operator(Operator::EventTriggerNb),
		0..3,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_property_implies_non_overlap,
	"|=>",
	vec![spanned_token!(
		Token::Operator(Operator::PropImplNonOverlap),
		0..3,
		Position::new(0, 0)
	),]
);

all_system_verilog_tokenizer_test!(
	operator_property_implies_overlap,
	"|->",
	vec![spanned_token!(
		Token::Operator(Operator::PropImplOverlap),
		0..3,
		Position::new(0, 0)
	),]
);
