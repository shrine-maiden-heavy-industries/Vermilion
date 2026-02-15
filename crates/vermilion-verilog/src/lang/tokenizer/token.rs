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
	Alias, // Added: IEEE 1800-2005
	Always,
	AlwaysComb,  // Added: IEEE 1800-2005
	AlwaysFF,    // Added: IEEE 1800-2005
	AlwaysLatch, // Added: IEEE 1800-2005
	And,
	Assert, // Added: IEEE 1800-2005
	Assign,
	Assume,    // Added: IEEE 1800-2005
	Automatic, // Added: IEEE 1364-2001
	Before,    // Added: IEEE 1800-2005
	Begin,
	Bind,   // Added: IEEE 1800-2005
	Bins,   // Added: IEEE 1800-2005
	BinsOf, // Added: IEEE 1800-2005
	Bit,    // Added: IEEE 1800-2005
	Break,  // Added: IEEE 1800-2005
	Buf,
	BufIf0,
	BufIf1,
	Byte, // Added: IEEE 1800-2005
	Case,
	CaseX,
	CaseZ,
	Cell,     // Added: IEEE 1364-2001
	CHandle,  // Added: IEEE 1800-2005
	Class,    // Added: IEEE 1800-2005
	Clocking, // Added: IEEE 1800-2005
	Cmos,
	Config,     // Added: IEEE 1364-2001
	Const,      // Added: IEEE 1800-2005
	Constraint, // Added: IEEE 1800-2005
	Context,    // Added: IEEE 1800-2005
	Continue,   // Added: IEEE 1800-2005
	Cover,      // Added: IEEE 1800-2005
	CoverGroup, // Added: IEEE 1800-2005
	CoverPoint, // Added: IEEE 1800-2005
	Cross,      // Added: IEEE 1800-2005
	Deassign,
	Default,
	DefParam,
	Design, // Added: IEEE 1364-2001
	Disable,
	Dist, // Added: IEEE 1800-2005
	Do,   // Added: IEEE 1800-2005
	Edge,
	Else,
	End,
	EndCase,
	EndClass,    // Added: IEEE 1800-2005
	EndClocking, // Added: IEEE 1800-2005
	EndConfig,   // Added: IEEE 1364-2001
	EndFunction,
	EndGenerate,  // Added: IEEE 1364-2001
	EndGroup,     // Added: IEEE 1800-2005
	EndInterface, // Added: IEEE 1800-2005
	EndModule,
	EndPackage, // Added: IEEE 1800-2005
	EndPrimitive,
	EndProgram,  // Added: IEEE 1800-2005
	EndProperty, // Added: IEEE 1800-2005
	EndSequence, // Added: IEEE 1800-2005
	EndSpecify,
	EndTable,
	EndTask,
	Enum, // Added: IEEE 1800-2005
	Event,
	Expect,     // Added: IEEE 1800-2005
	Export,     // Added: IEEE 1800-2005
	Extends,    // Added: IEEE 1800-2005
	Extern,     // Added: IEEE 1800-2005
	Final,      // Added: IEEE 1800-2005
	FirstMatch, // Added: IEEE 1800-2005
	For,
	Force,
	ForEach, // Added: IEEE 1800-2005
	Forever,
	Fork,
	ForkJoin, // Added: IEEE 1800-2005
	Function,
	Generate, // Added: IEEE 1364-2001
	Genvar,   // Added: IEEE 1364-2001
	HighZ0,
	HighZ1,
	If,
	Iff, // Added: IEEE 1800-2005
	IfNone,
	IgnoreBins,  // Added: IEEE 1800-2005
	IllegalBins, // Added: IEEE 1800-2005
	Import,      // Added: IEEE 1800-2005
	IncDir,      // Added: IEEE 1364-2001
	Include,     // Added: IEEE 1364-2001
	Initial,
	InOut,
	Input,
	Inside,   // Added: IEEE 1800-2005
	Instance, // Added: IEEE 1364-2001
	Int,      // Added: IEEE 1800-2005
	Integer,
	Interface, // Added: IEEE 1800-2005
	Intersect, // Added: IEEE 1800-2005
	Join,
	JoinAny,  // Added: IEEE 1800-2005
	JoinNone, // Added: IEEE 1800-2005
	Large,
	LibList,    // Added: IEEE 1364-2001
	Library,    // Added: IEEE 1364-2001
	Local,      // Added: IEEE 1800-2005
	LocalParam, // Added: IEEE 1364-2001
	Logic,      // Added: IEEE 1800-2005
	LongInt,    // Added: IEEE 1800-2005
	MacroModule,
	Matches, // Added: IEEE 1800-2005
	Medium,
	ModPort, // Added: IEEE 1800-2005
	Module,
	Nand,
	NegEdge,
	New, // Added: IEEE 1800-2005
	Nmos,
	Nor,
	NoShowCancelled, // Added: IEEE 1364-2001
	Not,
	NotIf0,
	NotIf1,
	Null, // Added: IEEE 1800-2005
	Or,
	Output,
	Package, // Added: IEEE 1800-2005
	Packed,  // Added: IEEE 1800-2005
	Parameter,
	Pmos,
	PosEdge,
	Primitive,
	Priority,  // Added: IEEE 1800-2005
	Program,   // Added: IEEE 1800-2005
	Property,  // Added: IEEE 1800-2005
	Protected, // Added: IEEE 1800-2005
	Pull0,
	Pull1,
	Pulldown,
	Pullup,
	PulseStyleOnDetect, // Added: IEEE 1364-2001
	PulseStyleOnEvent,  // Added: IEEE 1364-2001
	Pure,               // Added: IEEE 1800-2005
	Rand,               // Added: IEEE 1800-2005
	RandC,              // Added: IEEE 1800-2005
	RandCase,           // Added: IEEE 1800-2005
	RandSequence,       // Added: IEEE 1800-2005
	Rcmos,
	Real,
	Realtime,
	Ref, // Added: IEEE 1800-2005
	Reg,
	Release,
	Repeat,
	Return, // Added: IEEE 1800-2005
	Rnmos,
	Rpmos,
	Rtran,
	RtranIf0,
	RtranIf1,
	Scalared,
	Sequence,      // Added: IEEE 1800-2005
	ShortInt,      // Added: IEEE 1800-2005
	ShortReal,     // Added: IEEE 1800-2005
	ShowCancelled, // Added: IEEE 1364-2001
	Signed,        // Added: IEEE 1364-2001
	Small,
	Solve, // Added: IEEE 1800-2005
	Specify,
	SpecParam,
	Static, // Added: IEEE 1800-2005
	String, // Added: IEEE 1800-2005
	Strong0,
	Strong1,
	Struct, // Added: IEEE 1800-2005
	Super,  // Added: IEEE 1800-2005
	Supply0,
	Supply1,
	Table,
	Tagged, // Added: IEEE 1800-2005
	Task,
	This,       // Added: IEEE 1800-2005
	Throughout, // Added: IEEE 1800-2005
	Time,
	TimePrecision, // Added: IEEE 1800-2005
	TimeUnit,      // Added: IEEE 1800-2005
	Tran,
	TranIf0,
	TranIf1,
	Tri,
	Tri0,
	Tri1,
	Triand,
	Trior,
	Trireg,
	Type,     // Added: IEEE 1800-2005
	TypeDef,  // Added: IEEE 1800-2005
	Union,    // Added: IEEE 1800-2005
	Unique,   // Added: IEEE 1800-2005
	Unsigned, // Added: IEEE 1364-2001
	Use,      // Added: IEEE 1364-2001
	Uwire,    // Added: IEEE 1364-2005
	Var,      // Added: IEEE 1800-2005
	Vectored,
	Virtual, // Added: IEEE 1800-2005
	Void,    // Added: IEEE 1800-2005
	Wait,
	WaitOrder, // Added: IEEE 1800-2005
	Wand,
	Weak0,
	Weak1,
	While,
	Wildcard, // Added: IEEE 1800-2005
	Wire,
	With,   // Added: IEEE 1800-2005
	Within, // Added: IEEE 1800-2005
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
				Self::Alias => "alias", // Added: IEEE 1800-2005
				Self::Always => "always",
				Self::AlwaysComb => "always_comb",   // Added: IEEE 1800-2005
				Self::AlwaysFF => "always_ff",       // Added: IEEE 1800-2005
				Self::AlwaysLatch => "always_latch", // Added: IEEE 1800-2005
				Self::And => "and",
				Self::Assert => "assert", // Added: IEEE 1800-2005
				Self::Assign => "assign",
				Self::Assume => "assume",       // Added: IEEE 1800-2005
				Self::Automatic => "automatic", // Added: IEEE 1364-2001
				Self::Before => "before",       // Added: IEEE 1800-2005
				Self::Begin => "begin",
				Self::Bind => "bind",     // Added: IEEE 1800-2005
				Self::Bins => "bins",     // Added: IEEE 1800-2005
				Self::BinsOf => "binsof", // Added: IEEE 1800-2005
				Self::Bit => "bit",       // Added: IEEE 1800-2005
				Self::Break => "break",   // Added: IEEE 1800-2005
				Self::Buf => "buf",
				Self::BufIf0 => "bufif0",
				Self::BufIf1 => "bufif1",
				Self::Byte => "byte", // Added: IEEE 1800-2005
				Self::Case => "case",
				Self::CaseX => "casex",
				Self::CaseZ => "casez",
				Self::Cell => "cell",         // Added: IEEE 1364-2001
				Self::CHandle => "chandle",   // Added: IEEE 1800-2005
				Self::Class => "class",       // Added: IEEE 1800-2005
				Self::Clocking => "clocking", // Added: IEEE 1800-2005
				Self::Cmos => "cmos",
				Self::Config => "config",         // Added: IEEE 1364-2001
				Self::Const => "const",           // Added: IEEE 1800-2005
				Self::Constraint => "constraint", // Added: IEEE 1800-2005
				Self::Context => "context",       // Added: IEEE 1800-2005
				Self::Continue => "continue",     // Added: IEEE 1800-2005
				Self::Cover => "cover",           // Added: IEEE 1800-2005
				Self::CoverGroup => "covergroup", // Added: IEEE 1800-2005
				Self::CoverPoint => "coverpoint", // Added: IEEE 1800-2005
				Self::Cross => "cross",           // Added: IEEE 1800-2005
				Self::Deassign => "deassign",
				Self::Default => "default",
				Self::DefParam => "defparam",
				Self::Design => "design", // Added: IEEE 1364-2001
				Self::Disable => "disable",
				Self::Dist => "dist", // Added: IEEE 1800-2005
				Self::Do => "do",     // Added: IEEE 1800-2005
				Self::Edge => "edge",
				Self::Else => "else",
				Self::End => "end",
				Self::EndCase => "endcase",
				Self::EndClass => "endclass",       // Added: IEEE 1800-2005
				Self::EndClocking => "endclocking", // Added: IEEE 1800-2005
				Self::EndConfig => "endconfig",     // Added: IEEE 1364-2001
				Self::EndFunction => "endfunction",
				Self::EndGenerate => "endgenerate",   // Added: IEEE 1364-2001
				Self::EndGroup => "endgroup",         // Added: IEEE 1800-2005
				Self::EndInterface => "endinterface", // Added: IEEE 1800-2005
				Self::EndModule => "endmodule",
				Self::EndPackage => "endpackage", // Added: IEEE 1800-2005
				Self::EndPrimitive => "endprimitive",
				Self::EndProgram => "endprogram",   // Added: IEEE 1800-2005
				Self::EndProperty => "endproperty", // Added: IEEE 1800-2005
				Self::EndSequence => "endsequence", // Added: IEEE 1800-2005
				Self::EndSpecify => "endspecify",
				Self::EndTable => "endtable",
				Self::EndTask => "endtask",
				Self::Enum => "enum", // Added: IEEE 1800-2005
				Self::Event => "event",
				Self::Expect => "expect",          // Added: IEEE 1800-2005
				Self::Export => "export",          // Added: IEEE 1800-2005
				Self::Extends => "extends",        // Added: IEEE 1800-2005
				Self::Extern => "extern",          // Added: IEEE 1800-2005
				Self::Final => "final",            // Added: IEEE 1800-2005
				Self::FirstMatch => "first_match", // Added: IEEE 1800-2005
				Self::For => "for",
				Self::Force => "force",
				Self::ForEach => "foreach", // Added: IEEE 1800-2005
				Self::Forever => "forever",
				Self::Fork => "fork",
				Self::ForkJoin => "forkjoin", // Added: IEEE 1800-2005
				Self::Function => "function",
				Self::Generate => "generate", // Added: IEEE 1364-2001
				Self::Genvar => "genvar",     // Added: IEEE 1364-2001
				Self::HighZ0 => "highz0",
				Self::HighZ1 => "highz1",
				Self::If => "if",
				Self::Iff => "iff", // Added: IEEE 1800-2005
				Self::IfNone => "ifnone",
				Self::IgnoreBins => "ignore_bins",   // Added: IEEE 1800-2005
				Self::IllegalBins => "illegal_bins", // Added: IEEE 1800-2005
				Self::Import => "import",            // Added: IEEE 1800-2005
				Self::IncDir => "incdir",            // Added: IEEE 1364-2001
				Self::Include => "include",          // Added: IEEE 1364-2001
				Self::Initial => "initial",
				Self::InOut => "inout",
				Self::Input => "input",
				Self::Inside => "inside",     // Added: IEEE 1800-2005
				Self::Instance => "instance", // Added: IEEE 1364-2001
				Self::Int => "int",           // Added: IEEE 1800-2005
				Self::Integer => "integer",
				Self::Interface => "interface", // Added: IEEE 1800-2005
				Self::Intersect => "intersect", // Added: IEEE 1800-2005
				Self::Join => "join",
				Self::JoinAny => "join_any",   // Added: IEEE 1800-2005
				Self::JoinNone => "join_none", // Added: IEEE 1800-2005
				Self::Large => "large",
				Self::LibList => "liblist",       // Added: IEEE 1364-2001
				Self::Library => "library",       // Added: IEEE 1364-2001
				Self::Local => "local",           // Added: IEEE 1800-2005
				Self::LocalParam => "localparam", // Added: IEEE 1364-2001
				Self::Logic => "logic",           // Added: IEEE 1800-2005
				Self::LongInt => "longint",       // Added: IEEE 1800-2005
				Self::MacroModule => "macromodule",
				Self::Matches => "matches", // Added: IEEE 1800-2005
				Self::Medium => "medium",
				Self::ModPort => "modport", // Added: IEEE 1800-2005
				Self::Module => "module",
				Self::Nand => "nand",
				Self::NegEdge => "negedge",
				Self::New => "new", // Added: IEEE 1800-2005
				Self::Nmos => "nmos",
				Self::Nor => "nor",
				Self::NoShowCancelled => "noshowcancelled", // Added: IEEE 1364-2001
				Self::Not => "not",
				Self::NotIf0 => "notif0",
				Self::NotIf1 => "notif1",
				Self::Null => "null", // Added: IEEE 1800-2005
				Self::Or => "or",
				Self::Output => "output",
				Self::Package => "package", // Added: IEEE 1800-2005
				Self::Packed => "packed",   // Added: IEEE 1800-2005
				Self::Parameter => "parameter",
				Self::Pmos => "pmos",
				Self::PosEdge => "posedge",
				Self::Primitive => "primitive",
				Self::Priority => "priority",   // Added: IEEE 1800-2005
				Self::Program => "program",     // Added: IEEE 1800-2005
				Self::Property => "property",   // Added: IEEE 1800-2005
				Self::Protected => "protected", // Added: IEEE 1800-2005
				Self::Pull0 => "pull0",
				Self::Pull1 => "pull1",
				Self::Pulldown => "pulldown",
				Self::Pullup => "pullup",
				Self::PulseStyleOnDetect => "pulsestyle_ondetect", // Added: IEEE 1364-2001
				Self::PulseStyleOnEvent => "pulsestyle_onevent",   // Added: IEEE 1364-2001
				Self::Pure => "pure",                              // Added: IEEE 1800-2005
				Self::Rand => "rand",                              // Added: IEEE 1800-2005
				Self::RandC => "randc",                            // Added: IEEE 1800-2005
				Self::RandCase => "randcase",                      // Added: IEEE 1800-2005
				Self::RandSequence => "randsequence",              // Added: IEEE 1800-2005
				Self::Rcmos => "rcmos",
				Self::Real => "real",
				Self::Realtime => "realtime",
				Self::Ref => "ref", // Added: IEEE 1800-2005
				Self::Reg => "reg",
				Self::Release => "release",
				Self::Repeat => "repeat",
				Self::Return => "return", // Added: IEEE 1800-2005
				Self::Rnmos => "rnmos",
				Self::Rpmos => "rpmos",
				Self::Rtran => "rtran",
				Self::RtranIf0 => "rtranif0",
				Self::RtranIf1 => "rtranif1",
				Self::Scalared => "scalared",
				Self::Sequence => "sequence",           // Added: IEEE 1800-2005
				Self::ShortInt => "shortint",           // Added: IEEE 1800-2005
				Self::ShortReal => "shortreal",         // Added: IEEE 1800-2005
				Self::ShowCancelled => "showcancelled", // Added: IEEE 1364-2001
				Self::Signed => "signed",               // Added: IEEE 1364-2001
				Self::Small => "small",
				Self::Solve => "solve", // Added: IEEE 1800-2005
				Self::Specify => "specify",
				Self::SpecParam => "specparam",
				Self::Static => "static", // Added: IEEE 1800-2005
				Self::String => "string", // Added: IEEE 1800-2005
				Self::Strong0 => "strong0",
				Self::Strong1 => "strong1",
				Self::Struct => "struct", // Added: IEEE 1800-2005
				Self::Super => "super",   // Added: IEEE 1800-2005
				Self::Supply0 => "supply0",
				Self::Supply1 => "supply1",
				Self::Table => "table",
				Self::Tagged => "tagged", // Added: IEEE 1800-2005
				Self::Task => "task",
				Self::This => "this",             // Added: IEEE 1800-2005
				Self::Throughout => "throughout", // Added: IEEE 1800-2005
				Self::Time => "time",
				Self::TimePrecision => "timeprecision", // Added: IEEE 1800-2005
				Self::TimeUnit => "timeunit",           // Added: IEEE 1800-2005
				Self::Tran => "tran",
				Self::TranIf0 => "tranif0",
				Self::TranIf1 => "tranif1",
				Self::Tri => "tri",
				Self::Tri0 => "tri0",
				Self::Tri1 => "tri1",
				Self::Triand => "triand",
				Self::Trior => "trior",
				Self::Trireg => "trireg",
				Self::Type => "type",         // Added: IEEE 1800-2005
				Self::TypeDef => "typedef",   // Added: IEEE 1800-2005
				Self::Union => "union",       // Added: IEEE 1800-2005
				Self::Unique => "unique",     // Added: IEEE 1800-2005
				Self::Unsigned => "unsigned", // Added: IEEE 1364-2001
				Self::Use => "use",           // Added: IEEE 1364-2001
				Self::Uwire => "uwire",       // Added: IEEE 1364-2005
				Self::Var => "var",           // Added: IEEE 1800-2005
				Self::Vectored => "vectored",
				Self::Virtual => "virtual", // Added: IEEE 1800-2005
				Self::Void => "void",       // Added: IEEE 1800-2005
				Self::Wait => "wait",
				Self::WaitOrder => "wait_order", // Added: IEEE 1800-2005
				Self::Wand => "wand",
				Self::Weak0 => "weak0",
				Self::Weak1 => "weak1",
				Self::While => "while",
				Self::Wildcard => "wildcard", // Added: IEEE 1800-2005
				Self::Wire => "wire",
				Self::With => "with",     // Added: IEEE 1800-2005
				Self::Within => "within", // Added: IEEE 1800-2005
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
