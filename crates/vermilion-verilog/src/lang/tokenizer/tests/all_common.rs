// SPDX-License-Identifier: BSD-3-Clause

all_tokenizer_test!(
	whitespace_space,
	" ",
	vec![spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	)]
);

all_tokenizer_test!(
	whitespace_tab,
	"\t",
	vec![spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	whitespace_mixed,
	"\t ",
	vec![spanned_token!(
		Token::Whitespace("\t ".as_bytes().into()),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	newline_unix,
	"\n",
	vec![spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	newline_win32,
	"\r\n",
	vec![spanned_token!(
		Token::Newline("\r\n".as_bytes().into()),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	newline_mixed,
	"\r\n\n\n",
	vec![
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
		),
	]
);

all_tokenizer_test!(
	whitespace_newlines_mixed,
	" \r\n \t\n\t\n\r",
	vec![
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
		),
	]
);

all_tokenizer_test!(
	control_at,
	"@",
	vec![spanned_token!(Token::Control(Control::At), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	control_octothorp,
	"#",
	vec![spanned_token!(
		Token::Control(Control::Octothorp),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_paren_open,
	"(",
	vec![spanned_token!(
		Token::Control(Control::ParenOpen),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_paren_close,
	")",
	vec![spanned_token!(
		Token::Control(Control::ParenClose),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_bracket_open,
	"[",
	vec![spanned_token!(
		Token::Control(Control::BracketOpen),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_bracket_close,
	"]",
	vec![spanned_token!(
		Token::Control(Control::BracketClose),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_brace_open,
	"{",
	vec![spanned_token!(
		Token::Control(Control::BraceOpen),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_brace_close,
	"}",
	vec![spanned_token!(
		Token::Control(Control::BraceClose),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_colon,
	":",
	vec![spanned_token!(Token::Control(Control::Colon), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	control_semicolon,
	";",
	vec![spanned_token!(
		Token::Control(Control::Semicolon),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	control_comma,
	",",
	vec![spanned_token!(Token::Control(Control::Comma), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	control_dot,
	".",
	vec![spanned_token!(Token::Control(Control::Dot), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	control_dollar,
	"$",
	vec![spanned_token!(Token::Control(Control::Dollar), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	control_question,
	"?",
	vec![spanned_token!(Token::Control(Control::Question), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	operator_exclame,
	"!",
	vec![spanned_token!(
		Token::Operator(Operator::Exclamation),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_logical_inequality,
	"!=",
	vec![spanned_token!(
		Token::Operator(Operator::LogicalInequality),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_case_inequality,
	"!==",
	vec![spanned_token!(
		Token::Operator(Operator::CaseInequality),
		0..3,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_equal,
	"=",
	vec![spanned_token!(Token::Operator(Operator::Equals), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	operator_logical_equality,
	"==",
	vec![spanned_token!(
		Token::Operator(Operator::LogicalEquality),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_case_equality,
	"===",
	vec![spanned_token!(
		Token::Operator(Operator::CaseEquality),
		0..3,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_ampersand,
	"&",
	vec![spanned_token!(
		Token::Operator(Operator::Ampersand),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_logical_and,
	"&&",
	vec![spanned_token!(
		Token::Operator(Operator::LogicalAnd),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_triple_and,
	"&&&",
	vec![spanned_token!(
		Token::Operator(Operator::TripleAnd),
		0..3,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_tilde,
	"~",
	vec![spanned_token!(Token::Operator(Operator::Tilde), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	operator_tilde_circumflex_right,
	"~^",
	vec![spanned_token!(
		Token::Operator(Operator::TildeCircumflex(false)),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_reduction_nand,
	"~&",
	vec![spanned_token!(
		Token::Operator(Operator::ReductionNand),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_reduction_nor,
	"~|",
	vec![spanned_token!(
		Token::Operator(Operator::ReductionNor),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_circumflex,
	"^",
	vec![spanned_token!(
		Token::Operator(Operator::Circumflex),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_tilde_circumflex_left,
	"^~",
	vec![spanned_token!(
		Token::Operator(Operator::TildeCircumflex(true)),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_pipe,
	"|",
	vec![spanned_token!(Token::Operator(Operator::Pipe), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	operator_logical_or,
	"||",
	vec![spanned_token!(
		Token::Operator(Operator::LogicalOr),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_less_than,
	">",
	vec![spanned_token!(
		Token::Operator(Operator::LessThan),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_less_than_equal,
	">=",
	vec![spanned_token!(
		Token::Operator(Operator::LessThanEqual),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_shift_right,
	">>",
	vec![spanned_token!(
		Token::Operator(Operator::ShiftRight),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_greater_than,
	"<",
	vec![spanned_token!(
		Token::Operator(Operator::GreaterThan),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_greater_than_equal,
	"<=",
	vec![spanned_token!(
		Token::Operator(Operator::GreaterThanEqual),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_shift_left,
	"<<",
	vec![spanned_token!(
		Token::Operator(Operator::ShiftLeft),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_percent,
	"%",
	vec![spanned_token!(
		Token::Operator(Operator::Percent),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_asterisk,
	"*",
	vec![spanned_token!(
		Token::Operator(Operator::Asterisk),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_plus,
	"+",
	vec![spanned_token!(Token::Operator(Operator::Plus), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	operator_minus,
	"-",
	vec![spanned_token!(Token::Operator(Operator::Minus), 0..1, Position::new(0, 0)),]
);

all_tokenizer_test!(
	operator_event_trigger,
	"->",
	vec![spanned_token!(
		Token::Operator(Operator::EventTrigger),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_solidus,
	"/",
	vec![spanned_token!(
		Token::Operator(Operator::Solidus),
		0..1,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	comment_single_line,
	"// This Is A Test",
	vec![spanned_token!(
		Token::Comment(Comment::SingleLine(" This Is A Test".as_bytes().into())),
		0..17,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	comment_multi_line,
	"/*\nThis Is A\n\tMulti Line Comment\n*/",
	vec![spanned_token!(
		Token::Comment(Comment::MultiLine(
			"/*\nThis Is A\n\tMulti Line Comment\n*/".as_bytes().into()
		)),
		0..35,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	comment_multi_line_invalid,
	"/*\nThis Is A\n/*Multi Line Comment\n*/",
	vec![spanned_token!(
		Token::Comment(Comment::Invalid(
			"/*\nThis Is A\n/*Multi Line Comment\n*/".as_bytes().into()
		)),
		0..36,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	comment_multi_line_truncated,
	"/*\nThis Is A\n",
	vec![spanned_token!(
		Token::Comment(Comment::Invalid("/*\nThis Is A\n".as_bytes().into())),
		0..13,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	defined_text_macro,
	"`meow",
	vec![spanned_token!(
		Token::TextMacro(TextMacro::Other("meow".as_bytes().into())),
		0..5,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	compiler_directive_arg,
	"`define meow",
	vec![
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
	]
);

all_tokenizer_test!(
	compiler_directive_multi_arg,
	"`define nya 8",
	vec![
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
		),
	]
);

all_tokenizer_test!(
	string,
	r#""This Is A Simple String :3""#,
	vec![spanned_token!(
		Token::String("This Is A Simple String :3".as_bytes().into()),
		0..28,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	string_eof,
	r#""This Is A Simple String :3"#,
	vec![spanned_token!(
		Token::String("This Is A Simple String :3".as_bytes().into()),
		0..27,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	ident,
	"meow",
	vec![spanned_token!(
		Token::Identifier("meow".as_bytes().into()),
		0..4,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	ident_adv,
	"m3ow_me0w",
	vec![spanned_token!(
		Token::Identifier("m3ow_me0w".as_bytes().into()),
		0..9,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_always,
	"always",
	vec![spanned_token!(Token::Keyword(Keyword::Always), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_and,
	"and",
	vec![spanned_token!(Token::Keyword(Keyword::And), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_assign,
	"assign",
	vec![spanned_token!(Token::Keyword(Keyword::Assign), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_begin,
	"begin",
	vec![spanned_token!(Token::Keyword(Keyword::Begin), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_buf,
	"buf",
	vec![spanned_token!(Token::Keyword(Keyword::Buf), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_bufif0,
	"bufif0",
	vec![spanned_token!(Token::Keyword(Keyword::BufIf0), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_bufif1,
	"bufif1",
	vec![spanned_token!(Token::Keyword(Keyword::BufIf1), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_case,
	"case",
	vec![spanned_token!(Token::Keyword(Keyword::Case), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_casex,
	"casex",
	vec![spanned_token!(Token::Keyword(Keyword::CaseX), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_casez,
	"casez",
	vec![spanned_token!(Token::Keyword(Keyword::CaseZ), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_cmos,
	"cmos",
	vec![spanned_token!(Token::Keyword(Keyword::Cmos), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_deassign,
	"deassign",
	vec![spanned_token!(Token::Keyword(Keyword::Deassign), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_default,
	"default",
	vec![spanned_token!(Token::Keyword(Keyword::Default), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_defparam,
	"defparam",
	vec![spanned_token!(Token::Keyword(Keyword::DefParam), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_disable,
	"disable",
	vec![spanned_token!(Token::Keyword(Keyword::Disable), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_edge,
	"edge",
	vec![spanned_token!(Token::Keyword(Keyword::Edge), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_else,
	"else",
	vec![spanned_token!(Token::Keyword(Keyword::Else), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_end,
	"end",
	vec![spanned_token!(Token::Keyword(Keyword::End), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_endcase,
	"endcase",
	vec![spanned_token!(Token::Keyword(Keyword::EndCase), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_endfunction,
	"endfunction",
	vec![spanned_token!(
		Token::Keyword(Keyword::EndFunction),
		0..11,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_endmodule,
	"endmodule",
	vec![spanned_token!(
		Token::Keyword(Keyword::EndModule),
		0..9,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_endprimitive,
	"endprimitive",
	vec![spanned_token!(
		Token::Keyword(Keyword::EndPrimitive),
		0..12,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_endspecify,
	"endspecify",
	vec![spanned_token!(
		Token::Keyword(Keyword::EndSpecify),
		0..10,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_endtable,
	"endtable",
	vec![spanned_token!(Token::Keyword(Keyword::EndTable), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_endtask,
	"endtask",
	vec![spanned_token!(Token::Keyword(Keyword::EndTask), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_event,
	"event",
	vec![spanned_token!(Token::Keyword(Keyword::Event), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_for,
	"for",
	vec![spanned_token!(Token::Keyword(Keyword::For), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_force,
	"force",
	vec![spanned_token!(Token::Keyword(Keyword::Force), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_forever,
	"forever",
	vec![spanned_token!(Token::Keyword(Keyword::Forever), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_fork,
	"fork",
	vec![spanned_token!(Token::Keyword(Keyword::Fork), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_function,
	"function",
	vec![spanned_token!(Token::Keyword(Keyword::Function), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_highz0,
	"highz0",
	vec![spanned_token!(Token::Keyword(Keyword::HighZ0), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_highz1,
	"highz1",
	vec![spanned_token!(Token::Keyword(Keyword::HighZ1), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_if,
	"if",
	vec![spanned_token!(Token::Keyword(Keyword::If), 0..2, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_ifnone,
	"ifnone",
	vec![spanned_token!(Token::Keyword(Keyword::IfNone), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_initial,
	"initial",
	vec![spanned_token!(Token::Keyword(Keyword::Initial), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_inout,
	"inout",
	vec![spanned_token!(Token::Keyword(Keyword::InOut), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_input,
	"input",
	vec![spanned_token!(Token::Keyword(Keyword::Input), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_integer,
	"integer",
	vec![spanned_token!(Token::Keyword(Keyword::Integer), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_join,
	"join",
	vec![spanned_token!(Token::Keyword(Keyword::Join), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_large,
	"large",
	vec![spanned_token!(Token::Keyword(Keyword::Large), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_macromodule,
	"macromodule",
	vec![spanned_token!(
		Token::Keyword(Keyword::MacroModule),
		0..11,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_medium,
	"medium",
	vec![spanned_token!(Token::Keyword(Keyword::Medium), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_module,
	"module",
	vec![spanned_token!(Token::Keyword(Keyword::Module), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_nand,
	"nand",
	vec![spanned_token!(Token::Keyword(Keyword::Nand), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_negedge,
	"negedge",
	vec![spanned_token!(Token::Keyword(Keyword::NegEdge), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_nmos,
	"nmos",
	vec![spanned_token!(Token::Keyword(Keyword::Nmos), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_nor,
	"nor",
	vec![spanned_token!(Token::Keyword(Keyword::Nor), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_not,
	"not",
	vec![spanned_token!(Token::Keyword(Keyword::Not), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_notif0,
	"notif0",
	vec![spanned_token!(Token::Keyword(Keyword::NotIf0), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_notif1,
	"notif1",
	vec![spanned_token!(Token::Keyword(Keyword::NotIf1), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_or,
	"or",
	vec![spanned_token!(Token::Keyword(Keyword::Or), 0..2, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_output,
	"output",
	vec![spanned_token!(Token::Keyword(Keyword::Output), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_parameter,
	"parameter",
	vec![spanned_token!(
		Token::Keyword(Keyword::Parameter),
		0..9,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_pmos,
	"pmos",
	vec![spanned_token!(Token::Keyword(Keyword::Pmos), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_posedge,
	"posedge",
	vec![spanned_token!(Token::Keyword(Keyword::PosEdge), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_primitive,
	"primitive",
	vec![spanned_token!(
		Token::Keyword(Keyword::Primitive),
		0..9,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_pull0,
	"pull0",
	vec![spanned_token!(Token::Keyword(Keyword::Pull0), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_pull1,
	"pull1",
	vec![spanned_token!(Token::Keyword(Keyword::Pull1), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_pulldown,
	"pulldown",
	vec![spanned_token!(Token::Keyword(Keyword::Pulldown), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_pullup,
	"pullup",
	vec![spanned_token!(Token::Keyword(Keyword::Pullup), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_rcmos,
	"rcmos",
	vec![spanned_token!(Token::Keyword(Keyword::Rcmos), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_real,
	"real",
	vec![spanned_token!(Token::Keyword(Keyword::Real), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_realtime,
	"realtime",
	vec![spanned_token!(Token::Keyword(Keyword::Realtime), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_reg,
	"reg",
	vec![spanned_token!(Token::Keyword(Keyword::Reg), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_release,
	"release",
	vec![spanned_token!(Token::Keyword(Keyword::Release), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_repeat,
	"repeat",
	vec![spanned_token!(Token::Keyword(Keyword::Repeat), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_rnmos,
	"rnmos",
	vec![spanned_token!(Token::Keyword(Keyword::Rnmos), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_rpmos,
	"rpmos",
	vec![spanned_token!(Token::Keyword(Keyword::Rpmos), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_rtran,
	"rtran",
	vec![spanned_token!(Token::Keyword(Keyword::Rtran), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_rtranif0,
	"rtranif0",
	vec![spanned_token!(Token::Keyword(Keyword::RtranIf0), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_rtranif1,
	"rtranif1",
	vec![spanned_token!(Token::Keyword(Keyword::RtranIf1), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_scalared,
	"scalared",
	vec![spanned_token!(Token::Keyword(Keyword::Scalared), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_small,
	"small",
	vec![spanned_token!(Token::Keyword(Keyword::Small), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_specify,
	"specify",
	vec![spanned_token!(Token::Keyword(Keyword::Specify), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_specparam,
	"specparam",
	vec![spanned_token!(
		Token::Keyword(Keyword::SpecParam),
		0..9,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	keyword_strong0,
	"strong0",
	vec![spanned_token!(Token::Keyword(Keyword::Strong0), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_strong1,
	"strong1",
	vec![spanned_token!(Token::Keyword(Keyword::Strong1), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_supply0,
	"supply0",
	vec![spanned_token!(Token::Keyword(Keyword::Supply0), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_supply1,
	"supply1",
	vec![spanned_token!(Token::Keyword(Keyword::Supply1), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_table,
	"table",
	vec![spanned_token!(Token::Keyword(Keyword::Table), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_task,
	"task",
	vec![spanned_token!(Token::Keyword(Keyword::Task), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_time,
	"time",
	vec![spanned_token!(Token::Keyword(Keyword::Time), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_tran,
	"tran",
	vec![spanned_token!(Token::Keyword(Keyword::Tran), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_tranif0,
	"tranif0",
	vec![spanned_token!(Token::Keyword(Keyword::TranIf0), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_tranif1,
	"tranif1",
	vec![spanned_token!(Token::Keyword(Keyword::TranIf1), 0..7, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_tri,
	"tri",
	vec![spanned_token!(Token::Keyword(Keyword::Tri), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_tri0,
	"tri0",
	vec![spanned_token!(Token::Keyword(Keyword::Tri0), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_tri1,
	"tri1",
	vec![spanned_token!(Token::Keyword(Keyword::Tri1), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_triand,
	"triand",
	vec![spanned_token!(Token::Keyword(Keyword::Triand), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_trior,
	"trior",
	vec![spanned_token!(Token::Keyword(Keyword::Trior), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_trireg,
	"trireg",
	vec![spanned_token!(Token::Keyword(Keyword::Trireg), 0..6, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_vectored,
	"vectored",
	vec![spanned_token!(Token::Keyword(Keyword::Vectored), 0..8, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_wait,
	"wait",
	vec![spanned_token!(Token::Keyword(Keyword::Wait), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_wand,
	"wand",
	vec![spanned_token!(Token::Keyword(Keyword::Wand), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_weak0,
	"weak0",
	vec![spanned_token!(Token::Keyword(Keyword::Weak0), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_weak1,
	"weak1",
	vec![spanned_token!(Token::Keyword(Keyword::Weak1), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_while,
	"while",
	vec![spanned_token!(Token::Keyword(Keyword::While), 0..5, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_wire,
	"wire",
	vec![spanned_token!(Token::Keyword(Keyword::Wire), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_wor,
	"wor",
	vec![spanned_token!(Token::Keyword(Keyword::Wor), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_xnor,
	"xnor",
	vec![spanned_token!(Token::Keyword(Keyword::Xnor), 0..4, Position::new(0, 0)),]
);

all_tokenizer_test!(
	keyword_xor,
	"xor",
	vec![spanned_token!(Token::Keyword(Keyword::Xor), 0..3, Position::new(0, 0)),]
);

all_tokenizer_test!(
	extended_ident,
	r"\meow",
	vec![spanned_token!(
		Token::Identifier(r"\meow".as_bytes().into()),
		0..5,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	extended_ident_adv,
	r"\nya$owo@uwu",
	vec![spanned_token!(
		Token::Identifier(r"\nya$owo@uwu".as_bytes().into()),
		0..12,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	unsigned_number,
	"1234",
	vec![spanned_token!(
		Token::UnsignedNumber("1234".as_bytes().into()),
		0..4,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	real_number_simple,
	"1.23",
	vec![spanned_token!(
		Token::Real { value: 1.23, exponent: None },
		0..4,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	real_number_simple_pos,
	"+1.23",
	vec![
		spanned_token!(Token::Operator(Operator::Plus), 0..1, Position::new(0, 0)),
		spanned_token!(
			Token::Real { value: 1.23, exponent: None },
			1..5,
			Position::new(0, 1)
		),
	]
);

all_tokenizer_test!(
	real_number_simple_neg,
	"-1.23",
	vec![
		spanned_token!(Token::Operator(Operator::Minus), 0..1, Position::new(0, 0)),
		spanned_token!(
			Token::Real { value: 1.23, exponent: None },
			1..5,
			Position::new(0, 1)
		),
	]
);

all_tokenizer_test!(
	real_number_exponent,
	"1e7",
	vec![spanned_token!(
		Token::Real {
			value:    1e7,
			exponent: Some("e7".as_bytes().into()),
		},
		0..3,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	real_number_dec_exponent,
	"1.2e6",
	vec![spanned_token!(
		Token::Real {
			value:    1.2e6,
			exponent: Some("e6".as_bytes().into()),
		},
		0..5,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	real_number_exponent_pos,
	"1e+6",
	vec![spanned_token!(
		Token::Real {
			value:    1e6,
			exponent: Some("e+6".as_bytes().into()),
		},
		0..4,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	real_number_exponent_neg,
	"1e-6",
	vec![spanned_token!(
		Token::Real {
			value:    1e-6,
			exponent: Some("e-6".as_bytes().into()),
		},
		0..4,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	binary_prefixed,
	"4'b1001",
	vec![
		spanned_token!(
			Token::UnsignedNumber("4".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("1001".as_bytes().into()),
			3..7,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	binary_prefixed_dont_care,
	"4'bxx1x",
	vec![
		spanned_token!(
			Token::UnsignedNumber("4".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("xx1x".as_bytes().into()),
			3..7,
			Position::new(0, 3)
		),
	]
);
all_tokenizer_test!(
	binary_prefixed_all_dont_care,
	"1'bx",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	binary_prefixed_high_z,
	"4'bzz11",
	vec![
		spanned_token!(
			Token::UnsignedNumber("4".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("zz11".as_bytes().into()),
			3..7,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	binary_prefixed_all_high_z,
	"1'bz",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	binary_prefixed_padded,
	"4 'b1001",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("1001".as_bytes().into()),
			4..8,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	binary_prefixed_padded_dont_care,
	"4 'bxx1x",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("xx1x".as_bytes().into()),
			4..8,
			Position::new(0, 4)
		),
	]
);
all_tokenizer_test!(
	binary_prefixed_padded_all_dont_care,
	"1 'bx",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	binary_prefixed_padded_high_z,
	"4 'bzz11",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("zz11".as_bytes().into()),
			4..8,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	binary_prefixed_padded_all_high_z,
	"1 'bz",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	binary_naked,
	"'b10",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("10".as_bytes().into()),
			2..4,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	binary_naked_dont_care,
	"'bxx1x",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("xx1x".as_bytes().into()),
			2..6,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	binary_naked_all_dont_care,
	"'bx",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			2..3,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	binary_naked_high_z,
	"'bzz11",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("zz11".as_bytes().into()),
			2..6,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	binary_naked_all_high_z,
	"'bz",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Binary, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			2..3,
			Position::new(0, 2)
		),
	]
);

// TODO(aki): Invalid Digits

all_tokenizer_test!(
	binary_lump,
	"4 'b01zx\n+1'B ?\n2'bZX\n",
	vec![
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
			Token::BaseSpecifier(BaseSpecifier::Binary, false),
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
			Token::BaseSpecifier(BaseSpecifier::Binary, true),
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
			Token::BaseSpecifier(BaseSpecifier::Binary, false),
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
		),
	]
);

all_tokenizer_test!(
	octal_prefixed,
	"3'o666",
	vec![
		spanned_token!(
			Token::UnsignedNumber("3".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("666".as_bytes().into()),
			3..6,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	octal_prefixed_dont_care,
	"3'ox3x",
	vec![
		spanned_token!(
			Token::UnsignedNumber("3".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("x3x".as_bytes().into()),
			3..6,
			Position::new(0, 3)
		),
	]
);
all_tokenizer_test!(
	octal_prefixed_all_dont_care,
	"1'ox",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	octal_prefixed_high_z,
	"3'ozz4",
	vec![
		spanned_token!(
			Token::UnsignedNumber("3".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("zz4".as_bytes().into()),
			3..6,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	octal_prefixed_all_high_z,
	"1'oz",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	octal_prefixed_padded,
	"3 'o464",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("464".as_bytes().into()),
			4..7,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	octal_prefixed_padded_dont_care,
	"3 'ox2x",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("x2x".as_bytes().into()),
			4..7,
			Position::new(0, 4)
		),
	]
);
all_tokenizer_test!(
	octal_prefixed_padded_all_dont_care,
	"1 'ox",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	octal_prefixed_padded_high_z,
	"3 'ozz1",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("zz1".as_bytes().into()),
			4..7,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	octal_prefixed_padded_all_high_z,
	"1 'oz",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	octal_naked,
	"'o66",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("66".as_bytes().into()),
			2..4,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	octal_naked_dont_care,
	"'ox3x",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("x3x".as_bytes().into()),
			2..5,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	octal_naked_all_dont_care,
	"'ox",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			2..3,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	octal_naked_high_z,
	"'ozz7",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("zz7".as_bytes().into()),
			2..5,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	octal_naked_all_high_z,
	"'oz",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Octal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			2..3,
			Position::new(0, 2)
		),
	]
);

// TODO(aki): Invalid Digits

all_tokenizer_test!(
	octal_lump,
	"4 'o06zx\n+1'O ?\n2'oZX\n",
	vec![
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
			Token::BaseSpecifier(BaseSpecifier::Octal, false),
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
			Token::BaseSpecifier(BaseSpecifier::Octal, true),
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
			Token::BaseSpecifier(BaseSpecifier::Octal, false),
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
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed,
	"2'd69",
	vec![
		spanned_token!(
			Token::UnsignedNumber("2".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("69".as_bytes().into()),
			3..5,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_dont_care,
	"3'dx3x",
	vec![
		spanned_token!(
			Token::UnsignedNumber("3".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Invalid(Some("x3x".as_bytes().into())),
			3..6,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_all_dont_care,
	"1'dx",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Invalid(Some("x".as_bytes().into())),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_high_z,
	"3'dzz4",
	vec![
		spanned_token!(
			Token::UnsignedNumber("3".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Invalid(Some("zz4".as_bytes().into())),
			3..6,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_all_high_z,
	"1'dz",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Invalid(Some("z".as_bytes().into())),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_padded,
	"2 'd99",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("99".as_bytes().into()),
			4..6,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_padded_dont_care,
	"3 'dx9x",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Invalid(Some("x9x".as_bytes().into())),
			4..7,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_padded_all_dont_care,
	"1 'dx",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Invalid(Some("x".as_bytes().into())),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_padded_high_z,
	"3 'dzz7",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Invalid(Some("zz7".as_bytes().into())),
			4..7,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	decimal_prefixed_padded_all_high_z,
	"1 'dz",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Invalid(Some("z".as_bytes().into())),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	decimal_naked,
	"'d90",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("90".as_bytes().into()),
			2..4,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	decimal_naked_dont_care,
	"'dx9x",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Invalid(Some("x9x".as_bytes().into())),
			2..5,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	decimal_naked_all_dont_care,
	"'dx",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Invalid(Some("x".as_bytes().into())),
			2..3,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	decimal_naked_high_z,
	"'dzz9",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Invalid(Some("zz9".as_bytes().into())),
			2..5,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	decimal_naked_all_high_z,
	"'dz",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Decimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Invalid(Some("z".as_bytes().into())),
			2..3,
			Position::new(0, 2)
		),
	]
);

// TODO(aki): Invalid Digits

all_tokenizer_test!(
	decimal_lump,
	"4 'd06zx\n+1'D ?\n2'dZX\n",
	vec![
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
			Token::BaseSpecifier(BaseSpecifier::Decimal, false),
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
			Token::BaseSpecifier(BaseSpecifier::Decimal, true),
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
			Token::BaseSpecifier(BaseSpecifier::Decimal, false),
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
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed,
	"4'h1a4F",
	vec![
		spanned_token!(
			Token::UnsignedNumber("4".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("1a4F".as_bytes().into()),
			3..7,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed_dont_care,
	"3'hxDx",
	vec![
		spanned_token!(
			Token::UnsignedNumber("3".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("xDx".as_bytes().into()),
			3..6,
			Position::new(0, 3)
		),
	]
);
all_tokenizer_test!(
	hexadecimal_prefixed_all_dont_care,
	"1'hx",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed_high_z,
	"3'hzzF",
	vec![
		spanned_token!(
			Token::UnsignedNumber("3".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("zzF".as_bytes().into()),
			3..6,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed_all_high_z,
	"1'hz",
	vec![
		spanned_token!(
			Token::UnsignedNumber("1".as_bytes().into()),
			0..1,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			1..3,
			Position::new(0, 1)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			3..4,
			Position::new(0, 3)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed_padded,
	"3 'ha7d",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("a7d".as_bytes().into()),
			4..7,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed_padded_dont_care,
	"3 'hxEx",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("xEx".as_bytes().into()),
			4..7,
			Position::new(0, 4)
		),
	]
);
all_tokenizer_test!(
	hexadecimal_prefixed_padded_all_dont_care,
	"1 'hx",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed_padded_high_z,
	"3 'hzzA",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("zzA".as_bytes().into()),
			4..7,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_prefixed_padded_all_high_z,
	"1 'hz",
	vec![
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
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			2..4,
			Position::new(0, 2)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			4..5,
			Position::new(0, 4)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_naked,
	"'h7F",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("7F".as_bytes().into()),
			2..4,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_naked_dont_care,
	"'hxAx",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("xAx".as_bytes().into()),
			2..5,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_naked_all_dont_care,
	"'hx",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("x".as_bytes().into()),
			2..3,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_naked_high_z,
	"'hzz7",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("zz7".as_bytes().into()),
			2..5,
			Position::new(0, 2)
		),
	]
);

all_tokenizer_test!(
	hexadecimal_naked_all_high_z,
	"'hz",
	vec![
		spanned_token!(
			Token::BaseSpecifier(token::BaseSpecifier::Hexadecimal, false),
			0..2,
			Position::new(0, 0)
		),
		spanned_token!(
			Token::Number("z".as_bytes().into()),
			2..3,
			Position::new(0, 2)
		),
	]
);

// TODO(aki): Invalid Digits

all_tokenizer_test!(
	hexadecimal_lump,
	"4 'h7Fzx\n+1'H ?\n2'hZX\n",
	vec![
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
			Token::BaseSpecifier(BaseSpecifier::Hexadecimal, false),
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
			Token::BaseSpecifier(BaseSpecifier::Hexadecimal, true),
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
			Token::BaseSpecifier(BaseSpecifier::Hexadecimal, false),
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
		),
	]
);

all_tokenizer_test!(
	simple,
	r#"
module foo();
	wire a = 7'b1010x01;
endmodule
	"#,
	vec![
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
			Token::BaseSpecifier(BaseSpecifier::Binary, false),
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
		),
	]
);

all_tokenizer_test!(
	operator_full_connection,
	"*>",
	vec![spanned_token!(
		Token::Operator(Operator::FullConnection),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_parallel_connection,
	"=>",
	vec![spanned_token!(
		Token::Operator(Operator::ParallelConnection),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_wildcard_export_partial_01,
	"*:",
	vec![spanned_token!(
		Token::Invalid(Some("*:".as_bytes().into())),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_wildcard_export_partial_02,
	"*::",
	vec![spanned_token!(
		Token::Invalid(Some("*::".as_bytes().into())),
		0..3,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_abs_tolerance_partial,
	"+/",
	vec![spanned_token!(
		Token::Invalid(Some("+/".as_bytes().into())),
		0..2,
		Position::new(0, 0)
	),]
);

all_tokenizer_test!(
	operator_rel_tolerance_partial,
	"+%",
	vec![spanned_token!(
		Token::Invalid(Some("+%".as_bytes().into())),
		0..2,
		Position::new(0, 0)
	),]
);
