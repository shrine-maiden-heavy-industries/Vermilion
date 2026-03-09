// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use phf::{phf_map, phf_set};

use crate::LanguageStd;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
	Above,      // Added: Verilog-AMS 2009
	Abs,        // Added: Verilog-AMS 2009
	AbsDelay,   // Added: Verilog-AMS 2009
	AbsDelta,   // Added: Verilog-AMS 2014
	AbsTol,     // Added: Verilog-AMS 2009
	AcceptOn,   // Added: IEEE 1800-2009
	Access,     // Added: Verilog-AMS 2009
	Acos,       // Added: Verilog-AMS 2009
	Acosh,      // Added: Verilog-AMS 2009
	AcStim,     // Added: Verilog-AMS 2009
	Alias,      // Added: IEEE 1800-2005
	Aliasparam, // Added: Verilog-AMS 2009
	Always,
	AlwaysComb,  // Added: IEEE 1800-2005
	AlwaysFF,    // Added: IEEE 1800-2005
	AlwaysLatch, // Added: IEEE 1800-2005
	Analog,      // Added: Verilog-AMS 2009
	Analysis,    // Added: Verilog-AMS 2009
	And,
	Asin,   // Added: Verilog-AMS 2009
	Asinh,  // Added: Verilog-AMS 2009
	Assert, // Added: IEEE 1800-2005 & Verilog-AMS 2009 Removed: Verilog-AMS 2023
	Assign,
	Assume,    // Added: IEEE 1800-2005
	Atan,      // Added: Verilog-AMS 2009
	Atan2,     // Added: Verilog-AMS 2009
	Atanh,     // Added: Verilog-AMS 2009
	Automatic, // Added: IEEE 1364-2001
	Before,    // Added: IEEE 1800-2005
	Begin,
	Bind,   // Added: IEEE 1800-2005
	Bins,   // Added: IEEE 1800-2005
	BinsOf, // Added: IEEE 1800-2005
	Bit,    // Added: IEEE 1800-2005
	Branch, // Added: Verilog-AMS 2009
	Break,  // Added: IEEE 1800-2005 & Verilog-AMS 2023
	Buf,
	BufIf0,
	BufIf1,
	Byte, // Added: IEEE 1800-2005
	Case,
	CaseX,
	CaseZ,
	Ceil,     // Added: Verilog-AMS 2009
	Cell,     // Added: IEEE 1364-2001
	CHandle,  // Added: IEEE 1800-2005
	Checker,  // Added: IEEE 1800-2009
	Class,    // Added: IEEE 1800-2005
	Clocking, // Added: IEEE 1800-2005
	Cmos,
	Config,        // Added: IEEE 1364-2001
	Connect,       // Added: Verilog-AMS 2009
	ConnectModule, // Added: Verilog-AMS 2009
	ConnectRules,  // Added: Verilog-AMS 2009
	Const,         // Added: IEEE 1800-2005
	Constraint,    // Added: IEEE 1800-2005
	Context,       // Added: IEEE 1800-2005
	Continue,      // Added: IEEE 1800-2005 & Verilog-AMS 2023
	Continuous,    // Added: Verilog-AMS 2009
	Cos,           // Added: Verilog-AMS 2009
	Cosh,          // Added: Verilog-AMS 2009
	Cover,         // Added: IEEE 1800-2005
	CoverGroup,    // Added: IEEE 1800-2005
	CoverPoint,    // Added: IEEE 1800-2005
	Cross,         // Added: IEEE 1800-2005 & Verilog-AMS 2009
	Ddt,           // Added: Verilog-AMS 2009
	DdtNature,     // Added: Verilog-AMS 2009
	Ddx,           // Added: Verilog-AMS 2009
	Deassign,
	Default,
	DefParam,
	Design, // Added: IEEE 1364-2001
	Disable,
	Discipline,   // Added: Verilog-AMS 2009
	Discrete,     // Added: Verilog-AMS 2009
	Dist,         // Added: IEEE 1800-2005
	Do,           // Added: IEEE 1800-2005
	Domain,       // Added: Verilog-AMS 2009
	DriverUpdate, // Added: Verilog-AMS 2009
	Edge,
	Else,
	End,
	EndCase,
	EndChecker,      // Added: IEEE 1800-2009
	EndClass,        // Added: IEEE 1800-2005
	EndClocking,     // Added: IEEE 1800-2005
	EndConfig,       // Added: IEEE 1364-2001
	EndConnectRules, // Added: Verilog-AMS 2009
	EndDiscipline,   // Added: Verilog-AMS 2009
	EndFunction,
	EndGenerate,  // Added: IEEE 1364-2001
	EndGroup,     // Added: IEEE 1800-2005
	EndInterface, // Added: IEEE 1800-2005
	EndModule,
	EndNature,   // Added: Verilog-AMS 2009
	EndPackage,  // Added: IEEE 1800-2005
	EndParamSet, // Added: Verilog-AMS 2009
	EndPrimitive,
	EndProgram,  // Added: IEEE 1800-2005
	EndProperty, // Added: IEEE 1800-2005
	EndSequence, // Added: IEEE 1800-2005
	EndSpecify,
	EndTable,
	EndTask,
	Enum, // Added: IEEE 1800-2005
	Event,
	Eventually,   // Added: IEEE 1800-2009
	Exclude,      // Added: Verilog-AMS 2009
	Exp,          // Added: Verilog-AMS 2009
	Expect,       // Added: IEEE 1800-2005
	Expm1,        // Added: Verilog-AMS 2023
	Export,       // Added: IEEE 1800-2005
	Extends,      // Added: IEEE 1800-2005
	Extern,       // Added: IEEE 1800-2005
	Final,        // Added: IEEE 1800-2005
	FinalStep,    // Added: Verilog-AMS 2009
	FirstMatch,   // Added: IEEE 1800-2005
	FlickerNoise, // Added: Verilog-AMS 2009
	Floor,        // Added: Verilog-AMS 2009
	Flow,         // Added: Verilog-AMS 2009
	For,
	Force,
	ForEach, // Added: IEEE 1800-2005
	Forever,
	Fork,
	ForkJoin, // Added: IEEE 1800-2005
	From,     // Added: Verilog-AMS 2009
	Function,
	Generate, // Added: IEEE 1364-2001
	Genvar,   // Added: IEEE 1364-2001
	Global,   // Added: IEEE 1800-2009
	Ground,   // Added: Verilog-AMS 2009
	HighZ0,
	HighZ1,
	Hypot,     // Added: Verilog-AMS 2009
	Idt,       // Added: Verilog-AMS 2009
	IdtMod,    // Added: Verilog-AMS 2009
	IdtNature, // Added: Verilog-AMS 2009
	If,
	Iff, // Added: IEEE 1800-2005
	IfNone,
	IgnoreBins,  // Added: IEEE 1800-2005
	IllegalBins, // Added: IEEE 1800-2005
	Implements,  // Added: IEEE 1800-2012
	Implies,     // Added: IEEE 1800-2009
	Import,      // Added: IEEE 1800-2005
	IncDir,      // Added: IEEE 1364-2001
	Include,     // Added: IEEE 1364-2001
	Inf,         // Added: Verilog-AMS 2009
	Initial,
	InitialStep, // Added: Verilog-AMS 2009
	InOut,
	Input,
	Inside,   // Added: IEEE 1800-2005
	Instance, // Added: IEEE 1364-2001
	Int,      // Added: IEEE 1800-2005
	Integer,
	Interconnect, // Added: IEEE 1800-2012
	Interface,    // Added: IEEE 1800-2005
	Intersect,    // Added: IEEE 1800-2005
	Join,
	JoinAny,   // Added: IEEE 1800-2005
	JoinNone,  // Added: IEEE 1800-2005
	LaplaceNd, // Added: Verilog-AMS 2009
	LaplaceNp, // Added: Verilog-AMS 2009
	LaplaceZd, // Added: Verilog-AMS 2009
	LaplaceZp, // Added: Verilog-AMS 2009
	Large,
	LastCrossing, // Added: Verilog-AMS 2009
	Let,          // Added: IEEE 1800-2009
	LibList,      // Added: IEEE 1364-2001
	Library,      // Added: IEEE 1364-2001
	LimExp,       // Added: Verilog-AMS 2009
	Ln,           // Added: Verilog-AMS 2009
	Ln1p,         // Added: Verilog-AMS 2023
	Local,        // Added: IEEE 1800-2005
	LocalParam,   // Added: IEEE 1364-2001
	Log,          // Added: Verilog-AMS 2009
	Logic,        // Added: IEEE 1800-2005
	LongInt,      // Added: IEEE 1800-2005
	MacroModule,
	Matches, // Added: IEEE 1800-2005
	Max,     // Added: Verilog-AMS 2009
	Medium,
	Merged,  // Added: Verilog-AMS 2009
	Min,     // Added: Verilog-AMS 2009
	ModPort, // Added: IEEE 1800-2005
	Module,
	Nand,
	Nature, // Added: Verilog-AMS 2009
	NegEdge,
	NetResolution, // Added: Verilog-AMS 2009 Removed: Verilog-AMS 2023
	NetType,       // Added: IEEE 1800-2012
	New,           // Added: IEEE 1800-2005
	NextTime,      // Added: IEEE 1800-2009
	Nmos,
	NoiseTable,    // Added: Verilog-AMS 2009
	NoiseTableLog, // Added: Verilog-AMS 2014
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
	ParamSet, // Added: Verilog-AMS 2009
	Pmos,
	PosEdge,
	Potential, // Added: Verilog-AMS 2009
	Pow,       // Added: Verilog-AMS 2009
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
	RejectOn, // Added: IEEE 1800-2009
	Release,
	Repeat,
	ResolveTo, // Added: Verilog-AMS 2009
	Restrict,  // Added: IEEE 1800-2009
	Return,    // Added: IEEE 1800-2005 & Verilog-AMS 2023
	Rnmos,
	Rpmos,
	Rtran,
	RtranIf0,
	RtranIf1,
	SAlways, // Added: IEEE 1800-2009
	Scalared,
	Sequence,      // Added: IEEE 1800-2005
	SEventually,   // Added: IEEE 1800-2009
	ShortInt,      // Added: IEEE 1800-2005
	ShortReal,     // Added: IEEE 1800-2005
	ShowCancelled, // Added: IEEE 1364-2001
	Signed,        // Added: IEEE 1364-2001
	Sin,           // Added: Verilog-AMS 2009
	Sinh,          // Added: Verilog-AMS 2009
	Slew,          // Added: Verilog-AMS 2009
	Small,
	SNextTime, // Added: IEEE 1800-2009
	Soft,      // Added: IEEE 1800-2012
	Solve,     // Added: IEEE 1800-2005
	Specify,
	SpecParam,
	Split,  // Added: Verilog-AMS 2009
	Sqrt,   // Added: Verilog-AMS 2009
	Static, // Added: IEEE 1800-2005
	String, // Added: IEEE 1800-2005 & Verilog-AMS 2009
	Strong, // Added: IEEE 1800-2009
	Strong0,
	Strong1,
	Struct,     // Added: IEEE 1800-2005
	SUntil,     // Added: IEEE 1800-2009
	SUntilWith, // Added: IEEE 1800-2009
	Super,      // Added: IEEE 1800-2005
	Supply0,
	Supply1,
	SyncAcceptOn, // Added: IEEE 1800-2009
	SyncRejectOn, // Added: IEEE 1800-2009
	Table,
	Tagged, // Added: IEEE 1800-2005
	Tan,    // Added: Verilog-AMS 2009
	Tanh,   // Added: Verilog-AMS 2009
	Task,
	This,       // Added: IEEE 1800-2005
	Throughout, // Added: IEEE 1800-2005
	Time,
	TimePrecision, // Added: IEEE 1800-2005
	Timer,         // Added: Verilog-AMS 2009
	TimeUnit,      // Added: IEEE 1800-2005
	Tran,
	TranIf0,
	TranIf1,
	Transition, // Added: Verilog-AMS 2009
	Tri,
	Tri0,
	Tri1,
	Triand,
	Trior,
	Trireg,
	Type,      // Added: IEEE 1800-2005
	TypeDef,   // Added: IEEE 1800-2005
	Union,     // Added: IEEE 1800-2005
	Unique,    // Added: IEEE 1800-2005
	Unique0,   // Added: IEEE 1800-2009
	Units,     // Added: Verilog-AMS 2009
	Unsigned,  // Added: IEEE 1364-2001
	Until,     // Added: IEEE 1800-2009
	UntilWith, // Added: IEEE 1800-2009
	Untyped,   // Added: IEEE 1800-2009
	Use,       // Added: IEEE 1364-2001
	Uwire,     // Added: IEEE 1364-2005
	Var,       // Added: IEEE 1800-2005
	Vectored,
	Virtual, // Added: IEEE 1800-2005
	Void,    // Added: IEEE 1800-2005
	Wait,
	WaitOrder, // Added: IEEE 1800-2005
	Wand,
	Weak, // Added: IEEE 1800-2009
	Weak0,
	Weak1,
	While,
	WhiteNoise, // Added: Verilog-AMS 2009
	Wildcard,   // Added: IEEE 1800-2005
	Wire,
	With,   // Added: IEEE 1800-2005
	Within, // Added: IEEE 1800-2005
	Wor,
	Wreal, // Added: Verilog-AMS 2009
	Xnor,
	Xor,
	ZiNd, // Added: Verilog-AMS 2009
	ZiNp, // Added: Verilog-AMS 2009
	ZiZd, // Added: Verilog-AMS 2009
	ZiZp, // Added: Verilog-AMS 2009
}

/// IEEE 1364-1995 (Verilog 1995) Keyword to [`Keyword`] token map
pub static VERILOG_95_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assign" => Keyword::Assign,
	"begin" => Keyword::Begin,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cmos" => Keyword::Cmos,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"disable" => Keyword::Disable,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endfunction" => Keyword::EndFunction,
	"endmodule" => Keyword::EndModule,
	"endprimitive" => Keyword::EndPrimitive,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"event" => Keyword::Event,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"function" => Keyword::Function,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"ifnone" => Keyword::IfNone,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"integer" => Keyword::Integer,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"macromodule" => Keyword::MacroModule,
	"medium" => Keyword::Medium,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"reg" => Keyword::Reg,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"scalared" => Keyword::Scalared,
	"small" => Keyword::Small,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"table" => Keyword::Table,
	"task" => Keyword::Task,
	"time" => Keyword::Time,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"vectored" => Keyword::Vectored,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wire" => Keyword::Wire,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1364-1995 (Verilog 1995) Keyword set
pub static VERILOG_95_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"always",   "edge",         "function",    "negedge",   "pullup",   "specparam", "trior",
	"and",      "else",         "highz0",      "nmos",      "rcmos",    "strong0",   "trireg",
	"assign",   "end",          "highz1",      "nor",       "real",     "strong1",   "vectored",
	"begin",    "endcase",      "if",          "not",       "realtime", "supply0",   "wait",
	"buf",      "endfunction",  "ifnone",      "notif0",    "reg",      "supply1",   "wand",
	"bufif0",   "endmodule",    "initial",     "notif1",    "release",  "table",     "weak0",
	"bufif1",   "endprimitive", "inout",       "or",        "repeat",   "task",      "weak1",
	"case",     "endspecify",   "input",       "output",    "rnmos",    "time",      "while",
	"casex",    "endtable",     "integer",     "parameter", "rpmos",    "tran",      "wire",
	"casez",    "endtask",      "join",        "pmos",      "rtran",    "tranif0",   "wor",
	"cmos",     "event",        "large",       "posedge",   "rtranif0", "tranif1",   "xnor",
	"deassign", "for",          "macromodule", "primitive", "rtranif1", "tri",       "xor",
	"default",  "force",        "medium",      "pull0",     "scalared", "tri0",
	"defparam", "forever",      "module",      "pull1",     "small",    "tri1",
	"disable",  "fork",         "nand",        "pulldown",  "specify",  "triand",
};

