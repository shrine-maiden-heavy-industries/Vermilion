// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::VerilogVariant;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
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
		value:    f64,
		exponent: Option<AtomicByteTendril>,
	},
	Operator(Operator),
	String(AtomicByteTendril),
	UnsignedNumber(AtomicByteTendril),
	Whitespace(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompilerDirective {
	Name(AtomicByteTendril),
	Arg(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comment {
	Invalid(AtomicByteTendril),
	SingleLine(AtomicByteTendril),
	MultiLine(AtomicByteTendril),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseSpecifier {
	Binary,
	Octal,
	Decimal,
	Hexadecimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Control {
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
pub enum Keyword {
	Always,
	And,
	Assign,
	Automatic, // Added: IEEE 1364-2001
	Begin,
	Buf,
	BufIf0,
	BufIf1,
	Case,
	CaseX,
	CaseZ,
	Cell, // Added: IEEE 1364-2001
	Cmos,
	Config, // Added: IEEE 1364-2001
	Deassign,
	Default,
	DefParam,
	Design, // Added: IEEE 1364-2001
	Disable,
	Edge,
	Else,
	End,
	EndCase,
	EndConfig, // Added: IEEE 1364-2001
	EndFunction,
	EndGenerate, // Added: IEEE 1364-2001
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
	Generate, // Added: IEEE 1364-2001
	Genvar,   // Added: IEEE 1364-2001
	HighZ0,
	HighZ1,
	If,
	IfNone,
	IncDir,  // Added: IEEE 1364-2001
	Include, // Added: IEEE 1364-2001
	Initial,
	InOut,
	Input,
	Instance, // Added: IEEE 1364-2001
	Integer,
	Join,
	Large,
	LibList,    // Added: IEEE 1364-2001
	Library,    // Added: IEEE 1364-2001
	LocalParam, // Added: IEEE 1364-2001
	MacroModule,
	Medium,
	Module,
	Nand,
	NegEdge,
	Nmos,
	Nor,
	NoShowCancelled, // Added: IEEE 1364-2001
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
	PulseStyleOnDetect, // Added: IEEE 1364-2001
	PulseStyleOnEvent,  // Added: IEEE 1364-2001
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
	ShowCancelled, // Added: IEEE 1364-2001
	Signed,        // Added: IEEE 1364-2001
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
	Unsigned, // Added: IEEE 1364-2001
	Use,      // Added: IEEE 1364-2001
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
pub enum Operator {
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
			Self::Invalid(_tendril) => todo!(),
			Self::ContextuallyInvalid(_tendril, _verilog_variant) => todo!(),
			Self::BaseSpecifier(base_specifier, _) => base_specifier.fmt(f),
			Self::Comment(comment) => comment.fmt(f),
			Self::CompilerDirective(compiler_directive) => compiler_directive.fmt(f),
			Self::Control(control) => control.fmt(f),
			Self::Identifier(tendril) => write!(f, "Identifier({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Keyword(keyword) => keyword.fmt(f),
			Self::Newline(tendril) => write!(f, "Newline({})", tendril.len()),
			Self::Number(tendril) => write!(f, "Number({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Real { value, exponent } => {
				write!(f, "RealNumber(value: {}, exp: {:?})", value, exponent)
			},
			Self::Operator(operator) => operator.fmt(f),
			Self::String(tendril) => write!(f, "String(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::UnsignedNumber(tendril) => write!(f, "UnsignedNumber({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Whitespace(tendril) => write!(f, "Whitespace({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Display for CompilerDirective {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Name(tendril) => write!(f, "CompilerDirective(`{})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Arg(tendril) => write!(f, "CompilerDirectiveArg({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Display for Comment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Invalid(tendril) => write!(f, "InvalidComment(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::SingleLine(tendril) => write!(f, "SingleLineComment(\"// {}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::MultiLine(tendril) => write!(f, "MultiLineComment(\"{}\")", unsafe {
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
				Self::Binary => "'b",
				Self::Octal => "'o",
				Self::Decimal => "'d",
				Self::Hexadecimal => "'h",
			}
		)
	}
}

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"'{}'",
			match self {
				Self::At => "@",
				Self::BraceClose => "}",
				Self::BraceOpen => "{",
				Self::BracketClose => "]",
				Self::BracketOpen => "[",
				Self::Colon => ":",
				Self::Comma => ",",
				Self::Dollar => "$",
				Self::Dot => ".",
				Self::Octothorp => "#",
				Self::ParenClose => ")",
				Self::ParenOpen => "(",
				Self::Question => "?",
				Self::Semicolon => ";",
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
				Self::Always => "always",
				Self::And => "and",
				Self::Assign => "assign",
				Self::Automatic => "automatic", // Added: IEEE 1364-2001
				Self::Begin => "begin",
				Self::Buf => "buf",
				Self::BufIf0 => "bufif0",
				Self::BufIf1 => "bufif1",
				Self::Case => "case",
				Self::CaseX => "casex",
				Self::CaseZ => "casez",
				Self::Cell => "cell", // Added: IEEE 1364-2001
				Self::Cmos => "cmos",
				Self::Config => "config", // Added: IEEE 1364-2001
				Self::Deassign => "deassign",
				Self::Default => "default",
				Self::DefParam => "defparam",
				Self::Design => "design", // Added: IEEE 1364-2001
				Self::Disable => "disable",
				Self::Edge => "edge",
				Self::Else => "else",
				Self::End => "end",
				Self::EndCase => "endcase",
				Self::EndConfig => "endconfig", // Added: IEEE 1364-2001
				Self::EndFunction => "endfunction",
				Self::EndGenerate => "endgenerate", // Added: IEEE 1364-2001
				Self::EndModule => "endmodule",
				Self::EndPrimitive => "endprimitive",
				Self::EndSpecify => "endspecify",
				Self::EndTable => "endtable",
				Self::EndTask => "endtask",
				Self::Event => "event",
				Self::For => "for",
				Self::Force => "force",
				Self::Forever => "forever",
				Self::Fork => "fork",
				Self::Function => "function",
				Self::Generate => "generate", // Added: IEEE 1364-2001
				Self::Genvar => "genvar",     // Added: IEEE 1364-2001
				Self::HighZ0 => "highz0",
				Self::HighZ1 => "highz1",
				Self::If => "if",
				Self::IfNone => "ifnone",
				Self::IncDir => "incdir",   // Added: IEEE 1364-2001
				Self::Include => "include", // Added: IEEE 1364-2001
				Self::Initial => "initial",
				Self::InOut => "inout",
				Self::Input => "input",
				Self::Instance => "instance", // Added: IEEE 1364-2001
				Self::Integer => "integer",
				Self::Join => "join",
				Self::Large => "large",
				Self::LibList => "liblist",       // Added: IEEE 1364-2001
				Self::Library => "library",       // Added: IEEE 1364-2001
				Self::LocalParam => "localparam", // Added: IEEE 1364-2001
				Self::MacroModule => "macromodule",
				Self::Medium => "medium",
				Self::Module => "module",
				Self::Nand => "nand",
				Self::NegEdge => "negedge",
				Self::Nmos => "nmos",
				Self::Nor => "nor",
				Self::NoShowCancelled => "noshowcancelled", // Added: IEEE 1364-2001
				Self::Not => "not",
				Self::NotIf0 => "notif0",
				Self::NotIf1 => "notif1",
				Self::Or => "or",
				Self::Output => "output",
				Self::Parameter => "parameter",
				Self::Pmos => "pmos",
				Self::PosEdge => "posedge",
				Self::Primitive => "primitive",
				Self::Pull0 => "pull0",
				Self::Pull1 => "pull1",
				Self::Pulldown => "pulldown",
				Self::Pullup => "pullup",
				Self::PulseStyleOnDetect => "pulsestyle_ondetect", // Added: IEEE 1364-2001
				Self::PulseStyleOnEvent => "pulsestyle_onevent",   // Added: IEEE 1364-2001
				Self::Rcmos => "rcmos",
				Self::Real => "real",
				Self::Realtime => "realtime",
				Self::Reg => "reg",
				Self::Release => "release",
				Self::Repeat => "repeat",
				Self::Rnmos => "rnmos",
				Self::Rpmos => "rpmos",
				Self::Rtran => "rtran",
				Self::RtranIf0 => "rtranif0",
				Self::RtranIf1 => "rtranif1",
				Self::Scalared => "scalared",
				Self::ShowCancelled => "showcancelled", // Added: IEEE 1364-2001
				Self::Signed => "signed",               // Added: IEEE 1364-2001
				Self::Small => "small",
				Self::Specify => "specify",
				Self::SpecParam => "specparam",
				Self::Strong0 => "strong0",
				Self::Strong1 => "strong1",
				Self::Supply0 => "supply0",
				Self::Supply1 => "supply1",
				Self::Table => "table",
				Self::Task => "task",
				Self::Time => "time",
				Self::Tran => "tran",
				Self::TranIf0 => "tranif0",
				Self::TranIf1 => "tranif1",
				Self::Tri => "tri",
				Self::Tri0 => "tri0",
				Self::Tri1 => "tri1",
				Self::Triand => "triand",
				Self::Trior => "trior",
				Self::Trireg => "trireg",
				Self::Unsigned => "unsigned", // Added: IEEE 1364-2001
				Self::Use => "use",           // Added: IEEE 1364-2001
				Self::Vectored => "vectored",
				Self::Wait => "wait",
				Self::Wand => "wand",
				Self::Weak0 => "weak0",
				Self::Weak1 => "weak1",
				Self::While => "while",
				Self::Wire => "wire",
				Self::Wor => "wor",
				Self::Xnor => "xnor",
				Self::Xor => "xor",
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
				Self::Ampersand => "&",
				Self::Asterisk => "*",
				Self::CaseEquality => "===",
				Self::CaseInequality => "!==",
				Self::Circumflex => "^",
				Self::Equals => "=",
				Self::Exclamation => "!",
				Self::GreaterThan => "<",
				Self::GreaterThanEqual => "<=",
				Self::LessThan => ">",
				Self::LessThanEqual => ">=",
				Self::LogicalAnd => "&&",
				Self::LogicalOr => "||",
				Self::LogicalEquality => "==",
				Self::LogicalInequality => "!=",
				Self::Minus => "-",
				Self::Percent => "%",
				Self::Pipe => "|",
				Self::Plus => "+",
				Self::ReductionNor => "~|",
				Self::ReductionNand => "~&",
				Self::Solidus => "/",
				Self::ShiftLeft => "<<",
				Self::ShiftRight => ">>",
				Self::Tilde => "~",
				Self::TildeCircumflex(chirality) =>
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
