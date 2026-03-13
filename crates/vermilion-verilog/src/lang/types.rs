// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::AtomicByteTendril;

use crate::lang::{
	directives::BuiltinDirective, sysfuncs::BuiltinSysFunc, text_macros::BuiltinTextMacro,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NetType {
	Supply0,
	Supply1,
	Tri,
	Tri0,
	Tri1,
	TriAnd,
	TriOr,
	Wire,
	WireAnd,
	WireOr,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LowStrength {
	Supply,
	Strong,
	Pull,
	Weak,
	HighZ,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HighStrength {
	Supply,
	Strong,
	Pull,
	Weak,
	HighZ,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DriveStrength {
	HighLow(HighStrength, LowStrength),
	LowHigh(LowStrength, HighStrength),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChargeStrength {
	Small,
	#[default] // IEEE 1364-1995 § 3.4.1
	Medium,
	Large,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EdgeDescriptor {
	PosEdge,
	NegEdge,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EdgeIdentifier {
	PosEdge,
	NegEdge,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimeUnit {
	Seconds,
	Milliseconds,
	Microseconds,
	Nanoseconds,
	Picoseconds,
	Femtoseconds,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BaseSpecifier {
	Binary,
	Decimal,
	Hexadecimal,
	Octal,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BasedLiteralSpecifier {
	specifier: BaseSpecifier,
	uppercase: bool,
	signed:    bool,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Comment {
	Invalid(AtomicByteTendril),
	MultiLine(AtomicByteTendril),
	SingleLine(AtomicByteTendril),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompilerDirective {
	Builtin(BuiltinDirective),
	TextMacro(TextMacro),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Identifier {
	Simple(AtomicByteTendril),
	Escaped(AtomicByteTendril),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SingleQuotedString(AtomicByteTendril);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SystemFunc {
	Builtin(BuiltinSysFunc),
	Other(AtomicByteTendril),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TextMacro {
	Builtin(BuiltinTextMacro),
	Other(AtomicByteTendril),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TripleQuotedString(AtomicByteTendril); // Added: IEEE 1800-2023

impl BasedLiteralSpecifier {
	pub fn new(specifier: BaseSpecifier, uppercase: bool, signed: bool) -> Self {
		Self { specifier, uppercase, signed }
	}
}

impl SingleQuotedString {
	pub fn new(tendril: AtomicByteTendril) -> Self {
		Self(tendril)
	}
}

impl TripleQuotedString {
	pub fn new(tendril: AtomicByteTendril) -> Self {
		Self(tendril)
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
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

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for BasedLiteralSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"BasedLiteralSpecifier({}, uppercase: {}, signed: {})",
			self.specifier, self.uppercase, self.signed
		)
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
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

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for CompilerDirective {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::TextMacro(text_macro) => write!(f, "CompilerDirective({})", text_macro),
			Self::Builtin(directive) => write!(f, "CompilerDirective({})", directive),
		}
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
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

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for Identifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Simple(tendril) => write!(f, "Identifier({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Escaped(tendril) => write!(f, "Identifier(\\{})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
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

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for SingleQuotedString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "SingleQuotedString(\"{}\")", unsafe {
			str::from_utf8_unchecked(&self.0)
		})
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for SystemFunc {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Builtin(builtin) => write!(f, "SystemFunc({})", builtin),
			Self::Other(tendril) => write!(f, "SystemFunc(Other(${}))", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for TextMacro {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Builtin(builtin) => write!(f, "TextMacro({})", builtin),
			Self::Other(tendril) => write!(f, "TextMacro(Other({}))", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for TripleQuotedString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "TripleQuotedString(\"{}\")", unsafe {
			str::from_utf8_unchecked(&self.0)
		})
	}
}