/// IEEE 1364-2001 (Verilog 2001) Keyword to [`Keyword`] token map
pub static VERILOG_01_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assign" => Keyword::Assign,
	"automatic" => Keyword::Automatic,
	"begin" => Keyword::Begin,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cell" => Keyword::Cell,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endconfig" => Keyword::EndConfig,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endmodule" => Keyword::EndModule,
	"endprimitive" => Keyword::EndPrimitive,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"event" => Keyword::Event,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"ifnone" => Keyword::IfNone,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"instance" => Keyword::Instance,
	"integer" => Keyword::Integer,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"localparam" => Keyword::LocalParam,
	"macromodule" => Keyword::MacroModule,
	"medium" => Keyword::Medium,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"reg" => Keyword::Reg,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"scalared" => Keyword::Scalared,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"small" => Keyword::Small,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"table" => Keyword::Table,
	"task" => Keyword::Task,
	"time" => Keyword::Time,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"unsigned" => Keyword::Unsigned,
	"use" => Keyword::Use,
	"vectored" => Keyword::Vectored,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wire" => Keyword::Wire,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1364-2001 (Verilog 2001) Keyword set
pub static VERILOG_01_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"always",      "endgenerate",  "join",            "pullup",              "task",
	"and",         "endmodule",    "large",           "pulsestyle_ondetect", "time",
	"assign",      "endprimitive", "liblist",         "pulsestyle_onevent",  "tran",
	"automatic",   "endspecify",   "library",         "rcmos",               "tranif0",
	"begin",       "endtable",     "localparam",      "real",                "tranif1",
	"buf",         "endtask",      "macromodule",     "realtime",            "tri",
	"bufif0",      "event",        "medium",          "reg",                 "tri0",
	"bufif1",      "for",          "module",          "release",             "tri1",
	"case",        "force",        "nand",            "repeat",              "triand",
	"casex",       "forever",      "negedge",         "rnmos",               "trior",
	"casez",       "fork",         "nmos",            "rpmos",               "trireg",
	"cell",        "function",     "nor",             "rtran",               "unsigned",
	"cmos",        "generate",     "noshowcancelled", "rtranif0",            "use",
	"config",      "genvar",       "not",             "rtranif1",            "vectored",
	"deassign",    "highz0",       "notif0",          "scalared",            "wait",
	"default",     "highz1",       "notif1",          "showcancelled",       "wand",
	"defparam",    "if",           "or",              "signed",              "weak0",
	"design",      "ifnone",       "output",          "small",               "weak1",
	"disable",     "incdir",       "parameter",       "specify",             "while",
	"edge",        "include",      "pmos",            "specparam",           "wire",
	"else",        "initial",      "posedge",         "strong0",             "wor",
	"end",         "inout",        "primitive",       "strong1",             "xnor",
	"endcase",     "input",        "pull0",           "supply0",             "xor",
	"endconfig",   "instance",     "pull1",           "supply1",
	"endfunction", "integer",      "pulldown",        "table",
};

/// IEEE 1364-2005 (Verilog 2005) Keyword to [`Keyword`] token map
pub static VERILOG_05_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assign" => Keyword::Assign,
	"automatic" => Keyword::Automatic,
	"begin" => Keyword::Begin,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cell" => Keyword::Cell,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endconfig" => Keyword::EndConfig,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endmodule" => Keyword::EndModule,
	"endprimitive" => Keyword::EndPrimitive,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"event" => Keyword::Event,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"ifnone" => Keyword::IfNone,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"instance" => Keyword::Instance,
	"integer" => Keyword::Integer,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"localparam" => Keyword::LocalParam,
	"macromodule" => Keyword::MacroModule,
	"medium" => Keyword::Medium,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"reg" => Keyword::Reg,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"scalared" => Keyword::Scalared,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"small" => Keyword::Small,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"table" => Keyword::Table,
	"task" => Keyword::Task,
	"time" => Keyword::Time,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"unsigned" => Keyword::Unsigned,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"vectored" => Keyword::Vectored,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wire" => Keyword::Wire,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1364-2005 (Verilog 2005) Keyword set
pub static VERILOG_05_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"always",      "endgenerate",  "join",            "pullup",              "task",
	"and",         "endmodule",    "large",           "pulsestyle_ondetect", "time",
	"assign",      "endprimitive", "liblist",         "pulsestyle_onevent",  "tran",
	"automatic",   "endspecify",   "library",         "rcmos",               "tranif0",
	"begin",       "endtable",     "localparam",      "real",                "tranif1",
	"buf",         "endtask",      "macromodule",     "realtime",            "tri",
	"bufif0",      "event",        "medium",          "reg",                 "tri0",
	"bufif1",      "for",          "module",          "release",             "tri1",
	"case",        "force",        "nand",            "repeat",              "triand",
	"casex",       "forever",      "negedge",         "rnmos",               "trior",
	"casez",       "fork",         "nmos",            "rpmos",               "trireg",
	"cell",        "function",     "nor",             "rtran",               "unsigned",
	"cmos",        "generate",     "noshowcancelled", "rtranif0",            "use",
	"config",      "genvar",       "not",             "rtranif1",            "uwire",
	"deassign",    "highz0",       "notif0",          "scalared",            "vectored",
	"default",     "highz1",       "notif1",          "showcancelled",       "wait",
	"defparam",    "if",           "or",              "signed",              "wand",
	"design",      "ifnone",       "output",          "small",               "weak0",
	"disable",     "incdir",       "parameter",       "specify",             "weak1",
	"edge",        "include",      "pmos",            "specparam",           "while",
	"else",        "initial",      "posedge",         "strong0",             "wire",
	"end",         "inout",        "primitive",       "strong1",             "wor",
	"endcase",     "input",        "pull0",           "supply0",             "xnor",
	"endconfig",   "instance",     "pull1",           "supply1",             "xor",
	"endfunction", "integer",      "pulldown",        "table",
};

/// IEEE 1800-2005 (SystemVerilog 2005) Keyword to [`Keyword`] token map
pub static SYSTEM_VERILOG_05_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"alias" => Keyword::Alias,
	"always_comb" => Keyword::AlwaysComb,
	"always_ff" => Keyword::AlwaysFF,
	"always_latch" => Keyword::AlwaysLatch,
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assert" => Keyword::Assert,
	"assign" => Keyword::Assign,
	"assume" => Keyword::Assume,
	"automatic" => Keyword::Automatic,
	"before" => Keyword::Before,
	"begin" => Keyword::Begin,
	"bind" => Keyword::Bind,
	"bins" => Keyword::Bins,
	"binsof" => Keyword::BinsOf,
	"bit" => Keyword::Bit,
	"break" => Keyword::Break,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"byte" => Keyword::Byte,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cell" => Keyword::Cell,
	"chandle" => Keyword::CHandle,
	"class" => Keyword::Class,
	"clocking" => Keyword::Clocking,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"const" => Keyword::Const,
	"constraint" => Keyword::Constraint,
	"context" => Keyword::Context,
	"continue" => Keyword::Continue,
	"cover" => Keyword::Cover,
	"covergroup" => Keyword::CoverGroup,
	"coverpoint" => Keyword::CoverPoint,
	"cross" => Keyword::Cross,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"dist" => Keyword::Dist,
	"do" => Keyword::Do,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endclass" => Keyword::EndClass,
	"endclocking" => Keyword::EndClocking,
	"endconfig" => Keyword::EndConfig,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endgroup" => Keyword::EndGroup,
	"endinterface" => Keyword::EndInterface,
	"endmodule" => Keyword::EndModule,
	"endpackage" => Keyword::EndPackage,
	"endprimitive" => Keyword::EndPrimitive,
	"endprogram" => Keyword::EndProgram,
	"endproperty" => Keyword::EndProperty,
	"endsequence" => Keyword::EndSequence,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"enum" => Keyword::Enum,
	"event" => Keyword::Event,
	"expect" => Keyword::Expect,
	"export" => Keyword::Export,
	"extends" => Keyword::Extends,
	"extern" => Keyword::Extern,
	"final" => Keyword::Final,
	"first_match" => Keyword::FirstMatch,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"foreach" => Keyword::ForEach,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"forkjoin" => Keyword::ForkJoin,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"iff" => Keyword::Iff,
	"ifnone" => Keyword::IfNone,
	"ignore_bins" => Keyword::IgnoreBins,
	"illegal_bins" => Keyword::IllegalBins,
	"import" => Keyword::Import,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"inside" => Keyword::Inside,
	"instance" => Keyword::Instance,
	"int" => Keyword::Int,
	"integer" => Keyword::Integer,
	"interface" => Keyword::Interface,
	"intersect" => Keyword::Intersect,
	"join_any" => Keyword::JoinAny,
	"join_none" => Keyword::JoinNone,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"local" => Keyword::Local,
	"localparam" => Keyword::LocalParam,
	"logic" => Keyword::Logic,
	"longint" => Keyword::LongInt,
	"macromodule" => Keyword::MacroModule,
	"matches" => Keyword::Matches,
	"medium" => Keyword::Medium,
	"modport" => Keyword::ModPort,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"new" => Keyword::New,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"null" => Keyword::Null,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"package" => Keyword::Package,
	"packed" => Keyword::Packed,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"priority" => Keyword::Priority,
	"program" => Keyword::Program,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"pure" => Keyword::Pure,
	"rand" => Keyword::Rand,
	"randc" => Keyword::RandC,
	"randcase" => Keyword::RandCase,
	"randsequence" => Keyword::RandSequence,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"ref" => Keyword::Ref,
	"reg" => Keyword::Reg,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"return" => Keyword::Return,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"scalared" => Keyword::Scalared,
	"sequence" => Keyword::Sequence,
	"shortint" => Keyword::ShortInt,
	"shortreal" => Keyword::ShortReal,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"small" => Keyword::Small,
	"solve" => Keyword::Solve,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"static" => Keyword::Static,
	"string" => Keyword::String,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"struct" => Keyword::Struct,
	"super" => Keyword::Super,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"table" => Keyword::Table,
	"tagged" => Keyword::Tagged,
	"task" => Keyword::Task,
	"this" => Keyword::This,
	"throughout" => Keyword::Throughout,
	"time" => Keyword::Time,
	"timeprecision" => Keyword::TimePrecision,
	"timeunit" => Keyword::TimeUnit,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"type" => Keyword::Type,
	"typedef" => Keyword::TypeDef,
	"union" => Keyword::Union,
	"unique" => Keyword::Unique,
	"unsigned" => Keyword::Unsigned,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"var" => Keyword::Var,
	"vectored" => Keyword::Vectored,
	"virtual" => Keyword::Virtual,
	"void" => Keyword::Void,
	"wait_order" => Keyword::WaitOrder,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wildcard" => Keyword::Wildcard,
	"wire" => Keyword::Wire,
	"with" => Keyword::With,
	"within" => Keyword::Within,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1800-2005 (SystemVerilog 2005) Keyword set
pub static SYSTEM_VERILOG_05_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"alias",        "edge",         "incdir",          "program",             "table",
	"always_comb",  "else",         "include",         "property",            "tagged",
	"always_ff",    "end",          "initial",         "protected",           "task",
	"always_latch", "endcase",      "inout",           "pull0",               "this",
	"always",       "endclass",     "input",           "pull1",               "throughout",
	"and",          "endclocking",  "inside",          "pulldown",            "time",
	"assert",       "endconfig",    "instance",        "pullup",              "timeprecision",
	"assign",       "endfunction",  "int",             "pulsestyle_ondetect", "timeunit",
	"assume",       "endgenerate",  "integer",         "pulsestyle_onevent",  "tran",
	"automatic",    "endgroup",     "interface",       "pure",                "tranif0",
	"before",       "endinterface", "intersect",       "rand",                "tranif1",
	"begin",        "endmodule",    "join_any",        "randc",               "tri",
	"bind",         "endpackage",   "join_none",       "randcase",            "tri0",
	"bins",         "endprimitive", "join",            "randsequence",        "tri1",
	"binsof",       "endprogram",   "large",           "rcmos",               "triand",
	"bit",          "endproperty",  "liblist",         "real",                "trior",
	"break",        "endsequence",  "library",         "realtime",            "trireg",
	"buf",          "endspecify",   "local",           "ref",                 "type",
	"bufif0",       "endtable",     "localparam",      "reg",                 "typedef",
	"bufif1",       "endtask",      "logic",           "release",             "union",
	"byte",         "enum",         "longint",         "repeat",              "unique",
	"case",         "event",        "macromodule",     "return",              "unsigned",
	"casex",        "expect",       "matches",         "rnmos",               "use",
	"casez",        "export",       "medium",          "rpmos",               "uwire",
	"cell",         "extends",      "modport",         "rtran",               "var",
	"chandle",      "extern",       "module",          "rtranif0",            "vectored",
	"class",        "final",        "nand",            "rtranif1",            "virtual",
	"clocking",     "first_match",  "negedge",         "scalared",            "void",
	"cmos",         "for",          "new",             "sequence",            "wait_order",
	"config",       "force",        "nmos",            "shortint",            "wait",
	"const",        "foreach",      "nor",             "shortreal",           "wand",
	"constraint",   "forever",      "noshowcancelled", "showcancelled",       "weak0",
	"context",      "fork",         "not",             "signed",              "weak1",
	"continue",     "forkjoin",     "notif0",          "small",               "while",
	"cover",        "function",     "notif1",          "solve",               "wildcard",
	"covergroup",   "generate",     "null",            "specify",             "wire",
	"coverpoint",   "genvar",       "or",              "specparam",           "with",
	"cross",        "highz0",       "output",          "static",              "within",
	"deassign",     "highz1",       "package",         "string",              "wor",
	"default",      "if",           "packed",          "strong0",             "xnor",
	"defparam",     "iff",          "parameter",       "strong1",             "xor",
	"design",       "ifnone",       "pmos",            "struct",
	"disable",      "ignore_bins",  "posedge",         "super",
	"dist",         "illegal_bins", "primitive",       "supply0",
	"do",           "import",       "priority",        "supply1",
};

