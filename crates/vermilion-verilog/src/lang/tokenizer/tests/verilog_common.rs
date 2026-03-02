// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	verilog_all,
	operator_branch_contribution,
	"<+",
	spanned_token!(
		Token::ContextuallyInvalid("<+".as_bytes().into(), LanguageStd::VERILOG_AMS_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	line_text_macro,
	"`__LINE__",
	spanned_token!(
		Token::TextMacro(TextMacro::Other("__LINE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	file_text_macro,
	"`__FILE__",
	spanned_token!(
		Token::TextMacro(TextMacro::Other("__FILE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	triple_quote_string,
	r#""""meow meow""""#,
	spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageStd::Sv23),
		0..15,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	control_apostrophe,
	"'",
	spanned_token!(
		Token::ContextuallyInvalid("'".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_add_equals,
	"+=",
	spanned_token!(
		Token::ContextuallyInvalid("+=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_sub_equals,
	"-=",
	spanned_token!(
		Token::ContextuallyInvalid("-=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_and_equals,
	"&=",
	spanned_token!(
		Token::ContextuallyInvalid("&=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_arithmetic_shr_equals,
	">>>=",
	spanned_token!(
		Token::ContextuallyInvalid(">>>=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_arithmetic_shl_equals,
	"<<<=",
	spanned_token!(
		Token::ContextuallyInvalid("<<<=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_or_equals,
	"|=",
	spanned_token!(
		Token::ContextuallyInvalid("|=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_mul_equals,
	"*=",
	spanned_token!(
		Token::ContextuallyInvalid("*=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_rem_equals,
	"%=",
	spanned_token!(
		Token::ContextuallyInvalid("%=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_div_equals,
	"/=",
	spanned_token!(
		Token::ContextuallyInvalid("/=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_inc,
	"++",
	spanned_token!(
		Token::ContextuallyInvalid("++".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_dec,
	"--",
	spanned_token!(
		Token::ContextuallyInvalid("--".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_shl_equals,
	"<<=",
	spanned_token!(
		Token::ContextuallyInvalid("<<=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_shr_equals,
	">>=",
	spanned_token!(
		Token::ContextuallyInvalid(">>=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_class_scope_resolution,
	"::",
	spanned_token!(
		Token::ContextuallyInvalid("::".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_weight_assign_dist,
	":/",
	spanned_token!(
		Token::ContextuallyInvalid(":/".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_weight_assign_unit,
	":=",
	spanned_token!(
		Token::ContextuallyInvalid(":=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_xor_equals,
	"^=",
	spanned_token!(
		Token::ContextuallyInvalid("^=".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_wildcard_equal,
	"==?",
	spanned_token!(
		Token::ContextuallyInvalid("==?".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_wildcard_not_equal,
	"!=?",
	spanned_token!(
		Token::ContextuallyInvalid("!=?".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_wildcard,
	".*",
	spanned_token!(
		Token::ContextuallyInvalid(".*".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_cycle_delay,
	"##",
	spanned_token!(
		Token::ContextuallyInvalid("##".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_event_trigger_nonblocking,
	"->>",
	spanned_token!(
		Token::ContextuallyInvalid("->>".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_property_implies_non_overlap,
	"|=>",
	spanned_token!(
		Token::ContextuallyInvalid("|=>".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_property_implies_overlap,
	"|->",
	spanned_token!(
		Token::ContextuallyInvalid("|->".as_bytes().into(), LanguageStd::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_equivalence,
	"<->",
	spanned_token!(
		Token::ContextuallyInvalid(
			"<->".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_followed_by_nonoverlapped,
	"#=#",
	spanned_token!(
		Token::ContextuallyInvalid(
			"#=#".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_followed_by_overlapped,
	"#-#",
	spanned_token!(
		Token::ContextuallyInvalid(
			"#-#".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_wildcard_export,
	"*::*",
	spanned_token!(
		Token::ContextuallyInvalid(
			"*::*".as_bytes().into(),
			LanguageStd::SYSTEM_VERILOG_STDS & !LanguageStd::Sv05
		),
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_abs_tolerance,
	"+/-",
	spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageStd::Sv23),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	verilog_all,
	operator_rel_tolerance,
	"+%-",
	spanned_token!(
		Token::ContextuallyInvalid("+%-".as_bytes().into(), LanguageStd::Sv23),
		0..3,
		Position::new(0, 0)
	)
);
