// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::LanguageStd;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
	Comment(Comment),
	/// Hold the VHDL variant for when this token would become valid
	ContextuallyInvalid(AtomicByteTendril, LanguageStd),
	Control(Control),
	Identifier(AtomicByteTendril),
	Invalid(Option<AtomicByteTendril>),
	Keyword(Keyword),
	Newline(AtomicByteTendril),
	Number(AtomicByteTendril),
	Operator(Operator),
	String(AtomicByteTendril),
	Whitespace(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comment {
	Invalid(AtomicByteTendril),
	MultiLine(AtomicByteTendril),
	SingleLine(AtomicByteTendril),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Control {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
	Abs,
	Access,
	Across, // Added: IEEE 1076.1-1999
	After,
	Alias,
	All,
	And,
	Architecture,
	Array,
	Assert,
	Assume,          // Added: IEEE 1076-2008
	AssumeGuarantee, // Added: IEEE 1076-2008
	Attribute,
	Begin,
	Block,
	Body,
	Break, // Added: IEEE 1076.1-1999
	Buffer,
	Bus,
	Case,
	Component,
	Configuration,
	Constant,
	Context, // Added: IEEE 1076-2008
	Cover,   // Added: IEEE 1076-2008
	Default, // Added: IEEE 1076-2008
	Disconnect,
	DownTo,
	Else,
	Elsif,
	End,
	Entity,
	Exit,
	Fairness, // Added: IEEE 1076-2008
	File,
	For,
	Force, // Added: IEEE 1076-2008
	Function,
	Generate,
	Generic,
	Group, // Added: IEEE 1076-1993
	Guarded,
	If,
	Impure, // Added: IEEE 1076-1993
	In,
	Inertial, // Added: IEEE 1076-1993
	InOut,
	Is,
	Label,
	Library,
	Limit, // Added: IEEE 1076.1-1999
	Linkage,
	Literal, // Added: IEEE 1076-1993
	Loop,
	Map,
	Mod,
	Nand,
	Nature, // Added: IEEE 1076.1-1999
	New,
	Next,
	Noise, // Added: IEEE 1076.1-1999
	Nor,
	Not,
	Null,
	Of,
	On,
	Open,
	Or,
	Others,
	Out,
	Package,
	Parameter, // Added: IEEE 1076-2008
	Port,
	Postponed,  // Added: IEEE 1076-1993
	Private,    // Added: IEEE 1076-2019
	Procedural, // Added: IEEE 1076-2000 & IEEE 1076.1-1999
	Procedure,
	Process,
	Property,  // Added: IEEE 1076-2008
	Protected, // Added: IEEE 1076-2000 & IEEE 1076.1-2007
	Pure,      // Added: IEEE 1076-1993
	Quantity,  // Added: IEEE 1076.1-1999
	Range,
	Record,
	Reference, // Added: IEEE 1076-2000 & IEEE 1076.1-1999
	Register,
	Reject,  // Added: IEEE 1076-1993
	Release, // Added: IEEE 1076-2008
	Rem,
	Report,
	Restrict,          // Added: IEEE 1076-2008
	RestrictGuarantee, // Added: IEEE 1076-2008
	Return,
	Rol, // Added: IEEE 1076-1993
	Ror, // Added: IEEE 1076-1993
	Select,
	Sequence, // Added: IEEE 1076-2008
	Severity,
	Shared, // Added: IEEE 1076-1993
	Signal,
	Sla,       // Added: IEEE 1076-1993
	Sll,       // Added: IEEE 1076-1993
	Spectrum,  // Added: IEEE 1076.1-1999
	Sra,       // Added: IEEE 1076-1993
	Srl,       // Added: IEEE 1076-1993
	Strong,    // Added: IEEE 1076-2008
	SubNature, // Added: IEEE 1076.1-1999
	Subtype,
	Terminal, // Added: IEEE 1076.1-1999
	Then,
	Through, // Added: IEEE 1076.1-1999
	To,
	Tolerance, // Added: IEEE 1076.1-1999
	Transport,
	Type,
	Unaffected, // Added: IEEE 1076-1993
	Units,
	Until,
	Use,
	Variable,
	View,  // Added: IEEE 1076-2019
	Vmode, // Added: IEEE 1076-2008
	Vpkg,  // Added: IEEE 1076-2019
	Vprop, // Added: IEEE 1076-2008
	Vunit, // Added: IEEE 1076-2008
	Wait,
	When,
	While,
	With,
	Xnor, // Added: IEEE 1076-1993
	Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Comment(comment) => comment.fmt(f),
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
			Self::String(tendril) => write!(f, "String(\"{}\")", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
			Self::Whitespace(tendril) => write!(f, "Whitespace({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Display for Comment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Comment({})",
			todo!()
		)
	}
}

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Control({})",
			todo!()
		)
	}
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Keyword({})",
			match self {
				Self::Abs => "abs",
				Self::Access => "access",
				Self::Across => "across", // Added: IEEE 1076.1-1999
				Self::After => "after",
				Self::Alias => "alias",
				Self::All => "all",
				Self::And => "and",
				Self::Architecture => "architecture",
				Self::Array => "array",
				Self::Assert => "assert",
				Self::Assume => "assume",                    // Added: IEEE 1076-2008
				Self::AssumeGuarantee => "assume_guarantee", // Added: IEEE 1076-2008
				Self::Attribute => "attribute",
				Self::Begin => "begin",
				Self::Block => "block",
				Self::Body => "body",
				Self::Break => "break", // Added: IEEE 1076.1-1999
				Self::Buffer => "buffer",
				Self::Bus => "bus",
				Self::Case => "case",
				Self::Component => "component",
				Self::Configuration => "configuration",
				Self::Constant => "constant",
				Self::Context => "context", // Added: IEEE 1076-2008
				Self::Cover => "cover",     // Added: IEEE 1076-2008
				Self::Default => "default", // Added: IEEE 1076-2008
				Self::Disconnect => "disconnect",
				Self::DownTo => "downto",
				Self::Else => "else",
				Self::Elsif => "elsif",
				Self::End => "end",
				Self::Entity => "entity",
				Self::Exit => "exit",
				Self::Fairness => "fairness", // Added: IEEE 1076-2008
				Self::File => "file",
				Self::For => "for",
				Self::Force => "force", // Added: IEEE 1076-2008
				Self::Function => "function",
				Self::Generate => "generate",
				Self::Generic => "generic",
				Self::Group => "group", // Added: IEEE 1076-1993
				Self::Guarded => "guarded",
				Self::If => "if",
				Self::Impure => "impure", // Added: IEEE 1076-1993
				Self::In => "in",
				Self::Inertial => "inertial", // Added: IEEE 1076-1993
				Self::InOut => "inout",
				Self::Is => "is",
				Self::Label => "label",
				Self::Library => "library",
				Self::Limit => "limit", // Added: IEEE 1076.1-1999
				Self::Linkage => "linkage",
				Self::Literal => "literal", // Added: IEEE 1076-1993
				Self::Loop => "loop",
				Self::Map => "map",
				Self::Mod => "mod",
				Self::Nand => "nand",
				Self::Nature => "nature", // Added: IEEE 1076.1-1999
				Self::New => "new",
				Self::Next => "next",
				Self::Noise => "noise", // Added: IEEE 1076.1-1999
				Self::Nor => "nor",
				Self::Not => "not",
				Self::Null => "null",
				Self::Of => "of",
				Self::On => "on",
				Self::Open => "open",
				Self::Or => "or",
				Self::Others => "others",
				Self::Out => "out",
				Self::Package => "package",
				Self::Parameter => "parameter", // Added: IEEE 1076-2008
				Self::Port => "port",
				Self::Postponed => "postponed",   // Added: IEEE 1076-1993
				Self::Private => "private",       // Added: IEEE 1076-2019
				Self::Procedural => "procedural", // Added: IEEE 1076-2000 & IEEE 1076.1-1999
				Self::Procedure => "procedure",
				Self::Process => "process",
				Self::Property => "property",   // Added: IEEE 1076-2008
				Self::Protected => "protected", // Added: IEEE 1076-2000 & IEEE 1076.1-2007
				Self::Pure => "pure",           // Added: IEEE 1076-1993
				Self::Quantity => "quantity",   // Added: IEEE 1076.1-1999
				Self::Range => "range",
				Self::Record => "record",
				Self::Reference => "reference", // Added: IEEE 1076-2000 & IEEE 1076.1-1999
				Self::Register => "register",
				Self::Reject => "reject",   // Added: IEEE 1076-1993
				Self::Release => "release", // Added: IEEE 1076-2008
				Self::Rem => "rem",
				Self::Report => "report",
				Self::Restrict => "restrict", // Added: IEEE 1076-2008
				Self::RestrictGuarantee => "restrict_guarantee", // Added: IEEE 1076-2008
				Self::Return => "return",
				Self::Rol => "rol", // Added: IEEE 1076-1993
				Self::Ror => "ror", // Added: IEEE 1076-1993
				Self::Select => "select",
				Self::Sequence => "sequence", // Added: IEEE 1076-2008
				Self::Severity => "severity",
				Self::Shared => "shared", // Added: IEEE 1076-1993
				Self::Signal => "signal",
				Self::Sla => "sla",             // Added: IEEE 1076-1993
				Self::Sll => "sll",             // Added: IEEE 1076-1993
				Self::Spectrum => "spectrum",   // Added: IEEE 1076.1-1999
				Self::Sra => "sra",             // Added: IEEE 1076-1993
				Self::Srl => "srl",             // Added: IEEE 1076-1993
				Self::Strong => "strong",       // Added: IEEE 1076-2008
				Self::SubNature => "subnature", // Added: IEEE 1076.1-1999
				Self::Subtype => "subtype",
				Self::Terminal => "terminal", // Added: IEEE 1076.1-1999
				Self::Then => "then",
				Self::Through => "through", // Added: IEEE 1076.1-1999
				Self::To => "to",
				Self::Tolerance => "tolerance", // Added: IEEE 1076.1-1999
				Self::Transport => "transport",
				Self::Type => "type",
				Self::Unaffected => "unaffected", // Added: IEEE 1076-1993
				Self::Units => "units",
				Self::Until => "until",
				Self::Use => "use",
				Self::Variable => "variable",
				Self::View => "view",   // Added: IEEE 1076-2019
				Self::Vmode => "vmode", // Added: IEEE 1076-2008
				Self::Vpkg => "vpkg",   // Added: IEEE 1076-2019
				Self::Vprop => "vprop", // Added: IEEE 1076-2008
				Self::Vunit => "vunit", // Added: IEEE 1076-2008
				Self::Wait => "wait",
				Self::When => "when",
				Self::While => "while",
				Self::With => "with",
				Self::Xnor => "xnor", // Added: IEEE 1076-1993
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
			todo!()
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
