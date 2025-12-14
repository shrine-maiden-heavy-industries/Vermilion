/* SPDX-License-Identifier: BSD-3-Clause */

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Whitespace {
	// TODO(aki): Two things:
	//  1. Should we keep track of which type of indentation it is?
	//  2. If not, should we just normalize to a usize for how many characters?
	Indentation(String),
	Newline,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Number<'src> {
	Binary(Option<isize>, &'src str),
	Octal(Option<isize>, &'src str),
	Dec(Option<isize>, &'src str),
	Hex(Option<isize>, &'src str),
	Real(f64),
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token<'src> {
	Whitespace(Whitespace),
	Comment(&'src str),
	CompilerDirective(&'src str),
	Ctrl(char),
	Str(&'src str),
	Op(&'src str),
	Number(Number<'src>),
	Ident(&'src str),
	Keyword(Keyword),
}

impl Display for Whitespace {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Indentation(ws) => write!(f, "Whitespace ({} chars)", ws.len()),
			Self::Newline => write!(f, "Newline"),
		}
	}
}

impl<'src> Display for Number<'src> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Number::Binary(size, number) => {
				if let Some(size) = size {
					write!(f, "{size}'b{number}")
				} else {
					write!(f, "'b{number}")
				}
			},
			Number::Octal(size, number) => {
				if let Some(size) = size {
					write!(f, "{size}'o{number}")
				} else {
					write!(f, "'o{number}")
				}
			},
			Number::Dec(size, number) => {
				if let Some(size) = size {
					write!(f, "{size}'d{number}")
				} else {
					write!(f, "'d{number}")
				}
			},
			Number::Hex(size, number) => {
				if let Some(size) = size {
					write!(f, "{size}'h{number}")
				} else {
					write!(f, "'h{number}")
				}
			},
			Number::Real(num) => write!(f, "{num}"),
		}
	}
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Always => write!(f, "always"),
			Self::And => write!(f, "and"),
			Self::Assign => write!(f, "assign"),
			Self::Begin => write!(f, "begin"),
			Self::Buf => write!(f, "buf"),
			Self::BufIf0 => write!(f, "bufif0"),
			Self::BufIf1 => write!(f, "bufif1"),
			Self::Case => write!(f, "case"),
			Self::CaseX => write!(f, "casex"),
			Self::CaseZ => write!(f, "casez"),
			Self::Cmos => write!(f, "cmos"),
			Self::Deassign => write!(f, "deassign"),
			Self::Default => write!(f, "default"),
			Self::DefParam => write!(f, "defparam"),
			Self::Disable => write!(f, "disable"),
			Self::Edge => write!(f, "edge"),
			Self::Else => write!(f, "else"),
			Self::End => write!(f, "end"),
			Self::EndCase => write!(f, "endcase"),
			Self::EndFunction => write!(f, "endfunction"),
			Self::EndModule => write!(f, "endmodule"),
			Self::EndPrimitive => write!(f, "endprimitive"),
			Self::EndSpecify => write!(f, "endspecify"),
			Self::EndTable => write!(f, "endtable"),
			Self::EndTask => write!(f, "endtask"),
			Self::Event => write!(f, "event"),
			Self::For => write!(f, "for"),
			Self::Force => write!(f, "force"),
			Self::Forever => write!(f, "forever"),
			Self::Fork => write!(f, "fork"),
			Self::Function => write!(f, "function"),
			Self::HighZ0 => write!(f, "highz0"),
			Self::HighZ1 => write!(f, "highz1"),
			Self::If => write!(f, "if"),
			Self::IfNone => write!(f, "ifnone"),
			Self::Initial => write!(f, "initial"),
			Self::InOut => write!(f, "inout"),
			Self::Input => write!(f, "input"),
			Self::Integer => write!(f, "integer"),
			Self::Join => write!(f, "join"),
			Self::Large => write!(f, "large"),
			Self::MacroModule => write!(f, "macromodule"),
			Self::Medium => write!(f, "medium"),
			Self::Module => write!(f, "module"),
			Self::Nand => write!(f, "nand"),
			Self::NegEdge => write!(f, "negedge"),
			Self::Nmos => write!(f, "nmos"),
			Self::Nor => write!(f, "nor"),
			Self::Not => write!(f, "not"),
			Self::NotIf0 => write!(f, "notif0"),
			Self::NotIf1 => write!(f, "notif1"),
			Self::Or => write!(f, "or"),
			Self::Output => write!(f, "output"),
			Self::Parameter => write!(f, "parameter"),
			Self::Pmos => write!(f, "pmos"),
			Self::PosEdge => write!(f, "posedge"),
			Self::Primitive => write!(f, "primitive"),
			Self::Pull0 => write!(f, "pull0"),
			Self::Pull1 => write!(f, "pull1"),
			Self::Pulldown => write!(f, "pulldown"),
			Self::Pullup => write!(f, "pullup"),
			Self::Rcmos => write!(f, "rcmos"),
			Self::Real => write!(f, "real"),
			Self::Realtime => write!(f, "realtime"),
			Self::Reg => write!(f, "reg"),
			Self::Release => write!(f, "release"),
			Self::Repeat => write!(f, "repeat"),
			Self::Rnmos => write!(f, "rnmos"),
			Self::Rpmos => write!(f, "rpmos"),
			Self::Rtran => write!(f, "rtran"),
			Self::RtranIf0 => write!(f, "rtranif0"),
			Self::RtranIf1 => write!(f, "rtranaof1"),
			Self::Scalared => write!(f, "scalard"),
			Self::Small => write!(f, "small"),
			Self::Specify => write!(f, "specify"),
			Self::SpecParam => write!(f, "specparam"),
			Self::Strong0 => write!(f, "strong0"),
			Self::Strong1 => write!(f, "strong1"),
			Self::Supply0 => write!(f, "supply0"),
			Self::Supply1 => write!(f, "supply1"),
			Self::Table => write!(f, "table"),
			Self::Task => write!(f, "task"),
			Self::Time => write!(f, "time"),
			Self::Tran => write!(f, "tran"),
			Self::TranIf0 => write!(f, "tranif0"),
			Self::TranIf1 => write!(f, "tranif1"),
			Self::Tri => write!(f, "tri"),
			Self::Tri0 => write!(f, "tri0"),
			Self::Tri1 => write!(f, "tri1"),
			Self::Triand => write!(f, "triand"),
			Self::Trior => write!(f, "trior"),
			Self::Trireg => write!(f, "trireg"),
			Self::Vectored => write!(f, "vectored"),
			Self::Wait => write!(f, "wait"),
			Self::Wand => write!(f, "wand"),
			Self::Weak0 => write!(f, "weak0"),
			Self::Weak1 => write!(f, "weak1"),
			Self::While => write!(f, "while"),
			Self::Wire => write!(f, "wire"),
			Self::Wor => write!(f, "wor"),
			Self::Xnor => write!(f, "xnor"),
			Self::Xor => write!(f, "xor"),
		}
	}
}

impl<'src> Display for Token<'src> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Whitespace(whitespace) => write!(f, "{whitespace}"),
			Self::Comment(text) => write!(f, "{text}"),
			Self::CompilerDirective(directive) => write!(f, "{directive}"),
			Self::Ctrl(ctrl) => write!(f, "{ctrl}"),
			Self::Str(text) => write!(f, "\"{text}\""),
			Self::Op(op) => write!(f, "{op}"),
			Self::Number(number) => write!(f, "{number}"),
			Self::Ident(ident) => write!(f, "{ident}"),
			Self::Keyword(kw) => write!(f, "{kw}"),
		}
	}
}
