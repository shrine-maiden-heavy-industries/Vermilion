// SPDX-License-Identifier: BSD-3-Clause

tokenizer_test!(
	all,
	whitespace_space,
	" ",
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	whitespace_tab,
	"\t",
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	whitespace_mixed,
	"\t ",
	spanned_token!(
		Token::Whitespace("\t ".as_bytes().into()),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	newline_unix,
	"\n",
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	newline_win32,
	"\r\n",
	spanned_token!(
		Token::Newline("\r\n".as_bytes().into()),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	newline_mixed,
	"\r\n\n\n",
	spanned_token!(
		Token::Newline("\r\n".as_bytes().into()),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		2u32..3u32,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		3u32..4u32,
		Position::new(2, 0)
	)
);

tokenizer_test!(
	all,
	whitespace_newlines_mixed,
	" \r\n \t\n\t\n\r",
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Newline("\r\n".as_bytes().into()),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Whitespace(" \t".as_bytes().into()),
		3u32..5u32,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		5u32..6u32,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		6u32..7u32,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		7u32..8u32,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Newline("\r".as_bytes().into()),
		8u32..9u32,
		Position::new(3, 0)
	)
);

tokenizer_test!(
	all,
	control_at,
	"@",
	spanned_token!(Token::Control(Control::At), 0u32..1u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	control_octothorp,
	"#",
	spanned_token!(
		Token::Control(Control::Octothorp),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_paren_open,
	"(",
	spanned_token!(
		Token::Control(Control::ParenOpen),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_paren_close,
	")",
	spanned_token!(
		Token::Control(Control::ParenClose),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_bracket_open,
	"[",
	spanned_token!(
		Token::Control(Control::BracketOpen),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_bracket_close,
	"]",
	spanned_token!(
		Token::Control(Control::BracketClose),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_brace_open,
	"{",
	spanned_token!(
		Token::Control(Control::BraceOpen),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_brace_close,
	"}",
	spanned_token!(
		Token::Control(Control::BraceClose),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_colon,
	":",
	spanned_token!(
		Token::Control(Control::Colon),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_semicolon,
	";",
	spanned_token!(
		Token::Control(Control::Semicolon),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_comma,
	",",
	spanned_token!(
		Token::Control(Control::Comma),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_dot,
	".",
	spanned_token!(
		Token::Control(Control::Dot),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_dollar,
	"$",
	spanned_token!(
		Token::Control(Control::Dollar),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	control_question,
	"?",
	spanned_token!(
		Token::Control(Control::Question),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_exclame,
	"!",
	spanned_token!(
		Token::Operator(Operator::Exclamation),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_logical_inequality,
	"!=",
	spanned_token!(
		Token::Operator(Operator::LogicalInequality),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_case_inequality,
	"!==",
	spanned_token!(
		Token::Operator(Operator::CaseInequality),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_equal,
	"=",
	spanned_token!(
		Token::Operator(Operator::Equals),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_logical_equality,
	"==",
	spanned_token!(
		Token::Operator(Operator::LogicalEquality),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_case_equality,
	"===",
	spanned_token!(
		Token::Operator(Operator::CaseEquality),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_ampersand,
	"&",
	spanned_token!(
		Token::Operator(Operator::Ampersand),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_logical_and,
	"&&",
	spanned_token!(
		Token::Operator(Operator::LogicalAnd),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_triple_and,
	"&&&",
	spanned_token!(
		Token::Operator(Operator::TripleAnd),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_tilde,
	"~",
	spanned_token!(
		Token::Operator(Operator::Tilde),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_tilde_circumflex_right,
	"~^",
	spanned_token!(
		Token::Operator(Operator::TildeCircumflex(false)),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_reduction_nand,
	"~&",
	spanned_token!(
		Token::Operator(Operator::ReductionNand),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_reduction_nor,
	"~|",
	spanned_token!(
		Token::Operator(Operator::ReductionNor),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_circumflex,
	"^",
	spanned_token!(
		Token::Operator(Operator::Circumflex),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_tilde_circumflex_left,
	"^~",
	spanned_token!(
		Token::Operator(Operator::TildeCircumflex(true)),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_pipe,
	"|",
	spanned_token!(
		Token::Operator(Operator::Pipe),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_logical_or,
	"||",
	spanned_token!(
		Token::Operator(Operator::LogicalOr),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_less_than,
	">",
	spanned_token!(
		Token::Operator(Operator::LessThan),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_less_than_equal,
	">=",
	spanned_token!(
		Token::Operator(Operator::LessThanEqual),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_shift_right,
	">>",
	spanned_token!(
		Token::Operator(Operator::ShiftRight),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_greater_than,
	"<",
	spanned_token!(
		Token::Operator(Operator::GreaterThan),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_greater_than_equal,
	"<=",
	spanned_token!(
		Token::Operator(Operator::GreaterThanEqual),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_shift_left,
	"<<",
	spanned_token!(
		Token::Operator(Operator::ShiftLeft),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_percent,
	"%",
	spanned_token!(
		Token::Operator(Operator::Percent),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_asterisk,
	"*",
	spanned_token!(
		Token::Operator(Operator::Asterisk),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_plus,
	"+",
	spanned_token!(
		Token::Operator(Operator::Plus),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_minus,
	"-",
	spanned_token!(
		Token::Operator(Operator::Minus),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_event_trigger,
	"->",
	spanned_token!(
		Token::Operator(Operator::EventTrigger),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_solidus,
	"/",
	spanned_token!(
		Token::Operator(Operator::Solidus),
		0u32..1u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	comment_single_line,
	"// This Is A Test",
	spanned_token!(
		Token::Comment(Comment::SingleLine(" This Is A Test".as_bytes().into())),
		0u32..17u32,
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
		0u32..35u32,
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
		0u32..36u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	comment_multi_line_truncated,
	"/*\nThis Is A\n",
	spanned_token!(
		Token::Comment(Comment::Invalid("/*\nThis Is A\n".as_bytes().into())),
		0u32..13u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	text_macro_med,
	"`meow",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"meow".as_bytes().into()
		))),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	text_macro_short,
	"`a",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"a".as_bytes().into()
		))),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	text_macro_long,
	"`isJwsaCVXwblkXJEOzBvanGDVeJlnpptnskLRcwkbjWuGglXVdyeSGTrVNoMluMYXkiPRzdHlMPzsHrAxkVbwCzTlfHYpOkevtwXYkYSiQsEIMiYKqnHBOznTyLafeGtZYYsTqwfcONEwFVIWWNCJzYNlkxWGmFBRbYOfAzQBPkutmVSFYSaXEEdddVEzqwkeDYmOAVsmtnrfKpAoxUVNKMxcZCQkcfzNuMqRAeFeTksVXsLubYqNluFvlqffcYoMeZSgEMSYDLUIWyIfJsJJAIgryfHcBOLxWbEyLMmmitaRbnoDwOsEbqkwMpClKWQcvyQZMUAMjTkNrvhsikdeDXvnVJWNshsSTCLRqtnDKaALjbSvOzoURLGGMhCjNHEkkVDiuSNwKdKPIGrCepATpsIcBswbUmKJSeqKRFlRcUTpuNmyhpaMJekpsBKPXlKNyUHgJSpfFtetkdreIZhHrPcfcajFqpBHmmuEwkzOXaaoCZKSedVAOWzpjrDNRvDrAYjEbLxRSoRJZkwRbqTBCTZUCdxRFeQIhpjNZKTDJyTHxHaKMGlePcerzLxzVngfCdqBmQQyjHpIgMCZbjDkxontYLESdIxSEtwCPlcUXgeFJOhqIwpboCwVKhVtSjVcyFTVRPqmAxGrerYGtMjkMdqFobCTirmRFrshUIYLlDgfOcHTzKFKRjZKFDrRdZaLbUuraWAmJXUzGYupGUYOXtXbCwrrvTeSNgEhZUWIQVWInZKbDqqbnkWcpBvwsROshPQFEqbZkWjGjudykARPvRssqPkrXtdwCcEVSdbGNQVEufqgYSiHiLZcvJlcxTxCFKZqnSyRcopAndoihbnNyzIPMtyjGbHgirhRhsNMHjYUwFBrfPyoddKGmFRZWtwKqWnfUHSIBGhitTymZPwmgFIGSovWoXyvSwHvPyXdGadKkrocdskpIvkOLIoHQNUIUZmPufWtHtnynsDgmdcVWeRpiIqvfYYcdlrYAPmAzPEocqwBCvJQGsFJjOUlVtLVExcoOVygHGpBfqtnpuyDHyGPFrTxXIVOZPfvBcmWDKGrPqHVykpZVXIFDsSOhhsqhvgsqoPGbhENHbNbgvZZatirAmcHsvwliPbxFGOxPMyqogrxqxDfZlfGnxKqEPIQNpKSEBRoHoFmILWnSYlQahXqDIbLwycWwFGfkKRNGiPtuiZwaJbFEKTPOJBqcxonVXRYuFcAeEmRJBabjZPWwCeVrZNmniLUvcMbtDDgmBaDnSkqEYMvVleagMLJudazgVVcejdzGitOnItqCFwAIUcSebefbTugSDhIlNJXHjSDSiQOdfZDCMaPePxKquLtsyFIEueykplgCAFFNehYSCcXfUMeHKlIwVKWweHkLoxKpiXObfzORZhaiOGmwolswbPovcNKxCAThbmulLLNmsXcWYmbVkheRDnXWToGDKGsbtZqKGDnRKgCvFekPWItrQHSkkbwuashiVlgydceRbNBoJDwGFQYFYemFUjIskKyeZVPbtLEtNgZmckjoJAVXvMmHfOroBVjTyjkIMFJXIofvYbWgtNMLkhQQWgDcMkxfWzVRoVkvkxvAgERIekIbadAYDrOKDNFGCBNpBoZNgDXDibAgdJIThshGGWXYRfycakqeJYTZWQgkTPCbGbdCXxPQlDqUJYKdFdofJQbFCNOfqliIdrPAhrGnhRemYUnwMHFGTHlQRJSfFkQJdRuOdSRKCsVtZDrVeXJrwrAhsGIpMlYWHHbLbzYXCdqeqzSnotSlKPcwROcuDhCRpJTykHhNKrRWfKEaclTvFYksrakemNFaenzyMSUYHOvHOoVMzAYAObrHshoFywHdSylgKsWwmpFytzBRKDDJKHGUejkwMwUZMCLZFvwdnldGvLkJMwNoqdVHxtPRQnYHqWOjXSGaTXrZXNBPbtOSwjQtkOynpqWzFkJXhMOiIqoBQKZPbLZbhPidUuAwKGAmWYIBtyzPhfoKoLEMlcxvlgqMFAJrgpJEqRgImLUUiZpbzWzCxjuKfyZeGqlCJwbqhkNnMMqnJqZwaYdSgExJAasFYcgGJzAryWpSjsotmtBQduntzJueJggLcLDHrzATsYGcZLGeDMjWADYaApnyFIuDrpmDxacJZoOAUvXwNxIzEjxmVUIVEzBbPacFIzeuJAarlZMZLBnaoZnxxMYpgETRnALTeqmgWzhRHZfhOGkIcOXXWLZSTruADjrHmldaycIKTWGBVdiFUFWcEGDqRVPnPfPJqsMTsXQLbZVBisaTePXbMxkHmvZzRkNsMZheFdYguMvNsrBdamaayuVYsSVXYNeSLZqOFxayrQEHlHrXKzXGWHMPMCWGJFmjbmqElKJfurhKIdWLeTufTamsPEznCdNgyNqNXswfMhSeJiqYoLXvBMrMwafHBsXTGrvxSixoywzKeRpMvIhMPNXUHedKCBjInNJNtAbDcMycglNNEJwDIQkKehhQcnxEKwHdlidpjpYJvCswhIBiZxMtEgGFUayMwpuGhgGbXMbDpUVZTgcEMPSrsnoVgSRXoEGyAMrOKvenNKuftKZVeLbMmBYFydbUJwJvvSzQMnAOfiGUcoEWjdmCMpbCubmabAyxaMPFMcdAKRaOeLAZmpwuFfjqEVroVPqDBnBicesYJKKPrvEldMsCEFAxJOBEuJSHjOoVvsLwFOEdwtymWhyHWOGtIZxfDuEyGrOxYEuZVdBXZxHypDsQUInUHUbAuJDjvIIfwqSpCKTIfSQFvKPjzuqDIUbzTlxtUcFQjSiQSVZFrwwLVPSsobGUcGNwcyrFwhSigVkhZdYhXkoKuwhZoYdttUQjjENzGXVdTeWDqwkMIWSOgCIfeosFQBOjBovvSxIdtKvelWiMWYvtDXGGkiVWviOTVfKxSpiEkNktgxItQHcjnUHsFgszTQOOYJfGRQeIZSxEYOxFdzeQwYsUgPhUKIRmUeZiagnuAmrLHDksIKeSQeJprsRPaWiDKpXtCkEfITxRlefaRGMpYlWZqPlCBluazZphLGPxwluJkHWAegDutwKIqycGiYhDEKMwPmozcmjltJzaZDPzJJlTxZbITgXPcsLvDJEpDFeCxqSXCmuCiTYflZJOCMzJUjlnbFeJVPaZYcQglsXAdBnXLBswUhssCgHKCGMBqjEfudCCtoBhMKKcOcaOnSqvNHZLhXbCaCJYfifsZCkKkSwjyXBJkUDROmzcFnhVhjggYbWCgeyyVmHoGeIKroYDKIxRWUqtMhzqiFdvJAiTdZhFrcRdwxUtVrgrAHFGVvxkxjEfFXtcTXRjhIhKSkcjetvxPJripsdFPqXeUwglwADxTxMZnXzlOouGqczuYEcWVKVbDvaIBhmQQDBXJnuRqdGvHuHkQJqAjWxlfIohJyQMKrpHxhZztHvJakhlTBGCpvACLqyjSZryIlmaKanjuGIyPbypeUaHFoXjWsoUwSLkpgsKJfKJVhIOueXrMCzqnpaInVVCUgMRjsvASiJMEAIkGsVwxzFnSMZqqFzzmQilwWnUmedeZnZFZpNXFOTmlJgyuyKoGwrciUukrzCEjUueQTAFpmVtZgZOlcqgZtvEEYKihIbZlvFaDdDRyXIHpuJtMcsMrnsqCHDKpwqMtBRtHgaXHrRCrpyWARKtpgwGVgVQqhsmOtjSwVHJlVZDJYLhLHUeqUGHSTgBXnNLpEJsGvAtruGOpEBJNTnQOYSHWfXyAoFvkwtNmdSWjQkuGlODMdXMnPllhrZTApYWLgyYmGDKCyIqMVmhYOjYvtKsoRflkAoHRZPbwQMGuSQUUsdkGpjnBsQwSBBUFVAGORNUBjykhYxeiZtaPvCttyPvqjXDfXXqeucpzEjfqkOfCnGyKKLMxRzxCMtEItqsVguZPyWuVvvbVHvfVuWhbIXnltJeiqKtINFdJxkdXlpTgcIOklxEtEqMNzJYRSlcEsrmzgexaHqtSmqhmUflekSzqqjuXwghPpumZVSVynLTwLtDAehmnZUAKIElZEDxhejuGriibOZStMpoYjypYhVmBrmuUhxmffZiEDcoLkWsgFpLPgzkVmHsquLVdSYgvwSMGKdmzprfXumJBHJSFLadsNpsiKvAjFMHzmlcmJzlQojhWhvapdRPYFUQUbkqZeqifxBdTkjSJvEOEEtukVsDjOJaNpGobtJStYZBPqXIIbalJQfOHozyICiOxkKORbCdoHjizfzMZdJYmFGAWNgrYLPzScdPKuoCcLOoYXYmlaiQJSOaBmgFSgLHsBgydSLlwfdXNufhHgQdTeqpTtAIzJMgnKUkImohtFwbleknaAYDjcYiUaJPLscOcYucjhTeWMdWWRJWNtfsIFWPPHoqvOBXrafakcKcEoJhzndLClOKmvwsxDAOVhDCADmlcLlvSAkFWqcFiOhhUjtBSSxCYVYlddCvgQYhpclpywyEHSESwNbcBcXHOQttgaqgfMVDblBWUKpTOTxABmLpTHFmhEyoOnqRmebTBXEuggqWPghRHRqQlyyxIjpVseiEZpHLZgpIVYujeJJOptVpzICkUfGQzWzLWxdUeoGmhUvvTIWXMQETLKhGdrhzFZzyUHWflCHJagtqwunsWTinUvEzKVCMBUtJSiCKJTNijuUBGuhyhyQSHPDkoDSbNulTimdAXHOSZJqiGukRotktLjnKSBwyYsBEFYIAcoUeKSGhpscKUScHnjykrWJBeSTFsxWuTrCWNHYegrUIXceAeVAlpwRHANRDWqdNVLUvqGpsSabfJCDFiwidgTkIVhtxbQZSiXlDdCmkRzLWzQrTOcNLvxYnxUwKimzsQunmKKAxQVXeoLWLrqHVAHshBAmbCaFzCdcBljedgeVsIxekffMkBQvWeTCVcNHkjPMCmIDtmXlRtNvGTtxxbLViiykokxKLyvmIsHECloUWNafyFDfRMjUZeHMzKGfDKBuacPJHECTJgQWqSLrKlMJIjNIlCHaFwYOHvbnoQhnzZGLdcCjMAISVdusugphxhvQoauWlXpFJGNEBYXTzuwLZLVejgcVUJEZAATtoQHKrrVEDWqyVnMJJcthVBHvlrWJsQqUUIdtiBBWGGSxPGZrsILeRSWpUWGbPPtATABFQftCCJpozMvWKpVmqAaNWBFVaWHrOfgDQUztlpFXBtIgXiOfXBePSCwVDVfPWzoxnVPVYJdNiXkFtICitJnOVnpHOmnJuVGmJjWLcKiZozwpdPDxNspqlgUbZpikTlxNRWLsFpLdJyBelxDdFhxgrBxUcvddAZfYdDhXIzccYtHlJyKPMSotcDlIJdLtzujNmqqtacrrHyqmyBQSuniGKzRTEiIEkjaakSjxEPYkrMOwVBoQosXWNitVJgkYdLlBoxdNpxbpUHWfruOsNVGkExvcqZJzIsZSuSrGJelsOdqcHYiLhhaLhxvMYUBmhpsrUFTrhSiioEXMLIaXcEOhkrWVxIAgIPwtRIXXjhpafGxdxHqwQXCEdjPNmeynszkmIaOePdjchUcVIulQnfwIYkiZryTMFvnNNHHohHwcGykOAZtTrASwqiSRJnqTRxOaSiACnNeLGzoviItbhnUOtFHWkUIUrwfIramOIxZzflIAtcRkaKvodTViHvydQPBZJReOVkypuEKHeEZMmokQeaMtMQvCVxENZyclBtuQNwcFGVYzmuHWIUdBsXUszjLAOCmoQajWSvtRPYFRaqjaAtYETvcInBDmmiuPhqJZppLtzIMJUhLioestsWRJsUfcaFiPUlytCgBJJWUNuoNbMbCGwLlDKxfNrDBfBIgiJiRSFCVUjCnMJZfuvtKAIHFLjgIgTHEyMpzLKHYZoLCeOrVPPekQYbsFuIwwQjwReXSQHKMEmmSvNVZuGtxoIRnSGkYEXUWYFMsFmsHuqHWVtZTBtSLBukGkjCyZLErgJRKLfDHOzcwIzpzSxSIsatWsvItkcYcNMogCLLPcXbCQLJMGngvuxocoFDdefCruJwWxGsYNLtGOuWQPPMNgtIuiRvUsoGCLImBnIlmjqPIoifaaFdvjKUnjyQCvPPvYZcapwOegWCzdJVMShqjNrsmIjAkgABtLPmkvBfciPbMYcimadmggHlaoqUGpDmEAuDBUQvqRvgIgNPLrakFZAilsOxcjjkHxSaLUtSHOISgynoQBdPUbpSVntIiLmwSanegcVPUfzAFAvFPCZNMUJiZqKKLbcopOuJAteNHrJAoSPWaJnIxnUJNWodQVHivFcMoBgnOCeONrHpJIptgmKJYYjRXkMZeRYAnNcmiXSMHyGgtloffJaCRhxNTzJVYMQtuJNWJwfEnnwTWNIdPsLiQpspDgpEYDsNAtkcUfFUCPWbmlfUAaWTWFcvfeATVtqofiXXPLLOwrQUCbuJfLWfgQvCKoRyxczDCYgOfMQtySivbbaWTNAhUcWIAvCExQXWslbWiqEtUlximjpQPlPSYgDcdohGktKCnOTVVJUMnRzJKDDEcMybZdkPaGtUqpnCZCXtskyXNsnpqbDsLFbjdoAifVVxbMGNnsQElypkwSzeQAsUwZIfjWuWbqUDizxrCoJVyTgkKrLdNlttgdejQUzXgyTsepVqMKcaeCZRfLUSeywZayUtEefGVPhlveoEAaRrjxjzhbeLBsibPvhCtHysCbQiPgtzTptXysgCgjiSWTxtcnLYiNqYiQZSKnilxVjEqigOTFSUdIKEIdjfevjRrRIqmRfUsfqyubrvphnOPpIxTvtGtfMpryuqSwvumswmjlbYTyOYuKrxhSIlPvNNjWKKZKEtBTIWAhObVONANYvfroLZatWSCTmUYMBqwPGaKlWitsvNGHBYdwVnVTuCcmZcjPVTVhLNkjtAFFCSUrJyMlqDzPmQiMXlhfaYsJoPViYongwFEorYCljttvUOBruNBGpURQaNAPsYsHmZmwIkuXEkEAOssuZVWqcbBWNuWKfFCfmJnGtFDhofGeGoZyZrqkWdaGISOxqDnwrqlORJcoHMOlOpHwIyxYUPRHogWuhGKNCRoUOtHuxDPUJfFQfwkKCMLCSyjayGxWIPMYaAQeFuesmGBRXrtLJYGSpZklMAfDYlaTuWPURXTrXvwbBPDhjRlNeAwFLKGarvfJVCALtVZmGTFRQuleAxPtIfTSDTzuPkmgLwTgSkyUBxQWuvlhjGdylUOhSmvWVfXDHjRXCWKXNbWxTsGPCswSSwUTkWleuNYtQmiMBWAIytfJLXxYNmDwyEBQnXmnNbzaZreEzkfcwyclOJVlWRcWsXwbBMVdafIxfAKIZhCCpBcUByxLJBIuQOYbVFVUQpCVQskzPbsfkACoHQZADqDwAOoWxzKGOgXZnyqsNxrajzGudUeCpGxKaifmwsVYOPJhBlWyAjSRlumfrQwlKpQDWyBepiytidNSGJksciWMZCJPmekWCvMBwFaasStuYEInxqzdmilzJVFCgqYtRLoPHMFWlrHpYUkMWVfVPiylFYEhgLDpnwWHwRULfXOFMDcuaRfFoRawHXZNPwezvaINnazWXKvnqxPaKiEdUhLOuDrHaXuemdzMfwaZmquXqidEwssOstpCesTMPXQUJxQCrIcmgjcFPUvfldKmzLuiYXTYoqAqarkdwhyMohvqUjQFpevtVgbQwXtpmaJTEkhjLmnfqpmtXrDdKjlBCYiFuldTbrsEXxImYKOzBHVLxSXTogWRxrilAwRUHjsnokzaVNNUTTCuAUgWOTbPimCKWiiKzeqLVrFnbTtlqHbUDfiqpIbxLUgaqCPEXBdLOnBmPaFWBTSmCtNHjJDWxcXCCRWQNSpTAWUYttmTJzCbUxfQuZmQnlSxHVclupWuhwZgCHQWnqitVQthjorthdmPXFYdsltonLryllEhVLWgNjMmKplJnmZnKSYnHFWQHoPbGpsVXNZvBcqSeTvNNqruJoVVvLaLfPvUVivmHDEIWVupIFOYzksrOUADONmvgprEdgNCYSeZPedIjrqpecxpzaTKdKLhtyoeROFgIvTyDoezoZUcMOiBxwtaJWahOhMUJqbbxpnaooQcxkkNzOhMRePBxtTUkzbzrnHkidDFCpPgUvBahNsYhqzHvjsEyugDgSpAltwgkzmCPagLdaiqwXPVXTfbwbLTrEIuMVwiWGTPMVHDBNtVPrcwwTIHYPPoxUKbmsxrrmc",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other("isJwsaCVXwblkXJEOzBvanGDVeJlnpptnskLRcwkbjWuGglXVdyeSGTrVNoMluMYXkiPRzdHlMPzsHrAxkVbwCzTlfHYpOkevtwXYkYSiQsEIMiYKqnHBOznTyLafeGtZYYsTqwfcONEwFVIWWNCJzYNlkxWGmFBRbYOfAzQBPkutmVSFYSaXEEdddVEzqwkeDYmOAVsmtnrfKpAoxUVNKMxcZCQkcfzNuMqRAeFeTksVXsLubYqNluFvlqffcYoMeZSgEMSYDLUIWyIfJsJJAIgryfHcBOLxWbEyLMmmitaRbnoDwOsEbqkwMpClKWQcvyQZMUAMjTkNrvhsikdeDXvnVJWNshsSTCLRqtnDKaALjbSvOzoURLGGMhCjNHEkkVDiuSNwKdKPIGrCepATpsIcBswbUmKJSeqKRFlRcUTpuNmyhpaMJekpsBKPXlKNyUHgJSpfFtetkdreIZhHrPcfcajFqpBHmmuEwkzOXaaoCZKSedVAOWzpjrDNRvDrAYjEbLxRSoRJZkwRbqTBCTZUCdxRFeQIhpjNZKTDJyTHxHaKMGlePcerzLxzVngfCdqBmQQyjHpIgMCZbjDkxontYLESdIxSEtwCPlcUXgeFJOhqIwpboCwVKhVtSjVcyFTVRPqmAxGrerYGtMjkMdqFobCTirmRFrshUIYLlDgfOcHTzKFKRjZKFDrRdZaLbUuraWAmJXUzGYupGUYOXtXbCwrrvTeSNgEhZUWIQVWInZKbDqqbnkWcpBvwsROshPQFEqbZkWjGjudykARPvRssqPkrXtdwCcEVSdbGNQVEufqgYSiHiLZcvJlcxTxCFKZqnSyRcopAndoihbnNyzIPMtyjGbHgirhRhsNMHjYUwFBrfPyoddKGmFRZWtwKqWnfUHSIBGhitTymZPwmgFIGSovWoXyvSwHvPyXdGadKkrocdskpIvkOLIoHQNUIUZmPufWtHtnynsDgmdcVWeRpiIqvfYYcdlrYAPmAzPEocqwBCvJQGsFJjOUlVtLVExcoOVygHGpBfqtnpuyDHyGPFrTxXIVOZPfvBcmWDKGrPqHVykpZVXIFDsSOhhsqhvgsqoPGbhENHbNbgvZZatirAmcHsvwliPbxFGOxPMyqogrxqxDfZlfGnxKqEPIQNpKSEBRoHoFmILWnSYlQahXqDIbLwycWwFGfkKRNGiPtuiZwaJbFEKTPOJBqcxonVXRYuFcAeEmRJBabjZPWwCeVrZNmniLUvcMbtDDgmBaDnSkqEYMvVleagMLJudazgVVcejdzGitOnItqCFwAIUcSebefbTugSDhIlNJXHjSDSiQOdfZDCMaPePxKquLtsyFIEueykplgCAFFNehYSCcXfUMeHKlIwVKWweHkLoxKpiXObfzORZhaiOGmwolswbPovcNKxCAThbmulLLNmsXcWYmbVkheRDnXWToGDKGsbtZqKGDnRKgCvFekPWItrQHSkkbwuashiVlgydceRbNBoJDwGFQYFYemFUjIskKyeZVPbtLEtNgZmckjoJAVXvMmHfOroBVjTyjkIMFJXIofvYbWgtNMLkhQQWgDcMkxfWzVRoVkvkxvAgERIekIbadAYDrOKDNFGCBNpBoZNgDXDibAgdJIThshGGWXYRfycakqeJYTZWQgkTPCbGbdCXxPQlDqUJYKdFdofJQbFCNOfqliIdrPAhrGnhRemYUnwMHFGTHlQRJSfFkQJdRuOdSRKCsVtZDrVeXJrwrAhsGIpMlYWHHbLbzYXCdqeqzSnotSlKPcwROcuDhCRpJTykHhNKrRWfKEaclTvFYksrakemNFaenzyMSUYHOvHOoVMzAYAObrHshoFywHdSylgKsWwmpFytzBRKDDJKHGUejkwMwUZMCLZFvwdnldGvLkJMwNoqdVHxtPRQnYHqWOjXSGaTXrZXNBPbtOSwjQtkOynpqWzFkJXhMOiIqoBQKZPbLZbhPidUuAwKGAmWYIBtyzPhfoKoLEMlcxvlgqMFAJrgpJEqRgImLUUiZpbzWzCxjuKfyZeGqlCJwbqhkNnMMqnJqZwaYdSgExJAasFYcgGJzAryWpSjsotmtBQduntzJueJggLcLDHrzATsYGcZLGeDMjWADYaApnyFIuDrpmDxacJZoOAUvXwNxIzEjxmVUIVEzBbPacFIzeuJAarlZMZLBnaoZnxxMYpgETRnALTeqmgWzhRHZfhOGkIcOXXWLZSTruADjrHmldaycIKTWGBVdiFUFWcEGDqRVPnPfPJqsMTsXQLbZVBisaTePXbMxkHmvZzRkNsMZheFdYguMvNsrBdamaayuVYsSVXYNeSLZqOFxayrQEHlHrXKzXGWHMPMCWGJFmjbmqElKJfurhKIdWLeTufTamsPEznCdNgyNqNXswfMhSeJiqYoLXvBMrMwafHBsXTGrvxSixoywzKeRpMvIhMPNXUHedKCBjInNJNtAbDcMycglNNEJwDIQkKehhQcnxEKwHdlidpjpYJvCswhIBiZxMtEgGFUayMwpuGhgGbXMbDpUVZTgcEMPSrsnoVgSRXoEGyAMrOKvenNKuftKZVeLbMmBYFydbUJwJvvSzQMnAOfiGUcoEWjdmCMpbCubmabAyxaMPFMcdAKRaOeLAZmpwuFfjqEVroVPqDBnBicesYJKKPrvEldMsCEFAxJOBEuJSHjOoVvsLwFOEdwtymWhyHWOGtIZxfDuEyGrOxYEuZVdBXZxHypDsQUInUHUbAuJDjvIIfwqSpCKTIfSQFvKPjzuqDIUbzTlxtUcFQjSiQSVZFrwwLVPSsobGUcGNwcyrFwhSigVkhZdYhXkoKuwhZoYdttUQjjENzGXVdTeWDqwkMIWSOgCIfeosFQBOjBovvSxIdtKvelWiMWYvtDXGGkiVWviOTVfKxSpiEkNktgxItQHcjnUHsFgszTQOOYJfGRQeIZSxEYOxFdzeQwYsUgPhUKIRmUeZiagnuAmrLHDksIKeSQeJprsRPaWiDKpXtCkEfITxRlefaRGMpYlWZqPlCBluazZphLGPxwluJkHWAegDutwKIqycGiYhDEKMwPmozcmjltJzaZDPzJJlTxZbITgXPcsLvDJEpDFeCxqSXCmuCiTYflZJOCMzJUjlnbFeJVPaZYcQglsXAdBnXLBswUhssCgHKCGMBqjEfudCCtoBhMKKcOcaOnSqvNHZLhXbCaCJYfifsZCkKkSwjyXBJkUDROmzcFnhVhjggYbWCgeyyVmHoGeIKroYDKIxRWUqtMhzqiFdvJAiTdZhFrcRdwxUtVrgrAHFGVvxkxjEfFXtcTXRjhIhKSkcjetvxPJripsdFPqXeUwglwADxTxMZnXzlOouGqczuYEcWVKVbDvaIBhmQQDBXJnuRqdGvHuHkQJqAjWxlfIohJyQMKrpHxhZztHvJakhlTBGCpvACLqyjSZryIlmaKanjuGIyPbypeUaHFoXjWsoUwSLkpgsKJfKJVhIOueXrMCzqnpaInVVCUgMRjsvASiJMEAIkGsVwxzFnSMZqqFzzmQilwWnUmedeZnZFZpNXFOTmlJgyuyKoGwrciUukrzCEjUueQTAFpmVtZgZOlcqgZtvEEYKihIbZlvFaDdDRyXIHpuJtMcsMrnsqCHDKpwqMtBRtHgaXHrRCrpyWARKtpgwGVgVQqhsmOtjSwVHJlVZDJYLhLHUeqUGHSTgBXnNLpEJsGvAtruGOpEBJNTnQOYSHWfXyAoFvkwtNmdSWjQkuGlODMdXMnPllhrZTApYWLgyYmGDKCyIqMVmhYOjYvtKsoRflkAoHRZPbwQMGuSQUUsdkGpjnBsQwSBBUFVAGORNUBjykhYxeiZtaPvCttyPvqjXDfXXqeucpzEjfqkOfCnGyKKLMxRzxCMtEItqsVguZPyWuVvvbVHvfVuWhbIXnltJeiqKtINFdJxkdXlpTgcIOklxEtEqMNzJYRSlcEsrmzgexaHqtSmqhmUflekSzqqjuXwghPpumZVSVynLTwLtDAehmnZUAKIElZEDxhejuGriibOZStMpoYjypYhVmBrmuUhxmffZiEDcoLkWsgFpLPgzkVmHsquLVdSYgvwSMGKdmzprfXumJBHJSFLadsNpsiKvAjFMHzmlcmJzlQojhWhvapdRPYFUQUbkqZeqifxBdTkjSJvEOEEtukVsDjOJaNpGobtJStYZBPqXIIbalJQfOHozyICiOxkKORbCdoHjizfzMZdJYmFGAWNgrYLPzScdPKuoCcLOoYXYmlaiQJSOaBmgFSgLHsBgydSLlwfdXNufhHgQdTeqpTtAIzJMgnKUkImohtFwbleknaAYDjcYiUaJPLscOcYucjhTeWMdWWRJWNtfsIFWPPHoqvOBXrafakcKcEoJhzndLClOKmvwsxDAOVhDCADmlcLlvSAkFWqcFiOhhUjtBSSxCYVYlddCvgQYhpclpywyEHSESwNbcBcXHOQttgaqgfMVDblBWUKpTOTxABmLpTHFmhEyoOnqRmebTBXEuggqWPghRHRqQlyyxIjpVseiEZpHLZgpIVYujeJJOptVpzICkUfGQzWzLWxdUeoGmhUvvTIWXMQETLKhGdrhzFZzyUHWflCHJagtqwunsWTinUvEzKVCMBUtJSiCKJTNijuUBGuhyhyQSHPDkoDSbNulTimdAXHOSZJqiGukRotktLjnKSBwyYsBEFYIAcoUeKSGhpscKUScHnjykrWJBeSTFsxWuTrCWNHYegrUIXceAeVAlpwRHANRDWqdNVLUvqGpsSabfJCDFiwidgTkIVhtxbQZSiXlDdCmkRzLWzQrTOcNLvxYnxUwKimzsQunmKKAxQVXeoLWLrqHVAHshBAmbCaFzCdcBljedgeVsIxekffMkBQvWeTCVcNHkjPMCmIDtmXlRtNvGTtxxbLViiykokxKLyvmIsHECloUWNafyFDfRMjUZeHMzKGfDKBuacPJHECTJgQWqSLrKlMJIjNIlCHaFwYOHvbnoQhnzZGLdcCjMAISVdusugphxhvQoauWlXpFJGNEBYXTzuwLZLVejgcVUJEZAATtoQHKrrVEDWqyVnMJJcthVBHvlrWJsQqUUIdtiBBWGGSxPGZrsILeRSWpUWGbPPtATABFQftCCJpozMvWKpVmqAaNWBFVaWHrOfgDQUztlpFXBtIgXiOfXBePSCwVDVfPWzoxnVPVYJdNiXkFtICitJnOVnpHOmnJuVGmJjWLcKiZozwpdPDxNspqlgUbZpikTlxNRWLsFpLdJyBelxDdFhxgrBxUcvddAZfYdDhXIzccYtHlJyKPMSotcDlIJdLtzujNmqqtacrrHyqmyBQSuniGKzRTEiIEkjaakSjxEPYkrMOwVBoQosXWNitVJgkYdLlBoxdNpxbpUHWfruOsNVGkExvcqZJzIsZSuSrGJelsOdqcHYiLhhaLhxvMYUBmhpsrUFTrhSiioEXMLIaXcEOhkrWVxIAgIPwtRIXXjhpafGxdxHqwQXCEdjPNmeynszkmIaOePdjchUcVIulQnfwIYkiZryTMFvnNNHHohHwcGykOAZtTrASwqiSRJnqTRxOaSiACnNeLGzoviItbhnUOtFHWkUIUrwfIramOIxZzflIAtcRkaKvodTViHvydQPBZJReOVkypuEKHeEZMmokQeaMtMQvCVxENZyclBtuQNwcFGVYzmuHWIUdBsXUszjLAOCmoQajWSvtRPYFRaqjaAtYETvcInBDmmiuPhqJZppLtzIMJUhLioestsWRJsUfcaFiPUlytCgBJJWUNuoNbMbCGwLlDKxfNrDBfBIgiJiRSFCVUjCnMJZfuvtKAIHFLjgIgTHEyMpzLKHYZoLCeOrVPPekQYbsFuIwwQjwReXSQHKMEmmSvNVZuGtxoIRnSGkYEXUWYFMsFmsHuqHWVtZTBtSLBukGkjCyZLErgJRKLfDHOzcwIzpzSxSIsatWsvItkcYcNMogCLLPcXbCQLJMGngvuxocoFDdefCruJwWxGsYNLtGOuWQPPMNgtIuiRvUsoGCLImBnIlmjqPIoifaaFdvjKUnjyQCvPPvYZcapwOegWCzdJVMShqjNrsmIjAkgABtLPmkvBfciPbMYcimadmggHlaoqUGpDmEAuDBUQvqRvgIgNPLrakFZAilsOxcjjkHxSaLUtSHOISgynoQBdPUbpSVntIiLmwSanegcVPUfzAFAvFPCZNMUJiZqKKLbcopOuJAteNHrJAoSPWaJnIxnUJNWodQVHivFcMoBgnOCeONrHpJIptgmKJYYjRXkMZeRYAnNcmiXSMHyGgtloffJaCRhxNTzJVYMQtuJNWJwfEnnwTWNIdPsLiQpspDgpEYDsNAtkcUfFUCPWbmlfUAaWTWFcvfeATVtqofiXXPLLOwrQUCbuJfLWfgQvCKoRyxczDCYgOfMQtySivbbaWTNAhUcWIAvCExQXWslbWiqEtUlximjpQPlPSYgDcdohGktKCnOTVVJUMnRzJKDDEcMybZdkPaGtUqpnCZCXtskyXNsnpqbDsLFbjdoAifVVxbMGNnsQElypkwSzeQAsUwZIfjWuWbqUDizxrCoJVyTgkKrLdNlttgdejQUzXgyTsepVqMKcaeCZRfLUSeywZayUtEefGVPhlveoEAaRrjxjzhbeLBsibPvhCtHysCbQiPgtzTptXysgCgjiSWTxtcnLYiNqYiQZSKnilxVjEqigOTFSUdIKEIdjfevjRrRIqmRfUsfqyubrvphnOPpIxTvtGtfMpryuqSwvumswmjlbYTyOYuKrxhSIlPvNNjWKKZKEtBTIWAhObVONANYvfroLZatWSCTmUYMBqwPGaKlWitsvNGHBYdwVnVTuCcmZcjPVTVhLNkjtAFFCSUrJyMlqDzPmQiMXlhfaYsJoPViYongwFEorYCljttvUOBruNBGpURQaNAPsYsHmZmwIkuXEkEAOssuZVWqcbBWNuWKfFCfmJnGtFDhofGeGoZyZrqkWdaGISOxqDnwrqlORJcoHMOlOpHwIyxYUPRHogWuhGKNCRoUOtHuxDPUJfFQfwkKCMLCSyjayGxWIPMYaAQeFuesmGBRXrtLJYGSpZklMAfDYlaTuWPURXTrXvwbBPDhjRlNeAwFLKGarvfJVCALtVZmGTFRQuleAxPtIfTSDTzuPkmgLwTgSkyUBxQWuvlhjGdylUOhSmvWVfXDHjRXCWKXNbWxTsGPCswSSwUTkWleuNYtQmiMBWAIytfJLXxYNmDwyEBQnXmnNbzaZreEzkfcwyclOJVlWRcWsXwbBMVdafIxfAKIZhCCpBcUByxLJBIuQOYbVFVUQpCVQskzPbsfkACoHQZADqDwAOoWxzKGOgXZnyqsNxrajzGudUeCpGxKaifmwsVYOPJhBlWyAjSRlumfrQwlKpQDWyBepiytidNSGJksciWMZCJPmekWCvMBwFaasStuYEInxqzdmilzJVFCgqYtRLoPHMFWlrHpYUkMWVfVPiylFYEhgLDpnwWHwRULfXOFMDcuaRfFoRawHXZNPwezvaINnazWXKvnqxPaKiEdUhLOuDrHaXuemdzMfwaZmquXqidEwssOstpCesTMPXQUJxQCrIcmgjcFPUvfldKmzLuiYXTYoqAqarkdwhyMohvqUjQFpevtVgbQwXtpmaJTEkhjLmnfqpmtXrDdKjlBCYiFuldTbrsEXxImYKOzBHVLxSXTogWRxrilAwRUHjsnokzaVNNUTTCuAUgWOTbPimCKWiiKzeqLVrFnbTtlqHbUDfiqpIbxLUgaqCPEXBdLOnBmPaFWBTSmCtNHjJDWxcXCCRWQNSpTAWUYttmTJzCbUxfQuZmQnlSxHVclupWuhwZgCHQWnqitVQthjorthdmPXFYdsltonLryllEhVLWgNjMmKplJnmZnKSYnHFWQHoPbGpsVXNZvBcqSeTvNNqruJoVVvLaLfPvUVivmHDEIWVupIFOYzksrOUADONmvgprEdgNCYSeZPedIjrqpecxpzaTKdKLhtyoeROFgIvTyDoezoZUcMOiBxwtaJWahOhMUJqbbxpnaooQcxkkNzOhMRePBxtTUkzbzrnHkidDFCpPgUvBahNsYhqzHvjsEyugDgSpAltwgkzmCPagLdaiqwXPVXTfbwbLTrEIuMVwiWGTPMVHDBNtVPrcwwTIHYPPoxUKbmsxrrmc".as_bytes().into()))),
		0u32..8193u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	compiler_directive_arg,
	"`define meow",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Builtin(BuiltinDirective::Define)),
		0u32..7u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		7u32..8u32,
		Position::new(0, 7)
	),
	spanned_token!(
		Token::Identifier(Identifier::Simple("meow".as_bytes().into())),
		8u32..12u32,
		Position::new(0, 8)
	)
);

tokenizer_test!(
	all,
	compiler_directive_multi_arg,
	"`define nya 8",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Builtin(BuiltinDirective::Define)),
		0u32..7u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		7u32..8u32,
		Position::new(0, 7)
	),
	spanned_token!(
		Token::Identifier(Identifier::Simple("nya".as_bytes().into())),
		8u32..11u32,
		Position::new(0, 8)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		11u32..12u32,
		Position::new(0, 11)
	),
	spanned_token!(
		Token::UnsignedNumber("8".as_bytes().into()),
		12u32..13u32,
		Position::new(0, 12)
	)
);

tokenizer_test!(
	all,
	string,
	r#""This Is A Simple String :3""#,
	spanned_token!(
		Token::SingleQuotedString(SingleQuotedString::new(
			"This Is A Simple String :3".as_bytes().into()
		)),
		0u32..28u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	string_eof,
	r#""This Is A Simple String :3"#,
	spanned_token!(
		Token::SingleQuotedString(SingleQuotedString::new(
			"This Is A Simple String :3".as_bytes().into()
		)),
		0u32..27u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	ident,
	"meow",
	spanned_token!(
		Token::Identifier(Identifier::Simple("meow".as_bytes().into())),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	ident_adv,
	"m3ow_me0w",
	spanned_token!(
		Token::Identifier(Identifier::Simple("m3ow_me0w".as_bytes().into())),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_always,
	"always",
	spanned_token!(
		Token::Keyword(Keyword::Always),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_and,
	"and",
	spanned_token!(
		Token::Keyword(Keyword::And),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_assign,
	"assign",
	spanned_token!(
		Token::Keyword(Keyword::Assign),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_begin,
	"begin",
	spanned_token!(
		Token::Keyword(Keyword::Begin),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_buf,
	"buf",
	spanned_token!(
		Token::Keyword(Keyword::Buf),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_bufif0,
	"bufif0",
	spanned_token!(
		Token::Keyword(Keyword::BufIf0),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_bufif1,
	"bufif1",
	spanned_token!(
		Token::Keyword(Keyword::BufIf1),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_case,
	"case",
	spanned_token!(
		Token::Keyword(Keyword::Case),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_casex,
	"casex",
	spanned_token!(
		Token::Keyword(Keyword::CaseX),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_casez,
	"casez",
	spanned_token!(
		Token::Keyword(Keyword::CaseZ),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_cmos,
	"cmos",
	spanned_token!(
		Token::Keyword(Keyword::Cmos),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_deassign,
	"deassign",
	spanned_token!(
		Token::Keyword(Keyword::Deassign),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_default,
	"default",
	spanned_token!(
		Token::Keyword(Keyword::Default),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_defparam,
	"defparam",
	spanned_token!(
		Token::Keyword(Keyword::DefParam),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_disable,
	"disable",
	spanned_token!(
		Token::Keyword(Keyword::Disable),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_edge,
	"edge",
	spanned_token!(
		Token::Keyword(Keyword::Edge),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_else,
	"else",
	spanned_token!(
		Token::Keyword(Keyword::Else),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_end,
	"end",
	spanned_token!(
		Token::Keyword(Keyword::End),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endcase,
	"endcase",
	spanned_token!(
		Token::Keyword(Keyword::EndCase),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endfunction,
	"endfunction",
	spanned_token!(
		Token::Keyword(Keyword::EndFunction),
		0u32..11u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endmodule,
	"endmodule",
	spanned_token!(
		Token::Keyword(Keyword::EndModule),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endprimitive,
	"endprimitive",
	spanned_token!(
		Token::Keyword(Keyword::EndPrimitive),
		0u32..12u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endspecify,
	"endspecify",
	spanned_token!(
		Token::Keyword(Keyword::EndSpecify),
		0u32..10u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endtable,
	"endtable",
	spanned_token!(
		Token::Keyword(Keyword::EndTable),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_endtask,
	"endtask",
	spanned_token!(
		Token::Keyword(Keyword::EndTask),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_event,
	"event",
	spanned_token!(
		Token::Keyword(Keyword::Event),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_for,
	"for",
	spanned_token!(
		Token::Keyword(Keyword::For),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_force,
	"force",
	spanned_token!(
		Token::Keyword(Keyword::Force),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_forever,
	"forever",
	spanned_token!(
		Token::Keyword(Keyword::Forever),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_fork,
	"fork",
	spanned_token!(
		Token::Keyword(Keyword::Fork),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_function,
	"function",
	spanned_token!(
		Token::Keyword(Keyword::Function),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_highz0,
	"highz0",
	spanned_token!(
		Token::Keyword(Keyword::HighZ0),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_highz1,
	"highz1",
	spanned_token!(
		Token::Keyword(Keyword::HighZ1),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_if,
	"if",
	spanned_token!(Token::Keyword(Keyword::If), 0u32..2u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_ifnone,
	"ifnone",
	spanned_token!(
		Token::Keyword(Keyword::IfNone),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_initial,
	"initial",
	spanned_token!(
		Token::Keyword(Keyword::Initial),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_inout,
	"inout",
	spanned_token!(
		Token::Keyword(Keyword::InOut),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_input,
	"input",
	spanned_token!(
		Token::Keyword(Keyword::Input),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_integer,
	"integer",
	spanned_token!(
		Token::Keyword(Keyword::Integer),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_join,
	"join",
	spanned_token!(
		Token::Keyword(Keyword::Join),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_large,
	"large",
	spanned_token!(
		Token::Keyword(Keyword::Large),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_macromodule,
	"macromodule",
	spanned_token!(
		Token::Keyword(Keyword::MacroModule),
		0u32..11u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_medium,
	"medium",
	spanned_token!(
		Token::Keyword(Keyword::Medium),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_module,
	"module",
	spanned_token!(
		Token::Keyword(Keyword::Module),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_nand,
	"nand",
	spanned_token!(
		Token::Keyword(Keyword::Nand),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_negedge,
	"negedge",
	spanned_token!(
		Token::Keyword(Keyword::NegEdge),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_nmos,
	"nmos",
	spanned_token!(
		Token::Keyword(Keyword::Nmos),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_nor,
	"nor",
	spanned_token!(
		Token::Keyword(Keyword::Nor),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_not,
	"not",
	spanned_token!(
		Token::Keyword(Keyword::Not),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_notif0,
	"notif0",
	spanned_token!(
		Token::Keyword(Keyword::NotIf0),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_notif1,
	"notif1",
	spanned_token!(
		Token::Keyword(Keyword::NotIf1),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_or,
	"or",
	spanned_token!(Token::Keyword(Keyword::Or), 0u32..2u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	keyword_output,
	"output",
	spanned_token!(
		Token::Keyword(Keyword::Output),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_parameter,
	"parameter",
	spanned_token!(
		Token::Keyword(Keyword::Parameter),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_pmos,
	"pmos",
	spanned_token!(
		Token::Keyword(Keyword::Pmos),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_posedge,
	"posedge",
	spanned_token!(
		Token::Keyword(Keyword::PosEdge),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_primitive,
	"primitive",
	spanned_token!(
		Token::Keyword(Keyword::Primitive),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_pull0,
	"pull0",
	spanned_token!(
		Token::Keyword(Keyword::Pull0),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_pull1,
	"pull1",
	spanned_token!(
		Token::Keyword(Keyword::Pull1),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_pulldown,
	"pulldown",
	spanned_token!(
		Token::Keyword(Keyword::Pulldown),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_pullup,
	"pullup",
	spanned_token!(
		Token::Keyword(Keyword::Pullup),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_rcmos,
	"rcmos",
	spanned_token!(
		Token::Keyword(Keyword::Rcmos),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_real,
	"real",
	spanned_token!(
		Token::Keyword(Keyword::Real),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_realtime,
	"realtime",
	spanned_token!(
		Token::Keyword(Keyword::Realtime),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_reg,
	"reg",
	spanned_token!(
		Token::Keyword(Keyword::Reg),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_release,
	"release",
	spanned_token!(
		Token::Keyword(Keyword::Release),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_repeat,
	"repeat",
	spanned_token!(
		Token::Keyword(Keyword::Repeat),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_rnmos,
	"rnmos",
	spanned_token!(
		Token::Keyword(Keyword::Rnmos),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_rpmos,
	"rpmos",
	spanned_token!(
		Token::Keyword(Keyword::Rpmos),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_rtran,
	"rtran",
	spanned_token!(
		Token::Keyword(Keyword::Rtran),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_rtranif0,
	"rtranif0",
	spanned_token!(
		Token::Keyword(Keyword::RtranIf0),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_rtranif1,
	"rtranif1",
	spanned_token!(
		Token::Keyword(Keyword::RtranIf1),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_scalared,
	"scalared",
	spanned_token!(
		Token::Keyword(Keyword::Scalared),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_small,
	"small",
	spanned_token!(
		Token::Keyword(Keyword::Small),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_specify,
	"specify",
	spanned_token!(
		Token::Keyword(Keyword::Specify),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_specparam,
	"specparam",
	spanned_token!(
		Token::Keyword(Keyword::SpecParam),
		0u32..9u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_strong0,
	"strong0",
	spanned_token!(
		Token::Keyword(Keyword::Strong0),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_strong1,
	"strong1",
	spanned_token!(
		Token::Keyword(Keyword::Strong1),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_supply0,
	"supply0",
	spanned_token!(
		Token::Keyword(Keyword::Supply0),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_supply1,
	"supply1",
	spanned_token!(
		Token::Keyword(Keyword::Supply1),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_table,
	"table",
	spanned_token!(
		Token::Keyword(Keyword::Table),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_task,
	"task",
	spanned_token!(
		Token::Keyword(Keyword::Task),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_time,
	"time",
	spanned_token!(
		Token::Keyword(Keyword::Time),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_tran,
	"tran",
	spanned_token!(
		Token::Keyword(Keyword::Tran),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_tranif0,
	"tranif0",
	spanned_token!(
		Token::Keyword(Keyword::TranIf0),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_tranif1,
	"tranif1",
	spanned_token!(
		Token::Keyword(Keyword::TranIf1),
		0u32..7u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_tri,
	"tri",
	spanned_token!(
		Token::Keyword(Keyword::Tri),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_tri0,
	"tri0",
	spanned_token!(
		Token::Keyword(Keyword::Tri0),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_tri1,
	"tri1",
	spanned_token!(
		Token::Keyword(Keyword::Tri1),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_triand,
	"triand",
	spanned_token!(
		Token::Keyword(Keyword::Triand),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_trior,
	"trior",
	spanned_token!(
		Token::Keyword(Keyword::Trior),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_trireg,
	"trireg",
	spanned_token!(
		Token::Keyword(Keyword::Trireg),
		0u32..6u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_vectored,
	"vectored",
	spanned_token!(
		Token::Keyword(Keyword::Vectored),
		0u32..8u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_wait,
	"wait",
	spanned_token!(
		Token::Keyword(Keyword::Wait),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_wand,
	"wand",
	spanned_token!(
		Token::Keyword(Keyword::Wand),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_weak0,
	"weak0",
	spanned_token!(
		Token::Keyword(Keyword::Weak0),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_weak1,
	"weak1",
	spanned_token!(
		Token::Keyword(Keyword::Weak1),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_while,
	"while",
	spanned_token!(
		Token::Keyword(Keyword::While),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_wire,
	"wire",
	spanned_token!(
		Token::Keyword(Keyword::Wire),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_wor,
	"wor",
	spanned_token!(
		Token::Keyword(Keyword::Wor),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_xnor,
	"xnor",
	spanned_token!(
		Token::Keyword(Keyword::Xnor),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	keyword_xor,
	"xor",
	spanned_token!(
		Token::Keyword(Keyword::Xor),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	escaped_ident,
	r"\meow",
	spanned_token!(
		Token::Identifier(Identifier::Escaped(r"meow".as_bytes().into())),
		0u32..5u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	escaped_ident_adv,
	r"\nya$owo@uwu",
	spanned_token!(
		Token::Identifier(Identifier::Escaped(r"nya$owo@uwu".as_bytes().into())),
		0u32..12u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	unsigned_number,
	"1234",
	spanned_token!(
		Token::UnsignedNumber("1234".as_bytes().into()),
		0u32..4u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	real_number_simple,
	"1.23",
	spanned_token!(Token::Real(1.23), 0u32..4u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	real_number_simple_pos,
	"+1.23",
	spanned_token!(
		Token::Operator(Operator::Plus),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(Token::Real(1.23), 1u32..5u32, Position::new(0, 1))
);

tokenizer_test!(
	all,
	real_number_simple_neg,
	"-1.23",
	spanned_token!(
		Token::Operator(Operator::Minus),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(Token::Real(1.23), 1u32..5u32, Position::new(0, 1))
);

tokenizer_test!(
	all,
	real_number_exponent,
	"1e7",
	spanned_token!(Token::Real(1e7), 0u32..3u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	real_number_dec_exponent,
	"1.2e6",
	spanned_token!(Token::Real(1.2e6), 0u32..5u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	real_number_exponent_pos,
	"1e+6",
	spanned_token!(Token::Real(1e6), 0u32..4u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	real_number_exponent_neg,
	"1e-6",
	spanned_token!(Token::Real(1e-6), 0u32..4u32, Position::new(0, 0))
);

tokenizer_test!(
	all,
	binary_prefixed,
	"4'b1001",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("1001".as_bytes().into()),
		3u32..7u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_dont_care,
	"4'bxx1x",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("xx1x".as_bytes().into()),
		3u32..7u32,
		Position::new(0, 3)
	)
);
tokenizer_test!(
	all,
	binary_prefixed_all_dont_care,
	"1'bx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_high_z,
	"4'bzz11",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("zz11".as_bytes().into()),
		3u32..7u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_all_high_z,
	"1'bz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded,
	"4 'b1001",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("1001".as_bytes().into()),
		4u32..8u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded_dont_care,
	"4 'bxx1x",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("xx1x".as_bytes().into()),
		4u32..8u32,
		Position::new(0, 4)
	)
);
tokenizer_test!(
	all,
	binary_prefixed_padded_all_dont_care,
	"1 'bx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded_high_z,
	"4 'bzz11",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("zz11".as_bytes().into()),
		4u32..8u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_prefixed_padded_all_high_z,
	"1 'bz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	binary_naked,
	"'b10",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("10".as_bytes().into()),
		2u32..4u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_dont_care,
	"'bxx1x",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("xx1x".as_bytes().into()),
		2u32..6u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_all_dont_care,
	"'bx",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		2u32..3u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_high_z,
	"'bzz11",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("zz11".as_bytes().into()),
		2u32..6u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	binary_naked_all_high_z,
	"'bz",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		2u32..3u32,
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
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("01zx".as_bytes().into()),
		4u32..8u32,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8u32..9u32,
		Position::new(0, 8)
	),
	spanned_token!(
		Token::Operator(Operator::Plus),
		9u32..10u32,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10u32..11u32,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			true,
			false,
		)),
		11u32..13u32,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13u32..14u32,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Number("?".as_bytes().into()),
		14u32..15u32,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15u32..16u32,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16u32..17u32,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		17u32..19u32,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Number("ZX".as_bytes().into()),
		19u32..21u32,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21u32..22u32,
		Position::new(2, 5)
	)
);

tokenizer_test!(
	all,
	octal_prefixed,
	"3'o666",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("666".as_bytes().into()),
		3u32..6u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_dont_care,
	"3'ox3x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x3x".as_bytes().into()),
		3u32..6u32,
		Position::new(0, 3)
	)
);
tokenizer_test!(
	all,
	octal_prefixed_all_dont_care,
	"1'ox",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_high_z,
	"3'ozz4",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("zz4".as_bytes().into()),
		3u32..6u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_all_high_z,
	"1'oz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded,
	"3 'o464",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("464".as_bytes().into()),
		4u32..7u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded_dont_care,
	"3 'ox2x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x2x".as_bytes().into()),
		4u32..7u32,
		Position::new(0, 4)
	)
);
tokenizer_test!(
	all,
	octal_prefixed_padded_all_dont_care,
	"1 'ox",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded_high_z,
	"3 'ozz1",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("zz1".as_bytes().into()),
		4u32..7u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_prefixed_padded_all_high_z,
	"1 'oz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	octal_naked,
	"'o66",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("66".as_bytes().into()),
		2u32..4u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_dont_care,
	"'ox3x",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x3x".as_bytes().into()),
		2u32..5u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_all_dont_care,
	"'ox",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		2u32..3u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_high_z,
	"'ozz7",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("zz7".as_bytes().into()),
		2u32..5u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	octal_naked_all_high_z,
	"'oz",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		2u32..3u32,
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
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("06zx".as_bytes().into()),
		4u32..8u32,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8u32..9u32,
		Position::new(0, 8)
	),
	spanned_token!(
		Token::Operator(Operator::Plus),
		9u32..10u32,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10u32..11u32,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			true,
			false,
		)),
		11u32..13u32,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13u32..14u32,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Number("?".as_bytes().into()),
		14u32..15u32,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15u32..16u32,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16u32..17u32,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Octal,
			false,
			false,
		)),
		17u32..19u32,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Number("ZX".as_bytes().into()),
		19u32..21u32,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21u32..22u32,
		Position::new(2, 5)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed,
	"2'd69",
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("69".as_bytes().into()),
		3u32..5u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_dont_care,
	"3'dx3x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("x3x".as_bytes().into())),
		3u32..6u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_all_dont_care,
	"1'dx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("x".as_bytes().into())),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_high_z,
	"3'dzz4",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("zz4".as_bytes().into())),
		3u32..6u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_all_high_z,
	"1'dz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Invalid(Some("z".as_bytes().into())),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded,
	"2 'd99",
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("99".as_bytes().into()),
		4u32..6u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_dont_care,
	"3 'dx9x",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("x9x".as_bytes().into())),
		4u32..7u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_all_dont_care,
	"1 'dx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("x".as_bytes().into())),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_high_z,
	"3 'dzz7",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("zz7".as_bytes().into())),
		4u32..7u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_prefixed_padded_all_high_z,
	"1 'dz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("z".as_bytes().into())),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	decimal_naked,
	"'d90",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("90".as_bytes().into()),
		2u32..4u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_dont_care,
	"'dx9x",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("x9x".as_bytes().into())),
		2u32..5u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_all_dont_care,
	"'dx",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("x".as_bytes().into())),
		2u32..3u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_high_z,
	"'dzz9",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("zz9".as_bytes().into())),
		2u32..5u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	decimal_naked_all_high_z,
	"'dz",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("z".as_bytes().into())),
		2u32..3u32,
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
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Invalid(Some("06zx".as_bytes().into())),
		4u32..8u32,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8u32..9u32,
		Position::new(0, 8)
	),
	spanned_token!(
		Token::Operator(Operator::Plus),
		9u32..10u32,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10u32..11u32,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			true,
			false,
		)),
		11u32..13u32,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13u32..14u32,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Invalid(Some("?".as_bytes().into())),
		14u32..15u32,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15u32..16u32,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16u32..17u32,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Decimal,
			false,
			false,
		)),
		17u32..19u32,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Invalid(Some("ZX".as_bytes().into())),
		19u32..21u32,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21u32..22u32,
		Position::new(2, 5)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed,
	"4'h1a4F",
	spanned_token!(
		Token::UnsignedNumber("4".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("1a4F".as_bytes().into()),
		3u32..7u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_dont_care,
	"3'hxDx",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("xDx".as_bytes().into()),
		3u32..6u32,
		Position::new(0, 3)
	)
);
tokenizer_test!(
	all,
	hexadecimal_prefixed_all_dont_care,
	"1'hx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_high_z,
	"3'hzzF",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("zzF".as_bytes().into()),
		3u32..6u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_all_high_z,
	"1'hz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		1u32..3u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		3u32..4u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded,
	"3 'ha7d",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("a7d".as_bytes().into()),
		4u32..7u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_dont_care,
	"3 'hxEx",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("xEx".as_bytes().into()),
		4u32..7u32,
		Position::new(0, 4)
	)
);
tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_all_dont_care,
	"1 'hx",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_high_z,
	"3 'hzzA",
	spanned_token!(
		Token::UnsignedNumber("3".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("zzA".as_bytes().into()),
		4u32..7u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_prefixed_padded_all_high_z,
	"1 'hz",
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		4u32..5u32,
		Position::new(0, 4)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked,
	"'h7F",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("7F".as_bytes().into()),
		2u32..4u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_dont_care,
	"'hxAx",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("xAx".as_bytes().into()),
		2u32..5u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_all_dont_care,
	"'hx",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("x".as_bytes().into()),
		2u32..3u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_high_z,
	"'hzz7",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("zz7".as_bytes().into()),
		2u32..5u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	hexadecimal_naked_all_high_z,
	"'hz",
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Number("z".as_bytes().into()),
		2u32..3u32,
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
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		1u32..2u32,
		Position::new(0, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		2u32..4u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::Number("7Fzx".as_bytes().into()),
		4u32..8u32,
		Position::new(0, 4)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		8u32..9u32,
		Position::new(0, 8)
	),
	spanned_token!(
		Token::Operator(Operator::Plus),
		9u32..10u32,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::UnsignedNumber("1".as_bytes().into()),
		10u32..11u32,
		Position::new(1, 1)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			true,
			false,
		)),
		11u32..13u32,
		Position::new(1, 2)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		13u32..14u32,
		Position::new(1, 4)
	),
	spanned_token!(
		Token::Number("?".as_bytes().into()),
		14u32..15u32,
		Position::new(1, 5)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		15u32..16u32,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::UnsignedNumber("2".as_bytes().into()),
		16u32..17u32,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Hexadecimal,
			false,
			false,
		)),
		17u32..19u32,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Number("ZX".as_bytes().into()),
		19u32..21u32,
		Position::new(2, 3)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		21u32..22u32,
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
		0u32..1u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Keyword(Keyword::Module),
		1u32..7u32,
		Position::new(1, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		7u32..8u32,
		Position::new(1, 6)
	),
	spanned_token!(
		Token::Identifier(Identifier::Simple("foo".as_bytes().into())),
		8u32..11u32,
		Position::new(1, 7)
	),
	spanned_token!(
		Token::Control(Control::ParenOpen),
		11u32..12u32,
		Position::new(1, 10)
	),
	spanned_token!(
		Token::Control(Control::ParenClose),
		12u32..13u32,
		Position::new(1, 11)
	),
	spanned_token!(
		Token::Control(Control::Semicolon),
		13u32..14u32,
		Position::new(1, 12)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		14u32..15u32,
		Position::new(1, 13)
	),
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		15u32..16u32,
		Position::new(2, 0)
	),
	spanned_token!(
		Token::Keyword(Keyword::Wire),
		16u32..20u32,
		Position::new(2, 1)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		20u32..21u32,
		Position::new(2, 5)
	),
	spanned_token!(
		Token::Identifier(Identifier::Simple("a".as_bytes().into())),
		21u32..22u32,
		Position::new(2, 6)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		22u32..23u32,
		Position::new(2, 7)
	),
	spanned_token!(
		Token::Operator(Operator::Equals),
		23u32..24u32,
		Position::new(2, 8)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		24u32..25u32,
		Position::new(2, 9)
	),
	spanned_token!(
		Token::UnsignedNumber("7".as_bytes().into()),
		25u32..26u32,
		Position::new(2, 10)
	),
	spanned_token!(
		Token::BasedLiteralSpecifier(BasedLiteralSpecifier::new(
			BaseSpecifier::Binary,
			false,
			false,
		)),
		26u32..28u32,
		Position::new(2, 11)
	),
	spanned_token!(
		Token::Number("1010x01".as_bytes().into()),
		28u32..35u32,
		Position::new(2, 13)
	),
	spanned_token!(
		Token::Control(Control::Semicolon),
		35u32..36u32,
		Position::new(2, 20)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		36u32..37u32,
		Position::new(2, 21)
	),
	spanned_token!(
		Token::Keyword(Keyword::EndModule),
		37u32..46u32,
		Position::new(3, 0)
	),
	spanned_token!(
		Token::Newline("\n".as_bytes().into()),
		46u32..47u32,
		Position::new(3, 9)
	),
	spanned_token!(
		Token::Whitespace("\t".as_bytes().into()),
		47u32..48u32,
		Position::new(4, 0)
	)
);

tokenizer_test!(
	all,
	operator_full_connection,
	"*>",
	spanned_token!(
		Token::Operator(Operator::FullConnection),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_parallel_connection,
	"=>",
	spanned_token!(
		Token::Operator(Operator::ParallelConnection),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_wildcard_export_partial_01,
	"*:",
	spanned_token!(
		Token::Invalid(Some("*:".as_bytes().into())),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_wildcard_export_partial_02,
	"*::",
	spanned_token!(
		Token::Invalid(Some("*::".as_bytes().into())),
		0u32..3u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_abs_tolerance_partial,
	"+/",
	spanned_token!(
		Token::Invalid(Some("+/".as_bytes().into())),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	operator_rel_tolerance_partial,
	"+%",
	spanned_token!(
		Token::Invalid(Some("+%".as_bytes().into())),
		0u32..2u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	invalid_identifier_unicode,
	"あたしはねこです！ニャ〜",
	spanned_token!(
		Token::Invalid(Some("あたしはねこです！ニャ〜".as_bytes().into())),
		0u32..36u32,
		Position::new(0, 0)
	)
);

tokenizer_test!(
	all,
	text_macro_short_num_prefix,
	"00`a",
	spanned_token!(
		Token::UnsignedNumber("00".as_bytes().into()),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"a".as_bytes().into()
		))),
		2u32..4u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	text_macro_short_num_prefix_whitespace,
	"00 `a",
	spanned_token!(
		Token::UnsignedNumber("00".as_bytes().into()),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		2u32..3u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"a".as_bytes().into()
		))),
		3u32..5u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	text_macro_short_ident_prefix,
	"aa`a",
	spanned_token!(
		Token::Identifier(Identifier::Simple("aa".as_bytes().into())),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"a".as_bytes().into()
		))),
		2u32..4u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	text_macro_short_ident_prefix_whitespace,
	"aa `a",
	spanned_token!(
		Token::Identifier(Identifier::Simple("aa".as_bytes().into())),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		2u32..3u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"a".as_bytes().into()
		))),
		3u32..5u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	text_macro_med_num_prefix,
	"00`meow",
	spanned_token!(
		Token::UnsignedNumber("00".as_bytes().into()),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"meow".as_bytes().into()
		))),
		2u32..7u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	text_macro_med_num_prefix_whitespace,
	"00 `meow",
	spanned_token!(
		Token::UnsignedNumber("00".as_bytes().into()),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		2u32..3u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"meow".as_bytes().into()
		))),
		3u32..8u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	text_macro_med_ident_prefix,
	"aa`meow",
	spanned_token!(
		Token::Identifier(Identifier::Simple("aa".as_bytes().into())),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"meow".as_bytes().into()
		))),
		2u32..7u32,
		Position::new(0, 2)
	)
);

tokenizer_test!(
	all,
	text_macro_med_ident_prefix_whitespace,
	"aa `meow",
	spanned_token!(
		Token::Identifier(Identifier::Simple("aa".as_bytes().into())),
		0u32..2u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Whitespace(" ".as_bytes().into()),
		2u32..3u32,
		Position::new(0, 2)
	),
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::TextMacro(TextMacro::Other(
			"meow".as_bytes().into()
		))),
		3u32..8u32,
		Position::new(0, 3)
	)
);

tokenizer_test!(
	all,
	garbage_in_compiler_directive,
	"`default_nettype\x1Anone",
	spanned_token!(
		Token::CompilerDirective(CompilerDirective::Builtin(BuiltinDirective::DefaultNetType)),
		0u32..16u32,
		Position::new(0, 0)
	),
	spanned_token!(
		Token::Invalid(Some("\x1A".as_bytes().into())),
		16u32..17u32,
		Position::new(0, 16)
	),
	spanned_token!(
		Token::Identifier(Identifier::Simple("none".as_bytes().into())),
		17u32..21u32,
		Position::new(0, 17)
	)
);
