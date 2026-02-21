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
