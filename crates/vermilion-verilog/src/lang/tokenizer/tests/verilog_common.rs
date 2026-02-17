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
