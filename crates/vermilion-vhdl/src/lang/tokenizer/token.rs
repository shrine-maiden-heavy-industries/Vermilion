// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::AtomicByteTendril;

use crate::LanguageStd;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Comment {
	Invalid(AtomicByteTendril),
	MultiLine(AtomicByteTendril),
	SingleLine(AtomicByteTendril),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Control {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
	AtPins,          // Added: IEEE 1149.1-2013 (BSDL)
	Attribute,
	Bc0,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc1,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc2,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc3,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc4,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc5,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc6,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc7,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc8,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc9,  // Added: IEEE 1149.1-2013 (BSDL)
	Bc10, // Added: IEEE 1149.1-2013 (BSDL)
	Bc11, // Added: IEEE 1149.1-2013 (BSDL)
	Bc12, // Added: IEEE 1149.1-2013 (BSDL)
	Bc13, // Added: IEEE 1149.1-2013 (BSDL)
	Bc14, // Added: IEEE 1149.1-2013 (BSDL)
	Bc15, // Added: IEEE 1149.1-2013 (BSDL)
	Bc16, // Added: IEEE 1149.1-2013 (BSDL)
	Bc17, // Added: IEEE 1149.1-2013 (BSDL)
	Bc18, // Added: IEEE 1149.1-2013 (BSDL)
	Bc19, // Added: IEEE 1149.1-2013 (BSDL)
	Bc20, // Added: IEEE 1149.1-2013 (BSDL)
	Bc21, // Added: IEEE 1149.1-2013 (BSDL)
	Bc22, // Added: IEEE 1149.1-2013 (BSDL)
	Bc23, // Added: IEEE 1149.1-2013 (BSDL)
	Bc24, // Added: IEEE 1149.1-2013 (BSDL)
	Bc25, // Added: IEEE 1149.1-2013 (BSDL)
	Bc26, // Added: IEEE 1149.1-2013 (BSDL)
	Bc27, // Added: IEEE 1149.1-2013 (BSDL)
	Bc28, // Added: IEEE 1149.1-2013 (BSDL)
	Bc29, // Added: IEEE 1149.1-2013 (BSDL)
	Bc30, // Added: IEEE 1149.1-2013 (BSDL)
	Bc31, // Added: IEEE 1149.1-2013 (BSDL)
	Bc32, // Added: IEEE 1149.1-2013 (BSDL)
	Bc33, // Added: IEEE 1149.1-2013 (BSDL)
	Bc34, // Added: IEEE 1149.1-2013 (BSDL)
	Bc35, // Added: IEEE 1149.1-2013 (BSDL)
	Bc36, // Added: IEEE 1149.1-2013 (BSDL)
	Bc37, // Added: IEEE 1149.1-2013 (BSDL)
	Bc38, // Added: IEEE 1149.1-2013 (BSDL)
	Bc39, // Added: IEEE 1149.1-2013 (BSDL)
	Bc40, // Added: IEEE 1149.1-2013 (BSDL)
	Bc41, // Added: IEEE 1149.1-2013 (BSDL)
	Bc42, // Added: IEEE 1149.1-2013 (BSDL)
	Bc43, // Added: IEEE 1149.1-2013 (BSDL)
	Bc44, // Added: IEEE 1149.1-2013 (BSDL)
	Bc45, // Added: IEEE 1149.1-2013 (BSDL)
	Bc46, // Added: IEEE 1149.1-2013 (BSDL)
	Bc47, // Added: IEEE 1149.1-2013 (BSDL)
	Bc48, // Added: IEEE 1149.1-2013 (BSDL)
	Bc49, // Added: IEEE 1149.1-2013 (BSDL)
	Bc50, // Added: IEEE 1149.1-2013 (BSDL)
	Bc51, // Added: IEEE 1149.1-2013 (BSDL)
	Bc52, // Added: IEEE 1149.1-2013 (BSDL)
	Bc53, // Added: IEEE 1149.1-2013 (BSDL)
	Bc54, // Added: IEEE 1149.1-2013 (BSDL)
	Bc55, // Added: IEEE 1149.1-2013 (BSDL)
	Bc56, // Added: IEEE 1149.1-2013 (BSDL)
	Bc57, // Added: IEEE 1149.1-2013 (BSDL)
	Bc58, // Added: IEEE 1149.1-2013 (BSDL)
	Bc59, // Added: IEEE 1149.1-2013 (BSDL)
	Bc60, // Added: IEEE 1149.1-2013 (BSDL)
	Bc61, // Added: IEEE 1149.1-2013 (BSDL)
	Bc62, // Added: IEEE 1149.1-2013 (BSDL)
	Bc63, // Added: IEEE 1149.1-2013 (BSDL)
	Bc64, // Added: IEEE 1149.1-2013 (BSDL)
	Bc65, // Added: IEEE 1149.1-2013 (BSDL)
	Bc66, // Added: IEEE 1149.1-2013 (BSDL)
	Bc67, // Added: IEEE 1149.1-2013 (BSDL)
	Bc68, // Added: IEEE 1149.1-2013 (BSDL)
	Bc69, // Added: IEEE 1149.1-2013 (BSDL)
	Bc70, // Added: IEEE 1149.1-2013 (BSDL)
	Bc71, // Added: IEEE 1149.1-2013 (BSDL)
	Bc72, // Added: IEEE 1149.1-2013 (BSDL)
	Bc73, // Added: IEEE 1149.1-2013 (BSDL)
	Bc74, // Added: IEEE 1149.1-2013 (BSDL)
	Bc75, // Added: IEEE 1149.1-2013 (BSDL)
	Bc76, // Added: IEEE 1149.1-2013 (BSDL)
	Bc77, // Added: IEEE 1149.1-2013 (BSDL)
	Bc78, // Added: IEEE 1149.1-2013 (BSDL)
	Bc79, // Added: IEEE 1149.1-2013 (BSDL)
	Bc80, // Added: IEEE 1149.1-2013 (BSDL)
	Bc81, // Added: IEEE 1149.1-2013 (BSDL)
	Bc82, // Added: IEEE 1149.1-2013 (BSDL)
	Bc83, // Added: IEEE 1149.1-2013 (BSDL)
	Bc84, // Added: IEEE 1149.1-2013 (BSDL)
	Bc85, // Added: IEEE 1149.1-2013 (BSDL)
	Bc86, // Added: IEEE 1149.1-2013 (BSDL)
	Bc87, // Added: IEEE 1149.1-2013 (BSDL)
	Bc88, // Added: IEEE 1149.1-2013 (BSDL)
	Bc89, // Added: IEEE 1149.1-2013 (BSDL)
	Bc90, // Added: IEEE 1149.1-2013 (BSDL)
	Bc91, // Added: IEEE 1149.1-2013 (BSDL)
	Bc92, // Added: IEEE 1149.1-2013 (BSDL)
	Bc93, // Added: IEEE 1149.1-2013 (BSDL)
	Bc94, // Added: IEEE 1149.1-2013 (BSDL)
	Bc95, // Added: IEEE 1149.1-2013 (BSDL)
	Bc96, // Added: IEEE 1149.1-2013 (BSDL)
	Bc97, // Added: IEEE 1149.1-2013 (BSDL)
	Bc98, // Added: IEEE 1149.1-2013 (BSDL)
	Bc99, // Added: IEEE 1149.1-2013 (BSDL)
	Begin,
	BiDir,    // Added: IEEE 1149.1-2013 (BSDL)
	BiDirIn,  // Added: IEEE 1149.1-2013 (BSDL)
	BiDirOut, // Added: IEEE 1149.1-2013 (BSDL)
	Block,
	Body,
	Both,             // Added: IEEE 1149.1-2013 (BSDL)
	Boundary,         // Added: IEEE 1149.1-2013 (BSDL)
	BoundaryLength,   // Added: IEEE 1149.1-2013 (BSDL)
	BoundaryRegister, // Added: IEEE 1149.1-2013 (BSDL)
	BoundarySegment,  // Added: IEEE 1149.1-2013 (BSDL)
	Break,            // Added: IEEE 1076.1-1999
	BroadcastField,   // Added: IEEE 1149.1-2013 (BSDL)
	BroadcastValues,  // Added: IEEE 1149.1-2013 (BSDL)
	BscanInst,        // Added: IEEE 1149.1-2013 (BSDL)
	BsdlExtension,    // Added: IEEE 1149.1-2013 (BSDL)
	Buffer,
	Bus,
	Bypass,   // Added: IEEE 1149.1-2013 (BSDL)
	Cap,      // Added: IEEE 1149.1-2013 (BSDL)
	CapData,  // Added: IEEE 1149.1-2013 (BSDL)
	Captures, // Added: IEEE 1149.1-2013 (BSDL)
	Case,
	CelLData,           // Added: IEEE 1149.1-2013 (BSDL)
	CellInfo,           // Added: IEEE 1149.1-2013 (BSDL)
	ChReset,            // Added: IEEE 1149.1-2013 (BSDL)
	Clamp,              // Added: IEEE 1149.1-2013 (BSDL)
	ClampHold,          // Added: IEEE 1149.1-2013 (BSDL)
	ClampRelease,       // Added: IEEE 1149.1-2013 (BSDL)
	Clock,              // Added: IEEE 1149.1-2013 (BSDL)
	ClockInfo,          // Added: IEEE 1149.1-2013 (BSDL)
	ClockLevel,         // Added: IEEE 1149.1-2013 (BSDL)
	CompliancePatterns, // Added: IEEE 1149.1-2013 (BSDL)
	Component,
	ComponentConformance, // Added: IEEE 1149.1-2013 (BSDL)
	Configuration,
	Constant,
	Context,             // Added: IEEE 1076-2008
	Control,             // Added: IEEE 1149.1-2013 (BSDL)
	ControlR,            // Added: IEEE 1149.1-2013 (BSDL)
	Cover,               // Added: IEEE 1076-2008
	Default,             // Added: IEEE 1076-2008 & IEEE 1149.1-2013 (BSDL)
	DelayPo,             // Added: IEEE 1149.1-2013 (BSDL)
	DesignWarning,       // Added: IEEE 1149.1-2013 (BSDL)
	DeviceID,            // Added: IEEE 1149.1-2013 (BSDL)
	DifferentialCurrent, // Added: IEEE 1149.1-2013 (BSDL)
	DifferentialVoltage, // Added: IEEE 1149.1-2013 (BSDL)
	Disconnect,
	Domain,         // Added: IEEE 1149.1-2013 (BSDL)
	DomainExternal, // Added: IEEE 1149.1-2013 (BSDL)
	DomCtrl,        // Added: IEEE 1149.1-2013 (BSDL)
	DomPor,         // Added: IEEE 1149.1-2013 (BSDL)
	DownTo,
	Ecid,     // Added: IEEE 1149.1-2013 (BSDL)
	Ecidcode, // Added: IEEE 1149.1-2013 (BSDL)
	Else,
	Elsif,
	End,
	Entity,
	Exit,
	Expect0,    // Added: IEEE 1149.1-2013 (BSDL)
	Expect1,    // Added: IEEE 1149.1-2013 (BSDL)
	ExpectData, // Added: IEEE 1149.1-2013 (BSDL)
	ExTest,     // Added: IEEE 1149.1-2013 (BSDL)
	Fairness,   // Added: IEEE 1076-2008
	File,
	For,
	Force, // Added: IEEE 1076-2008
	Function,
	Generate,
	Generic,
	Group, // Added: IEEE 1076-1993
	Guarded,
	HierReset,      // Added: IEEE 1149.1-2013 (BSDL)
	HighZ,          // Added: IEEE 1149.1-2013 (BSDL)
	IcReset,        // Added: IEEE 1149.1-2013 (BSDL)
	IdBits,         // Added: IEEE 1149.1-2013 (BSDL)
	Idcode,         // Added: IEEE 1149.1-2013 (BSDL)
	IdcodeRegister, // Added: IEEE 1149.1-2013 (BSDL)
	IdString,       // Added: IEEE 1149.1-2013 (BSDL)
	If,
	Impure, // Added: IEEE 1076-1993
	In,
	Inertial,       // Added: IEEE 1076-1993
	InitData,       // Added: IEEE 1149.1-2013 (BSDL)
	InitRun,        // Added: IEEE 1149.1-2013 (BSDL)
	InitSetup,      // Added: IEEE 1149.1-2013 (BSDL)
	InitSetupClamp, // Added: IEEE 1149.1-2013 (BSDL)
	InitStatus,     // Added: IEEE 1149.1-2013 (BSDL)
	InOut,
	Input,              // Added: IEEE 1149.1-2013 (BSDL)
	InstructionCapture, // Added: IEEE 1149.1-2013 (BSDL)
	InstructionLength,  // Added: IEEE 1149.1-2013 (BSDL)
	InstructionOpcode,  // Added: IEEE 1149.1-2013 (BSDL)
	InstructionPrivate, // Added: IEEE 1149.1-2013 (BSDL)
	Internal,           // Added: IEEE 1149.1-2013 (BSDL)
	InTest,             // Added: IEEE 1149.1-2013 (BSDL)
	InTestExecution,    // Added: IEEE 1149.1-2013 (BSDL)
	Is,
	Keeper, // Added: IEEE 1149.1-2013 (BSDL)
	Label,
	Library,
	Limit, // Added: IEEE 1076.1-1999
	Linkage,
	LinkageBuffer,     // Added: IEEE 1149.1-2013 (BSDL)
	LinkageIn,         // Added: IEEE 1149.1-2013 (BSDL)
	LinkageInOut,      // Added: IEEE 1149.1-2013 (BSDL)
	LinkageMechanical, // Added: IEEE 1149.1-2013 (BSDL)
	LinkageOut,        // Added: IEEE 1149.1-2013 (BSDL)
	Literal,           // Added: IEEE 1076-1993
	Loop,
	Low, // Added: IEEE 1149.1-2013 (BSDL)
	Map,
	Mod,
	Mon, // Added: IEEE 1149.1-2013 (BSDL)
	Nand,
	Nature, // Added: IEEE 1076.1-1999
	New,
	Next,
	Noise, // Added: IEEE 1076.1-1999
	Nopi,  // Added: IEEE 1149.1-2013 (BSDL)
	Nopo,  // Added: IEEE 1149.1-2013 (BSDL)
	Nor,
	NoRetain, // Added: IEEE 1149.1-2013 (BSDL)
	Not,
	NoUpd, // Added: IEEE 1149.1-2013 (BSDL)
	Null,
	ObserveOnly, // Added: IEEE 1149.1-2013 (BSDL)
	Observing,   // Added: IEEE 1149.1-2013 (BSDL)
	Of,
	On,
	One,    // Added: IEEE 1149.1-2013 (BSDL)
	OneHot, // Added: IEEE 1149.1-2013 (BSDL)
	Open,   // Added: IEEE 1076-1987 & IEEE 1149.1-2013 (BSDL)
	Open0,  // Added: IEEE 1149.1-2013 (BSDL)
	Open1,  // Added: IEEE 1149.1-2013 (BSDL)
	OpenX,  // Added: IEEE 1149.1-2013 (BSDL)
	Or,
	Others,
	Out,
	Output2, // Added: IEEE 1149.1-2013 (BSDL)
	Output3, // Added: IEEE 1149.1-2013 (BSDL)
	Package,
	Parameter,      // Added: IEEE 1076-2008
	PhysicalPinMap, // Added: IEEE 1149.1-2013 (BSDL)
	Pi,             // Added: IEEE 1149.1-2013 (BSDL)
	PinMap,         // Added: IEEE 1149.1-2013 (BSDL)
	PinMapString,   // Added: IEEE 1149.1-2013 (BSDL)
	Po,             // Added: IEEE 1149.1-2013 (BSDL)
	PorReset,       // Added: IEEE 1149.1-2013 (BSDL)
	Port,
	PortGrouping,         // Added: IEEE 1149.1-2013 (BSDL)
	Postponed,            // Added: IEEE 1076-1993
	Power0,               // Added: IEEE 1149.1-2013 (BSDL)
	PowerNeg,             // Added: IEEE 1149.1-2013 (BSDL)
	PowerPortAssociation, // Added: IEEE 1149.1-2013 (BSDL)
	PowerPos,             // Added: IEEE 1149.1-2013 (BSDL)
	Preload,              // Added: IEEE 1149.1-2013 (BSDL)
	Private,              // Added: IEEE 1076-2019
	Procedural,           // Added: IEEE 1076-2000 & IEEE 1076.1-1999
	Procedure,
	Process,
	Property,  // Added: IEEE 1076-2008
	Protected, // Added: IEEE 1076-2000 & IEEE 1076.1-2007
	Pull0,     // Added: IEEE 1149.1-2013 (BSDL)
	Pull1,     // Added: IEEE 1149.1-2013 (BSDL)
	Pulse0,    // Added: IEEE 1149.1-2013 (BSDL)
	Pulse1,    // Added: IEEE 1149.1-2013 (BSDL)
	Pure,      // Added: IEEE 1076-1993
	Quantity,  // Added: IEEE 1076.1-1999
	Range,
	Record,
	Reference, // Added: IEEE 1076-2000 & IEEE 1076.1-1999
	Register,
	RegisterAccess,      // Added: IEEE 1149.1-2013 (BSDL)
	RegisterAssembly,    // Added: IEEE 1149.1-2013 (BSDL)
	RegisterAssociation, // Added: IEEE 1149.1-2013 (BSDL)
	RegisterConstraints, // Added: IEEE 1149.1-2013 (BSDL)
	RegisterFields,      // Added: IEEE 1149.1-2013 (BSDL)
	RegisterMnemonics,   // Added: IEEE 1149.1-2013 (BSDL)
	Reject,              // Added: IEEE 1076-1993
	Release,             // Added: IEEE 1076-2008
	Rem,
	Report,
	ResetSelect,       // Added: IEEE 1149.1-2013 (BSDL)
	ResetVal,          // Added: IEEE 1149.1-2013 (BSDL)
	Restrict,          // Added: IEEE 1076-2008
	RestrictGuarantee, // Added: IEEE 1076-2008
	Return,
	Rol,              // Added: IEEE 1076-1993
	Ror,              // Added: IEEE 1076-1993
	RunBist,          // Added: IEEE 1149.1-2013 (BSDL)
	RunBistExecution, // Added: IEEE 1149.1-2013 (BSDL)
	Safe,             // Added: IEEE 1149.1-2013 (BSDL)
	Sample,           // Added: IEEE 1149.1-2013 (BSDL)
	Segment,          // Added: IEEE 1149.1-2013 (BSDL)
	SegMux,           // Added: IEEE 1149.1-2013 (BSDL)
	SegStart,         // Added: IEEE 1149.1-2013 (BSDL)
	Select,
	SelectField,  // Added: IEEE 1149.1-2013 (BSDL)
	SelectMux,    // Added: IEEE 1149.1-2013 (BSDL)
	SelectValues, // Added: IEEE 1149.1-2013 (BSDL)
	Sequence,     // Added: IEEE 1076-2008
	Severity,
	Shared, // Added: IEEE 1076-1993 & IEEE 1149.1-2013 (BSDL)
	Signal,
	Sla,       // Added: IEEE 1076-1993
	Sll,       // Added: IEEE 1076-1993
	Spectrum,  // Added: IEEE 1076.1-1999
	Sra,       // Added: IEEE 1076-1993
	Srl,       // Added: IEEE 1076-1993
	Strong,    // Added: IEEE 1076-2008
	SubNature, // Added: IEEE 1076.1-1999
	Subtype,
	TapReset,     // Added: IEEE 1149.1-2013 (BSDL)
	TapScanClock, // Added: IEEE 1149.1-2013 (BSDL)
	TapScanIn,    // Added: IEEE 1149.1-2013 (BSDL)
	TapScanMode,  // Added: IEEE 1149.1-2013 (BSDL)
	TapScanOut,   // Added: IEEE 1149.1-2013 (BSDL)
	TapScanReset, // Added: IEEE 1149.1-2013 (BSDL)
	Terminal,     // Added: IEEE 1076.1-1999
	Then,
	Through,   // Added: IEEE 1076.1-1999
	Tie0,      // Added: IEEE 1149.1-2013 (BSDL)
	Tie1,      // Added: IEEE 1149.1-2013 (BSDL)
	TmpStatus, // Added: IEEE 1149.1-2013 (BSDL)
	To,
	Tolerance, // Added: IEEE 1076.1-1999
	Transport,
	TrstReset, // Added: IEEE 1149.1-2013 (BSDL)
	Type,
	Unaffected, // Added: IEEE 1076-1993
	Units,
	Until,
	Upd, // Added: IEEE 1149.1-2013 (BSDL)
	Use,
	User,             // Added: IEEE 1149.1-2013 (BSDL)
	UserCode,         // Added: IEEE 1149.1-2013 (BSDL)
	UserCodeRegister, // Added: IEEE 1149.1-2013 (BSDL)
	Variable,
	View,    // Added: IEEE 1076-2019
	Vmode,   // Added: IEEE 1076-2008
	Vpkg,    // Added: IEEE 1076-2019
	Vprop,   // Added: IEEE 1076-2008
	VrefIn,  // Added: IEEE 1149.1-2013 (BSDL)
	VrefOut, // Added: IEEE 1149.1-2013 (BSDL)
	Vunit,   // Added: IEEE 1076-2008
	Wait,
	WaitDuration, // Added: IEEE 1149.1-2013 (BSDL)
	Weak0,        // Added: IEEE 1149.1-2013 (BSDL)
	Weak1,        // Added: IEEE 1149.1-2013 (BSDL)
	When,
	While,
	With,
	X,    // Added: IEEE 1149.1-2013 (BSDL)
	Xnor, // Added: IEEE 1076-1993
	Xor,
	Z,    // Added: IEEE 1149.1-2013 (BSDL)
	Zero, // Added: IEEE 1149.1-2013 (BSDL)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
		write!(f, "Comment()")
	}
}

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Control()")
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
				Self::AtPins => "at_pins",                   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Attribute => "attribute",
				Self::Bc0 => "bc_0",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc1 => "bc_1",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc2 => "bc_2",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc3 => "bc_3",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc4 => "bc_4",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc5 => "bc_5",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc6 => "bc_6",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc7 => "bc_7",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc8 => "bc_8",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc9 => "bc_9",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc10 => "bc_10", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc11 => "bc_11", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc12 => "bc_12", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc13 => "bc_13", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc14 => "bc_14", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc15 => "bc_15", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc16 => "bc_16", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc17 => "bc_17", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc18 => "bc_18", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc19 => "bc_19", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc20 => "bc_20", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc21 => "bc_21", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc22 => "bc_22", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc23 => "bc_23", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc24 => "bc_24", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc25 => "bc_25", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc26 => "bc_26", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc27 => "bc_27", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc28 => "bc_28", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc29 => "bc_29", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc30 => "bc_30", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc31 => "bc_31", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc32 => "bc_32", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc33 => "bc_33", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc34 => "bc_34", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc35 => "bc_35", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc36 => "bc_36", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc37 => "bc_37", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc38 => "bc_38", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc39 => "bc_39", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc40 => "bc_40", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc41 => "bc_41", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc42 => "bc_42", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc43 => "bc_43", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc44 => "bc_44", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc45 => "bc_45", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc46 => "bc_46", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc47 => "bc_47", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc48 => "bc_48", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc49 => "bc_49", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc50 => "bc_50", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc51 => "bc_51", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc52 => "bc_52", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc53 => "bc_53", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc54 => "bc_54", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc55 => "bc_55", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc56 => "bc_56", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc57 => "bc_57", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc58 => "bc_58", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc59 => "bc_59", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc60 => "bc_60", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc61 => "bc_61", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc62 => "bc_62", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc63 => "bc_63", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc64 => "bc_64", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc65 => "bc_65", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc66 => "bc_66", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc67 => "bc_67", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc68 => "bc_68", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc69 => "bc_69", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc70 => "bc_70", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc71 => "bc_71", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc72 => "bc_72", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc73 => "bc_73", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc74 => "bc_74", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc75 => "bc_75", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc76 => "bc_76", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc77 => "bc_77", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc78 => "bc_78", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc79 => "bc_79", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc80 => "bc_80", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc81 => "bc_81", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc82 => "bc_82", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc83 => "bc_83", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc84 => "bc_84", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc85 => "bc_85", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc86 => "bc_86", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc87 => "bc_87", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc88 => "bc_88", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc89 => "bc_89", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc90 => "bc_90", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc91 => "bc_91", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc92 => "bc_92", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc93 => "bc_93", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc94 => "bc_94", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc95 => "bc_95", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc96 => "bc_96", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc97 => "bc_97", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc98 => "bc_98", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Bc99 => "bc_99", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Begin => "begin",
				Self::BiDir => "bidir",        // Added: IEEE 1149.1-2013 (BSDL)
				Self::BiDirIn => "bidir_in",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::BiDirOut => "bidir_out", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Block => "block",
				Self::Body => "body",
				Self::Both => "both",                      // Added: IEEE 1149.1-2013 (BSDL)
				Self::Boundary => "boundary",              // Added: IEEE 1149.1-2013 (BSDL)
				Self::BoundaryLength => "boundary_length", // Added: IEEE 1149.1-2013 (BSDL)
				Self::BoundaryRegister => "boundary_register", // Added: IEEE 1149.1-2013 (BSDL)
				Self::BoundarySegment => "boundary_segment", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Break => "break",                    // Added: IEEE 1076.1-1999
				Self::BroadcastField => "broadcastfield",  // Added: IEEE 1149.1-2013 (BSDL)
				Self::BroadcastValues => "broadcastvalues", // Added: IEEE 1149.1-2013 (BSDL)
				Self::BscanInst => "bscan_inst",           // Added: IEEE 1149.1-2013 (BSDL)
				Self::BsdlExtension => "bsdl_extension",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Buffer => "buffer",
				Self::Bus => "bus",
				Self::Bypass => "bypass",     // Added: IEEE 1149.1-2013 (BSDL)
				Self::Cap => "cap",           // Added: IEEE 1149.1-2013 (BSDL)
				Self::CapData => "cap_data",  // Added: IEEE 1149.1-2013 (BSDL)
				Self::Captures => "captures", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Case => "case",
				Self::CelLData => "celL_data",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::CellInfo => "cell_info",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::ChReset => "ch_reset",     // Added: IEEE 1149.1-2013 (BSDL)
				Self::Clamp => "clamp",          // Added: IEEE 1149.1-2013 (BSDL)
				Self::ClampHold => "clamp_hold", // Added: IEEE 1149.1-2013 (BSDL)
				Self::ClampRelease => "clamp_release", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Clock => "clock",          // Added: IEEE 1149.1-2013 (BSDL)
				Self::ClockInfo => "clock_info", // Added: IEEE 1149.1-2013 (BSDL)
				Self::ClockLevel => "clock_level", // Added: IEEE 1149.1-2013 (BSDL)
				Self::CompliancePatterns => "compliance_patterns", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Component => "component",
				Self::ComponentConformance => "component_conformance", // Added: IEEE
				// 1149.1-2013 (BSDL)
				Self::Configuration => "configuration",
				Self::Constant => "constant",
				Self::Context => "context",   // Added: IEEE 1076-2008
				Self::Control => "control",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::ControlR => "controlr", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Cover => "cover",       // Added: IEEE 1076-2008
				Self::Default => "default",   // Added: IEEE 1076-2008 & IEEE
				// 1149.1-2013 (BSDL)
				Self::DelayPo => "delaypo", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DesignWarning => "design_warning", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DeviceID => "device_id", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DifferentialCurrent => "differential_current", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DifferentialVoltage => "differential_voltage", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Disconnect => "disconnect",
				Self::Domain => "domain", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DomainExternal => "domain_external", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DomCtrl => "domctrl", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DomPor => "dompor", // Added: IEEE 1149.1-2013 (BSDL)
				Self::DownTo => "downto",
				Self::Ecid => "ecid",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::Ecidcode => "ecidcode", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Else => "else",
				Self::Elsif => "elsif",
				Self::End => "end",
				Self::Entity => "entity",
				Self::Exit => "exit",
				Self::Expect0 => "expect0",        // Added: IEEE 1149.1-2013 (BSDL)
				Self::Expect1 => "expect1",        // Added: IEEE 1149.1-2013 (BSDL)
				Self::ExpectData => "expect_data", // Added: IEEE 1149.1-2013 (BSDL)
				Self::ExTest => "ex_test",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::Fairness => "fairness",      // Added: IEEE 1076-2008
				Self::File => "file",
				Self::For => "for",
				Self::Force => "force", // Added: IEEE 1076-2008
				Self::Function => "function",
				Self::Generate => "generate",
				Self::Generic => "generic",
				Self::Group => "group", // Added: IEEE 1076-1993
				Self::Guarded => "guarded",
				Self::HierReset => "hierreset", // Added: IEEE 1149.1-2013 (BSDL)
				Self::HighZ => "highz",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::IcReset => "ic_reset",    // Added: IEEE 1149.1-2013 (BSDL)
				Self::IdBits => "id_bits",      // Added: IEEE 1149.1-2013 (BSDL)
				Self::Idcode => "idcode",       // Added: IEEE 1149.1-2013 (BSDL)
				Self::IdcodeRegister => "idcode_register", // Added: IEEE 1149.1-2013 (BSDL)
				Self::IdString => "id_string",  // Added: IEEE 1149.1-2013 (BSDL)
				Self::If => "if",
				Self::Impure => "impure", // Added: IEEE 1076-1993
				Self::In => "in",
				Self::Inertial => "inertial",               // Added: IEEE 1076-1993
				Self::InitData => "init_data",              // Added: IEEE 1149.1-2013 (BSDL)
				Self::InitRun => "init_run",                // Added: IEEE 1149.1-2013 (BSDL)
				Self::InitSetup => "init_setup",            // Added: IEEE 1149.1-2013 (BSDL)
				Self::InitSetupClamp => "Init_setup_clamp", // Added: IEEE 1149.1-2013 (BSDL)
				Self::InitStatus => "init_status",          // Added: IEEE 1149.1-2013 (BSDL)
				Self::InOut => "inout",
				Self::Input => "input", // Added: IEEE 1149.1-2013 (BSDL)
				Self::InstructionCapture => "instruction_capture", // Added: IEEE 1149.1-2013 (BSDL)
				Self::InstructionLength => "instruction_length", // Added: IEEE 1149.1-2013 (BSDL)
				Self::InstructionOpcode => "instruction_opcode", // Added: IEEE 1149.1-2013 (BSDL)
				Self::InstructionPrivate => "instruction_private", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Internal => "internal", // Added: IEEE 1149.1-2013 (BSDL)
				Self::InTest => "intest", // Added: IEEE 1149.1-2013 (BSDL)
				Self::InTestExecution => "intest_execution", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Is => "is",
				Self::Keeper => "keeper", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Label => "label",
				Self::Library => "library",
				Self::Limit => "limit", // Added: IEEE 1076.1-1999
				Self::Linkage => "linkage",
				Self::LinkageBuffer => "linkage_buffer", // Added: IEEE 1149.1-2013 (BSDL)
				Self::LinkageIn => "linkage_in",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::LinkageInOut => "linkage_inout",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::LinkageMechanical => "linkage_mechanical", // Added: IEEE 1149.1-2013 (BSDL)
				Self::LinkageOut => "linkage_out",       // Added: IEEE 1149.1-2013 (BSDL)
				Self::Literal => "literal",              // Added: IEEE 1076-1993
				Self::Loop => "loop",
				Self::Low => "low", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Map => "map",
				Self::Mod => "mod",
				Self::Mon => "mon", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Nand => "nand",
				Self::Nature => "nature", // Added: IEEE 1076.1-1999
				Self::New => "new",
				Self::Next => "next",
				Self::Noise => "noise", // Added: IEEE 1076.1-1999
				Self::Nopi => "nopi",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Nopo => "nopo",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::Nor => "nor",
				Self::NoRetain => "noretain", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Not => "not",
				Self::NoUpd => "noupd", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Null => "null",
				Self::ObserveOnly => "observe_only", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Observing => "observing",      // Added: IEEE 1149.1-2013 (BSDL)
				Self::Of => "of",
				Self::On => "on",
				Self::One => "one",        // Added: IEEE 1149.1-2013 (BSDL)
				Self::OneHot => "one_hot", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Open => "open",      // Added: IEEE 1076-1987 & IEEE 1149.1-2013 (BSDL)
				Self::Open0 => "open0",    // Added: IEEE 1149.1-2013 (BSDL)
				Self::Open1 => "open1",    // Added: IEEE 1149.1-2013 (BSDL)
				Self::OpenX => "openX",    // Added: IEEE 1149.1-2013 (BSDL)
				Self::Or => "or",
				Self::Others => "others",
				Self::Out => "out",
				Self::Output2 => "output2", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Output3 => "output3", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Package => "package",
				Self::Parameter => "parameter", // Added: IEEE 1076-2008
				Self::PhysicalPinMap => "physical_pin_map", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Pi => "pi",               // Added: IEEE 1149.1-2013 (BSDL)
				Self::PinMap => "pin_map",      // Added: IEEE 1149.1-2013 (BSDL)
				Self::PinMapString => "pin_map_string", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Po => "po",               // Added: IEEE 1149.1-2013 (BSDL)
				Self::PorReset => "por_reset",  // Added: IEEE 1149.1-2013 (BSDL)
				Self::Port => "port",
				Self::PortGrouping => "port_grouping", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Postponed => "postponed",        // Added: IEEE 1076-1993
				Self::Power0 => "power0",              // Added: IEEE 1149.1-2013 (BSDL)
				Self::PowerNeg => "power_neg",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::PowerPortAssociation => "power_port_association", // Added: IEEE 1149.1-2013 (BSDL)
				Self::PowerPos => "power_pos",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::Preload => "preload",            // Added: IEEE 1149.1-2013 (BSDL)
				Self::Private => "private",            // Added: IEEE 1076-2019
				Self::Procedural => "procedural",      // Added: IEEE 1076-2000 & IEEE 1076.1-1999
				Self::Procedure => "procedure",
				Self::Process => "process",
				Self::Property => "property",   // Added: IEEE 1076-2008
				Self::Protected => "protected", // Added: IEEE 1076-2000 & IEEE 1076.1-2007
				Self::Pull0 => "pull0",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::Pull1 => "pull1",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::Pulse0 => "pulse0",       // Added: IEEE 1149.1-2013 (BSDL)
				Self::Pulse1 => "pulse1",       // Added: IEEE 1149.1-2013 (BSDL)
				Self::Pure => "pure",           // Added: IEEE 1076-1993
				Self::Quantity => "quantity",   // Added: IEEE 1076.1-1999
				Self::Range => "range",
				Self::Record => "record",
				Self::Reference => "reference", // Added: IEEE 1076-2000 & IEEE 1076.1-1999
				Self::Register => "register",
				Self::RegisterAccess => "register_access", // Added: IEEE 1149.1-2013 (BSDL)
				Self::RegisterAssembly => "register_assembly", // Added: IEEE 1149.1-2013 (BSDL)
				Self::RegisterAssociation => "register_association", // Added: IEEE 1149.1-2013 (BSDL)
				Self::RegisterConstraints => "register_constraints", // Added: IEEE 1149.1-2013 (BSDL)
				Self::RegisterFields => "register_fields", // Added: IEEE 1149.1-2013 (BSDL)
				Self::RegisterMnemonics => "register_mnemonics", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Reject => "reject",                  // Added: IEEE 1076-1993
				Self::Release => "release",                // Added: IEEE 1076-2008
				Self::Rem => "rem",
				Self::Report => "report",
				Self::ResetSelect => "reset_select", // Added: IEEE 1149.1-2013 (BSDL)
				Self::ResetVal => "resetval",        // Added: IEEE 1149.1-2013 (BSDL)
				Self::Restrict => "restrict",        // Added: IEEE 1076-2008
				Self::RestrictGuarantee => "restrict_guarantee", // Added: IEEE 1076-2008
				Self::Return => "return",
				Self::Rol => "rol",                            // Added: IEEE 1076-1993
				Self::Ror => "ror",                            // Added: IEEE 1076-1993
				Self::RunBist => "runbist",                    // Added: IEEE 1149.1-2013 (BSDL)
				Self::RunBistExecution => "runbist_execution", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Safe => "safe",                          // Added: IEEE 1149.1-2013 (BSDL)
				Self::Sample => "sample",                      // Added: IEEE 1149.1-2013 (BSDL)
				Self::Segment => "segment",                    // Added: IEEE 1149.1-2013 (BSDL)
				Self::SegMux => "segmux",                      // Added: IEEE 1149.1-2013 (BSDL)
				Self::SegStart => "segstart",                  // Added: IEEE 1149.1-2013 (BSDL)
				Self::Select => "select",
				Self::SelectField => "selectfield", // Added: IEEE 1149.1-2013 (BSDL)
				Self::SelectMux => "selectmux",     // Added: IEEE 1149.1-2013 (BSDL)
				Self::SelectValues => "selectvalues", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Sequence => "sequence",       // Added: IEEE 1076-2008
				Self::Severity => "severity",
				Self::Shared => "shared", // Added: IEEE 1076-1993 & IEEE 1149.1-2013 (BSDL)
				Self::Signal => "signal",
				Self::Sla => "sla",             // Added: IEEE 1076-1993
				Self::Sll => "sll",             // Added: IEEE 1076-1993
				Self::Spectrum => "spectrum",   // Added: IEEE 1076.1-1999
				Self::Sra => "sra",             // Added: IEEE 1076-1993
				Self::Srl => "srl",             // Added: IEEE 1076-1993
				Self::Strong => "strong",       // Added: IEEE 1076-2008
				Self::SubNature => "subnature", // Added: IEEE 1076.1-1999
				Self::Subtype => "subtype",
				Self::TapReset => "tapreset", // Added: IEEE 1149.1-2013 (BSDL)
				Self::TapScanClock => "tap_scan_clock", // Added: IEEE 1149.1-2013 (BSDL)
				Self::TapScanIn => "tap_scan_in", // Added: IEEE 1149.1-2013 (BSDL)
				Self::TapScanMode => "tap_scan_mode", // Added: IEEE 1149.1-2013 (BSDL)
				Self::TapScanOut => "tap_scan_out", // Added: IEEE 1149.1-2013 (BSDL)
				Self::TapScanReset => "tap_scan_reset", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Terminal => "terminal", // Added: IEEE 1076.1-1999
				Self::Then => "then",
				Self::Through => "through",      // Added: IEEE 1076.1-1999
				Self::Tie0 => "tie0",            // Added: IEEE 1149.1-2013 (BSDL)
				Self::Tie1 => "tie1",            // Added: IEEE 1149.1-2013 (BSDL)
				Self::TmpStatus => "tmp_status", // Added: IEEE 1149.1-2013 (BSDL)
				Self::To => "to",
				Self::Tolerance => "tolerance", // Added: IEEE 1076.1-1999
				Self::Transport => "transport",
				Self::TrstReset => "trstreset", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Type => "type",
				Self::Unaffected => "unaffected", // Added: IEEE 1076-1993
				Self::Units => "units",
				Self::Until => "until",
				Self::Upd => "upd", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Use => "use",
				Self::User => "user",         // Added: IEEE 1149.1-2013 (BSDL)
				Self::UserCode => "usercode", // Added: IEEE 1149.1-2013 (BSDL)
				Self::UserCodeRegister => "usercode_register", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Variable => "variable",
				Self::View => "view",        // Added: IEEE 1076-2019
				Self::Vmode => "vmode",      // Added: IEEE 1076-2008
				Self::Vpkg => "vpkg",        // Added: IEEE 1076-2019
				Self::Vprop => "vprop",      // Added: IEEE 1076-2008
				Self::VrefIn => "vref_in",   // Added: IEEE 1149.1-2013 (BSDL)
				Self::VrefOut => "vref_out", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Vunit => "vunit",      // Added: IEEE 1076-2008
				Self::Wait => "wait",
				Self::WaitDuration => "wait_duration", // Added: IEEE 1149.1-2013 (BSDL)
				Self::Weak0 => "weak0",                // Added: IEEE 1149.1-2013 (BSDL)
				Self::Weak1 => "weak1",                // Added: IEEE 1149.1-2013 (BSDL)
				Self::When => "when",
				Self::While => "while",
				Self::With => "with",
				Self::X => "x",       // Added: IEEE 1149.1-2013 (BSDL)
				Self::Xnor => "xnor", // Added: IEEE 1076-1993
				Self::Xor => "xor",
				Self::Z => "z",       // Added: IEEE 1149.1-2013 (BSDL)
				Self::Zero => "zero", // Added: IEEE 1149.1-2013 (BSDL)
			}
		)
	}
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Operator()")
	}
}

impl Default for Token {
	fn default() -> Self {
		Self::Invalid(None)
	}
}
