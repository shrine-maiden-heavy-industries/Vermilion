/* SPDX-License-Identifier: BSD-3-Clause */

use tendril::ByteTendril;
use vermilion_lang::{Position, Spanned};

use crate::VerilogVariant;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Token {
	Invalid(Option<ByteTendril>),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(ByteTendril, VerilogVariant),
	BaseSpecifier(BaseSpecifier, bool),
	Comment(Comment),
	CompilerDirective(ByteTendril),
	CompilerDirectiveArg(ByteTendril),
	Control(Control),
	Identifier(ByteTendril),
	Keyword(Keyword),
	Newline(ByteTendril),
	Number(ByteTendril),
	Operator(Operator),
	Sign(Sign),
	String(ByteTendril),
	UnsignedNumber(ByteTendril),
	Whitespace(ByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Comment {
	SingleLine(ByteTendril),
	Block(ByteTendril),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Sign {
	Positive,
	Negative,
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