/// IEEE 1800-2009 (SystemVerilog 2009) Keyword to [`Keyword`] token map
pub static SYSTEM_VERILOG_09_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"accept_on" => Keyword::AcceptOn,
	"alias" => Keyword::Alias,
	"always_comb" => Keyword::AlwaysComb,
	"always_ff" => Keyword::AlwaysFF,
	"always_latch" => Keyword::AlwaysLatch,
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assert" => Keyword::Assert,
	"assign" => Keyword::Assign,
	"assume" => Keyword::Assume,
	"automatic" => Keyword::Automatic,
	"before" => Keyword::Before,
	"begin" => Keyword::Begin,
	"bind" => Keyword::Bind,
	"bins" => Keyword::Bins,
	"binsof" => Keyword::BinsOf,
	"bit" => Keyword::Bit,
	"break" => Keyword::Break,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"byte" => Keyword::Byte,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cell" => Keyword::Cell,
	"chandle" => Keyword::CHandle,
	"checker" => Keyword::Checker,
	"class" => Keyword::Class,
	"clocking" => Keyword::Clocking,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"const" => Keyword::Const,
	"constraint" => Keyword::Constraint,
	"context" => Keyword::Context,
	"continue" => Keyword::Continue,
	"cover" => Keyword::Cover,
	"covergroup" => Keyword::CoverGroup,
	"coverpoint" => Keyword::CoverPoint,
	"cross" => Keyword::Cross,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"dist" => Keyword::Dist,
	"do" => Keyword::Do,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endchecker" => Keyword::EndChecker,
	"endclass" => Keyword::EndClass,
	"endclocking" => Keyword::EndClocking,
	"endconfig" => Keyword::EndConfig,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endgroup" => Keyword::EndGroup,
	"endinterface" => Keyword::EndInterface,
	"endmodule" => Keyword::EndModule,
	"endpackage" => Keyword::EndPackage,
	"endprimitive" => Keyword::EndPrimitive,
	"endprogram" => Keyword::EndProgram,
	"endproperty" => Keyword::EndProperty,
	"endsequence" => Keyword::EndSequence,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"enum" => Keyword::Enum,
	"event" => Keyword::Event,
	"eventually" => Keyword::Eventually,
	"expect" => Keyword::Expect,
	"export" => Keyword::Export,
	"extends" => Keyword::Extends,
	"extern" => Keyword::Extern,
	"final" => Keyword::Final,
	"first_match" => Keyword::FirstMatch,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"foreach" => Keyword::ForEach,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"forkjoin" => Keyword::ForkJoin,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"global" => Keyword::Global,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"iff" => Keyword::Iff,
	"ifnone" => Keyword::IfNone,
	"ignore_bins" => Keyword::IgnoreBins,
	"illegal_bins" => Keyword::IllegalBins,
	"implies" => Keyword::Implies,
	"import" => Keyword::Import,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"inside" => Keyword::Inside,
	"instance" => Keyword::Instance,
	"int" => Keyword::Int,
	"integer" => Keyword::Integer,
	"interface" => Keyword::Interface,
	"intersect" => Keyword::Intersect,
	"join_any" => Keyword::JoinAny,
	"join_none" => Keyword::JoinNone,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"let" => Keyword::Let,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"local" => Keyword::Local,
	"localparam" => Keyword::LocalParam,
	"logic" => Keyword::Logic,
	"longint" => Keyword::LongInt,
	"macromodule" => Keyword::MacroModule,
	"matches" => Keyword::Matches,
	"medium" => Keyword::Medium,
	"modport" => Keyword::ModPort,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"new" => Keyword::New,
	"nexttime" => Keyword::NextTime,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"null" => Keyword::Null,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"package" => Keyword::Package,
	"packed" => Keyword::Packed,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"priority" => Keyword::Priority,
	"program" => Keyword::Program,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"pure" => Keyword::Pure,
	"rand" => Keyword::Rand,
	"randc" => Keyword::RandC,
	"randcase" => Keyword::RandCase,
	"randsequence" => Keyword::RandSequence,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"ref" => Keyword::Ref,
	"reg" => Keyword::Reg,
	"reject_on" => Keyword::RejectOn,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"s_always" => Keyword::SAlways,
	"s_eventually" => Keyword::SEventually,
	"s_nexttime" => Keyword::SNextTime,
	"s_until_with" => Keyword::SUntilWith,
	"s_until" => Keyword::SUntil,
	"scalared" => Keyword::Scalared,
	"sequence" => Keyword::Sequence,
	"shortint" => Keyword::ShortInt,
	"shortreal" => Keyword::ShortReal,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"small" => Keyword::Small,
	"solve" => Keyword::Solve,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"static" => Keyword::Static,
	"string" => Keyword::String,
	"strong" => Keyword::Strong,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"struct" => Keyword::Struct,
	"super" => Keyword::Super,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"sync_accpet_on" => Keyword::SyncAcceptOn,
	"sync_reject_on" => Keyword::SyncRejectOn,
	"table" => Keyword::Table,
	"tagged" => Keyword::Tagged,
	"task" => Keyword::Task,
	"this" => Keyword::This,
	"throughout" => Keyword::Throughout,
	"time" => Keyword::Time,
	"timeprecision" => Keyword::TimePrecision,
	"timeunit" => Keyword::TimeUnit,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"type" => Keyword::Type,
	"typedef" => Keyword::TypeDef,
	"union" => Keyword::Union,
	"unique" => Keyword::Unique,
	"unique0" => Keyword::Unique0,
	"unsigned" => Keyword::Unsigned,
	"until_with" => Keyword::UntilWith,
	"until" => Keyword::Until,
	"untyped" => Keyword::Untyped,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"var" => Keyword::Var,
	"vectored" => Keyword::Vectored,
	"virtual" => Keyword::Virtual,
	"void" => Keyword::Void,
	"wait_order" => Keyword::WaitOrder,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak" => Keyword::Weak,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wildcard" => Keyword::Wildcard,
	"wire" => Keyword::Wire,
	"with" => Keyword::With,
	"within" => Keyword::Within,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1800-2009 (SystemVerilog 2009) Keyword set
pub static SYSTEM_VERILOG_09_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"accept_on",    "end",          "initial",         "pull1",               "sync_accpet_on",
	"alias",        "endcase",      "inout",           "pulldown",            "sync_reject_on",
	"always_comb",  "endchecker",   "input",           "pullup",              "table",
	"always_ff",    "endclass",     "inside",          "pulsestyle_ondetect", "tagged",
	"always_latch", "endclocking",  "instance",        "pulsestyle_onevent",  "task",
	"always",       "endconfig",    "int",             "pure",                "this",
	"and",          "endfunction",  "integer",         "rand",                "throughout",
	"assert",       "endgenerate",  "interface",       "randc",               "time",
	"assign",       "endgroup",     "intersect",       "randcase",            "timeprecision",
	"assume",       "endinterface", "join_any",        "randsequence",        "timeunit",
	"automatic",    "endmodule",    "join_none",       "rcmos",               "tran",
	"before",       "endpackage",   "join",            "real",                "tranif0",
	"begin",        "endprimitive", "large",           "realtime",            "tranif1",
	"bind",         "endprogram",   "let",             "ref",                 "tri",
	"bins",         "endproperty",  "liblist",         "reg",                 "tri0",
	"binsof",       "endsequence",  "library",         "reject_on",           "tri1",
	"bit",          "endspecify",   "local",           "release",             "triand",
	"break",        "endtable",     "localparam",      "repeat",              "trior",
	"buf",          "endtask",      "logic",           "restrict",            "trireg",
	"bufif0",       "enum",         "longint",         "return",              "type",
	"bufif1",       "event",        "macromodule",     "rnmos",               "typedef",
	"byte",         "eventually",   "matches",         "rpmos",               "union",
	"case",         "expect",       "medium",          "rtran",               "unique",
	"casex",        "export",       "modport",         "rtranif0",            "unique0",
	"casez",        "extends",      "module",          "rtranif1",            "unsigned",
	"cell",         "extern",       "nand",            "s_always",            "until_with",
	"chandle",      "final",        "negedge",         "s_eventually",        "until",
	"checker",      "first_match",  "new",             "s_nexttime",          "untyped",
	"class",        "for",          "nexttime",        "s_until_with",        "use",
	"clocking",     "force",        "nmos",            "s_until",             "uwire",
	"cmos",         "foreach",      "nor",             "scalared",            "var",
	"config",       "forever",      "noshowcancelled", "sequence",            "vectored",
	"const",        "fork",         "not",             "shortint",            "virtual",
	"constraint",   "forkjoin",     "notif0",          "shortreal",           "void",
	"context",      "function",     "notif1",          "showcancelled",       "wait_order",
	"continue",     "generate",     "null",            "signed",              "wait",
	"cover",        "genvar",       "or",              "small",               "wand",
	"covergroup",   "global",       "output",          "solve",               "weak",
	"coverpoint",   "highz0",       "package",         "specify",             "weak0",
	"cross",        "highz1",       "packed",          "specparam",           "weak1",
	"deassign",     "if",           "parameter",       "static",              "while",
	"default",      "iff",          "pmos",            "string",              "wildcard",
	"defparam",     "ifnone",       "posedge",         "strong",              "wire",
	"design",       "ignore_bins",  "primitive",       "strong0",             "with",
	"disable",      "illegal_bins", "priority",        "strong1",             "within",
	"dist",         "implies",      "program",         "struct",              "wor",
	"do",           "import",       "property",        "super",               "xnor",
	"edge",         "incdir",       "protected",       "supply0",             "xor",
	"else",         "include",      "pull0",           "supply1",
};

/// IEEE 1800-2012 (SystemVerilog 2012) Keyword to [`Keyword`] token map
pub static SYSTEM_VERILOG_12_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"accept_on" => Keyword::AcceptOn,
	"alias" => Keyword::Alias,
	"always_comb" => Keyword::AlwaysComb,
	"always_ff" => Keyword::AlwaysFF,
	"always_latch" => Keyword::AlwaysLatch,
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assert" => Keyword::Assert,
	"assign" => Keyword::Assign,
	"assume" => Keyword::Assume,
	"automatic" => Keyword::Automatic,
	"before" => Keyword::Before,
	"begin" => Keyword::Begin,
	"bind" => Keyword::Bind,
	"bins" => Keyword::Bins,
	"binsof" => Keyword::BinsOf,
	"bit" => Keyword::Bit,
	"break" => Keyword::Break,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"byte" => Keyword::Byte,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cell" => Keyword::Cell,
	"chandle" => Keyword::CHandle,
	"checker" => Keyword::Checker,
	"class" => Keyword::Class,
	"clocking" => Keyword::Clocking,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"const" => Keyword::Const,
	"constraint" => Keyword::Constraint,
	"context" => Keyword::Context,
	"continue" => Keyword::Continue,
	"cover" => Keyword::Cover,
	"covergroup" => Keyword::CoverGroup,
	"coverpoint" => Keyword::CoverPoint,
	"cross" => Keyword::Cross,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"dist" => Keyword::Dist,
	"do" => Keyword::Do,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endchecker" => Keyword::EndChecker,
	"endclass" => Keyword::EndClass,
	"endclocking" => Keyword::EndClocking,
	"endconfig" => Keyword::EndConfig,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endgroup" => Keyword::EndGroup,
	"endinterface" => Keyword::EndInterface,
	"endmodule" => Keyword::EndModule,
	"endpackage" => Keyword::EndPackage,
	"endprimitive" => Keyword::EndPrimitive,
	"endprogram" => Keyword::EndProgram,
	"endproperty" => Keyword::EndProperty,
	"endsequence" => Keyword::EndSequence,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"enum" => Keyword::Enum,
	"event" => Keyword::Event,
	"eventually" => Keyword::Eventually,
	"expect" => Keyword::Expect,
	"export" => Keyword::Export,
	"extends" => Keyword::Extends,
	"extern" => Keyword::Extern,
	"final" => Keyword::Final,
	"first_match" => Keyword::FirstMatch,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"foreach" => Keyword::ForEach,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"forkjoin" => Keyword::ForkJoin,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"global" => Keyword::Global,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"iff" => Keyword::Iff,
	"ifnone" => Keyword::IfNone,
	"ignore_bins" => Keyword::IgnoreBins,
	"illegal_bins" => Keyword::IllegalBins,
	"implements" => Keyword::Implements,
	"implies" => Keyword::Implies,
	"import" => Keyword::Import,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"inside" => Keyword::Inside,
	"instance" => Keyword::Instance,
	"int" => Keyword::Int,
	"integer" => Keyword::Integer,
	"interconnect" => Keyword::Interconnect,
	"interface" => Keyword::Interface,
	"intersect" => Keyword::Intersect,
	"join_any" => Keyword::JoinAny,
	"join_none" => Keyword::JoinNone,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"let" => Keyword::Let,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"local" => Keyword::Local,
	"localparam" => Keyword::LocalParam,
	"logic" => Keyword::Logic,
	"longint" => Keyword::LongInt,
	"macromodule" => Keyword::MacroModule,
	"matches" => Keyword::Matches,
	"medium" => Keyword::Medium,
	"modport" => Keyword::ModPort,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"nettype" => Keyword::NetType,
	"new" => Keyword::New,
	"nexttime" => Keyword::NextTime,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"null" => Keyword::Null,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"package" => Keyword::Package,
	"packed" => Keyword::Packed,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"priority" => Keyword::Priority,
	"program" => Keyword::Program,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"pure" => Keyword::Pure,
	"rand" => Keyword::Rand,
	"randc" => Keyword::RandC,
	"randcase" => Keyword::RandCase,
	"randsequence" => Keyword::RandSequence,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"ref" => Keyword::Ref,
	"reg" => Keyword::Reg,
	"reject_on" => Keyword::RejectOn,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"s_always" => Keyword::SAlways,
	"s_eventually" => Keyword::SEventually,
	"s_nexttime" => Keyword::SNextTime,
	"s_until_with" => Keyword::SUntilWith,
	"s_until" => Keyword::SUntil,
	"scalared" => Keyword::Scalared,
	"sequence" => Keyword::Sequence,
	"shortint" => Keyword::ShortInt,
	"shortreal" => Keyword::ShortReal,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"small" => Keyword::Small,
	"soft" => Keyword::Soft,
	"solve" => Keyword::Solve,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"static" => Keyword::Static,
	"string" => Keyword::String,
	"strong" => Keyword::Strong,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"struct" => Keyword::Struct,
	"super" => Keyword::Super,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"sync_accpet_on" => Keyword::SyncAcceptOn,
	"sync_reject_on" => Keyword::SyncRejectOn,
	"table" => Keyword::Table,
	"tagged" => Keyword::Tagged,
	"task" => Keyword::Task,
	"this" => Keyword::This,
	"throughout" => Keyword::Throughout,
	"time" => Keyword::Time,
	"timeprecision" => Keyword::TimePrecision,
	"timeunit" => Keyword::TimeUnit,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"type" => Keyword::Type,
	"typedef" => Keyword::TypeDef,
	"union" => Keyword::Union,
	"unique" => Keyword::Unique,
	"unique0" => Keyword::Unique0,
	"unsigned" => Keyword::Unsigned,
	"until_with" => Keyword::UntilWith,
	"until" => Keyword::Until,
	"untyped" => Keyword::Untyped,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"var" => Keyword::Var,
	"vectored" => Keyword::Vectored,
	"virtual" => Keyword::Virtual,
	"void" => Keyword::Void,
	"wait_order" => Keyword::WaitOrder,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak" => Keyword::Weak,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wildcard" => Keyword::Wildcard,
	"wire" => Keyword::Wire,
	"with" => Keyword::With,
	"within" => Keyword::Within,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1800-2012 (SystemVerilog 2012) Keyword set
