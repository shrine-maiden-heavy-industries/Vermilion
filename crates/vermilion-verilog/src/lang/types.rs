// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::AtomicByteTendril;

use crate::lang::{directives::Directive, sysfuncs::BuiltinSysFunc};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemFunc {
	Builtin(BuiltinSysFunc),
	Other(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextMacro {
	DunderFile, // Added: Verilog-AMS 2023 & IEEE 1800-2009
	DunderLine, // Added: Verilog-AMS 2023 & IEEE 1800-2009
	Other(AtomicByteTendril),
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

impl Display for SystemFunc {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SystemFunc::Builtin(builtin) => write!(f, "SystemFunc({})", builtin),
			SystemFunc::Other(tendril) => write!(f, "SystemFunc(Other(${}))", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
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
