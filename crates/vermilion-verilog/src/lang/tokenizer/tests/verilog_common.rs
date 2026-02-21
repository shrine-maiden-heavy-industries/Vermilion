// SPDX-License-Identifier: BSD-3-Clause

all_verilog_tokenizer_test!(
	operator_branch_contribution,
	"<+",
	vec![spanned_token!(
		Token::ContextuallyInvalid("<+".as_bytes().into(), LanguageSet::VERILOG_AMS_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	line_text_macro,
	"`__LINE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__LINE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	file_text_macro,
	"`__FILE__",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("__FILE__".as_bytes().into())),
		0..9,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	triple_quote_string,
	r#""""meow meow""""#,
	vec![spanned_token!(
		Token::ContextuallyInvalid(r#""""meow meow""""#.as_bytes().into(), LanguageSet::Sv23),
		0..15,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	control_apostrophe,
	"'",
	vec![spanned_token!(
		Token::ContextuallyInvalid("'".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..1,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_add_equals,
	"+=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("+=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_sub_equals,
	"-=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("-=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_and_equals,
	"&=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("&=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_arithmetic_shr_equals,
	">>>=",
	vec![spanned_token!(
		Token::ContextuallyInvalid(">>>=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..4,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_arithmetic_shl_equals,
	"<<<=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("<<<=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..4,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_or_equals,
	"|=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("|=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_mul_equals,
	"*=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("*=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_rem_equals,
	"%=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("%=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_div_equals,
	"/=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("/=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_inc,
	"++",
	vec![spanned_token!(
		Token::ContextuallyInvalid("++".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_dec,
	"--",
	vec![spanned_token!(
		Token::ContextuallyInvalid("--".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_shl_equals,
	"<<=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("<<=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_shr_equals,
	">>=",
	vec![spanned_token!(
		Token::ContextuallyInvalid(">>=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_class_scope_resolution,
	"::",
	vec![spanned_token!(
		Token::ContextuallyInvalid("::".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_weight_assign_dist,
	":/",
	vec![spanned_token!(
		Token::ContextuallyInvalid(":/".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_weight_assign_unit,
	":=",
	vec![spanned_token!(
		Token::ContextuallyInvalid(":=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_xor_equals,
	"^=",
	vec![spanned_token!(
		Token::ContextuallyInvalid("^=".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_wildcard_equal,
	"==?",
	vec![spanned_token!(
		Token::ContextuallyInvalid("==?".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_wildcard_not_equal,
	"!=?",
	vec![spanned_token!(
		Token::ContextuallyInvalid("!=?".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_wildcard,
	".*",
	vec![spanned_token!(
		Token::ContextuallyInvalid(".*".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_cycle_delay,
	"##",
	vec![spanned_token!(
		Token::ContextuallyInvalid("##".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..2,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_event_trigger_nonblocking,
	"->>",
	vec![spanned_token!(
		Token::ContextuallyInvalid("->>".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_property_implies_non_overlap,
	"|=>",
	vec![spanned_token!(
		Token::ContextuallyInvalid("|=>".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_property_implies_overlap,
	"|->",
	vec![spanned_token!(
		Token::ContextuallyInvalid("|->".as_bytes().into(), LanguageSet::SYSTEM_VERILOG_STDS),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_equivalence,
	"<->",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"<->".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_followed_by_nonoverlapped,
	"#=#",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"#=#".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_followed_by_overlapped,
	"#-#",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"#-#".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..3,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_wildcard_export,
	"*::*",
	vec![spanned_token!(
		Token::ContextuallyInvalid(
			"*::*".as_bytes().into(),
			LanguageSet::SYSTEM_VERILOG_STDS & !LanguageSet::Sv05
		),
		0..4,
		Position::new(0, 0)
	),]
);

all_verilog_tokenizer_test!(
	operator_abs_tolerance,
	"+/-",
	vec![spanned_token!(
		Token::ContextuallyInvalid("+/-".as_bytes().into(), LanguageSet::Sv23),
		0..3,
		Position::new(0, 0)
	),]
);