pub static SYSTEM_VERILOG_12_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"accept_on",    "endcase",      "inout",           "pull1",               "sync_accpet_on",
	"alias",        "endchecker",   "input",           "pulldown",            "sync_reject_on",
	"always_comb",  "endclass",     "inside",          "pullup",              "table",
	"always_ff",    "endclocking",  "instance",        "pulsestyle_ondetect", "tagged",
	"always_latch", "endconfig",    "int",             "pulsestyle_onevent",  "task",
	"always",       "endfunction",  "integer",         "pure",                "this",
	"and",          "endgenerate",  "interconnect",    "rand",                "throughout",
	"assert",       "endgroup",     "interface",       "randc",               "time",
	"assign",       "endinterface", "intersect",       "randcase",            "timeprecision",
	"assume",       "endmodule",    "join_any",        "randsequence",        "timeunit",
	"automatic",    "endpackage",   "join_none",       "rcmos",               "tran",
	"before",       "endprimitive", "join",            "real",                "tranif0",
	"begin",        "endprogram",   "large",           "realtime",            "tranif1",
	"bind",         "endproperty",  "let",             "ref",                 "tri",
	"bins",         "endsequence",  "liblist",         "reg",                 "tri0",
	"binsof",       "endspecify",   "library",         "reject_on",           "tri1",
	"bit",          "endtable",     "local",           "release",             "triand",
	"break",        "endtask",      "localparam",      "repeat",              "trior",
	"buf",          "enum",         "logic",           "restrict",            "trireg",
	"bufif0",       "event",        "longint",         "return",              "type",
	"bufif1",       "eventually",   "macromodule",     "rnmos",               "typedef",
	"byte",         "expect",       "matches",         "rpmos",               "union",
	"case",         "export",       "medium",          "rtran",               "unique",
	"casex",        "extends",      "modport",         "rtranif0",            "unique0",
	"casez",        "extern",       "module",          "rtranif1",            "unsigned",
	"cell",         "final",        "nand",            "s_always",            "until_with",
	"chandle",      "first_match",  "negedge",         "s_eventually",        "until",
	"checker",      "for",          "nettype",         "s_nexttime",          "untyped",
	"class",        "force",        "new",             "s_until_with",        "use",
	"clocking",     "foreach",      "nexttime",        "s_until",             "uwire",
	"cmos",         "forever",      "nmos",            "scalared",            "var",
	"config",       "fork",         "nor",             "sequence",            "vectored",
	"const",        "forkjoin",     "noshowcancelled", "shortint",            "virtual",
	"constraint",   "function",     "not",             "shortreal",           "void",
	"context",      "generate",     "notif0",          "showcancelled",       "wait_order",
	"continue",     "genvar",       "notif1",          "signed",              "wait",
	"cover",        "global",       "null",            "small",               "wand",
	"covergroup",   "highz0",       "or",              "soft",                "weak",
	"coverpoint",   "highz1",       "output",          "solve",               "weak0",
	"cross",        "if",           "package",         "specify",             "weak1",
	"deassign",     "iff",          "packed",          "specparam",           "while",
	"default",      "ifnone",       "parameter",       "static",              "wildcard",
	"defparam",     "ignore_bins",  "pmos",            "string",              "wire",
	"design",       "illegal_bins", "posedge",         "strong",              "with",
	"disable",      "implements",   "primitive",       "strong0",             "within",
	"dist",         "implies",      "priority",        "strong1",             "wor",
	"do",           "import",       "program",         "struct",              "xnor",
	"edge",         "incdir",       "property",        "super",               "xor",
	"else",         "include",      "protected",       "supply0",
	"end",          "initial",      "pull0",           "supply1",
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_12_KEYWORD_MAP`
/// IEEE 1800-2017 (SystemVerilog 2017) Keyword to [`Keyword`] token map
pub static SYSTEM_VERILOG_17_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"accept_on" => Keyword::AcceptOn,
	"alias" => Keyword::Alias,
	"always_comb" => Keyword::AlwaysComb,
	"always_ff" => Keyword::AlwaysFF,
	"always_latch" => Keyword::AlwaysLatch,
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assert" => Keyword::Assert,
	"assign" => Keyword::Assign,
	"assume" => Keyword::Assume,
	"automatic" => Keyword::Automatic,
	"before" => Keyword::Before,
	"begin" => Keyword::Begin,
	"bind" => Keyword::Bind,
	"bins" => Keyword::Bins,
	"binsof" => Keyword::BinsOf,
	"bit" => Keyword::Bit,
	"break" => Keyword::Break,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"byte" => Keyword::Byte,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cell" => Keyword::Cell,
	"chandle" => Keyword::CHandle,
	"checker" => Keyword::Checker,
	"class" => Keyword::Class,
	"clocking" => Keyword::Clocking,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"const" => Keyword::Const,
	"constraint" => Keyword::Constraint,
	"context" => Keyword::Context,
	"continue" => Keyword::Continue,
	"cover" => Keyword::Cover,
	"covergroup" => Keyword::CoverGroup,
	"coverpoint" => Keyword::CoverPoint,
	"cross" => Keyword::Cross,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"dist" => Keyword::Dist,
	"do" => Keyword::Do,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endchecker" => Keyword::EndChecker,
	"endclass" => Keyword::EndClass,
	"endclocking" => Keyword::EndClocking,
	"endconfig" => Keyword::EndConfig,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endgroup" => Keyword::EndGroup,
	"endinterface" => Keyword::EndInterface,
	"endmodule" => Keyword::EndModule,
	"endpackage" => Keyword::EndPackage,
	"endprimitive" => Keyword::EndPrimitive,
	"endprogram" => Keyword::EndProgram,
	"endproperty" => Keyword::EndProperty,
	"endsequence" => Keyword::EndSequence,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"enum" => Keyword::Enum,
	"event" => Keyword::Event,
	"eventually" => Keyword::Eventually,
	"expect" => Keyword::Expect,
	"export" => Keyword::Export,
	"extends" => Keyword::Extends,
	"extern" => Keyword::Extern,
	"final" => Keyword::Final,
	"first_match" => Keyword::FirstMatch,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"foreach" => Keyword::ForEach,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"forkjoin" => Keyword::ForkJoin,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"global" => Keyword::Global,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"iff" => Keyword::Iff,
	"ifnone" => Keyword::IfNone,
	"ignore_bins" => Keyword::IgnoreBins,
	"illegal_bins" => Keyword::IllegalBins,
	"implements" => Keyword::Implements,
	"implies" => Keyword::Implies,
	"import" => Keyword::Import,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"inside" => Keyword::Inside,
	"instance" => Keyword::Instance,
	"int" => Keyword::Int,
	"integer" => Keyword::Integer,
	"interconnect" => Keyword::Interconnect,
	"interface" => Keyword::Interface,
	"intersect" => Keyword::Intersect,
	"join_any" => Keyword::JoinAny,
	"join_none" => Keyword::JoinNone,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"let" => Keyword::Let,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"local" => Keyword::Local,
	"localparam" => Keyword::LocalParam,
	"logic" => Keyword::Logic,
	"longint" => Keyword::LongInt,
	"macromodule" => Keyword::MacroModule,
	"matches" => Keyword::Matches,
	"medium" => Keyword::Medium,
	"modport" => Keyword::ModPort,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"nettype" => Keyword::NetType,
	"new" => Keyword::New,
	"nexttime" => Keyword::NextTime,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"null" => Keyword::Null,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"package" => Keyword::Package,
	"packed" => Keyword::Packed,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"priority" => Keyword::Priority,
	"program" => Keyword::Program,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"pure" => Keyword::Pure,
	"rand" => Keyword::Rand,
	"randc" => Keyword::RandC,
	"randcase" => Keyword::RandCase,
	"randsequence" => Keyword::RandSequence,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"ref" => Keyword::Ref,
	"reg" => Keyword::Reg,
	"reject_on" => Keyword::RejectOn,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"s_always" => Keyword::SAlways,
	"s_eventually" => Keyword::SEventually,
	"s_nexttime" => Keyword::SNextTime,
	"s_until_with" => Keyword::SUntilWith,
	"s_until" => Keyword::SUntil,
	"scalared" => Keyword::Scalared,
	"sequence" => Keyword::Sequence,
	"shortint" => Keyword::ShortInt,
	"shortreal" => Keyword::ShortReal,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"small" => Keyword::Small,
	"soft" => Keyword::Soft,
	"solve" => Keyword::Solve,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"static" => Keyword::Static,
	"string" => Keyword::String,
	"strong" => Keyword::Strong,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"struct" => Keyword::Struct,
	"super" => Keyword::Super,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"sync_accpet_on" => Keyword::SyncAcceptOn,
	"sync_reject_on" => Keyword::SyncRejectOn,
	"table" => Keyword::Table,
	"tagged" => Keyword::Tagged,
	"task" => Keyword::Task,
	"this" => Keyword::This,
	"throughout" => Keyword::Throughout,
	"time" => Keyword::Time,
	"timeprecision" => Keyword::TimePrecision,
	"timeunit" => Keyword::TimeUnit,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"type" => Keyword::Type,
	"typedef" => Keyword::TypeDef,
	"union" => Keyword::Union,
	"unique" => Keyword::Unique,
	"unique0" => Keyword::Unique0,
	"unsigned" => Keyword::Unsigned,
	"until_with" => Keyword::UntilWith,
	"until" => Keyword::Until,
	"untyped" => Keyword::Untyped,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"var" => Keyword::Var,
	"vectored" => Keyword::Vectored,
	"virtual" => Keyword::Virtual,
	"void" => Keyword::Void,
	"wait_order" => Keyword::WaitOrder,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak" => Keyword::Weak,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wildcard" => Keyword::Wildcard,
	"wire" => Keyword::Wire,
	"with" => Keyword::With,
	"within" => Keyword::Within,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_12_KEYWORD_SET`
/// IEEE 1800-2017 (SystemVerilog 2017) Keyword set
pub static SYSTEM_VERILOG_17_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"accept_on",    "endcase",      "inout",           "pull1",               "sync_accpet_on",
	"alias",        "endchecker",   "input",           "pulldown",            "sync_reject_on",
	"always_comb",  "endclass",     "inside",          "pullup",              "table",
	"always_ff",    "endclocking",  "instance",        "pulsestyle_ondetect", "tagged",
	"always_latch", "endconfig",    "int",             "pulsestyle_onevent",  "task",
	"always",       "endfunction",  "integer",         "pure",                "this",
	"and",          "endgenerate",  "interconnect",    "rand",                "throughout",
	"assert",       "endgroup",     "interface",       "randc",               "time",
	"assign",       "endinterface", "intersect",       "randcase",            "timeprecision",
	"assume",       "endmodule",    "join_any",        "randsequence",        "timeunit",
	"automatic",    "endpackage",   "join_none",       "rcmos",               "tran",
	"before",       "endprimitive", "join",            "real",                "tranif0",
	"begin",        "endprogram",   "large",           "realtime",            "tranif1",
	"bind",         "endproperty",  "let",             "ref",                 "tri",
	"bins",         "endsequence",  "liblist",         "reg",                 "tri0",
	"binsof",       "endspecify",   "library",         "reject_on",           "tri1",
	"bit",          "endtable",     "local",           "release",             "triand",
	"break",        "endtask",      "localparam",      "repeat",              "trior",
	"buf",          "enum",         "logic",           "restrict",            "trireg",
	"bufif0",       "event",        "longint",         "return",              "type",
	"bufif1",       "eventually",   "macromodule",     "rnmos",               "typedef",
	"byte",         "expect",       "matches",         "rpmos",               "union",
	"case",         "export",       "medium",          "rtran",               "unique",
	"casex",        "extends",      "modport",         "rtranif0",            "unique0",
	"casez",        "extern",       "module",          "rtranif1",            "unsigned",
	"cell",         "final",        "nand",            "s_always",            "until_with",
	"chandle",      "first_match",  "negedge",         "s_eventually",        "until",
	"checker",      "for",          "nettype",         "s_nexttime",          "untyped",
	"class",        "force",        "new",             "s_until_with",        "use",
	"clocking",     "foreach",      "nexttime",        "s_until",             "uwire",
	"cmos",         "forever",      "nmos",            "scalared",            "var",
	"config",       "fork",         "nor",             "sequence",            "vectored",
	"const",        "forkjoin",     "noshowcancelled", "shortint",            "virtual",
	"constraint",   "function",     "not",             "shortreal",           "void",
	"context",      "generate",     "notif0",          "showcancelled",       "wait_order",
	"continue",     "genvar",       "notif1",          "signed",              "wait",
	"cover",        "global",       "null",            "small",               "wand",
	"covergroup",   "highz0",       "or",              "soft",                "weak",
	"coverpoint",   "highz1",       "output",          "solve",               "weak0",
	"cross",        "if",           "package",         "specify",             "weak1",
	"deassign",     "iff",          "packed",          "specparam",           "while",
	"default",      "ifnone",       "parameter",       "static",              "wildcard",
	"defparam",     "ignore_bins",  "pmos",            "string",              "wire",
	"design",       "illegal_bins", "posedge",         "strong",              "with",
	"disable",      "implements",   "primitive",       "strong0",             "within",
	"dist",         "implies",      "priority",        "strong1",             "wor",
	"do",           "import",       "program",         "struct",              "xnor",
	"edge",         "incdir",       "property",        "super",               "xor",
	"else",         "include",      "protected",       "supply0",
	"end",          "initial",      "pull0",           "supply1",
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_12_KEYWORD_MAP`
/// IEEE 1800-2023 (SystemVerilog 2023) Keyword to [`Keyword`] token map
pub static SYSTEM_VERILOG_23_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"accept_on" => Keyword::AcceptOn,
	"alias" => Keyword::Alias,
	"always_comb" => Keyword::AlwaysComb,
	"always_ff" => Keyword::AlwaysFF,
	"always_latch" => Keyword::AlwaysLatch,
	"always" => Keyword::Always,
	"and" => Keyword::And,
	"assert" => Keyword::Assert,
	"assign" => Keyword::Assign,
	"assume" => Keyword::Assume,
	"automatic" => Keyword::Automatic,
	"before" => Keyword::Before,
	"begin" => Keyword::Begin,
	"bind" => Keyword::Bind,
	"bins" => Keyword::Bins,
	"binsof" => Keyword::BinsOf,
	"bit" => Keyword::Bit,
	"break" => Keyword::Break,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"byte" => Keyword::Byte,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"cell" => Keyword::Cell,
	"chandle" => Keyword::CHandle,
	"checker" => Keyword::Checker,
	"class" => Keyword::Class,
	"clocking" => Keyword::Clocking,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"const" => Keyword::Const,
	"constraint" => Keyword::Constraint,
	"context" => Keyword::Context,
	"continue" => Keyword::Continue,
	"cover" => Keyword::Cover,
	"covergroup" => Keyword::CoverGroup,
	"coverpoint" => Keyword::CoverPoint,
	"cross" => Keyword::Cross,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"dist" => Keyword::Dist,
	"do" => Keyword::Do,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endchecker" => Keyword::EndChecker,
	"endclass" => Keyword::EndClass,
	"endclocking" => Keyword::EndClocking,
	"endconfig" => Keyword::EndConfig,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endgroup" => Keyword::EndGroup,
	"endinterface" => Keyword::EndInterface,
	"endmodule" => Keyword::EndModule,
	"endpackage" => Keyword::EndPackage,
	"endprimitive" => Keyword::EndPrimitive,
	"endprogram" => Keyword::EndProgram,
	"endproperty" => Keyword::EndProperty,
	"endsequence" => Keyword::EndSequence,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"enum" => Keyword::Enum,
	"event" => Keyword::Event,
	"eventually" => Keyword::Eventually,
	"expect" => Keyword::Expect,
	"export" => Keyword::Export,
	"extends" => Keyword::Extends,
	"extern" => Keyword::Extern,
	"final" => Keyword::Final,
	"first_match" => Keyword::FirstMatch,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"foreach" => Keyword::ForEach,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"forkjoin" => Keyword::ForkJoin,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"global" => Keyword::Global,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"if" => Keyword::If,
	"iff" => Keyword::Iff,
	"ifnone" => Keyword::IfNone,
	"ignore_bins" => Keyword::IgnoreBins,
	"illegal_bins" => Keyword::IllegalBins,
	"implements" => Keyword::Implements,
	"implies" => Keyword::Implies,
	"import" => Keyword::Import,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"inside" => Keyword::Inside,
	"instance" => Keyword::Instance,
	"int" => Keyword::Int,
	"integer" => Keyword::Integer,
	"interconnect" => Keyword::Interconnect,
	"interface" => Keyword::Interface,
	"intersect" => Keyword::Intersect,
	"join_any" => Keyword::JoinAny,
	"join_none" => Keyword::JoinNone,
	"join" => Keyword::Join,
	"large" => Keyword::Large,
	"let" => Keyword::Let,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"local" => Keyword::Local,
	"localparam" => Keyword::LocalParam,
	"logic" => Keyword::Logic,
	"longint" => Keyword::LongInt,
	"macromodule" => Keyword::MacroModule,
	"matches" => Keyword::Matches,
	"medium" => Keyword::Medium,
	"modport" => Keyword::ModPort,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"nettype" => Keyword::NetType,
	"new" => Keyword::New,
	"nexttime" => Keyword::NextTime,
	"nmos" => Keyword::Nmos,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"null" => Keyword::Null,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"package" => Keyword::Package,
	"packed" => Keyword::Packed,
	"parameter" => Keyword::Parameter,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"primitive" => Keyword::Primitive,
	"priority" => Keyword::Priority,
	"program" => Keyword::Program,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"pure" => Keyword::Pure,
	"rand" => Keyword::Rand,
	"randc" => Keyword::RandC,
	"randcase" => Keyword::RandCase,
	"randsequence" => Keyword::RandSequence,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"ref" => Keyword::Ref,
	"reg" => Keyword::Reg,
	"reject_on" => Keyword::RejectOn,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"s_always" => Keyword::SAlways,
	"s_eventually" => Keyword::SEventually,
	"s_nexttime" => Keyword::SNextTime,
	"s_until_with" => Keyword::SUntilWith,
	"s_until" => Keyword::SUntil,
	"scalared" => Keyword::Scalared,
	"sequence" => Keyword::Sequence,
	"shortint" => Keyword::ShortInt,
	"shortreal" => Keyword::ShortReal,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"small" => Keyword::Small,
	"soft" => Keyword::Soft,
	"solve" => Keyword::Solve,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"static" => Keyword::Static,
	"string" => Keyword::String,
	"strong" => Keyword::Strong,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"struct" => Keyword::Struct,
	"super" => Keyword::Super,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"sync_accpet_on" => Keyword::SyncAcceptOn,
	"sync_reject_on" => Keyword::SyncRejectOn,
	"table" => Keyword::Table,
	"tagged" => Keyword::Tagged,
	"task" => Keyword::Task,
	"this" => Keyword::This,
	"throughout" => Keyword::Throughout,
	"time" => Keyword::Time,
	"timeprecision" => Keyword::TimePrecision,
	"timeunit" => Keyword::TimeUnit,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"type" => Keyword::Type,
	"typedef" => Keyword::TypeDef,
	"union" => Keyword::Union,
	"unique" => Keyword::Unique,
	"unique0" => Keyword::Unique0,
	"unsigned" => Keyword::Unsigned,
	"until_with" => Keyword::UntilWith,
	"until" => Keyword::Until,
	"untyped" => Keyword::Untyped,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"var" => Keyword::Var,
	"vectored" => Keyword::Vectored,
	"virtual" => Keyword::Virtual,
	"void" => Keyword::Void,
	"wait_order" => Keyword::WaitOrder,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak" => Keyword::Weak,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"wildcard" => Keyword::Wildcard,
	"wire" => Keyword::Wire,
	"with" => Keyword::With,
	"within" => Keyword::Within,
	"wor" => Keyword::Wor,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_12_KEYWORD_SET`
