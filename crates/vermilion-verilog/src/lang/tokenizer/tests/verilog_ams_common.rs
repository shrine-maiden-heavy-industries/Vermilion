// SPDX-License-Identifier: BSD-3-Clause

all_verilog_ams_tokenizer_test!(
	operator_arithmetic_shift_right,
	">>>",
	vec![spanned_token!(
		Token::Operator(Operator::ArithmeticShr),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_arithmetic_shift_left,
	"<<<",
	vec![spanned_token!(
		Token::Operator(Operator::ArithmeticShl),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_branch_contribution,
	"<+",
	vec![spanned_token!(
		Token::Operator(Operator::BranchContribution),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	triple_quote_string,
	r#""""meow meow""""#,
	vec![spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageSet::Sv23),
		0..15,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	control_apostrophe,
	"'",
	vec![spanned_token!(
		Token::ContextuallyInvalid("'".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..1,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_add_equals,
	"+=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("+=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_sub_equals,
	"-=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("-=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_and_equals,
	"&=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("&=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_arithmetic_shr_equals,
	">>>=",
	vec![spanned_token!(
		Token::ContextuallyInvalid(">>>=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..4,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_arithmetic_shl_equals,
	"<<<=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("<<<=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..4,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_or_equals,
	"|=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("|=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_mul_equals,
	"*=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("*=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_rem_equals,
	"%=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("%=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_div_equals,
	"/=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("/=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_inc,
	"++",
	vec![spanned_token!(
		Token::ContextuallyInvalid("++".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_dec,
	"--",
	vec![spanned_token!(
		Token::ContextuallyInvalid("--".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_ams_tokenizer_test!(
	operator_shl_equals,
	"<<=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("<<=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

