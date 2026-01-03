/* SPDX-License-Identifier: BSD-3-Clause */

use std::fmt::Display;

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::VerilogVariant;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Token {
	Invalid(Option<AtomicByteTendril>),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(AtomicByteTendril, VerilogVariant),
	BaseSpecifier(BaseSpecifier, bool),
	Comment(Comment),
	CompilerDirective(CompilerDirective),
	Control(Control),
	Identifier(AtomicByteTendril),
	Keyword(Keyword),
	Newline(AtomicByteTendril),
	Number(AtomicByteTendril),
	Real {
		value: f64,
		exponent: Option<AtomicByteTendril>,
	},
	Operator(Operator),
	String(AtomicByteTendril),
	UnsignedNumber(AtomicByteTendril),
	Whitespace(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CompilerDirective {
	Name(AtomicByteTendril),
	Arg(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Comment {
	Invalid(AtomicByteTendril),
	SingleLine(AtomicByteTendril),
	MultiLine(AtomicByteTendril),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BaseSpecifier {
	Binary,
	Octal,
	Decimal,
	Hexadecimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Control {
	At,
	BraceClose,
	BraceOpen,
	BracketClose,
	BracketOpen,
	Colon,
	Comma,
	Dollar,
	Dot,
	Octothorp,
	ParenClose,
	ParenOpen,
	Question,
	Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Keyword {
	Always,
	And,
	Assign,
	Begin,
	Buf,
	BufIf0,
	BufIf1,
	Case,
	CaseX,
	CaseZ,
	Cmos,
	Deassign,
	Default,
	DefParam,
	Disable,
	Edge,
	Else,
	End,
	EndCase,
	EndFunction,
	EndModule,
	EndPrimitive,
	EndSpecify,
	EndTable,
	EndTask,
	Event,
	For,
	Force,
	Forever,
	Fork,
	Function,
	HighZ0,
	HighZ1,
	If,
	IfNone,
	Initial,
	InOut,
	Input,
	Integer,
	Join,
	Large,
	MacroModule,
	Medium,
	Module,
	Nand,
	NegEdge,
	Nmos,
	Nor,
	Not,
	NotIf0,
	NotIf1,
	Or,
	Output,
	Parameter,
	Pmos,
	PosEdge,
	Primitive,
	Pull0,
	Pull1,
	Pulldown,
	Pullup,
	Rcmos,
	Real,
	Realtime,
	Reg,
	Release,
	Repeat,
	Rnmos,
	Rpmos,
	Rtran,
	RtranIf0,
	RtranIf1,
	Scalared,
	Small,
	Specify,
	SpecParam,
	Strong0,
	Strong1,
	Supply0,
	Supply1,
	Table,
	Task,
	Time,
	Tran,
	TranIf0,
	TranIf1,
	Tri,
	Tri0,
	Tri1,
	Triand,
	Trior,
	Trireg,
	Vectored,
	Wait,
	Wand,
	Weak0,
	Weak1,
	While,
	Wire,
	Wor,
	Xnor,
	Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Operator {
	Ampersand,
	Asterisk,
	CaseEquality,
	CaseInequality,
	Circumflex,
	Equals,
	Exclamation,
	GreaterThan,
	GreaterThanEqual,
	LessThan,
	LessThanEqual,
	LogicalAnd,
	LogicalOr,
	LogicalEquality,
	LogicalInequality,
	Minus,
	Percent,
	Pipe,
	Plus,
	ReductionNor,
	ReductionNand,
	Solidus,
	ShiftLeft,
	ShiftRight,
	Tilde,
	TildeCircumflex(bool), // NOTE(aki): this is the chirality of the `^~`/`~^` operator
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Invalid(_tendril) => todo!(),
			Token::ContextuallyInvalid(_tendril, _verilog_variant) => todo!(),
			Token::BaseSpecifier(base_specifier, _) => write!(f, "{}", base_specifier),
			Token::Comment(comment) => write!(f, "{}", comment),
			Token::CompilerDirective(compiler_directive) => write!(f, "{}", compiler_directive),
			Token::Control(control) => write!(f, "{}", control),
			Token::Identifier(tendril) => write!(f, "Identifier({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Token::Keyword(keyword) => write!(f, "{}", keyword),
			Token::Newline(tendril) => write!(f, "Newline({})", tendril.len()),
			Token::Number(tendril) => write!(f, "Number({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Token::Real { value, exponent } => {
				write!(f, "RealNumber(value: {}, exp: {:?})", value, exponent)
			},
			Token::Operator(operator) => write!(f, "{}", operator),
			Token::String(tendril) => write!(f, "String(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Token::UnsignedNumber(tendril) => write!(f, "UnsignedNumber({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Token::Whitespace(tendril) => write!(f, "Whitespace({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Display for CompilerDirective {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CompilerDirective::Name(tendril) => write!(f, "CompilerDirective(`{})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			CompilerDirective::Arg(tendril) => write!(f, "CompilerDirectiveArg({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Display for Comment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Comment::Invalid(tendril) => write!(f, "InvalidComment(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Comment::SingleLine(tendril) => write!(f, "SingleLineComment(\"// {}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Comment::MultiLine(tendril) => write!(f, "MultiLineComment(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Display for BaseSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"BaseSpecifier({})",
			match self {
				BaseSpecifier::Binary => "'b",
				BaseSpecifier::Octal => "'o",
				BaseSpecifier::Decimal => "'d",
				BaseSpecifier::Hexadecimal => "'h",
			}
		)
	}
}

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Control({})",
			match self {
				Control::At => "@",
				Control::BraceClose => "}",
				Control::BraceOpen => "{",
				Control::BracketClose => "]",
				Control::BracketOpen => "[",
				Control::Colon => ":",
				Control::Comma => ",",
				Control::Dollar => "$",
				Control::Dot => ".",
				Control::Octothorp => "#",
				Control::ParenClose => ")",
				Control::ParenOpen => "(",
				Control::Question => "?",
				Control::Semicolon => ";",
			}
		)
	}
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Keyword({})",
			match self {
				Keyword::Always => "always",
				Keyword::And => "and",
				Keyword::Assign => "assign",
				Keyword::Begin => "begin",
				Keyword::Buf => "buf",
				Keyword::BufIf0 => "bufif0",
				Keyword::BufIf1 => "bufif1",
				Keyword::Case => "case",
				Keyword::CaseX => "casex",
				Keyword::CaseZ => "casez",
				Keyword::Cmos => "cmos",
				Keyword::Deassign => "deassign",
				Keyword::Default => "default",
				Keyword::DefParam => "defparam",
				Keyword::Disable => "disable",
				Keyword::Edge => "edge",
				Keyword::Else => "else",
				Keyword::End => "end",
				Keyword::EndCase => "endcase",
				Keyword::EndFunction => "endfunction",
				Keyword::EndModule => "endmodule",
				Keyword::EndPrimitive => "endprimitive",
				Keyword::EndSpecify => "endspecify",
				Keyword::EndTable => "endtable",
				Keyword::EndTask => "endtask",
				Keyword::Event => "event",
				Keyword::For => "for",
				Keyword::Force => "force",
				Keyword::Forever => "forever",
				Keyword::Fork => "fork",
				Keyword::Function => "function",
				Keyword::HighZ0 => "highz0",
				Keyword::HighZ1 => "highz1",
				Keyword::If => "if",
				Keyword::IfNone => "ifnone",
				Keyword::Initial => "initial",
				Keyword::InOut => "inout",
				Keyword::Input => "input",
				Keyword::Integer => "integer",
				Keyword::Join => "join",
				Keyword::Large => "large",
				Keyword::MacroModule => "macromodule",
				Keyword::Medium => "medium",
				Keyword::Module => "module",
				Keyword::Nand => "nand",
				Keyword::NegEdge => "negedge",
				Keyword::Nmos => "nmos",
				Keyword::Nor => "nor",
				Keyword::Not => "not",
				Keyword::NotIf0 => "notif0",
				Keyword::NotIf1 => "notif1",
				Keyword::Or => "or",
				Keyword::Output => "output",
				Keyword::Parameter => "parameter",
				Keyword::Pmos => "pmos",
				Keyword::PosEdge => "posedge",
				Keyword::Primitive => "primitive",
				Keyword::Pull0 => "pull0",
				Keyword::Pull1 => "pull1",
				Keyword::Pulldown => "pulldown",
				Keyword::Pullup => "pullup",
				Keyword::Rcmos => "rcmos",
				Keyword::Real => "real",
				Keyword::Realtime => "realtime",
				Keyword::Reg => "reg",
				Keyword::Release => "release",
				Keyword::Repeat => "repeat",
				Keyword::Rnmos => "rnmos",
				Keyword::Rpmos => "rpmos",
				Keyword::Rtran => "rtran",
				Keyword::RtranIf0 => "rtranif0",
				Keyword::RtranIf1 => "rtranif1",
				Keyword::Scalared => "scalared",
				Keyword::Small => "small",
				Keyword::Specify => "specify",
				Keyword::SpecParam => "specparam",
				Keyword::Strong0 => "strong0",
				Keyword::Strong1 => "strong1",
				Keyword::Supply0 => "supply0",
				Keyword::Supply1 => "supply1",
				Keyword::Table => "table",
				Keyword::Task => "task",
				Keyword::Time => "time",
				Keyword::Tran => "tran",
				Keyword::TranIf0 => "tranif0",
				Keyword::TranIf1 => "tranif1",
				Keyword::Tri => "tri",
				Keyword::Tri0 => "tri0",
				Keyword::Tri1 => "tri1",
				Keyword::Triand => "triand",
				Keyword::Trior => "trior",
				Keyword::Trireg => "trireg",
				Keyword::Vectored => "vectored",
				Keyword::Wait => "wait",
				Keyword::Wand => "wand",
				Keyword::Weak0 => "weak0",
				Keyword::Weak1 => "weak1",
				Keyword::While => "while",
				Keyword::Wire => "wire",
				Keyword::Wor => "wor",
				Keyword::Xnor => "xnor",
				Keyword::Xor => "xor",
			}
		)
	}
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Operator({})",
			match self {
				Operator::Ampersand => "&",
				Operator::Asterisk => "*",
				Operator::CaseEquality => "===",
				Operator::CaseInequality => "!==",
				Operator::Circumflex => "^",
				Operator::Equals => "=",
				Operator::Exclamation => "!",
				Operator::GreaterThan => "<",
				Operator::GreaterThanEqual => "<=",
				Operator::LessThan => ">",
				Operator::LessThanEqual => ">=",
				Operator::LogicalAnd => "&&",
				Operator::LogicalOr => "||",
				Operator::LogicalEquality => "==",
				Operator::LogicalInequality => "!=",
				Operator::Minus => "-",
				Operator::Percent => "%",
				Operator::Pipe => "|",
				Operator::Plus => "+",
				Operator::ReductionNor => "~|",
				Operator::ReductionNand => "~&",
				Operator::Solidus => "/",
				Operator::ShiftLeft => "<<",
				Operator::ShiftRight => ">>",
				Operator::Tilde => "~",
				Operator::TildeCircumflex(chirality) =>
					if *chirality {
						"^~"
					} else {
						"~^"
					},
			}
		)
	}
}

impl Default for Token {
	fn default() -> Self {
		Self::Invalid(None)
	}
}

impl From<Token> for Spanned<Token, Position> {
	fn from(token: Token) -> Self {
		Self::new(token, None)
	}
}