/// IEEE 1800-2023 (SystemVerilog 2023) Keyword set
pub static SYSTEM_VERILOG_23_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"accept_on",    "endcase",      "inout",           "pull1",               "sync_accpet_on",
	"alias",        "endchecker",   "input",           "pulldown",            "sync_reject_on",
	"always_comb",  "endclass",     "inside",          "pullup",              "table",
	"always_ff",    "endclocking",  "instance",        "pulsestyle_ondetect", "tagged",
	"always_latch", "endconfig",    "int",             "pulsestyle_onevent",  "task",
	"always",       "endfunction",  "integer",         "pure",                "this",
	"and",          "endgenerate",  "interconnect",    "rand",                "throughout",
	"assert",       "endgroup",     "interface",       "randc",               "time",
	"assign",       "endinterface", "intersect",       "randcase",            "timeprecision",
	"assume",       "endmodule",    "join_any",        "randsequence",        "timeunit",
	"automatic",    "endpackage",   "join_none",       "rcmos",               "tran",
	"before",       "endprimitive", "join",            "real",                "tranif0",
	"begin",        "endprogram",   "large",           "realtime",            "tranif1",
	"bind",         "endproperty",  "let",             "ref",                 "tri",
	"bins",         "endsequence",  "liblist",         "reg",                 "tri0",
	"binsof",       "endspecify",   "library",         "reject_on",           "tri1",
	"bit",          "endtable",     "local",           "release",             "triand",
	"break",        "endtask",      "localparam",      "repeat",              "trior",
	"buf",          "enum",         "logic",           "restrict",            "trireg",
	"bufif0",       "event",        "longint",         "return",              "type",
	"bufif1",       "eventually",   "macromodule",     "rnmos",               "typedef",
	"byte",         "expect",       "matches",         "rpmos",               "union",
	"case",         "export",       "medium",          "rtran",               "unique",
	"casex",        "extends",      "modport",         "rtranif0",            "unique0",
	"casez",        "extern",       "module",          "rtranif1",            "unsigned",
	"cell",         "final",        "nand",            "s_always",            "until_with",
	"chandle",      "first_match",  "negedge",         "s_eventually",        "until",
	"checker",      "for",          "nettype",         "s_nexttime",          "untyped",
	"class",        "force",        "new",             "s_until_with",        "use",
	"clocking",     "foreach",      "nexttime",        "s_until",             "uwire",
	"cmos",         "forever",      "nmos",            "scalared",            "var",
	"config",       "fork",         "nor",             "sequence",            "vectored",
	"const",        "forkjoin",     "noshowcancelled", "shortint",            "virtual",
	"constraint",   "function",     "not",             "shortreal",           "void",
	"context",      "generate",     "notif0",          "showcancelled",       "wait_order",
	"continue",     "genvar",       "notif1",          "signed",              "wait",
	"cover",        "global",       "null",            "small",               "wand",
	"covergroup",   "highz0",       "or",              "soft",                "weak",
	"coverpoint",   "highz1",       "output",          "solve",               "weak0",
	"cross",        "if",           "package",         "specify",             "weak1",
	"deassign",     "iff",          "packed",          "specparam",           "while",
	"default",      "ifnone",       "parameter",       "static",              "wildcard",
	"defparam",     "ignore_bins",  "pmos",            "string",              "wire",
	"design",       "illegal_bins", "posedge",         "strong",              "with",
	"disable",      "implements",   "primitive",       "strong0",             "within",
	"dist",         "implies",      "priority",        "strong1",             "wor",
	"do",           "import",       "program",         "struct",              "xnor",
	"edge",         "incdir",       "property",        "super",               "xor",
	"else",         "include",      "protected",       "supply0",
	"end",          "initial",      "pull0",           "supply1",
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Keyword to [`Keyword`] token map
pub static VERILOG_AMS_09_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"above" => Keyword::Above,
	"abs" => Keyword::Abs,
	"absdelay" => Keyword::AbsDelay,
	"abstol" => Keyword::AbsTol,
	"ac_stim" => Keyword::AcStim,
	"access" => Keyword::Access,
	"acos" => Keyword::Acos,
	"acosh" => Keyword::Acosh,
	"aliasparam" => Keyword::Aliasparam,
	"always" => Keyword::Always,
	"analog" => Keyword::Analog,
	"analysis" => Keyword::Analysis,
	"and" => Keyword::And,
	"asin" => Keyword::Asin,
	"asinh" => Keyword::Asinh,
	"assert" => Keyword::Assert,
	"assign" => Keyword::Assign,
	"atan" => Keyword::Atan,
	"atan2" => Keyword::Atan2,
	"atanh" => Keyword::Atanh,
	"automatic" => Keyword::Automatic,
	"begin" => Keyword::Begin,
	"branch" => Keyword::Branch,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"ceil" => Keyword::Ceil,
	"cell" => Keyword::Cell,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"connect" => Keyword::Connect,
	"connectmodule" => Keyword::ConnectModule,
	"connectrules" => Keyword::ConnectRules,
	"continuous" => Keyword::Continuous,
	"cos" => Keyword::Cos,
	"cosh" => Keyword::Cosh,
	"cross" => Keyword::Cross,
	"ddt_nature" => Keyword::DdtNature,
	"ddt" => Keyword::Ddt,
	"ddx" => Keyword::Ddx,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"discipline" => Keyword::Discipline,
	"discrete" => Keyword::Discrete,
	"domain" => Keyword::Domain,
	"driver_update" => Keyword::DriverUpdate,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endconfig" => Keyword::EndConfig,
	"endconnectrules" => Keyword::EndConnectRules,
	"enddiscipline" => Keyword::EndDiscipline,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endmodule" => Keyword::EndModule,
	"endnature" => Keyword::EndNature,
	"endparamset" => Keyword::EndParamSet,
	"endprimitive" => Keyword::EndPrimitive,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"event" => Keyword::Event,
	"exclude" => Keyword::Exclude,
	"exp" => Keyword::Exp,
	"final_step" => Keyword::FinalStep,
	"flicker_noise" => Keyword::FlickerNoise,
	"floor" => Keyword::Floor,
	"flow" => Keyword::Flow,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"from" => Keyword::From,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"ground" => Keyword::Ground,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"hypot" => Keyword::Hypot,
	"idt_nature" => Keyword::IdtNature,
	"idt" => Keyword::Idt,
	"idtmod" => Keyword::IdtMod,
	"if" => Keyword::If,
	"ifnone" => Keyword::IfNone,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"inf" => Keyword::Inf,
	"initial_step" => Keyword::InitialStep,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"instance" => Keyword::Instance,
	"integer" => Keyword::Integer,
	"join" => Keyword::Join,
	"laplace_nd" => Keyword::LaplaceNd,
	"laplace_np" => Keyword::LaplaceNp,
	"laplace_zd" => Keyword::LaplaceZd,
	"laplace_zp" => Keyword::LaplaceZp,
	"large" => Keyword::Large,
	"last_crossing" => Keyword::LastCrossing,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"limexp" => Keyword::LimExp,
	"ln" => Keyword::Ln,
	"localparam" => Keyword::LocalParam,
	"log" => Keyword::Log,
	"macromodule" => Keyword::MacroModule,
	"max" => Keyword::Max,
	"medium" => Keyword::Medium,
	"merged" => Keyword::Merged,
	"min" => Keyword::Min,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"net_resolution" => Keyword::NetResolution,
	"nmos" => Keyword::Nmos,
	"noise_table" => Keyword::NoiseTable,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"parameter" => Keyword::Parameter,
	"paramset" => Keyword::ParamSet,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"potential" => Keyword::Potential,
	"pow" => Keyword::Pow,
	"primitive" => Keyword::Primitive,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"reg" => Keyword::Reg,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"resolveto" => Keyword::ResolveTo,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"scalared" => Keyword::Scalared,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"sin" => Keyword::Sin,
	"sinh" => Keyword::Sinh,
	"slew" => Keyword::Slew,
	"small" => Keyword::Small,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"split" => Keyword::Split,
	"sqrt" => Keyword::Sqrt,
	"string" => Keyword::String,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"table" => Keyword::Table,
	"tan" => Keyword::Tan,
	"tanh" => Keyword::Tanh,
	"task" => Keyword::Task,
	"time" => Keyword::Time,
	"timer" => Keyword::Timer,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"transition" => Keyword::Transition,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"units" => Keyword::Units,
	"unsigned" => Keyword::Unsigned,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"vectored" => Keyword::Vectored,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"white_noise" => Keyword::WhiteNoise,
	"wire" => Keyword::Wire,
	"wor" => Keyword::Wor,
	"wreal" => Keyword::Wreal,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
	"zi_nd" => Keyword::ZiNd,
	"zi_np" => Keyword::ZiNp,
	"zi_zd" => Keyword::ZiZd,
	"zi_zp" => Keyword::ZiZp,
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Keyword set
pub static VERILOG_AMS_09_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"above",         "ddx",             "highz0",         "noshowcancelled",     "string",
	"abs",           "deassign",        "highz1",         "not",                 "strong0",
	"absdelay",      "default",         "hypot",          "notif0",              "strong1",
	"abstol",        "defparam",        "idt_nature",     "notif1",              "supply0",
	"ac_stim",       "design",          "idt",            "or",                  "supply1",
	"access",        "disable",         "idtmod",         "output",              "table",
	"acos",          "discipline",      "if",             "parameter",           "tan",
	"acosh",         "discrete",        "ifnone",         "paramset",            "tanh",
	"aliasparam",    "domain",          "incdir",         "pmos",                "task",
	"always",        "driver_update",   "include",        "posedge",             "time",
	"analog",        "edge",            "inf",            "potential",           "timer",
	"analysis",      "else",            "initial_step",   "pow",                 "tran",
	"and",           "end",             "initial",        "primitive",           "tranif0",
	"asin",          "endcase",         "inout",          "pull0",               "tranif1",
	"asinh",         "endconfig",       "input",          "pull1",               "transition",
	"assert",        "endconnectrules", "instance",       "pulldown",            "tri",
	"assign",        "enddiscipline",   "integer",        "pullup",              "tri0",
	"atan",          "endfunction",     "join",           "pulsestyle_ondetect", "tri1",
	"atan2",         "endgenerate",     "laplace_nd",     "pulsestyle_onevent",  "triand",
	"atanh",         "endmodule",       "laplace_np",     "rcmos",               "trior",
	"automatic",     "endnature",       "laplace_zd",     "real",                "trireg",
	"begin",         "endparamset",     "laplace_zp",     "realtime",            "units",
	"branch",        "endprimitive",    "large",          "reg",                 "unsigned",
	"buf",           "endspecify",      "last_crossing",  "release",             "use",
	"bufif0",        "endtable",        "liblist",        "repeat",              "uwire",
	"bufif1",        "endtask",         "library",        "resolveto",           "vectored",
	"case",          "event",           "limexp",         "rnmos",               "wait",
	"casex",         "exclude",         "ln",             "rpmos",               "wand",
	"casez",         "exp",             "localparam",     "rtran",               "weak0",
	"ceil",          "final_step",      "log",            "rtranif0",            "weak1",
	"cell",          "flicker_noise",   "macromodule",    "rtranif1",            "while",
	"cmos",          "floor",           "max",            "scalared",            "white_noise",
	"config",        "flow",            "medium",         "showcancelled",       "wire",
	"connect",       "for",             "merged",         "signed",              "wor",
	"connectmodule", "force",           "min",            "sin",                 "wreal",
	"connectrules",  "forever",         "module",         "sinh",                "xnor",
	"continuous",    "fork",            "nand",           "slew",                "xor",
	"cos",           "from",            "negedge",        "small",               "zi_nd",
	"cosh",          "function",        "net_resolution", "specify",             "zi_np",
	"cross",         "generate",        "nmos",           "specparam",           "zi_zd",
	"ddt_nature",    "genvar",          "noise_table",    "split",               "zi_zp",
	"ddt",           "ground",          "nor",            "sqrt",
};

/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Keyword to [`Keyword`] token map
pub static VERILOG_AMS_14_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"above" => Keyword::Above,
	"abs" => Keyword::Abs,
	"absdelay" => Keyword::AbsDelay,
	"absdelta" => Keyword::AbsDelta,
	"abstol" => Keyword::AbsTol,
	"ac_stim" => Keyword::AcStim,
	"access" => Keyword::Access,
	"acos" => Keyword::Acos,
	"acosh" => Keyword::Acosh,
	"aliasparam" => Keyword::Aliasparam,
	"always" => Keyword::Always,
	"analog" => Keyword::Analog,
	"analysis" => Keyword::Analysis,
	"and" => Keyword::And,
	"asin" => Keyword::Asin,
	"asinh" => Keyword::Asinh,
	"assert" => Keyword::Assert,
	"assign" => Keyword::Assign,
	"atan" => Keyword::Atan,
	"atan2" => Keyword::Atan2,
	"atanh" => Keyword::Atanh,
	"automatic" => Keyword::Automatic,
	"begin" => Keyword::Begin,
	"branch" => Keyword::Branch,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"ceil" => Keyword::Ceil,
	"cell" => Keyword::Cell,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"connect" => Keyword::Connect,
	"connectmodule" => Keyword::ConnectModule,
	"connectrules" => Keyword::ConnectRules,
	"continuous" => Keyword::Continuous,
	"cos" => Keyword::Cos,
	"cosh" => Keyword::Cosh,
	"cross" => Keyword::Cross,
	"ddt_nature" => Keyword::DdtNature,
	"ddt" => Keyword::Ddt,
	"ddx" => Keyword::Ddx,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"discipline" => Keyword::Discipline,
	"discrete" => Keyword::Discrete,
	"domain" => Keyword::Domain,
	"driver_update" => Keyword::DriverUpdate,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endconfig" => Keyword::EndConfig,
	"endconnectrules" => Keyword::EndConnectRules,
	"enddiscipline" => Keyword::EndDiscipline,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endmodule" => Keyword::EndModule,
	"endnature" => Keyword::EndNature,
	"endparamset" => Keyword::EndParamSet,
	"endprimitive" => Keyword::EndPrimitive,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"event" => Keyword::Event,
	"exclude" => Keyword::Exclude,
	"exp" => Keyword::Exp,
	"final_step" => Keyword::FinalStep,
	"flicker_noise" => Keyword::FlickerNoise,
	"floor" => Keyword::Floor,
	"flow" => Keyword::Flow,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"from" => Keyword::From,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"ground" => Keyword::Ground,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"hypot" => Keyword::Hypot,
	"idt_nature" => Keyword::IdtNature,
	"idt" => Keyword::Idt,
	"idtmod" => Keyword::IdtMod,
	"if" => Keyword::If,
	"ifnone" => Keyword::IfNone,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"inf" => Keyword::Inf,
	"initial_step" => Keyword::InitialStep,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"instance" => Keyword::Instance,
	"integer" => Keyword::Integer,
	"join" => Keyword::Join,
	"laplace_nd" => Keyword::LaplaceNd,
	"laplace_np" => Keyword::LaplaceNp,
	"laplace_zd" => Keyword::LaplaceZd,
	"laplace_zp" => Keyword::LaplaceZp,
	"large" => Keyword::Large,
	"last_crossing" => Keyword::LastCrossing,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"limexp" => Keyword::LimExp,
	"ln" => Keyword::Ln,
	"localparam" => Keyword::LocalParam,
	"log" => Keyword::Log,
	"macromodule" => Keyword::MacroModule,
	"max" => Keyword::Max,
	"medium" => Keyword::Medium,
	"merged" => Keyword::Merged,
	"min" => Keyword::Min,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"net_resolution" => Keyword::NetResolution,
	"nmos" => Keyword::Nmos,
	"noise_table_log" => Keyword::NoiseTableLog,
	"noise_table" => Keyword::NoiseTable,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"parameter" => Keyword::Parameter,
	"paramset" => Keyword::ParamSet,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"potential" => Keyword::Potential,
	"pow" => Keyword::Pow,
	"primitive" => Keyword::Primitive,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"reg" => Keyword::Reg,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"resolveto" => Keyword::ResolveTo,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"scalared" => Keyword::Scalared,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"sin" => Keyword::Sin,
	"sinh" => Keyword::Sinh,
	"slew" => Keyword::Slew,
	"small" => Keyword::Small,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"split" => Keyword::Split,
	"sqrt" => Keyword::Sqrt,
	"string" => Keyword::String,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"table" => Keyword::Table,
	"tan" => Keyword::Tan,
	"tanh" => Keyword::Tanh,
	"task" => Keyword::Task,
	"time" => Keyword::Time,
	"timer" => Keyword::Timer,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"transition" => Keyword::Transition,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"units" => Keyword::Units,
	"unsigned" => Keyword::Unsigned,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"vectored" => Keyword::Vectored,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"white_noise" => Keyword::WhiteNoise,
	"wire" => Keyword::Wire,
	"wor" => Keyword::Wor,
	"wreal" => Keyword::Wreal,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
	"zi_nd" => Keyword::ZiNd,
	"zi_np" => Keyword::ZiNp,
	"zi_zd" => Keyword::ZiZd,
	"zi_zp" => Keyword::ZiZp,
};

