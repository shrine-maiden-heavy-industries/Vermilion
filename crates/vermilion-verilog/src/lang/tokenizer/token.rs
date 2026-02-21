// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::LanguageSet;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
	BaseSpecifier(BaseSpecifier, bool),
	Comment(Comment),
	CompilerDirective(CompilerDirective),
	/// Hold the verilog variant for when this token would become valid
	ContextuallyInvalid(AtomicByteTendril, LanguageSet),
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
	TextMacro(TextMacro),
	TripleQuotedString(AtomicByteTendril), // Added: IEEE 1800-2023
	UnsignedNumber(AtomicByteTendril),
	Whitespace(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextMacro {
	DunderFile, // Added: Verilog-AMS 2023 & IEEE 1800-2009
	DunderLine, // Added: Verilog-AMS 2023 & IEEE 1800-2009
	Other(AtomicByteTendril),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompilerDirective {
	Arg(AtomicByteTendril),
	Name(Directive),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comment {
	Invalid(AtomicByteTendril),
	MultiLine(AtomicByteTendril),
	SingleLine(AtomicByteTendril),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseSpecifier {
	Binary,
	Decimal,
	Hexadecimal,
	Octal,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
	AddEquals, // Added: IEEE 1800-2005
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
	EventTrigger,
	EventTriggerNb, // Added: IEEE 1800-2005
	Exclamation,
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
	Pow, // Added: IEEE 1364-2001
	ReductionNand,
	ReductionNor,
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
	WildcardNotEqual, // Added: IEEE 1800-2005
	XorEquals,        // Added: IEEE 1800-2005
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BaseSpecifier(base_specifier, _) => base_specifier.fmt(f),
			Self::Comment(comment) => comment.fmt(f),
			Self::CompilerDirective(compiler_directive) => compiler_directive.fmt(f),
			Self::ContextuallyInvalid(_tendril, _verilog_variant) => todo!(),
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

impl Display for TextMacro {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"`{}",
			match self {
				Self::DunderFile => "__FILE__", // Added: Verilog-AMS 2023 & IEEE 1800-2009
				Self::DunderLine => "__LINE__", // Added: Verilog-AMS 2023 & IEEE 1800-2009
				Self::Other(tendril) => unsafe { str::from_utf8_unchecked(tendril) },
			}
		)
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

impl Display for Directive {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"`{}",
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

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"'{}'",
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

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Operator({})",
			match self {
				Self::AddEquals => "+=", // Added: IEEE 1800-2005
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
				Self::EventTrigger => "->",
				Self::EventTriggerNb => "->>", // Added: IEEE 1800-2005
				Self::Exclamation => "!",
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
				Self::Pow => "**", // Added: IEEE 1364-2001
				Self::ReductionNand => "~&",
				Self::ReductionNor => "~|",
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
				Self::WildcardNotEqual => "!=?", // Added: IEEE 1800-2005
				Self::XorEquals => "^=",         // Added: IEEE 1800-2005
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
