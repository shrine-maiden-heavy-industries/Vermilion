/* SPDX-License-Identifier: BSD-3-Clause */

use tendril::ByteTendril;
use vermilion_lang::parser::Spanned;

use crate::VerilogVariant;

use super::Position;

#[derive(Clone)]
pub(crate) enum Token {
	Invalid(Option<ByteTendril>),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(ByteTendril, VerilogVariant),
	Whitespace(ByteTendril),
	Newline(ByteTendril),
	Identifier(ByteTendril),
	Sign(Sign),
	UnsignedNumber(ByteTendril),
	BaseSpecifier(BaseSpecifier, bool),
	Number(ByteTendril),
	Control(Control),
	Keyword(Keyword),
	Operator(Operator),
}

#[derive(Clone, Copy)]
pub(crate) enum Sign {
	Positive,
	Negative
}

#[derive(Clone, Copy)]
pub(crate) enum BaseSpecifier {
	Binary,
	Octal,
	Decimal,
	Hexadecimal,
}

#[derive(Clone, Copy)]
pub(crate) enum Control {
	At,
	Octothorp,
	ParenOpen,
	ParenClose,
	BracketOpen,
	BracketClose,
	BraceOpen,
	BraceClose,
	Colon,
	Semicolon,
	Comma,
	Dot,
	Dollar,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub(crate) enum Operator {
	Plus,
	Minus,
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