/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Keyword set
pub static VERILOG_AMS_14_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"above",         "ddx",             "highz1",          "not",                 "strong1",
	"abs",           "deassign",        "hypot",           "notif0",              "supply0",
	"absdelay",      "default",         "idt_nature",      "notif1",              "supply1",
	"absdelta",      "defparam",        "idt",             "or",                  "table",
	"abstol",        "design",          "idtmod",          "output",              "tan",
	"ac_stim",       "disable",         "if",              "parameter",           "tanh",
	"access",        "discipline",      "ifnone",          "paramset",            "task",
	"acos",          "discrete",        "incdir",          "pmos",                "time",
	"acosh",         "domain",          "include",         "posedge",             "timer",
	"aliasparam",    "driver_update",   "inf",             "potential",           "tran",
	"always",        "edge",            "initial_step",    "pow",                 "tranif0",
	"analog",        "else",            "initial",         "primitive",           "tranif1",
	"analysis",      "end",             "inout",           "pull0",               "transition",
	"and",           "endcase",         "input",           "pull1",               "tri",
	"asin",          "endconfig",       "instance",        "pulldown",            "tri0",
	"asinh",         "endconnectrules", "integer",         "pullup",              "tri1",
	"assert",        "enddiscipline",   "join",            "pulsestyle_ondetect", "triand",
	"assign",        "endfunction",     "laplace_nd",      "pulsestyle_onevent",  "trior",
	"atan",          "endgenerate",     "laplace_np",      "rcmos",               "trireg",
	"atan2",         "endmodule",       "laplace_zd",      "real",                "units",
	"atanh",         "endnature",       "laplace_zp",      "realtime",            "unsigned",
	"automatic",     "endparamset",     "large",           "reg",                 "use",
	"begin",         "endprimitive",    "last_crossing",   "release",             "uwire",
	"branch",        "endspecify",      "liblist",         "repeat",              "vectored",
	"buf",           "endtable",        "library",         "resolveto",           "wait",
	"bufif0",        "endtask",         "limexp",          "rnmos",               "wand",
	"bufif1",        "event",           "ln",              "rpmos",               "weak0",
	"case",          "exclude",         "localparam",      "rtran",               "weak1",
	"casex",         "exp",             "log",             "rtranif0",            "while",
	"casez",         "final_step",      "macromodule",     "rtranif1",            "white_noise",
	"ceil",          "flicker_noise",   "max",             "scalared",            "wire",
	"cell",          "floor",           "medium",          "showcancelled",       "wor",
	"cmos",          "flow",            "merged",          "signed",              "wreal",
	"config",        "for",             "min",             "sin",                 "xnor",
	"connect",       "force",           "module",          "sinh",                "xor",
	"connectmodule", "forever",         "nand",            "slew",                "zi_nd",
	"connectrules",  "fork",            "negedge",         "small",               "zi_np",
	"continuous",    "from",            "net_resolution",  "specify",             "zi_zd",
	"cos",           "function",        "nmos",            "specparam",           "zi_zp",
	"cosh",          "generate",        "noise_table_log", "split",
	"cross",         "genvar",          "noise_table",     "sqrt",
	"ddt_nature",    "ground",          "nor",             "string",
	"ddt",           "highz0",          "noshowcancelled", "strong0",
};

/// Verilog-AMS 2023 Keyword to [`Keyword`] token map
pub static VERILOG_AMS_23_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"above" => Keyword::Above,
	"abs" => Keyword::Abs,
	"absdelay" => Keyword::AbsDelay,
	"absdelta" => Keyword::AbsDelta,
	"abstol" => Keyword::AbsTol,
	"ac_stim" => Keyword::AcStim,
	"access" => Keyword::Access,
	"acos" => Keyword::Acos,
	"acosh" => Keyword::Acosh,
	"aliasparam" => Keyword::Aliasparam,
	"always" => Keyword::Always,
	"analog" => Keyword::Analog,
	"analysis" => Keyword::Analysis,
	"and" => Keyword::And,
	"asin" => Keyword::Asin,
	"asinh" => Keyword::Asinh,
	"assign" => Keyword::Assign,
	"atan" => Keyword::Atan,
	"atan2" => Keyword::Atan2,
	"atanh" => Keyword::Atanh,
	"automatic" => Keyword::Automatic,
	"begin" => Keyword::Begin,
	"branch" => Keyword::Branch,
	"break" => Keyword::Break,
	"buf" => Keyword::Buf,
	"bufif0" => Keyword::BufIf0,
	"bufif1" => Keyword::BufIf1,
	"case" => Keyword::Case,
	"casex" => Keyword::CaseX,
	"casez" => Keyword::CaseZ,
	"ceil" => Keyword::Ceil,
	"cell" => Keyword::Cell,
	"cmos" => Keyword::Cmos,
	"config" => Keyword::Config,
	"connect" => Keyword::Connect,
	"connectmodule" => Keyword::ConnectModule,
	"connectrules" => Keyword::ConnectRules,
	"continue" => Keyword::Continue,
	"continuous" => Keyword::Continuous,
	"cos" => Keyword::Cos,
	"cosh" => Keyword::Cosh,
	"cross" => Keyword::Cross,
	"ddt_nature" => Keyword::DdtNature,
	"ddt" => Keyword::Ddt,
	"ddx" => Keyword::Ddx,
	"deassign" => Keyword::Deassign,
	"default" => Keyword::Default,
	"defparam" => Keyword::DefParam,
	"design" => Keyword::Design,
	"disable" => Keyword::Disable,
	"discipline" => Keyword::Discipline,
	"discrete" => Keyword::Discrete,
	"domain" => Keyword::Domain,
	"driver_update" => Keyword::DriverUpdate,
	"edge" => Keyword::Edge,
	"else" => Keyword::Else,
	"end" => Keyword::End,
	"endcase" => Keyword::EndCase,
	"endconfig" => Keyword::EndConfig,
	"endconnectrules" => Keyword::EndConnectRules,
	"enddiscipline" => Keyword::EndDiscipline,
	"endfunction" => Keyword::EndFunction,
	"endgenerate" => Keyword::EndGenerate,
	"endmodule" => Keyword::EndModule,
	"endnature" => Keyword::EndNature,
	"endparamset" => Keyword::EndParamSet,
	"endprimitive" => Keyword::EndPrimitive,
	"endspecify" => Keyword::EndSpecify,
	"endtable" => Keyword::EndTable,
	"endtask" => Keyword::EndTask,
	"event" => Keyword::Event,
	"exclude" => Keyword::Exclude,
	"exp" => Keyword::Exp,
	"expm1" => Keyword::Expm1,
	"final_step" => Keyword::FinalStep,
	"flicker_noise" => Keyword::FlickerNoise,
	"floor" => Keyword::Floor,
	"flow" => Keyword::Flow,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"forever" => Keyword::Forever,
	"fork" => Keyword::Fork,
	"from" => Keyword::From,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"genvar" => Keyword::Genvar,
	"ground" => Keyword::Ground,
	"highz0" => Keyword::HighZ0,
	"highz1" => Keyword::HighZ1,
	"hypot" => Keyword::Hypot,
	"idt_nature" => Keyword::IdtNature,
	"idt" => Keyword::Idt,
	"idtmod" => Keyword::IdtMod,
	"if" => Keyword::If,
	"ifnone" => Keyword::IfNone,
	"incdir" => Keyword::IncDir,
	"include" => Keyword::Include,
	"inf" => Keyword::Inf,
	"initial_step" => Keyword::InitialStep,
	"initial" => Keyword::Initial,
	"inout" => Keyword::InOut,
	"input" => Keyword::Input,
	"instance" => Keyword::Instance,
	"integer" => Keyword::Integer,
	"join" => Keyword::Join,
	"laplace_nd" => Keyword::LaplaceNd,
	"laplace_np" => Keyword::LaplaceNp,
	"laplace_zd" => Keyword::LaplaceZd,
	"laplace_zp" => Keyword::LaplaceZp,
	"large" => Keyword::Large,
	"last_crossing" => Keyword::LastCrossing,
	"liblist" => Keyword::LibList,
	"library" => Keyword::Library,
	"limexp" => Keyword::LimExp,
	"ln" => Keyword::Ln,
	"ln1p" => Keyword::Ln1p,
	"localparam" => Keyword::LocalParam,
	"log" => Keyword::Log,
	"macromodule" => Keyword::MacroModule,
	"max" => Keyword::Max,
	"medium" => Keyword::Medium,
	"merged" => Keyword::Merged,
	"min" => Keyword::Min,
	"module" => Keyword::Module,
	"nand" => Keyword::Nand,
	"negedge" => Keyword::NegEdge,
	"nmos" => Keyword::Nmos,
	"noise_table_log" => Keyword::NoiseTableLog,
	"noise_table" => Keyword::NoiseTable,
	"nor" => Keyword::Nor,
	"noshowcancelled" => Keyword::NoShowCancelled,
	"not" => Keyword::Not,
	"notif0" => Keyword::NotIf0,
	"notif1" => Keyword::NotIf1,
	"or" => Keyword::Or,
	"output" => Keyword::Output,
	"parameter" => Keyword::Parameter,
	"paramset" => Keyword::ParamSet,
	"pmos" => Keyword::Pmos,
	"posedge" => Keyword::PosEdge,
	"potential" => Keyword::Potential,
	"pow" => Keyword::Pow,
	"primitive" => Keyword::Primitive,
	"pull0" => Keyword::Pull0,
	"pull1" => Keyword::Pull1,
	"pulldown" => Keyword::Pulldown,
	"pullup" => Keyword::Pullup,
	"pulsestyle_ondetect" => Keyword::PulseStyleOnDetect,
	"pulsestyle_onevent" => Keyword::PulseStyleOnEvent,
	"rcmos" => Keyword::Rcmos,
	"real" => Keyword::Real,
	"realtime" => Keyword::Realtime,
	"reg" => Keyword::Reg,
	"release" => Keyword::Release,
	"repeat" => Keyword::Repeat,
	"resolveto" => Keyword::ResolveTo,
	"return" => Keyword::Return,
	"rnmos" => Keyword::Rnmos,
	"rpmos" => Keyword::Rpmos,
	"rtran" => Keyword::Rtran,
	"rtranif0" => Keyword::RtranIf0,
	"rtranif1" => Keyword::RtranIf1,
	"scalared" => Keyword::Scalared,
	"showcancelled" => Keyword::ShowCancelled,
	"signed" => Keyword::Signed,
	"sin" => Keyword::Sin,
	"sinh" => Keyword::Sinh,
	"slew" => Keyword::Slew,
	"small" => Keyword::Small,
	"specify" => Keyword::Specify,
	"specparam" => Keyword::SpecParam,
	"split" => Keyword::Split,
	"sqrt" => Keyword::Sqrt,
	"string" => Keyword::String,
	"strong0" => Keyword::Strong0,
	"strong1" => Keyword::Strong1,
	"supply0" => Keyword::Supply0,
	"supply1" => Keyword::Supply1,
	"table" => Keyword::Table,
	"tan" => Keyword::Tan,
	"tanh" => Keyword::Tanh,
	"task" => Keyword::Task,
	"time" => Keyword::Time,
	"timer" => Keyword::Timer,
	"tran" => Keyword::Tran,
	"tranif0" => Keyword::TranIf0,
	"tranif1" => Keyword::TranIf1,
	"transition" => Keyword::Transition,
	"tri" => Keyword::Tri,
	"tri0" => Keyword::Tri0,
	"tri1" => Keyword::Tri1,
	"triand" => Keyword::Triand,
	"trior" => Keyword::Trior,
	"trireg" => Keyword::Trireg,
	"units" => Keyword::Units,
	"unsigned" => Keyword::Unsigned,
	"use" => Keyword::Use,
	"uwire" => Keyword::Uwire,
	"vectored" => Keyword::Vectored,
	"wait" => Keyword::Wait,
	"wand" => Keyword::Wand,
	"weak0" => Keyword::Weak0,
	"weak1" => Keyword::Weak1,
	"while" => Keyword::While,
	"white_noise" => Keyword::WhiteNoise,
	"wire" => Keyword::Wire,
	"wor" => Keyword::Wor,
	"wreal" => Keyword::Wreal,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
	"zi_nd" => Keyword::ZiNd,
	"zi_np" => Keyword::ZiNp,
	"zi_zd" => Keyword::ZiZd,
	"zi_zp" => Keyword::ZiZp,
};

/// Verilog-AMS 2023 Keyword set
pub static VERILOG_AMS_23_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"above",         "ddt",             "ground",          "nor",                 "sqrt",
	"abs",           "ddx",             "highz0",          "noshowcancelled",     "string",
	"absdelay",      "deassign",        "highz1",          "not",                 "strong0",
	"absdelta",      "default",         "hypot",           "notif0",              "strong1",
	"abstol",        "defparam",        "idt_nature",      "notif1",              "supply0",
	"ac_stim",       "design",          "idt",             "or",                  "supply1",
	"access",        "disable",         "idtmod",          "output",              "table",
	"acos",          "discipline",      "if",              "parameter",           "tan",
	"acosh",         "discrete",        "ifnone",          "paramset",            "tanh",
	"aliasparam",    "domain",          "incdir",          "pmos",                "task",
	"always",        "driver_update",   "include",         "posedge",             "time",
	"analog",        "edge",            "inf",             "potential",           "timer",
	"analysis",      "else",            "initial_step",    "pow",                 "tran",
	"and",           "end",             "initial",         "primitive",           "tranif0",
	"asin",          "endcase",         "inout",           "pull0",               "tranif1",
	"asinh",         "endconfig",       "input",           "pull1",               "transition",
	"assign",        "endconnectrules", "instance",        "pulldown",            "tri",
	"atan",          "enddiscipline",   "integer",         "pullup",              "tri0",
	"atan2",         "endfunction",     "join",            "pulsestyle_ondetect", "tri1",
	"atanh",         "endgenerate",     "laplace_nd",      "pulsestyle_onevent",  "triand",
	"automatic",     "endmodule",       "laplace_np",      "rcmos",               "trior",
	"begin",         "endnature",       "laplace_zd",      "real",                "trireg",
	"branch",        "endparamset",     "laplace_zp",      "realtime",            "units",
	"break",         "endprimitive",    "large",           "reg",                 "unsigned",
	"buf",           "endspecify",      "last_crossing",   "release",             "use",
	"bufif0",        "endtable",        "liblist",         "repeat",              "uwire",
	"bufif1",        "endtask",         "library",         "resolveto",           "vectored",
	"case",          "event",           "limexp",          "return",              "wait",
	"casex",         "exclude",         "ln",              "rnmos",               "wand",
	"casez",         "exp",             "ln1p",            "rpmos",               "weak0",
	"ceil",          "expm1",           "localparam",      "rtran",               "weak1",
	"cell",          "final_step",      "log",             "rtranif0",            "while",
	"cmos",          "flicker_noise",   "macromodule",     "rtranif1",            "white_noise",
	"config",        "floor",           "max",             "scalared",            "wire",
	"connect",       "flow",            "medium",          "showcancelled",       "wor",
	"connectmodule", "for",             "merged",          "signed",              "wreal",
	"connectrules",  "force",           "min",             "sin",                 "xnor",
	"continue",      "forever",         "module",          "sinh",                "xor",
	"continuous",    "fork",            "nand",            "slew",                "zi_nd",
	"cos",           "from",            "negedge",         "small",               "zi_np",
	"cosh",          "function",        "nmos",            "specify",             "zi_zd",
	"cross",         "generate",        "noise_table_log", "specparam",           "zi_zp",
	"ddt_nature",    "genvar",          "noise_table",     "split",
};

