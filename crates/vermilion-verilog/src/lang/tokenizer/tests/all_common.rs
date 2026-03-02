// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	all,
	whitespace_space,
	" ",
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	whitespace_tab,
	"\t",
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	whitespace_mixed,
	"\t ",
	spanned_token!(
		Token::Whitespace("\t ".as_bytes().into()),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	newline_unix,
	"\n",
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	newline_win32,
	"\r\n",
	spanned_token!(
		Token::Newline("\r\n".as_bytes().into()),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	newline_mixed,
	"\r\n\n\n",
	spanned_token!(
		Token::Newline("\r\n".as_bytes().into()),
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		2..3,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		3..4,
		Position::new(2, 0)
	)
);

tokenizer_test!(
	all,
	whitespace_newlines_mixed,
	" \r\n \t\n\t\n\r",
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Newline("\r\n".as_bytes().into()),
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Whitespace(" \t".as_bytes().into()),
		3..5,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		5..6,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		6..7,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		7..8,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Newline("\r".as_bytes().into()),
		8..9,
		Position::new(3, 0)
	)
);

tokenizer_test!(
	all,
	control_at,
	"@",
	spanned_token!(Token::Control(Control::At), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	control_octothorp,
	"#",
	spanned_token!(
		Token::Control(Control::Octothorp),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_paren_open,
	"(",
	spanned_token!(
		Token::Control(Control::ParenOpen),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_paren_close,
	")",
	spanned_token!(
		Token::Control(Control::ParenClose),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_bracket_open,
	"[",
	spanned_token!(
		Token::Control(Control::BracketOpen),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_bracket_close,
	"]",
	spanned_token!(
		Token::Control(Control::BracketClose),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_brace_open,
	"{",
	spanned_token!(
		Token::Control(Control::BraceOpen),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_brace_close,
	"}",
	spanned_token!(
		Token::Control(Control::BraceClose),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_colon,
	":",
	spanned_token!(Token::Control(Control::Colon), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	control_semicolon,
	";",
	spanned_token!(
		Token::Control(Control::Semicolon),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_comma,
	",",
	spanned_token!(Token::Control(Control::Comma), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	control_dot,
	".",
	spanned_token!(Token::Control(Control::Dot), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	control_dollar,
	"$",
	spanned_token!(Token::Control(Control::Dollar), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	control_question,
	"?",
	spanned_token!(Token::Control(Control::Question), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	operator_exclame,
	"!",
	spanned_token!(
		Token::Operator(Operator::Exclamation),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_logical_inequality,
	"!=",
	spanned_token!(
		Token::Operator(Operator::LogicalInequality),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_case_inequality,
	"!==",
	spanned_token!(
		Token::Operator(Operator::CaseInequality),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_equal,
	"=",
	spanned_token!(Token::Operator(Operator::Equals), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	operator_logical_equality,
	"==",
	spanned_token!(
		Token::Operator(Operator::LogicalEquality),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_case_equality,
	"===",
	spanned_token!(
		Token::Operator(Operator::CaseEquality),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_ampersand,
	"&",
	spanned_token!(
		Token::Operator(Operator::Ampersand),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_logical_and,
	"&&",
	spanned_token!(
		Token::Operator(Operator::LogicalAnd),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_triple_and,
	"&&&",
	spanned_token!(
		Token::Operator(Operator::TripleAnd),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_tilde,
	"~",
	spanned_token!(Token::Operator(Operator::Tilde), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	operator_tilde_circumflex_right,
	"~^",
	spanned_token!(
		Token::Operator(Operator::TildeCircumflex(false)),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_reduction_nand,
	"~&",
	spanned_token!(
		Token::Operator(Operator::ReductionNand),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_reduction_nor,
	"~|",
	spanned_token!(
		Token::Operator(Operator::ReductionNor),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_circumflex,
	"^",
	spanned_token!(
		Token::Operator(Operator::Circumflex),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_tilde_circumflex_left,
	"^~",
	spanned_token!(
		Token::Operator(Operator::TildeCircumflex(true)),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_pipe,
	"|",
	spanned_token!(Token::Operator(Operator::Pipe), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	operator_logical_or,
	"||",
	spanned_token!(
		Token::Operator(Operator::LogicalOr),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_less_than,
	">",
	spanned_token!(
		Token::Operator(Operator::LessThan),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_less_than_equal,
	">=",
	spanned_token!(
		Token::Operator(Operator::LessThanEqual),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_shift_right,
	">>",
	spanned_token!(
		Token::Operator(Operator::ShiftRight),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_greater_than,
	"<",
	spanned_token!(
		Token::Operator(Operator::GreaterThan),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_greater_than_equal,
	"<=",
	spanned_token!(
		Token::Operator(Operator::GreaterThanEqual),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_shift_left,
	"<<",
	spanned_token!(
		Token::Operator(Operator::ShiftLeft),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_percent,
	"%",
	spanned_token!(
		Token::Operator(Operator::Percent),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_asterisk,
	"*",
	spanned_token!(
		Token::Operator(Operator::Asterisk),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_plus,
	"+",
	spanned_token!(Token::Operator(Operator::Plus), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	operator_minus,
	"-",
	spanned_token!(Token::Operator(Operator::Minus), 0..1, Position::new(0, 0))
);

tokenizer_test!(
	all,
	operator_event_trigger,
	"->",
	spanned_token!(
		Token::Operator(Operator::EventTrigger),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_solidus,
	"/",
	spanned_token!(
		Token::Operator(Operator::Solidus),
		0..1,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	comment_single_line,
	"// This Is A Test",
	spanned_token!(
		Token::Comment(Comment::SingleLine(" This Is A Test".as_bytes().into())),
		0..17,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	comment_multi_line,
	"/*\nThis Is A\n\tMulti Line Comment\n*/",
	spanned_token!(
		Token::Comment(Comment::MultiLine(
			"/*\nThis Is A\n\tMulti Line Comment\n*/".as_bytes().into()
		)),
		0..35,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	comment_multi_line_invalid,
	"/*\nThis Is A\n/*Multi Line Comment\n*/",
	spanned_token!(
		Token::Comment(Comment::Invalid(
			"/*\nThis Is A\n/*Multi Line Comment\n*/".as_bytes().into()
		)),
		0..36,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	comment_multi_line_truncated,
	"/*\nThis Is A\n",
	spanned_token!(
		Token::Comment(Comment::Invalid("/*\nThis Is A\n".as_bytes().into())),
		0..13,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	defined_text_macro,
	"`meow",
	spanned_token!(
		Token::TextMacro(TextMacro::Other("meow".as_bytes().into())),
		0..5,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	compiler_directive_arg,
	"`define meow",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Name(Directive::Define)),
		0..7,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		7..8,
		Position::new(0, 7)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Arg("meow".as_bytes().into())),
		8..12,
		Position::new(0, 8)
	)
);

tokenizer_test!(
	all,
	compiler_directive_multi_arg,
	"`define nya 8",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Name(Directive::Define)),
		0..7,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		7..8,
		Position::new(0, 7)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Arg("nya".as_bytes().into())),
		8..11,
		Position::new(0, 8)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		11..12,
		Position::new(0, 11)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Arg("8".as_bytes().into())),
		12..13,
		Position::new(0, 12)
	)
);

tokenizer_test!(
	all,
	string,
	r#""This Is A Simple String :3""#,
	spanned_token!(
		Token::String("This Is A Simple String :3".as_bytes().into()),
		0..28,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	string_eof,
	r#""This Is A Simple String :3"#,
	spanned_token!(
		Token::String("This Is A Simple String :3".as_bytes().into()),
		0..27,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	ident,
	"meow",
	spanned_token!(
		Token::Identifier("meow".as_bytes().into()),
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	ident_adv,
	"m3ow_me0w",
	spanned_token!(
		Token::Identifier("m3ow_me0w".as_bytes().into()),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_always,
	"always",
	spanned_token!(Token::Keyword(Keyword::Always), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_and,
	"and",
	spanned_token!(Token::Keyword(Keyword::And), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_assign,
	"assign",
	spanned_token!(Token::Keyword(Keyword::Assign), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_begin,
	"begin",
	spanned_token!(Token::Keyword(Keyword::Begin), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_buf,
	"buf",
	spanned_token!(Token::Keyword(Keyword::Buf), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_bufif0,
	"bufif0",
	spanned_token!(Token::Keyword(Keyword::BufIf0), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_bufif1,
	"bufif1",
	spanned_token!(Token::Keyword(Keyword::BufIf1), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_case,
	"case",
	spanned_token!(Token::Keyword(Keyword::Case), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_casex,
	"casex",
	spanned_token!(Token::Keyword(Keyword::CaseX), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_casez,
	"casez",
	spanned_token!(Token::Keyword(Keyword::CaseZ), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_cmos,
	"cmos",
	spanned_token!(Token::Keyword(Keyword::Cmos), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_deassign,
	"deassign",
	spanned_token!(Token::Keyword(Keyword::Deassign), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_default,
	"default",
	spanned_token!(Token::Keyword(Keyword::Default), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_defparam,
	"defparam",
	spanned_token!(Token::Keyword(Keyword::DefParam), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_disable,
	"disable",
	spanned_token!(Token::Keyword(Keyword::Disable), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_edge,
	"edge",
	spanned_token!(Token::Keyword(Keyword::Edge), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_else,
	"else",
	spanned_token!(Token::Keyword(Keyword::Else), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_end,
	"end",
	spanned_token!(Token::Keyword(Keyword::End), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_endcase,
	"endcase",
	spanned_token!(Token::Keyword(Keyword::EndCase), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_endfunction,
	"endfunction",
	spanned_token!(
		Token::Keyword(Keyword::EndFunction),
		0..11,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endmodule,
	"endmodule",
	spanned_token!(
		Token::Keyword(Keyword::EndModule),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endprimitive,
	"endprimitive",
	spanned_token!(
		Token::Keyword(Keyword::EndPrimitive),
		0..12,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endspecify,
	"endspecify",
	spanned_token!(
		Token::Keyword(Keyword::EndSpecify),
		0..10,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endtable,
	"endtable",
	spanned_token!(Token::Keyword(Keyword::EndTable), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_endtask,
	"endtask",
	spanned_token!(Token::Keyword(Keyword::EndTask), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_event,
	"event",
	spanned_token!(Token::Keyword(Keyword::Event), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_for,
	"for",
	spanned_token!(Token::Keyword(Keyword::For), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_force,
	"force",
	spanned_token!(Token::Keyword(Keyword::Force), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_forever,
	"forever",
	spanned_token!(Token::Keyword(Keyword::Forever), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_fork,
	"fork",
	spanned_token!(Token::Keyword(Keyword::Fork), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_function,
	"function",
	spanned_token!(Token::Keyword(Keyword::Function), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_highz0,
	"highz0",
	spanned_token!(Token::Keyword(Keyword::HighZ0), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_highz1,
	"highz1",
	spanned_token!(Token::Keyword(Keyword::HighZ1), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_if,
	"if",
	spanned_token!(Token::Keyword(Keyword::If), 0..2, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_ifnone,
	"ifnone",
	spanned_token!(Token::Keyword(Keyword::IfNone), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_initial,
	"initial",
	spanned_token!(Token::Keyword(Keyword::Initial), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_inout,
	"inout",
	spanned_token!(Token::Keyword(Keyword::InOut), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_input,
	"input",
	spanned_token!(Token::Keyword(Keyword::Input), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_integer,
	"integer",
	spanned_token!(Token::Keyword(Keyword::Integer), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_join,
	"join",
	spanned_token!(Token::Keyword(Keyword::Join), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_large,
	"large",
	spanned_token!(Token::Keyword(Keyword::Large), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_macromodule,
	"macromodule",
	spanned_token!(
		Token::Keyword(Keyword::MacroModule),
		0..11,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_medium,
	"medium",
	spanned_token!(Token::Keyword(Keyword::Medium), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_module,
	"module",
	spanned_token!(Token::Keyword(Keyword::Module), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_nand,
	"nand",
	spanned_token!(Token::Keyword(Keyword::Nand), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_negedge,
	"negedge",
	spanned_token!(Token::Keyword(Keyword::NegEdge), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_nmos,
	"nmos",
	spanned_token!(Token::Keyword(Keyword::Nmos), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_nor,
	"nor",
	spanned_token!(Token::Keyword(Keyword::Nor), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_not,
	"not",
	spanned_token!(Token::Keyword(Keyword::Not), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_notif0,
	"notif0",
	spanned_token!(Token::Keyword(Keyword::NotIf0), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_notif1,
	"notif1",
	spanned_token!(Token::Keyword(Keyword::NotIf1), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_or,
	"or",
	spanned_token!(Token::Keyword(Keyword::Or), 0..2, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_output,
	"output",
	spanned_token!(Token::Keyword(Keyword::Output), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_parameter,
	"parameter",
	spanned_token!(
		Token::Keyword(Keyword::Parameter),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_pmos,
	"pmos",
	spanned_token!(Token::Keyword(Keyword::Pmos), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_posedge,
	"posedge",
	spanned_token!(Token::Keyword(Keyword::PosEdge), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_primitive,
	"primitive",
	spanned_token!(
		Token::Keyword(Keyword::Primitive),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_pull0,
	"pull0",
	spanned_token!(Token::Keyword(Keyword::Pull0), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_pull1,
	"pull1",
	spanned_token!(Token::Keyword(Keyword::Pull1), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_pulldown,
	"pulldown",
	spanned_token!(Token::Keyword(Keyword::Pulldown), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_pullup,
	"pullup",
	spanned_token!(Token::Keyword(Keyword::Pullup), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_rcmos,
	"rcmos",
	spanned_token!(Token::Keyword(Keyword::Rcmos), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_real,
	"real",
	spanned_token!(Token::Keyword(Keyword::Real), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_realtime,
	"realtime",
	spanned_token!(Token::Keyword(Keyword::Realtime), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_reg,
	"reg",
	spanned_token!(Token::Keyword(Keyword::Reg), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_release,
	"release",
	spanned_token!(Token::Keyword(Keyword::Release), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_repeat,
	"repeat",
	spanned_token!(Token::Keyword(Keyword::Repeat), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_rnmos,
	"rnmos",
	spanned_token!(Token::Keyword(Keyword::Rnmos), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_rpmos,
	"rpmos",
	spanned_token!(Token::Keyword(Keyword::Rpmos), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_rtran,
	"rtran",
	spanned_token!(Token::Keyword(Keyword::Rtran), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_rtranif0,
	"rtranif0",
	spanned_token!(Token::Keyword(Keyword::RtranIf0), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_rtranif1,
	"rtranif1",
	spanned_token!(Token::Keyword(Keyword::RtranIf1), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_scalared,
	"scalared",
	spanned_token!(Token::Keyword(Keyword::Scalared), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_small,
	"small",
	spanned_token!(Token::Keyword(Keyword::Small), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_specify,
	"specify",
	spanned_token!(Token::Keyword(Keyword::Specify), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_specparam,
	"specparam",
	spanned_token!(
		Token::Keyword(Keyword::SpecParam),
		0..9,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_strong0,
	"strong0",
	spanned_token!(Token::Keyword(Keyword::Strong0), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_strong1,
	"strong1",
	spanned_token!(Token::Keyword(Keyword::Strong1), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_supply0,
	"supply0",
	spanned_token!(Token::Keyword(Keyword::Supply0), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_supply1,
	"supply1",
	spanned_token!(Token::Keyword(Keyword::Supply1), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_table,
	"table",
	spanned_token!(Token::Keyword(Keyword::Table), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_task,
	"task",
	spanned_token!(Token::Keyword(Keyword::Task), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_time,
	"time",
	spanned_token!(Token::Keyword(Keyword::Time), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_tran,
	"tran",
	spanned_token!(Token::Keyword(Keyword::Tran), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_tranif0,
	"tranif0",
	spanned_token!(Token::Keyword(Keyword::TranIf0), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_tranif1,
	"tranif1",
	spanned_token!(Token::Keyword(Keyword::TranIf1), 0..7, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_tri,
	"tri",
	spanned_token!(Token::Keyword(Keyword::Tri), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_tri0,
	"tri0",
	spanned_token!(Token::Keyword(Keyword::Tri0), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_tri1,
	"tri1",
	spanned_token!(Token::Keyword(Keyword::Tri1), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_triand,
	"triand",
	spanned_token!(Token::Keyword(Keyword::Triand), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_trior,
	"trior",
	spanned_token!(Token::Keyword(Keyword::Trior), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_trireg,
	"trireg",
	spanned_token!(Token::Keyword(Keyword::Trireg), 0..6, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_vectored,
	"vectored",
	spanned_token!(Token::Keyword(Keyword::Vectored), 0..8, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_wait,
	"wait",
	spanned_token!(Token::Keyword(Keyword::Wait), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_wand,
	"wand",
	spanned_token!(Token::Keyword(Keyword::Wand), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_weak0,
	"weak0",
	spanned_token!(Token::Keyword(Keyword::Weak0), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_weak1,
	"weak1",
	spanned_token!(Token::Keyword(Keyword::Weak1), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_while,
	"while",
	spanned_token!(Token::Keyword(Keyword::While), 0..5, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_wire,
	"wire",
	spanned_token!(Token::Keyword(Keyword::Wire), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_wor,
	"wor",
	spanned_token!(Token::Keyword(Keyword::Wor), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_xnor,
	"xnor",
	spanned_token!(Token::Keyword(Keyword::Xnor), 0..4, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_xor,
	"xor",
	spanned_token!(Token::Keyword(Keyword::Xor), 0..3, Position::new(0, 0))
);

tokenizer_test!(
	all,
	extended_ident,
	r"\meow",
	spanned_token!(
		Token::Identifier(r"\meow".as_bytes().into()),
		0..5,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	extended_ident_adv,
	r"\nya$owo@uwu",
	spanned_token!(
		Token::Identifier(r"\nya$owo@uwu".as_bytes().into()),
		0..12,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	unsigned_number,
	"1234",
	spanned_token!(
		Token::UnsignedNumber("1234".as_bytes().into()),
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	real_number_simple,
	"1.23",
	spanned_token!(
		Token::Real { value: 1.23, exponent: None },
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	real_number_simple_pos,
	"+1.23",
	spanned_token!(Token::Operator(Operator::Plus), 0..1, Position::new(0, 0)),
	spanned_token!(
		Token::Real { value: 1.23, exponent: None },
		1..5,
		Position::new(0, 1)
	)
);

tokenizer_test!(
	all,
	real_number_simple_neg,
	"-1.23",
	spanned_token!(Token::Operator(Operator::Minus), 0..1, Position::new(0, 0)),
	spanned_token!(
		Token::Real { value: 1.23, exponent: None },
		1..5,
		Position::new(0, 1)
	)
);

tokenizer_test!(
	all,
	real_number_exponent,
	"1e7",
	spanned_token!(
		Token::Real {
			value:    1e7,
			exponent: Some("e7".as_bytes().into()),
		},
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	real_number_dec_exponent,
	"1.2e6",
	spanned_token!(
		Token::Real {
			value:    1.2e6,
			exponent: Some("e6".as_bytes().into()),
		},
		0..5,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	real_number_exponent_pos,
	"1e+6",
	spanned_token!(
		Token::Real {
			value:    1e6,
			exponent: Some("e+6".as_bytes().into()),
		},
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	real_number_exponent_neg,
	"1e-6",
	spanned_token!(
		Token::Real {
			value:    1e-6,
			exponent: Some("e-6".as_bytes().into()),
		},
		0..4,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	binary_prefixed,
	"4'b1001",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("1001".as_bytes().into()),
		3..7,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_dont_care,
	"4'bxx1x",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("xx1x".as_bytes().into()),
		3..7,
		Position::new(0, 3)
	)
);
tokenizer_test!(
	all,
	binary_prefixed_all_dont_care,
	"1'bx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_high_z,
	"4'bzz11",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("zz11".as_bytes().into()),
		3..7,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_all_high_z,
	"1'bz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded,
	"4 'b1001",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("1001".as_bytes().into()),
		4..8,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded_dont_care,
	"4 'bxx1x",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("xx1x".as_bytes().into()),
		4..8,
		Position::new(0, 4)
	)
);
tokenizer_test!(
	all,
	binary_prefixed_padded_all_dont_care,
	"1 'bx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded_high_z,
	"4 'bzz11",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("zz11".as_bytes().into()),
		4..8,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded_all_high_z,
	"1 'bz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_naked,
	"'b10",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("10".as_bytes().into()),
		2..4,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_dont_care,
	"'bxx1x",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("xx1x".as_bytes().into()),
		2..6,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_all_dont_care,
	"'bx",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		2..3,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_high_z,
	"'bzz11",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("zz11".as_bytes().into()),
		2..6,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_all_high_z,
	"'bz",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		2..3,
		Position::new(0, 2)
	)
);

// TODO(aki): Invalid Digits

tokenizer_test!(
	all,
	binary_lump,
	"4 'b01zx\n+1'B ?\n2'bZX\n",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("01zx".as_bytes().into()),
		4..8,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8..9,
		Position::new(0, 8)
	),
	spanned_token!(Token::Operator(Operator::Plus), 9..10, Position::new(1, 0)),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10..11,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Binary,
			uppercase: true,
			signed:    false,
		},
		11..13,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13..14,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Number("?".as_bytes().into()),
		14..15,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15..16,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16..17,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		17..19,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Number("ZX".as_bytes().into()),
		19..21,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21..22,
		Position::new(2, 5)
	)
);

tokenizer_test!(
	all,
	octal_prefixed,
	"3'o666",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("666".as_bytes().into()),
		3..6,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_dont_care,
	"3'ox3x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x3x".as_bytes().into()),
		3..6,
		Position::new(0, 3)
	)
);
tokenizer_test!(
	all,
	octal_prefixed_all_dont_care,
	"1'ox",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_high_z,
	"3'ozz4",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("zz4".as_bytes().into()),
		3..6,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_all_high_z,
	"1'oz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded,
	"3 'o464",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("464".as_bytes().into()),
		4..7,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded_dont_care,
	"3 'ox2x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x2x".as_bytes().into()),
		4..7,
		Position::new(0, 4)
	)
);
tokenizer_test!(
	all,
	octal_prefixed_padded_all_dont_care,
	"1 'ox",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded_high_z,
	"3 'ozz1",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("zz1".as_bytes().into()),
		4..7,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded_all_high_z,
	"1 'oz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_naked,
	"'o66",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("66".as_bytes().into()),
		2..4,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_dont_care,
	"'ox3x",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x3x".as_bytes().into()),
		2..5,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_all_dont_care,
	"'ox",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		2..3,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_high_z,
	"'ozz7",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("zz7".as_bytes().into()),
		2..5,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_all_high_z,
	"'oz",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		2..3,
		Position::new(0, 2)
	)
);

// TODO(aki): Invalid Digits

tokenizer_test!(
	all,
	octal_lump,
	"4 'o06zx\n+1'O ?\n2'oZX\n",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("06zx".as_bytes().into()),
		4..8,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8..9,
		Position::new(0, 8)
	),
	spanned_token!(Token::Operator(Operator::Plus), 9..10, Position::new(1, 0)),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10..11,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Octal,
			uppercase: true,
			signed:    false,
		},
		11..13,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13..14,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Number("?".as_bytes().into()),
		14..15,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15..16,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16..17,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Octal,
			uppercase: false,
			signed:    false,
		},
		17..19,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Number("ZX".as_bytes().into()),
		19..21,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21..22,
		Position::new(2, 5)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed,
	"2'd69",
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("69".as_bytes().into()),
		3..5,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_dont_care,
	"3'dx3x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("x3x".as_bytes().into())),
		3..6,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_all_dont_care,
	"1'dx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("x".as_bytes().into())),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_high_z,
	"3'dzz4",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("zz4".as_bytes().into())),
		3..6,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_all_high_z,
	"1'dz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("z".as_bytes().into())),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded,
	"2 'd99",
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("99".as_bytes().into()),
		4..6,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_dont_care,
	"3 'dx9x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("x9x".as_bytes().into())),
		4..7,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_all_dont_care,
	"1 'dx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("x".as_bytes().into())),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_high_z,
	"3 'dzz7",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("zz7".as_bytes().into())),
		4..7,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_all_high_z,
	"1 'dz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("z".as_bytes().into())),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_naked,
	"'d90",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("90".as_bytes().into()),
		2..4,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_dont_care,
	"'dx9x",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("x9x".as_bytes().into())),
		2..5,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_all_dont_care,
	"'dx",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("x".as_bytes().into())),
		2..3,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_high_z,
	"'dzz9",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("zz9".as_bytes().into())),
		2..5,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_all_high_z,
	"'dz",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("z".as_bytes().into())),
		2..3,
		Position::new(0, 2)
	)
);

// TODO(aki): Invalid Digits

tokenizer_test!(
	all,
	decimal_lump,
	"4 'd06zx\n+1'D ?\n2'dZX\n",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("06zx".as_bytes().into())),
		4..8,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8..9,
		Position::new(0, 8)
	),
	spanned_token!(Token::Operator(Operator::Plus), 9..10, Position::new(1, 0)),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10..11,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Decimal,
			uppercase: true,
			signed:    false,
		},
		11..13,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13..14,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Invalid(Some("?".as_bytes().into())),
		14..15,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15..16,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16..17,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Decimal,
			uppercase: false,
			signed:    false,
		},
		17..19,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Invalid(Some("ZX".as_bytes().into())),
		19..21,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21..22,
		Position::new(2, 5)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed,
	"4'h1a4F",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("1a4F".as_bytes().into()),
		3..7,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_dont_care,
	"3'hxDx",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("xDx".as_bytes().into()),
		3..6,
		Position::new(0, 3)
	)
);
tokenizer_test!(
	all,
	hexadecimal_prefixed_all_dont_care,
	"1'hx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_high_z,
	"3'hzzF",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("zzF".as_bytes().into()),
		3..6,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_all_high_z,
	"1'hz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		1..3,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		3..4,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded,
	"3 'ha7d",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("a7d".as_bytes().into()),
		4..7,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_dont_care,
	"3 'hxEx",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("xEx".as_bytes().into()),
		4..7,
		Position::new(0, 4)
	)
);
tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_all_dont_care,
	"1 'hx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_high_z,
	"3 'hzzA",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("zzA".as_bytes().into()),
		4..7,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_all_high_z,
	"1 'hz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		4..5,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked,
	"'h7F",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("7F".as_bytes().into()),
		2..4,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_dont_care,
	"'hxAx",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("xAx".as_bytes().into()),
		2..5,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_all_dont_care,
	"'hx",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		2..3,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_high_z,
	"'hzz7",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("zz7".as_bytes().into()),
		2..5,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_all_high_z,
	"'hz",
	spanned_token!(
		Token::BaseSpecifier {
			specifier: token::BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		0..2,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		2..3,
		Position::new(0, 2)
	)
);

// TODO(aki): Invalid Digits

tokenizer_test!(
	all,
	hexadecimal_lump,
	"4 'h7Fzx\n+1'H ?\n2'hZX\n",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1..2,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		2..4,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("7Fzx".as_bytes().into()),
		4..8,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8..9,
		Position::new(0, 8)
	),
	spanned_token!(Token::Operator(Operator::Plus), 9..10, Position::new(1, 0)),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10..11,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Hexadecimal,
			uppercase: true,
			signed:    false,
		},
		11..13,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13..14,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Number("?".as_bytes().into()),
		14..15,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15..16,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16..17,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Hexadecimal,
			uppercase: false,
			signed:    false,
		},
		17..19,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Number("ZX".as_bytes().into()),
		19..21,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21..22,
		Position::new(2, 5)
	)
);

tokenizer_test!(
	all,
	simple,
	r#"
module foo();
	wire a = 7'b1010x01;
endmodule
	"#,
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),
	spanned_token!(Token::Keyword(Keyword::Module), 1..7, Position::new(1, 0)),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		7..8,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::Identifier("foo".as_bytes().into()),
		8..11,
		Position::new(1, 7)
	),
	spanned_token!(
		Token::Control(Control::ParenOpen),
		11..12,
		Position::new(1, 10)
	),
	spanned_token!(
		Token::Control(Control::ParenClose),
		12..13,
		Position::new(1, 11)
	),
	spanned_token!(
		Token::Control(Control::Semicolon),
		13..14,
		Position::new(1, 12)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		14..15,
		Position::new(1, 13)
	),
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		15..16,
		Position::new(2, 0)
	),
	spanned_token!(Token::Keyword(Keyword::Wire), 16..20, Position::new(2, 1)),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		20..21,
		Position::new(2, 5)
	),
	spanned_token!(
		Token::Identifier("a".as_bytes().into()),
		21..22,
		Position::new(2, 6)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		22..23,
		Position::new(2, 7)
	),
	spanned_token!(
		Token::Operator(Operator::Equals),
		23..24,
		Position::new(2, 8)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		24..25,
		Position::new(2, 9)
	),
	spanned_token!(
		Token::UnsignedNumber("7".as_bytes().into()),
		25..26,
		Position::new(2, 10)
	),
	spanned_token!(
		Token::BaseSpecifier {
			specifier: BaseSpecifier::Binary,
			uppercase: false,
			signed:    false,
		},
		26..28,
		Position::new(2, 11)
	),
	spanned_token!(
		Token::Number("1010x01".as_bytes().into()),
		28..35,
		Position::new(2, 13)
	),
	spanned_token!(
		Token::Control(Control::Semicolon),
		35..36,
		Position::new(2, 20)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		36..37,
		Position::new(2, 21)
	),
	spanned_token!(
		Token::Keyword(Keyword::EndModule),
		37..46,
		Position::new(3, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		46..47,
		Position::new(3, 9)
	),
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		47..48,
		Position::new(4, 0)
	)
);

tokenizer_test!(
	all,
	operator_full_connection,
	"*>",
	spanned_token!(
		Token::Operator(Operator::FullConnection),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_parallel_connection,
	"=>",
	spanned_token!(
		Token::Operator(Operator::ParallelConnection),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_wildcard_export_partial_01,
	"*:",
	spanned_token!(
		Token::Invalid(Some("*:".as_bytes().into())),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_wildcard_export_partial_02,
	"*::",
	spanned_token!(
		Token::Invalid(Some("*::".as_bytes().into())),
		0..3,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_abs_tolerance_partial,
	"+/",
	spanned_token!(
		Token::Invalid(Some("+/".as_bytes().into())),
		0..2,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_rel_tolerance_partial,
	"+%",
	spanned_token!(
		Token::Invalid(Some("+%".as_bytes().into())),
		0..2,
		Position::new(0, 0)
	)
);
