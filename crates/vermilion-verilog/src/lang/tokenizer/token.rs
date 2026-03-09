// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::AtomicByteTendril;

use crate::{
	LanguageStd,
	lang::{keywords::Keyword, types::SystemFunc},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
	BaseSpecifier {
		specifier: BaseSpecifier,
		uppercase: bool,
		signed:    bool,
	},
	Comment(Comment),
	CompilerDirective(CompilerDirective),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(AtomicByteTendril, LanguageStd),
	Control(Control),
	Identifier(AtomicByteTendril),
	Invalid(Option<AtomicByteTendril>),
	Keyword(Keyword),
	Newline(AtomicByteTendril),
	Number(AtomicByteTendril),
	Operator(Operator),
	Real {
		value:    f64,
		exponent: Option<AtomicByteTendril>,
	},
	String(AtomicByteTendril),
	SystemFunc(SystemFunc),
	TextMacro(TextMacro),
	TripleQuotedString(AtomicByteTendril), // Added: IEEE 1800-2023
	UnsignedNumber(AtomicByteTendril),
	Whitespace(AtomicByteTendril),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseSpecifier {
	Binary,
	Decimal,
	Hexadecimal,
	Octal,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comment {
	Invalid(AtomicByteTendril),
	MultiLine(AtomicByteTendril),
	SingleLine(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompilerDirective {
	Arg(AtomicByteTendril),
	Name(Directive),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Control {
	Apostrophe, // Added: IEEE 1800-2005
	At,
	AttributeClose, // Added: IEEE 1364-2001
	AttributeOpen,  // Added: IEEE 1364-2001
	BraceClose,
	BraceOpen,
	BracketClose,
	BracketOpen,
	Colon,
	Comma,
	Dollar,
	Dot,
	Grave,
	Octothorp,
	ParenClose,
	ParenOpen,
	Question,
	ReverseSolidus,
	Semicolon,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Directive {
	BeginKeywords, // Added: IEEE 1364-2005
	CellDefine,
	DefaultDecayTime,
	DefaultDiscipline, // Added: Verilog-AMS 2009
	DefaultNetType,
	DefaultTransition, // Added: Verilog-AMS 2009
	DefaultTriRegStrength,
	Define,
	DelayModeDistributed,
	DelayModePath,
	DelayModeUnit,
	DelayModeZero,
	Else,
	ElsIf, // Added: IEEE 1364-2001
	EndCellDefine,
	EndIf,
	EndKeywords, // Added: IEEE 1364-2005
	IfDef,
	IfNotDef, // Added: IEEE 1364-2001
	Include,
	Line, // Added: IEEE 1364-2001
	NoUnconnectedDrive,
	Pragma, // Added: IEEE 1364-2005
	ResetAll,
	TimeScale,
	UnconnectedDrive,
	Undef,
	UndefineAll, // Added: IEEE 1800-2009
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
	AbsTolerance, // Added: IEEE 1800-2023
	AddEquals,    // Added: IEEE 1800-2005
	Ampersand,
	AndEquals,           // Added: IEEE 1800-2005
	ArithmeticShl,       // Added: IEEE 1364-2001
	ArithmeticShlEquals, // Added: IEEE 1800-2005
	ArithmeticShr,       // Added: IEEE 1364-2001
	ArithmeticShrEquals, // Added: IEEE 1800-2005
	Asterisk,
	BranchContribution, // Added: Verilog-AMS 2009
	CaseEquality,
	CaseInequality,
	Circumflex,
	ClassScopeResolution, // Added: IEEE 1800-2005
	CycleDelay,           // Added: IEEE 1800-2005
	Decrement,            // Added: IEEE 1800-2005
	DivEquals,            // Added: IEEE 1800-2005
	Equals,
	Equivalence, // Added: IEEE 1800-2009
	EventTrigger,
	EventTriggerNb, // Added: IEEE 1800-2005
	Exclamation,
	FollowedByNonOverlapped, // Added: IEEE 1800-2009
	FollowedByOverlapped,    // Added: IEEE 1800-2009
	FullConnection,
	GreaterThan,
	GreaterThanEqual,
	Increment,      // Added: IEEE 1800-2005
	IndexedPartNeg, // Added: IEEE 1364-2001
	IndexedPartPos, // Added: IEEE 1364-2001
	LessThan,
	LessThanEqual,
	LogicalAnd,
	LogicalEquality,
	LogicalInequality,
	LogicalOr,
	Minus,
	MulEquals, // Added: IEEE 1800-2005
	OrEquals,  // Added: IEEE 1800-2005
	ParallelConnection,
	Percent,
	Pipe,
	Plus,
	Pow,                // Added: IEEE 1364-2001
	PropImplNonOverlap, // Added: IEEE 1800-2005
	PropImplOverlap,    // Added: IEEE 1800-2005
	ReductionNand,
	ReductionNor,
	RelTolerance, // Added: IEEE 1800-2023
	RemEquals,    // Added: IEEE 1800-2005
	ShiftLeft,
	ShiftLeftEquals, // Added: IEEE 1800-2005
	ShiftRight,
	ShiftRightEquals, // Added: IEEE 1800-2005
	Solidus,
	SubEquals, // Added: IEEE 1800-2005
	Tilde,
	TildeCircumflex(bool), // NOTE(aki): this is the chirality of the `^~`/`~^` operator
	TripleAnd,
	WeightAssignDist, // Added: IEEE 1800-2005
	WeightAssignUnit, // Added: IEEE 1800-2005
	Wildcard,         // Added: IEEE 1800-2005
	WildcardEqual,    // Added: IEEE 1800-2005
	WildcardExport,   // Added: IEEE 1800-2009
	WildcardNotEqual, // Added: IEEE 1800-2005
	XorEquals,        // Added: IEEE 1800-2005
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextMacro {
	DunderFile, // Added: Verilog-AMS 2023 & IEEE 1800-2009
	DunderLine, // Added: Verilog-AMS 2023 & IEEE 1800-2009
	Other(AtomicByteTendril),
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BaseSpecifier { specifier, uppercase, signed } => {
				write!(
					f,
					"BaseSpecifier({}, uppercase: {}, signed: {})",
					specifier, uppercase, signed
				)
			},
			Self::Comment(comment) => comment.fmt(f),
			Self::CompilerDirective(compiler_directive) => compiler_directive.fmt(f),
			Self::ContextuallyInvalid(tendril, std) => {
				write!(
					f,
					"ContextuallyInvalid(\"{}\", {:?})",
					unsafe { str::from_utf8_unchecked(tendril) },
					std
				)
			},
			Self::Control(control) => control.fmt(f),
			Self::Identifier(tendril) => write!(f, "Identifier({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Invalid(_tendril) => todo!(),
			Self::Keyword(keyword) => keyword.fmt(f),
			Self::Newline(tendril) => write!(f, "Newline({})", tendril.len()),
			Self::Number(tendril) => write!(f, "Number({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Operator(operator) => operator.fmt(f),
			Self::Real { value, exponent } => {
				write!(f, "RealNumber(value: {}, exp: {:?})", value, exponent)
			},
			Self::String(tendril) => write!(f, "String(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::SystemFunc(sysfunc) => sysfunc.fmt(f),
			Self::TextMacro(text_macro) => write!(f, "TextMacro(\"{}\")", text_macro),
			Self::TripleQuotedString(tendril) => write!(f, "TripleQuotedString(\"{}\")", unsafe {
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

impl Display for BaseSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"BaseSpecifier({})",
			match self {
				Self::Binary => "'b",
				Self::Decimal => "'d",
				Self::Hexadecimal => "'h",
				Self::Octal => "'o",
			}
		)
	}
}

impl Display for Comment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Invalid(tendril) => write!(f, "InvalidComment(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::MultiLine(tendril) => write!(f, "MultiLineComment(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::SingleLine(tendril) => write!(f, "SingleLineComment(\"// {}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Display for CompilerDirective {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Arg(tendril) => write!(f, "CompilerDirectiveArg({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Name(directive) => write!(f, "CompilerDirective({})", directive),
		}
	}
}

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Control({})",
			match self {
				Self::Apostrophe => "'", // Added: IEEE 1800-2005
				Self::At => "@",
				Self::AttributeClose => "*)", // Added: IEEE 1364-2001
				Self::AttributeOpen => "(*",  // Added: IEEE 1364-2001
				Self::BraceClose => "}",
				Self::BraceOpen => "{",
				Self::BracketClose => "]",
				Self::BracketOpen => "[",
				Self::Colon => ":",
				Self::Comma => ",",
				Self::Dollar => "$",
				Self::Dot => ".",
				Self::Grave => "`",
				Self::Octothorp => "#",
				Self::ParenClose => ")",
				Self::ParenOpen => "(",
				Self::Question => "?",
				Self::ReverseSolidus => "\\",
				Self::Semicolon => ";",
			}
		)
	}
}

impl Display for Directive {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Directive({})",
			match self {
				Self::BeginKeywords => "begin_keywords", // Added: IEEE 1364-2005
				Self::CellDefine => "celldefine",
				Self::DefaultDecayTime => "default_decay_time",
				Self::DefaultDiscipline => "default_discipline", // Added: Verilog-AMS 2009
				Self::DefaultNetType => "default_nettype",
				Self::DefaultTransition => "default_transition", // Added: Verilog-AMS 2009
				Self::DefaultTriRegStrength => "default_trireg_strength",
				Self::Define => "define",
				Self::DelayModeDistributed => "delay_mode_distributed",
				Self::DelayModePath => "delay_mode_path",
				Self::DelayModeUnit => "delay_mode_unit",
				Self::DelayModeZero => "delay_mode_zero",
				Self::Else => "else",
				Self::ElsIf => "elsif", // Added: IEEE 1364-2001
				Self::EndCellDefine => "endcelldefine",
				Self::EndIf => "endif",
				Self::EndKeywords => "end_keywords", // Added: IEEE 1364-2005
				Self::IfDef => "ifdef",
				Self::IfNotDef => "ifndef", // Added: IEEE 1364-2001
				Self::Include => "include",
				Self::Line => "line", // Added: IEEE 1364-2001
				Self::NoUnconnectedDrive => "nounconnected_drive",
				Self::Pragma => "pragma", // Added: IEEE 1364-2005
				Self::ResetAll => "resetall",
				Self::TimeScale => "timescale",
				Self::UnconnectedDrive => "unconnected_drive",
				Self::Undef => "undef",
				Self::UndefineAll => "undefineall", // Added: IEEE 1800-2009
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
				Self::AddEquals => "+=",     // Added: IEEE 1800-2005
				Self::AbsTolerance => "+/-", // Added: IEEE 1800-2023
				Self::Ampersand => "&",
				Self::AndEquals => "&=",             // Added: IEEE 1800-2005
				Self::ArithmeticShl => "<<<",        // Added: IEEE 1364-2001
				Self::ArithmeticShlEquals => "<<<=", // Added: IEEE 1800-2005
				Self::ArithmeticShr => ">>>",        // Added: IEEE 1364-2001
				Self::ArithmeticShrEquals => ">>>=", // Added: IEEE 1800-2005
				Self::Asterisk => "*",
				Self::BranchContribution => "<+", // Added: Verilog-AMS 2009
				Self::CaseEquality => "===",
				Self::CaseInequality => "!==",
				Self::Circumflex => "^",
				Self::ClassScopeResolution => "::", // Added: IEEE 1800-2005
				Self::CycleDelay => "##",           // Added: IEEE 1800-2005
				Self::Decrement => "--",            // Added: IEEE 1800-2005
				Self::DivEquals => "/=",            // Added: IEEE 1800-2005
				Self::Equals => "=",
				Self::Equivalence => "<->", // Added: IEEE 1800-2009
				Self::EventTrigger => "->",
				Self::EventTriggerNb => "->>", // Added: IEEE 1800-2005
				Self::Exclamation => "!",
				Self::FollowedByNonOverlapped => "#=#", // Added: IEEE 1800-2009
				Self::FollowedByOverlapped => "#-#",    // Added: IEEE 1800-2009
				Self::FullConnection => "*>",
				Self::GreaterThan => "<",
				Self::GreaterThanEqual => "<=",
				Self::Increment => "++",      // Added: IEEE 1800-2005
				Self::IndexedPartNeg => "-:", // Added: IEEE 1364-2001
				Self::IndexedPartPos => "+:", // Added: IEEE 1364-2001
				Self::LessThan => ">",
				Self::LessThanEqual => ">=",
				Self::LogicalAnd => "&&",
				Self::LogicalEquality => "==",
				Self::LogicalInequality => "!=",
				Self::LogicalOr => "||",
				Self::Minus => "-",
				Self::MulEquals => "*=", // Added: IEEE 1800-2005
				Self::OrEquals => "|=",  // Added: IEEE 1800-2005
				Self::ParallelConnection => "=>",
				Self::Percent => "%",
				Self::Pipe => "|",
				Self::Plus => "+",
				Self::Pow => "**",                 // Added: IEEE 1364-2001
				Self::PropImplNonOverlap => "|=>", // Added: IEEE 1800-2005
				Self::PropImplOverlap => "|->",    // Added: IEEE 1800-2005
				Self::ReductionNand => "~&",
				Self::ReductionNor => "~|",
				Self::RelTolerance => "+%-", // Added: IEEE 1800-2023
				Self::RemEquals => "%=",     // Added: IEEE 1800-2005
				Self::ShiftLeft => "<<",
				Self::ShiftLeftEquals => "<<=", // Added: IEEE 1800-2005
				Self::ShiftRight => ">>",
				Self::ShiftRightEquals => ">>=", // Added: IEEE 1800-2005
				Self::Solidus => "/",
				Self::SubEquals => "-=", // Added: IEEE 1800-2005
				Self::Tilde => "~",
				Self::TildeCircumflex(chirality) =>
					if *chirality {
						"^~"
					} else {
						"~^"
					},
				Self::TripleAnd => "&&&",
				Self::WeightAssignDist => ":/",  // Added: IEEE 1800-2005
				Self::WeightAssignUnit => ":=",  // Added: IEEE 1800-2005
				Self::Wildcard => ".*",          // Added: IEEE 1800-2005
				Self::WildcardEqual => "==?",    // Added: IEEE 1800-2005
				Self::WildcardExport => "*::*",  // Added: IEEE 1800-2009
				Self::WildcardNotEqual => "!=?", // Added: IEEE 1800-2005
				Self::XorEquals => "^=",         // Added: IEEE 1800-2005
			}
		)
	}
}

impl Display for TextMacro {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"TextMacro({})",
			match self {
				Self::DunderFile => "__FILE__", // Added: Verilog-AMS 2023 & IEEE 1800-2009
				Self::DunderLine => "__LINE__", // Added: Verilog-AMS 2023 & IEEE 1800-2009
				Self::Other(tendril) => unsafe { str::from_utf8_unchecked(tendril) },
			}
		)
	}
}

impl Default for Token {
	fn default() -> Self {
		Self::Invalid(None)
	}
}