/// Get the Verilog 1995 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_95_keyword(ident: &str) -> Option<Keyword> {
	VERILOG_95_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 1995 keyword
///
/// This is used rather than [`get_verilog_95_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_verilog_95_keyword(ident: &str) -> bool {
	VERILOG_95_KEYWORD_SET.contains(ident)
}

/// Get the Verilog 2001 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_01_keyword(ident: &str) -> Option<Keyword> {
	VERILOG_01_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2001 keyword
///
/// This is used rather than [`get_verilog_01_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_verilog_01_keyword(ident: &str) -> bool {
	VERILOG_01_KEYWORD_SET.contains(ident)
}

/// Get the Verilog 2005 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_05_keyword(ident: &str) -> Option<Keyword> {
	VERILOG_05_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2005 keyword
///
/// This is used rather than [`get_verilog_05_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_verilog_05_keyword(ident: &str) -> bool {
	VERILOG_05_KEYWORD_SET.contains(ident)
}

/// Get the Verilog keyword for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Vl95 => get_verilog_95_keyword(ident),
		LanguageStd::Vl01 => get_verilog_01_keyword(ident),
		LanguageStd::Vl05 => get_verilog_05_keyword(ident),
		_ => None,
	}
}

/// Get the SystemVerilog 2005 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_05_keyword(ident: &str) -> Option<Keyword> {
	SYSTEM_VERILOG_05_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2005 keyword
///
/// This is used rather than [`get_system_verilog_05_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_05_keyword(ident: &str) -> bool {
	SYSTEM_VERILOG_05_KEYWORD_SET.contains(ident)
}

/// Get the SystemVerilog 2009 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_09_keyword(ident: &str) -> Option<Keyword> {
	SYSTEM_VERILOG_09_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2009 keyword
///
/// This is used rather than [`get_system_verilog_09_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_09_keyword(ident: &str) -> bool {
	SYSTEM_VERILOG_09_KEYWORD_SET.contains(ident)
}

/// Get the SystemVerilog 2012 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_12_keyword(ident: &str) -> Option<Keyword> {
	SYSTEM_VERILOG_12_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2012 keyword
///
/// This is used rather than [`get_system_verilog_12_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_12_keyword(ident: &str) -> bool {
	SYSTEM_VERILOG_12_KEYWORD_SET.contains(ident)
}

/// Get the SystemVerilog 2017 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_17_keyword(ident: &str) -> Option<Keyword> {
	SYSTEM_VERILOG_17_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2017 keyword
///
/// This is used rather than [`get_system_verilog_17_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_17_keyword(ident: &str) -> bool {
	SYSTEM_VERILOG_17_KEYWORD_SET.contains(ident)
}

/// Get the SystemVerilog 2023 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_23_keyword(ident: &str) -> Option<Keyword> {
	SYSTEM_VERILOG_23_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2023 keyword
///
/// This is used rather than [`get_system_verilog_23_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_23_keyword(ident: &str) -> bool {
	SYSTEM_VERILOG_23_KEYWORD_SET.contains(ident)
}

/// Get the SystemVerilog keyword for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Sv05 => get_system_verilog_05_keyword(ident),
		LanguageStd::Sv09 => get_system_verilog_09_keyword(ident),
		LanguageStd::Sv12 => get_system_verilog_12_keyword(ident),
		LanguageStd::Sv17 => get_system_verilog_17_keyword(ident),
		LanguageStd::Sv23 => get_system_verilog_23_keyword(ident),
		_ => None,
	}
}

/// Get the Verilog-AMS 2009 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_09_keyword(ident: &str) -> Option<Keyword> {
	VERILOG_AMS_09_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2009 keyword
///
/// This is used rather than [`get_verilog_ams_09_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_verilog_ams_09_keyword(ident: &str) -> bool {
	VERILOG_AMS_09_KEYWORD_SET.contains(ident)
}

/// Get the Verilog-AMS 2014 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_14_keyword(ident: &str) -> Option<Keyword> {
	VERILOG_AMS_14_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2014 keyword
///
/// This is used rather than [`get_verilog_ams_14_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_verilog_ams_14_keyword(ident: &str) -> bool {
	VERILOG_AMS_14_KEYWORD_SET.contains(ident)
}

/// Get the Verilog-AMS 2023 keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_23_keyword(ident: &str) -> Option<Keyword> {
	VERILOG_AMS_23_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2023 keyword
///
/// This is used rather than [`get_verilog_ams_23_keyword`] to test if the
/// found identifier for a lower Verilog standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_verilog_ams_23_keyword(ident: &str) -> bool {
	VERILOG_AMS_23_KEYWORD_SET.contains(ident)
}

/// Get the Verilog-AMS keyword for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Vams09 => get_verilog_ams_09_keyword(ident),
		LanguageStd::Vams14 => get_verilog_ams_14_keyword(ident),
		LanguageStd::Vams23 => get_verilog_ams_23_keyword(ident),
		_ => None,
	}
}

/// Get the given Verilog/Verilog-AMS/SystemVerilog keyword for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Vl95 => get_verilog_95_keyword(ident),
		LanguageStd::Vl01 => get_verilog_01_keyword(ident),
		LanguageStd::Vl05 => get_verilog_05_keyword(ident),
		LanguageStd::Sv05 => get_system_verilog_05_keyword(ident),
		LanguageStd::Sv09 => get_system_verilog_09_keyword(ident),
		LanguageStd::Sv12 => get_system_verilog_12_keyword(ident),
		LanguageStd::Sv17 => get_system_verilog_17_keyword(ident),
		LanguageStd::Sv23 => get_system_verilog_23_keyword(ident),
		LanguageStd::Vams09 => get_verilog_ams_09_keyword(ident),
		LanguageStd::Vams14 => get_verilog_ams_14_keyword(ident),
		LanguageStd::Vams23 => get_verilog_ams_23_keyword(ident),
		_ => None,
	}
}

/// Check to see if the given identifier is a keyword in a future Verilog standard, returning
/// the standard version if so.
#[allow(unused)]
#[inline]
pub fn keyword_in(ident: &str) -> LanguageStd {
	let mut supported = LanguageStd::none();

	if is_verilog_95_keyword(ident) {
		supported |= LanguageStd::Vl95;
	}

	if is_verilog_01_keyword(ident) {
		supported |= LanguageStd::Vl01;
	}

	if is_verilog_05_keyword(ident) {
		supported |= LanguageStd::Vl05;
	}

	if is_system_verilog_05_keyword(ident) {
		supported |= LanguageStd::Sv05;
	}

	if is_system_verilog_09_keyword(ident) {
		supported |= LanguageStd::Sv09;
	}

	if is_system_verilog_12_keyword(ident) {
		supported |= LanguageStd::Sv12;
	}

	if is_system_verilog_17_keyword(ident) {
		supported |= LanguageStd::Sv17;
	}

	if is_system_verilog_23_keyword(ident) {
		supported |= LanguageStd::Sv23;
	}

	if is_verilog_ams_09_keyword(ident) {
		supported |= LanguageStd::Vams09;
	}

	if is_verilog_ams_14_keyword(ident) {
		supported |= LanguageStd::Vams14;
	}

	if is_verilog_ams_23_keyword(ident) {
		supported |= LanguageStd::Vams23;
	}

	supported
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Keyword({})",
			match self {
				Self::Above => "above",           // Added: Verilog-AMS 2009
				Self::Abs => "abs",               // Added: Verilog-AMS 2009
				Self::AbsDelay => "absdelay",     // Added: Verilog-AMS 2009
				Self::AbsDelta => "absdelta",     // Added: Verilog-AMS 2014
				Self::AbsTol => "abstol",         // Added: Verilog-AMS 2009
				Self::AcceptOn => "accept_on",    // Added: IEEE 1800-2009
				Self::Access => "access",         // Added: Verilog-AMS 2009
				Self::Acos => "acos",             // Added: Verilog-AMS 2009
				Self::Acosh => "acosh",           // Added: Verilog-AMS 2009
				Self::AcStim => "ac_stim",        // Added: Verilog-AMS 2009
				Self::Alias => "alias",           // Added: IEEE 1800-2005
				Self::Aliasparam => "aliasparam", // Added: Verilog-AMS 2009
				Self::Always => "always",
				Self::AlwaysComb => "always_comb",   // Added: IEEE 1800-2005
				Self::AlwaysFF => "always_ff",       // Added: IEEE 1800-2005
				Self::AlwaysLatch => "always_latch", // Added: IEEE 1800-2005
				Self::Analog => "analog",            // Added: Verilog-AMS 2009
				Self::Analysis => "analysis",        // Added: Verilog-AMS 2009
				Self::And => "and",
				Self::Asin => "asin",     // Added: Verilog-AMS 2009
				Self::Asinh => "asinh",   // Added: Verilog-AMS 2009
				Self::Assert => "assert", // Added: IEEE 1800-2005 & Verilog-AMS 2009-2014
				Self::Assign => "assign",
				Self::Assume => "assume",       // Added: IEEE 1800-2005
				Self::Atan => "atan",           // Added: Verilog-AMS 2009
				Self::Atan2 => "atan2",         // Added: Verilog-AMS 2009
				Self::Atanh => "atanh",         // Added: Verilog-AMS 2009
				Self::Automatic => "automatic", // Added: IEEE 1364-2001
				Self::Before => "before",       // Added: IEEE 1800-2005
				Self::Begin => "begin",
				Self::Bind => "bind",     // Added: IEEE 1800-2005
				Self::Bins => "bins",     // Added: IEEE 1800-2005
				Self::BinsOf => "binsof", // Added: IEEE 1800-2005
				Self::Bit => "bit",       // Added: IEEE 1800-2005
				Self::Branch => "branch", // Added: Verilog-AMS 2009
				Self::Break => "break",   // Added: IEEE 1800-2005 & Verilog-AMS 2023
				Self::Buf => "buf",
				Self::BufIf0 => "bufif0",
				Self::BufIf1 => "bufif1",
				Self::Byte => "byte", // Added: IEEE 1800-2005
				Self::Case => "case",
				Self::CaseX => "casex",
				Self::CaseZ => "casez",
				Self::Ceil => "ceil",         // Added: Verilog-AMS 2009
				Self::Cell => "cell",         // Added: IEEE 1364-2001
				Self::CHandle => "chandle",   // Added: IEEE 1800-2005
				Self::Checker => "checker",   // Added: IEEE 1800-2009
				Self::Class => "class",       // Added: IEEE 1800-2005
				Self::Clocking => "clocking", // Added: IEEE 1800-2005
				Self::Cmos => "cmos",
				Self::Config => "config",               // Added: IEEE 1364-2001
				Self::Connect => "connect",             // Added: Verilog-AMS 2009
				Self::ConnectModule => "connectmodule", // Added: Verilog-AMS 2009
				Self::ConnectRules => "connectrules",   // Added: Verilog-AMS 2009
				Self::Const => "const",                 // Added: IEEE 1800-2005
				Self::Constraint => "constraint",       // Added: IEEE 1800-2005
				Self::Context => "context",             // Added: IEEE 1800-2005
				Self::Continue => "continue",           // Added: IEEE 1800-2005 & Verilog-AMS 2023
				Self::Continuous => "continuous",       // Added: Verilog-AMS 2009
				Self::Cos => "cos",                     // Added: Verilog-AMS 2009
				Self::Cosh => "cosh",                   // Added: Verilog-AMS 2009
				Self::Cover => "cover",                 // Added: IEEE 1800-2005
				Self::CoverGroup => "covergroup",       // Added: IEEE 1800-2005
				Self::CoverPoint => "coverpoint",       // Added: IEEE 1800-2005
				Self::Cross => "cross",                 // Added: IEEE 1800-2005 & Verilog-AMS 2009
				Self::Ddt => "ddt",                     // Added: Verilog-AMS 2009
				Self::DdtNature => "ddt_nature",        // Added: Verilog-AMS 2009
				Self::Ddx => "ddx",                     // Added: Verilog-AMS 2009
				Self::Deassign => "deassign",
				Self::Default => "default",
				Self::DefParam => "defparam",
				Self::Design => "design", // Added: IEEE 1364-2001
				Self::Disable => "disable",
				Self::Discipline => "discipline", // Added: Verilog-AMS 2009
				Self::Discrete => "discrete",     // Added: Verilog-AMS 2009
				Self::Dist => "dist",             // Added: IEEE 1800-2005
				Self::Do => "do",                 // Added: IEEE 1800-2005
				Self::Domain => "domain",         // Added: Verilog-AMS 2009
				Self::DriverUpdate => "driver_update", // Added: Verilog-AMS 2009
				Self::Edge => "edge",
				Self::Else => "else",
				Self::End => "end",
				Self::EndCase => "endcase",
				Self::EndChecker => "endchecker",   // Added: IEEE 1800-2009
				Self::EndClass => "endclass",       // Added: IEEE 1800-2005
				Self::EndClocking => "endclocking", // Added: IEEE 1800-2005
				Self::EndConfig => "endconfig",     // Added: IEEE 1364-2001
				Self::EndConnectRules => "endconnectrules", // Added: Verilog-AMS 2009
				Self::EndDiscipline => "enddiscipline", // Added: Verilog-AMS 2009
				Self::EndFunction => "endfunction",
				Self::EndGenerate => "endgenerate",   // Added: IEEE 1364-2001
				Self::EndGroup => "endgroup",         // Added: IEEE 1800-2005
				Self::EndInterface => "endinterface", // Added: IEEE 1800-2005
				Self::EndModule => "endmodule",
				Self::EndNature => "endnature",     // Added: Verilog-AMS 2009
				Self::EndPackage => "endpackage",   // Added: IEEE 1800-2005
				Self::EndParamSet => "endparamset", // Added: Verilog-AMS 2009
				Self::EndPrimitive => "endprimitive",
				Self::EndProgram => "endprogram",   // Added: IEEE 1800-2005
				Self::EndProperty => "endproperty", // Added: IEEE 1800-2005
				Self::EndSequence => "endsequence", // Added: IEEE 1800-2005
				Self::EndSpecify => "endspecify",
				Self::EndTable => "endtable",
				Self::EndTask => "endtask",
				Self::Enum => "enum", // Added: IEEE 1800-2005
				Self::Event => "event",
				Self::Eventually => "eventually",      // Added: IEEE 1800-2009
				Self::Exclude => "exclude",            // Added: Verilog-AMS 2009
				Self::Exp => "exp",                    // Added: Verilog-AMS 2009
				Self::Expect => "expect",              // Added: IEEE 1800-2005
				Self::Expm1 => "expm1",                // Added:Verilog-AMS 2023
				Self::Export => "export",              // Added: IEEE 1800-2005
				Self::Extends => "extends",            // Added: IEEE 1800-2005
				Self::Extern => "extern",              // Added: IEEE 1800-2005
				Self::Final => "final",                // Added: IEEE 1800-2005
				Self::FinalStep => "final_step",       // Added: Verilog-AMS 2009
				Self::FirstMatch => "first_match",     // Added: IEEE 1800-2005
				Self::FlickerNoise => "flicker_noise", // Added: Verilog-AMS 2009
				Self::Floor => "floor",                // Added: Verilog-AMS 2009
				Self::Flow => "flow",                  // Added: Verilog-AMS 2009
				Self::For => "for",
				Self::Force => "force",
				Self::ForEach => "foreach", // Added: IEEE 1800-2005
				Self::Forever => "forever",
				Self::Fork => "fork",
				Self::ForkJoin => "forkjoin", // Added: IEEE 1800-2005
				Self::From => "from",         // Added: Verilog-AMS 2009
				Self::Function => "function",
				Self::Generate => "generate", // Added: IEEE 1364-2001
				Self::Genvar => "genvar",     // Added: IEEE 1364-2001
				Self::Global => "global",     // Added: IEEE 1800-2009
				Self::Ground => "ground",     // Added: Verilog-AMS 2009
				Self::HighZ0 => "highz0",
				Self::HighZ1 => "highz1",
				Self::Hypot => "hypot",          // Added: Verilog-AMS 2009
				Self::Idt => "idt",              // Added: Verilog-AMS 2009
				Self::IdtMod => "idtmod",        // Added: Verilog-AMS 2009
				Self::IdtNature => "idt_nature", // Added: Verilog-AMS 2009
				Self::If => "if",
				Self::Iff => "iff", // Added: IEEE 1800-2005
				Self::IfNone => "ifnone",
				Self::IgnoreBins => "ignore_bins",   // Added: IEEE 1800-2005
				Self::IllegalBins => "illegal_bins", // Added: IEEE 1800-2005
				Self::Implements => "implements",    // Added: IEEE 1800-2012
				Self::Implies => "implies",          // Added: IEEE 1800-2009
				Self::Import => "import",            // Added: IEEE 1800-2005
				Self::IncDir => "incdir",            // Added: IEEE 1364-2001
				Self::Include => "include",          // Added: IEEE 1364-2001
				Self::Inf => "inf",                  // Added: Verilog-AMS 2009
				Self::Initial => "initial",
				Self::InitialStep => "initial_step", // Added: Verilog-AMS 2009
				Self::InOut => "inout",
				Self::Input => "input",
				Self::Inside => "inside",     // Added: IEEE 1800-2005
				Self::Instance => "instance", // Added: IEEE 1364-2001
				Self::Int => "int",           // Added: IEEE 1800-2005
				Self::Integer => "integer",
				Self::Interconnect => "interconnect", // Added: IEEE 1800-2012
				Self::Interface => "interface",       // Added: IEEE 1800-2005
				Self::Intersect => "intersect",       // Added: IEEE 1800-2005
				Self::Join => "join",
				Self::JoinAny => "join_any",     // Added: IEEE 1800-2005
				Self::JoinNone => "join_none",   // Added: IEEE 1800-2005
				Self::LaplaceNd => "laplace_nd", // Added: Verilog-AMS 2009
				Self::LaplaceNp => "laplace_np", // Added: Verilog-AMS 2009
				Self::LaplaceZd => "laplace_zd", // Added: Verilog-AMS 2009
				Self::LaplaceZp => "laplace_zp", // Added: Verilog-AMS 2009
				Self::Large => "large",
				Self::LastCrossing => "last_crossing", // Added: Verilog-AMS 2009
				Self::Let => "let",                    // Added: IEEE 1800-2009
				Self::LibList => "liblist",            // Added: IEEE 1364-2001
				Self::Library => "library",            // Added: IEEE 1364-2001
				Self::LimExp => "limexp",              // Added: Verilog-AMS 2009
				Self::Ln => "ln",                      // Added: Verilog-AMS 2009
				Self::Ln1p => "ln1p",                  // Added: Verilog-AMS 2023
				Self::Local => "local",                // Added: IEEE 1800-2005
				Self::LocalParam => "localparam",      // Added: IEEE 1364-2001
				Self::Log => "log",                    // Added: Verilog-AMS 2009
				Self::Logic => "logic",                // Added: IEEE 1800-2005
				Self::LongInt => "longint",            // Added: IEEE 1800-2005
				Self::MacroModule => "macromodule",
				Self::Matches => "matches", // Added: IEEE 1800-2005
				Self::Max => "max",         // Added: Verilog-AMS 2009
				Self::Medium => "medium",
				Self::Merged => "merged",   // Added: Verilog-AMS 2009
				Self::Min => "min",         // Added: Verilog-AMS 2009
				Self::ModPort => "modport", // Added: IEEE 1800-2005
				Self::Module => "module",
				Self::Nand => "nand",
				Self::Nature => "nature", // Added: Verilog-AMS 2009
				Self::NegEdge => "negedge",
				Self::NetResolution => "net_resolution", // Added: Verilog-AMS 2009-2014
				Self::NetType => "nettype",              // Added: IEEE 1800-2012
				Self::New => "new",                      // Added: IEEE 1800-2005
				Self::NextTime => "nexttime",            // Added: IEEE 1800-2009
				Self::Nmos => "nmos",
				Self::NoiseTable => "noise_table", // Added: Verilog-AMS 2009
				Self::NoiseTableLog => "noise_table_log", // Added: Verilog-AMS 2014
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
				Self::ParamSet => "paramset", // Added: Verilog-AMS 2009
				Self::Pmos => "pmos",
				Self::PosEdge => "posedge",
				Self::Potential => "potential", // Added: Verilog-AMS 2009
				Self::Pow => "pow",             // Added: Verilog-AMS 2009
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
				Self::RejectOn => "reject_on", // Added: IEEE 1800-2009
				Self::Release => "release",
				Self::Repeat => "repeat",
				Self::ResolveTo => "resolveto", // Added: Verilog-AMS 2009
				Self::Restrict => "restrict",   // Added: IEEE 1800-2009
				Self::Return => "return",       // Added: IEEE 1800-2005 & Verilog-AMS 2023
				Self::Rnmos => "rnmos",
				Self::Rpmos => "rpmos",
				Self::Rtran => "rtran",
				Self::RtranIf0 => "rtranif0",
				Self::RtranIf1 => "rtranif1",
				Self::SAlways => "s_always", // Added: IEEE 1800-2009
				Self::Scalared => "scalared",
				Self::Sequence => "sequence",           // Added: IEEE 1800-2005
				Self::SEventually => "s_eventually",    // Added: IEEE 1800-2009
				Self::ShortInt => "shortint",           // Added: IEEE 1800-2005
				Self::ShortReal => "shortreal",         // Added: IEEE 1800-2005
				Self::ShowCancelled => "showcancelled", // Added: IEEE 1364-2001
				Self::Signed => "signed",               // Added: IEEE 1364-2001
				Self::Sin => "sin",                     // Added: Verilog-AMS 2009
				Self::Sinh => "sinh",                   // Added: Verilog-AMS 2009
				Self::Slew => "slew",                   // Added: Verilog-AMS 2009
				Self::Small => "small",
				Self::SNextTime => "s_nexttime", // Added: IEEE 1800-2009
				Self::Soft => "soft",            // Added: IEEE 1800-2012
				Self::Solve => "solve",          // Added: IEEE 1800-2005
				Self::Specify => "specify",
				Self::SpecParam => "specparam",
				Self::Split => "split",   // Added: Verilog-AMS 2009
				Self::Sqrt => "sqrt",     // Added: Verilog-AMS 2009
				Self::Static => "static", // Added: IEEE 1800-2005
				Self::String => "string", // Added: IEEE 1800-2005 & Verilog-AMS 2009
				Self::Strong => "strong", // Added: IEEE 1800-2009
				Self::Strong0 => "strong0",
				Self::Strong1 => "strong1",
				Self::Struct => "struct",           // Added: IEEE 1800-2005
				Self::SUntil => "s_until",          // Added: IEEE 1800-2009
				Self::SUntilWith => "s_until_with", // Added: IEEE 1800-2009
				Self::Super => "super",             // Added: IEEE 1800-2005
				Self::Supply0 => "supply0",
				Self::Supply1 => "supply1",
				Self::SyncAcceptOn => "sync_accept_on", // Added: IEEE 1800-2009
				Self::SyncRejectOn => "sync_reject_on", // Added: IEEE 1800-2009
				Self::Table => "table",
				Self::Tagged => "tagged", // Added: IEEE 1800-2005
				Self::Tan => "tan",       // Added: Verilog-AMS 2009
				Self::Tanh => "tanh",     // Added: Verilog-AMS 2009
				Self::Task => "task",
				Self::This => "this",             // Added: IEEE 1800-2005
				Self::Throughout => "throughout", // Added: IEEE 1800-2005
				Self::Time => "time",
				Self::TimePrecision => "timeprecision", // Added: IEEE 1800-2005
				Self::Timer => "timer",                 // Added: Verilog-AMS 2009
				Self::TimeUnit => "timeunit",           // Added: IEEE 1800-2005
				Self::Tran => "tran",
				Self::TranIf0 => "tranif0",
				Self::TranIf1 => "tranif1",
				Self::Transition => "transition", // Added: Verilog-AMS 2009
				Self::Tri => "tri",
				Self::Tri0 => "tri0",
				Self::Tri1 => "tri1",
				Self::Triand => "triand",
				Self::Trior => "trior",
				Self::Trireg => "trireg",
				Self::Type => "type",            // Added: IEEE 1800-2005
				Self::TypeDef => "typedef",      // Added: IEEE 1800-2005
				Self::Union => "union",          // Added: IEEE 1800-2005
				Self::Unique => "unique",        // Added: IEEE 1800-2005
				Self::Unique0 => "unique0",      // Added: IEEE 1800-2009
				Self::Units => "units",          // Added: Verilog-AMS 2009
				Self::Unsigned => "unsigned",    // Added: IEEE 1364-2001
				Self::Until => "until",          // Added: IEEE 1800-2009
				Self::UntilWith => "until_with", // Added: IEEE 1800-2009
				Self::Untyped => "untyped",      // Added: IEEE 1800-2009
				Self::Use => "use",              // Added: IEEE 1364-2001
				Self::Uwire => "uwire",          // Added: IEEE 1364-2005
				Self::Var => "var",              // Added: IEEE 1800-2005
				Self::Vectored => "vectored",
				Self::Virtual => "virtual", // Added: IEEE 1800-2005
				Self::Void => "void",       // Added: IEEE 1800-2005
				Self::Wait => "wait",
				Self::WaitOrder => "wait_order", // Added: IEEE 1800-2005
				Self::Wand => "wand",
				Self::Weak => "weak", // Added: IEEE 1800-2009
				Self::Weak0 => "weak0",
				Self::Weak1 => "weak1",
				Self::While => "while",
				Self::WhiteNoise => "white_noise", // Added: Verilog-AMS 2009
				Self::Wildcard => "wildcard",      // Added: IEEE 1800-2005
				Self::Wire => "wire",
				Self::With => "with",     // Added: IEEE 1800-2005
				Self::Within => "within", // Added: IEEE 1800-2005
				Self::Wor => "wor",
				Self::Wreal => "wreal", // Added: Verilog-AMS 2009
				Self::Xnor => "xnor",
				Self::Xor => "xor",
				Self::ZiNd => "zi_nd", // Added: Verilog-AMS 2009
				Self::ZiNp => "zi_np", // Added: Verilog-AMS 2009
				Self::ZiZd => "zi_zd", // Added: Verilog-AMS 2009
				Self::ZiZp => "zi_zp", // Added: Verilog-AMS 2009
			}
		)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_keyword_verilog_95() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_verilog_95_keyword("nya"));
		assert_eq!(keyword_in("nya"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_verilog_95_keyword("always"));
		assert!(keyword_in("always").contains(LanguageStd::Vl95));

		// A keyword that is one version up
		assert!(!is_verilog_95_keyword("endconfig"));
		assert!(keyword_in("endconfig").contains(LanguageStd::Vl01));

		// Two versions up
		assert!(!is_verilog_95_keyword("uwire"));
		assert!(keyword_in("uwire").contains(LanguageStd::Vl05));

		// Three versions up
		assert!(!is_verilog_95_keyword("shortint"));
		assert!(keyword_in("shortint").contains(LanguageStd::Sv05));

		// Four versions up
		assert!(!is_verilog_95_keyword("accept_on"));
		assert!(keyword_in("accept_on").contains(LanguageStd::Sv09));

		// Five versions up
		assert!(!is_verilog_95_keyword("soft"));
		assert!(keyword_in("soft").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_verilog_01() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_verilog_01_keyword("meow"));
		assert_eq!(keyword_in("meow"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_verilog_01_keyword("include"));
		assert!(keyword_in("include").contains(LanguageStd::Vl01));

		// A keyword that is one version up
		assert!(!is_verilog_01_keyword("uwire"));
		assert!(keyword_in("uwire").contains(LanguageStd::Vl05));

		// Two versions up
		assert!(!is_verilog_01_keyword("wait_order"));
		assert!(keyword_in("wait_order").contains(LanguageStd::Sv05));

		// Three versions up
		assert!(!is_verilog_01_keyword("s_nexttime"));
		assert!(keyword_in("s_nexttime").contains(LanguageStd::Sv09));

		// Four versions up
		assert!(!is_verilog_01_keyword("interconnect"));
		assert!(keyword_in("interconnect").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_verilog_05() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_verilog_05_keyword("awoo"));
		assert_eq!(keyword_in("awoo"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_verilog_05_keyword("rtran"));
		assert!(keyword_in("rtran").contains(LanguageStd::Vl05));

		// A keyword that is one version up
		assert!(!is_verilog_05_keyword("throughout"));
		assert!(keyword_in("throughout").contains(LanguageStd::Sv05));

		// Two versions up
		assert!(!is_verilog_05_keyword("untyped"));
		assert!(keyword_in("untyped").contains(LanguageStd::Sv09));

		// Three versions up
		assert!(!is_verilog_05_keyword("implements"));
		assert!(keyword_in("implements").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_system_verilog_05() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_system_verilog_05_keyword("kon"));
		assert_eq!(keyword_in("kon"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_system_verilog_05_keyword("property"));
		assert!(keyword_in("property").contains(LanguageStd::Sv05));

		// A keyword that is one version up
		assert!(!is_system_verilog_05_keyword("unique0"));
		assert!(keyword_in("unique0").contains(LanguageStd::Sv09));

		// Two versions up
		assert!(!is_system_verilog_05_keyword("nettype"));
		assert!(keyword_in("nettype").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_system_verilog_09() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_system_verilog_09_keyword("wah"));
		assert_eq!(keyword_in("wah"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_system_verilog_09_keyword("ignore_bins"));
		assert!(keyword_in("ignore_bins").contains(LanguageStd::Sv09));

		// A keyword that is one version up
		assert!(!is_system_verilog_09_keyword("implements"));
		assert!(keyword_in("implements").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_system_verilog_12() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_system_verilog_12_keyword("meow"));
		assert_eq!(keyword_in("meow"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_system_verilog_12_keyword("soft"));
		assert!(keyword_in("soft").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_system_verilog_17() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_system_verilog_17_keyword("nya"));
		assert_eq!(keyword_in("nya"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_system_verilog_17_keyword("soft"));
		assert!(keyword_in("soft").contains(LanguageStd::Sv17));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_system_verilog_23() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_system_verilog_23_keyword("kon"));
		assert_eq!(keyword_in("kon"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_system_verilog_23_keyword("nettype"));
		assert!(keyword_in("nettype").contains(LanguageStd::Sv23));

		// SystemVerilog 2012, 2017, and 2023 all have the same keywords
	}

	#[test]
	fn test_keyword_verilog_ams_09() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_verilog_ams_09_keyword("wah"));
		assert_eq!(keyword_in("wah"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_verilog_ams_09_keyword("laplace_zp"));
		assert!(keyword_in("laplace_zp").contains(LanguageStd::Vams09));

		// A keyword that is one version up
		assert!(!is_verilog_ams_09_keyword("noise_table_log"));
		assert!(keyword_in("noise_table_log").contains(LanguageStd::Vams14));

		// Two versions up
		assert!(!is_verilog_ams_09_keyword("ln1p"));
		assert!(keyword_in("ln1p").contains(LanguageStd::Vams23));
	}

	#[test]
	fn test_keyword_verilog_ams_14() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_verilog_ams_14_keyword("nya"));
		assert_eq!(keyword_in("nya"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_verilog_ams_14_keyword("absdelta"));
		assert!(keyword_in("absdelta").contains(LanguageStd::Vams14));

		// A keyword that is one version up
		assert!(!is_verilog_ams_14_keyword("break"));
		assert!(keyword_in("break").contains(LanguageStd::Vams23));
	}

	#[test]
	fn test_keyword_verilog_ams_23() {
		// If it's a normal identifier, then we shouldn't get anything back
		assert!(!is_verilog_ams_23_keyword("meow"));
		assert_eq!(keyword_in("meow"), LanguageStd::none());

		// A keyword that is at least in the current version
		assert!(is_verilog_ams_23_keyword("expm1"));
		assert!(keyword_in("expm1").contains(LanguageStd::Vams23));
	}
}
