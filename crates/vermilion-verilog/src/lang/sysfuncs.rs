// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use phf::{phf_map, phf_set};

use crate::LanguageStd;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BuiltinSysFunc {
	Acos,               // Added: IEEE 1364-2005
	Acosh,              // Added: IEEE 1364-2005
	AnalogNodeAlias,    // Added: Verilog-AMS 2.4.0
	AnalogPortAlias,    // Added: Verilog-AMS 2.4.0
	Asin,               // Added: IEEE 1364-2005
	Asinh,              // Added: IEEE 1364-2005
	AssertControl,      // Added: IEEE 1800-2012
	AssertFailOff,      // Added: IEEE 1800-2009
	AssertFailOn,       // Added: IEEE 1800-2009
	AssertKill,         // Added: IEEE 1800-2005
	AssertNonVacuousOn, // Added: IEEE 1800-2009
	AssertOff,          // Added: IEEE 1800-2005
	AssertOn,           // Added: IEEE 1800-2005
	AssertPassOff,      // Added: IEEE 1800-2009
	AssertPassOn,       // Added: IEEE 1800-2009
	AssertVacuousOff,   // Added: IEEE 1800-2009
	AsyncAndArray,
	AsyncAndPlane,
	AsyncNandArray,
	AsyncNandPlane,
	AsyncNorArray,
	AsyncNorPlane,
	AsyncOrArray,
	AsyncOrPlane,
	Atan,  // Added: IEEE 1364-2005
	Atan2, // Added: IEEE 1364-2005
	Atanh, // Added: IEEE 1364-2005
	Bits,  // Added: IEEE 1800-2005
	BitsToReal,
	BitsToShortReal, // Added: IEEE 1800-2005
	Cast,            // Added: IEEE 1800-2005
	Ceil,            // Added: IEEE 1364-2005
	Changed,         // Added: IEEE 1800-2009
	ChangedGclk,     // Added: IEEE 1800-2009
	ChangingGclk,    // Added: IEEE 1800-2009
	Clog2,           // Added: IEEE 1364-2005
	Cos,             // Added: IEEE 1364-2005
	Cosh,            // Added: IEEE 1364-2005
	CountBits,       // Added: IEEE 1800-2012
	CountDrivers,
	CountOnes,       // Added: IEEE 1800-2005
	CoverageControl, // Added: IEEE 1800-2005
	CoverageGet,     // Added: IEEE 1800-2005
	CoverageGetMax,  // Added: IEEE 1800-2005
	CoverageMerge,   // Added: IEEE 1800-2005
	CoverageSave,    // Added: IEEE 1800-2005
	Dimensions,      // Added: IEEE 1800-2005
	Display,
	DisplayB,
	DisplayH,
	DisplayO,
	DistChiSquare,
	DistErlang,
	DistExponential,
	DistNormal,
	DistPoisson,
	DistT,
	DistUniform,
	DumpAll,
	DumpFile,
	DumpFlush,
	DumpLimit,
	DumpOff,
	DumpOn,
	DumpPorts,      // Added: IEEE 1364-2001
	DumpPortsAll,   // Added: IEEE 1364-2001
	DumpPortsFlush, // Added: IEEE 1364-2001
	DumpPortsLimit, // Added: IEEE 1364-2001
	DumpPortsOff,   // Added: IEEE 1364-2001
	DumpPortsOn,    // Added: IEEE 1364-2001
	DumpVars,
	Error,       // Added: IEEE 1800-2005
	Exit,        // Added: IEEE 1800-2005
	Exp,         // Added: IEEE 1364-2005
	FallingGclk, // Added: IEEE 1800-2009
	Fatal,       // Added: IEEE 1800-2005
	Fclose,
	Fdisplay,
	FdisplayB,
	FdisplayH,
	FdisplayO,
	Fell,     // Added: IEEE 1800-2005
	FellGclk, // Added: IEEE 1800-2009
	Feof,     // Added: IEEE 1364-2005
	Ferror,   // Added: IEEE 1364-2001
	Fflush,   // Added: IEEE 1364-2001
	Fgetc,    // Added: IEEE 1364-2001
	Fgets,    // Added: IEEE 1364-2001
	Finish,
	Floor, // Added: IEEE 1364-2005
	Fmonitor,
	FmonitorB,
	FmonitorH,
	FmonitorO,
	Fopen,
	Fread,  // Added: IEEE 1364-2001
	Fscanf, // Added: IEEE 1364-2001
	Fseek,  // Added: IEEE 1364-2001
	Fstrobe,
	FstrobeB,
	FstrobeH,
	FstrobeO,
	Ftell,      // Added: IEEE 1364-2001
	FutureGclk, // Added: IEEE 1800-2009
	Fwrite,
	FwriteB,
	FwriteH,
	FwriteO,
	GetCoverage, // Added: IEEE 1800-2005
	GetPattern,
	High,      // Added: IEEE 1800-2005
	Hold,      // Removed: IEEE 1364-2001
	Hypot,     // Added: IEEE 1364-2005
	Increment, // Added: IEEE 1800-2005
	IncSave,
	Info, // Added: IEEE 1800-2005
	Input,
	IsUnbounded, // Added: IEEE 1800-2005
	IsUnknown,   // Added: IEEE 1800-2005
	Itor,
	Key,
	Left, // Added: IEEE 1800-2005
	List,
	Ln,             // Added: IEEE 1364-2005
	LoadCoverageDB, // Added: IEEE 1800-2005
	Log,
	Log10, // Added: IEEE 1364-2005
	Low,   // Added: IEEE 1800-2005
	Monitor,
	MonitorB,
	MonitorH,
	MonitorO,
	MonitorOff,
	MonitorOn,
	NoChange, // Removed: IEEE 1364-2001
	NoKey,
	NoLog,
	OneHot,   // Added: IEEE 1800-2005
	OneHot0,  // Added: IEEE 1800-2005
	Past,     // Added: IEEE 1800-2005
	PastGclk, // Added: IEEE 1800-2009
	Period,   // Removed: IEEE 1364-2001
	Pow,      // Added: IEEE 1800-2009
	PrintTimescale,
	QAdd,
	QExam,
	QFull,
	QInitialize,
	QRemove,
	Random,
	ReadmemB,
	ReadmemH,
	RealTime,
	RealToBits,
	ReceiverCount, // Added: Verilog-AMS 23
	Recovery,      // Removed: IEEE 1364-2001
	Reset,
	ResetCount,
	ResetValue,
	Restart,
	Rewind,     // Added: IEEE 1364-2001
	Right,      // Added: IEEE 1800-2005
	RisingGclk, // Added: IEEE 1800-2009
	Rose,       // Added: IEEE 1800-2005
	RoseGclk,   // Added: IEEE 1800-2009
	Rtoi,
	Sampled, // Added: IEEE 1800-2005
	Save,
	Scale,
	Scope,
	SdfAnnotate,       // Added: IEEE 1364-2001
	SetCoverageDb,     // Added: IEEE 1800-2005-IEEE 1800-2009
	SetCoverageDbName, // Added: IEEE 1800-2009
	Setup,             // Removed: IEEE 1364-2001
	Setuphold,         // Removed: IEEE 1364-2001
	Sformat,           // Added: IEEE 1364-2001
	Sformatf,          // Added: IEEE 1800-2009
	ShortRealToBits,   // Added: IEEE 1800-2005
	Showscopes,
	Showvars,
	Signed,      // Added: IEEE 1364-2001
	SimParam,    // Added: Verilog-AMS 2.3.1
	SimParamStr, // Added: Verilog-AMS 2.3.1
	SimProbe,    // Added: Verilog-AMS 2.3.1
	Sin,         // Added: IEEE 1364-2005
	Sinh,        // Added: IEEE 1800-2009
	Size,        // Added: IEEE 1800-2005
	Skew,        // Removed: IEEE 1364-2001
	Sqrt,        // Added: IEEE 1364-2005
	SreadmemB,
	SreadmemH,
	Sscanf,     // Added: IEEE 1364-2001
	Stable,     // Added: IEEE 1800-2005
	StableGclk, // Added: IEEE 1800-2009
	Stacktrace, // Added: IEEE 1800-2023
	SteadyGclk, // Added: IEEE 1800-2009
	Stime,
	Stop,
	Strobe,
	StrobeB,
	StrobeH,
	StrobeO,
	Swrite,  // Added: IEEE 1364-2001
	SwriteB, // Added: IEEE 1364-2001
	SwriteH, // Added: IEEE 1364-2001
	SwriteO, // Added: IEEE 1364-2001
	SyncAndArray,
	SyncAndPlane,
	SyncNandArray,
	SyncNandPlane,
	SyncNorArray,
	SyncNorPlane,
	SyncOrArray,
	SyncOrPlane,
	System,       // Added: IEEE 1800-2009
	TableModel,   // Added: Verilog-AMS 2.3.1
	Tan,          // Added: IEEE 1364-2005
	Tanh,         // Added: IEEE 1364-2005
	Temperature,  // Added: Verilog-AMS 2.3.1
	TestPlusArgs, // Added: IEEE 1364-2001
	Time,
	TimeFormat,
	TimePrecision,      // Added: IEEE 1800-2023
	TimeUnit,           // Added: IEEE 1800-2023
	TypeName,           // Added: IEEE 1800-2005
	Ungetc,             // Added: IEEE 1364-2001
	UnpackedDimensions, // Added: IEEE 1800-2005
	Unsigned,           // Added: IEEE 1364-2001
	Urandom,            // Added: IEEE 1800-2005
	UrandomRange,       // Added: IEEE 1800-2005
	ValuePlusArgs,      // Added: IEEE 1364-2001
	Vflip,              // Added: Verilog-AMS 2.3.1
	Vt,                 // Added: Verilog-AMS 2.3.1
	Warning,            // Added: IEEE 1800-2005 & Verilog-AMS 2.3.1
	Width,              // Removed: IEEE 1364-2001
	Write,
	WriteB,
	WriteH,
	WritememB, // Added: IEEE 1800-2005
	WritememH, // Added: IEEE 1800-2005
	WriteO,
	XPosition, // Added: Verilog-AMS 2.3.1
	YPosition, // Added: Verilog-AMS 2.3.1
}

/// Timing Tasks, see note on [`is_timing_task`] for details
pub static TIMING_TASKS_SET: phf::Set<&'static str> = phf_set! {
	"fullskew", "nochange", "recovery", "removal", "setuphold", "timeskew",
	"hold",     "period",   "recrem",   "setup",   "skew",      "width",
};

/// Builtin System Functions that are not technically required.
///
/// See [`is_optional_builtin_sysfunc`] for more details.
pub static OPTIONAL_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"countdrivers", "key",   "nolog",       "restart", "showscopes",
	"getpattern",   "list",  "reset",       "save",    "showvars",
	"incsave",      "log",   "reset_count", "scale",   "sreadmemb",
	"input",        "nokey", "reset_value", "scope",   "sreadmemh",
};

/// IEEE 1364-1995 (Verilog 1995) Builtin System Function to [`BuiltinSysFunc`] token map
pub static VERILOG_95_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"finish" => BuiltinSysFunc::Finish,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"hold" => BuiltinSysFunc::Hold,
	"incsave" => BuiltinSysFunc::IncSave,
	"input" => BuiltinSysFunc::Input,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"list" => BuiltinSysFunc::List,
	"log" => BuiltinSysFunc::Log,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nochange" => BuiltinSysFunc::NoChange,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"period" => BuiltinSysFunc::Period,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"recovery" => BuiltinSysFunc::Recovery,
	"reset" => BuiltinSysFunc::Reset,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"restart" => BuiltinSysFunc::Restart,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"setup" => BuiltinSysFunc::Setup,
	"setuphold" => BuiltinSysFunc::Setuphold,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"skew" => BuiltinSysFunc::Skew,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"width" => BuiltinSysFunc::Width,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writeo" => BuiltinSysFunc::WriteO,
};

/// IEEE 1364-1995 (Verilog 1995) Builtin System Function set
pub static VERILOG_95_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"async$and$array",  "dumpflush", "fwriteo",        "q_initialize", "stime",
	"async$and$plane",  "dumplimit", "getpattern",     "q_remove",     "stop",
	"async$nand$array", "dumpoff",   "hold",           "random",       "strobe",
	"async$nand$plane", "dumpon",    "incsave",        "readmemb",     "strobeb",
	"async$nor$array",  "dumpvars",  "input",          "readmemh",     "strobeh",
	"async$nor$plane",  "fclose",    "itor",           "realtime",     "strobeo",
	"async$or$array",   "fdisplay",  "key",            "realtobits",   "sync$and$array",
	"async$or$plane",   "fdisplayb", "list",           "recovery",     "sync$and$plane",
	"bitstoreal",       "fdisplayh", "log",            "reset",        "sync$nand$array",
	"countdrivers",     "fdisplayo", "monitor",        "reset_count",  "sync$nand$plane",
	"display",          "finish",    "monitorb",       "reset_value",  "sync$nor$array",
	"displayb",         "fmonitor",  "monitorh",       "restart",      "sync$nor$plane",
	"displayh",         "fmonitorb", "monitoro",       "rtoi",         "sync$or$array",
	"displayo",         "fmonitorh", "monitoroff",     "save",         "sync$or$plane",
	"dist_chi_square",  "fmonitoro", "monitoron",      "scale",        "time",
	"dist_erlang",      "fopen",     "nochange",       "scope",        "timeformat",
	"dist_exponential", "fstrobe",   "nokey",          "setup",        "width",
	"dist_normal",      "fstrobeb",  "nolog",          "setuphold",    "write",
	"dist_poisson",     "fstrobeh",  "period",         "showscopes",   "writeb",
	"dist_t",           "fstrobeo",  "printtimescale", "showvars",     "writeh",
	"dist_uniform",     "fwrite",    "q_add",          "skew",         "writeo",
	"dumpall",          "fwriteb",   "q_exame",        "sreadmemb",
	"dumpfile",         "fwriteh",   "q_full",         "sreadmemh",
};

/// IEEE 1364-2001 (Verilog 2001) Builtin System Function to [`BuiltinSysFunc`] token map
pub static VERILOG_01_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"incsave" => BuiltinSysFunc::IncSave,
	"input" => BuiltinSysFunc::Input,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"list" => BuiltinSysFunc::List,
	"log" => BuiltinSysFunc::Log,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset" => BuiltinSysFunc::Reset,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"sformat" => BuiltinSysFunc::Sformat,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writeo	" => BuiltinSysFunc::WriteO,
};

/// IEEE 1364-2001 (Verilog 2001) Bultin System Function set
pub static VERILOG_01_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"async$and$array",  "dumpports",      "fstrobeh",       "q_remove",     "strobeh",
	"async$and$plane",  "dumpportsall",   "fstrobeo",       "random",       "strobeo",
	"async$nand$array", "dumpportsflush", "ftell",          "readmemb",     "swrite",
	"async$nand$plane", "dumpportslimit", "fwrite",         "readmemh",     "swriteb",
	"async$nor$array",  "dumpportsoff",   "fwriteb",        "realtime",     "swriteh",
	"async$nor$plane",  "dumpportson",    "fwriteh",        "realtobits",   "swriteo",
	"async$or$array",   "dumpvars",       "fwriteo",        "reset",        "sync$and$array",
	"async$or$plane",   "fclose",         "getpattern",     "reset_count",  "sync$and$plane",
	"bitstoreal",       "fdisplay",       "incsave",        "rest_value",   "sync$nand$array",
	"countdrivers",     "fdisplayb",      "input",          "restart",      "sync$nand$plane",
	"display",          "fdisplayh",      "itor",           "rewind",       "sync$nor$array",
	"displayb",         "fdisplayo",      "key",            "rtoi",         "sync$nor$plane",
	"displayh",         "ferror",         "list",           "save",         "sync$or$array",
	"displayo",         "fflush",         "log",            "scale",        "sync$or$plane",
	"dist_chi_square",  "fgetc",          "monitor",        "scope",        "test$plusargs",
	"dist_erlang",      "fgets",          "monitorb",       "sdf_annotate", "time",
	"dist_exponential", "finish",         "monitorh",       "sformat",      "timeformat",
	"dist_normal",      "fmonitor",       "monitoro",       "showscopes",   "ungetc",
	"dist_poisson",     "fmonitorb",      "monitoroff",     "showvars",     "unsigned",
	"dist_t",           "fmonitorh",      "monitoron",      "signed",       "value$plusargs",
	"dist_uniform",     "fmonitoro",      "nokey",          "sreadmemb",    "write",
	"dumpall",          "fopen",          "nolog",          "sreadmemh",    "writeb",
	"dumpfile",         "fread",          "printtimescale", "sscanf",       "writeh",
	"dumpflush",        "fscanf",         "q_add",          "stime",        "writeo",
	"dumplimit",        "fseek",          "q_exam",         "stop",
	"dumpoff",          "fstrobe",        "q_full",         "strobe",
	"dumpon",           "fstrobeb",       "q_initialize",   "strobeb",
};

/// IEEE 1364-2005 (Verilog 2005) Builtin System Function to [`BuiltinSysFunc`] token map
pub static VERILOG_05_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"ceil" => BuiltinSysFunc::Ceil,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"exp" => BuiltinSysFunc::Exp,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"hypot" => BuiltinSysFunc::Hypot,
	"incsave" => BuiltinSysFunc::IncSave,
	"input" => BuiltinSysFunc::Input,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"sformat" => BuiltinSysFunc::Sformat,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"sin" => BuiltinSysFunc::Sin,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writeo	" => BuiltinSysFunc::WriteO,
};

/// IEEE 1364-2005 (Verilog 2005) Bultin System Function set
pub static VERILOG_05_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"acos",             "dist_uniform",   "fopen",      "printtimescale", "stop",
	"acosh",            "dumpall",        "fread",      "q_add",          "strobe",
	"asin",             "dumpfile",       "fscanf",     "q_exam",         "strobeb",
	"asinh",            "dumpflush",      "fseek",      "q_full",         "strobeh",
	"async$and$array",  "dumplimit",      "fstrobe",    "q_initialize",   "strobeo",
	"async$and$plane",  "dumpoff",        "fstrobeb",   "q_remove",       "swrite",
	"async$nand$array", "dumpon",         "fstrobeh",   "random",         "swriteb",
	"async$nand$plane", "dumpports",      "fstrobeo",   "readmemb",       "swriteh",
	"async$nor$array",  "dumpportsall",   "ftell",      "readmemh",       "swriteo",
	"async$nor$plane",  "dumpportsflush", "fwrite",     "realtime",       "sync$and$array",
	"async$or$array",   "dumpportslimit", "fwriteb",    "realtobits",     "sync$and$plane",
	"async$or$plane",   "dumpportsoff",   "fwriteh",    "reset",          "sync$nand$array",
	"atan",             "dumpportson",    "fwriteo",    "reset_count",    "sync$nand$plane",
	"atan2",            "dumpvars",       "getpattern", "reset_value",    "sync$nor$array",
	"atanh",            "exp",            "hypot",      "restart",        "sync$nor$plane",
	"bitstoreal",       "fclose",         "incsave",    "rewind",         "sync$or$array",
	"ceil",             "fdisplay",       "input",      "rtoi",           "sync$or$plane",
	"clog2",            "fdisplayb",      "itor",       "save",           "tan",
	"cos",              "fdisplayh",      "key",        "scale",          "tanh",
	"cosh",             "fdisplayo",      "list",       "scope",          "test$plusargs",
	"countdrivers",     "feof",           "ln",         "sdf_annotate",   "time",
	"display",          "ferror",         "log",        "sformat",        "timeformat",
	"displayb",         "fflush",         "log10",      "showscopes",     "ungetc",
	"displayh",         "fgetc",          "monitor",    "showvars",       "unsigned",
	"displayo",         "fgets",          "monitorb",   "signed",         "value$plusargs",
	"dist_chi_square",  "finish",         "monitorh",   "sin",            "write",
	"dist_erlang",      "floor",          "monitoro",   "sqrt",           "writeb",
	"dist_exponential", "fmonitor",       "monitoroff", "sreadmemb",      "writeh",
	"dist_normal",      "fmonitorb",      "monitoron",  "sreadmemh",      "writeo",
	"dist_poisson",     "fmonitorh",      "nokey",      "sscanf",
	"dist_t",           "fmonitoro",      "nolog",      "stime",
};

/// IEEE 1800-2005 (SystemVerilog 2005) Builtin System Function to [`BuiltinSysFunc`] token map
pub static SYSTEM_VERILOG_05_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"assertkill" => BuiltinSysFunc::AssertKill,
	"assertoff" => BuiltinSysFunc::AssertOff,
	"asserton" => BuiltinSysFunc::AssertOn,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bits" => BuiltinSysFunc::Bits,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"bitstoshortreal" => BuiltinSysFunc::BitsToShortReal,
	"cast" => BuiltinSysFunc::Cast,
	"ceil" => BuiltinSysFunc::Ceil,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"countones" => BuiltinSysFunc::CountOnes,
	"coverage_control" => BuiltinSysFunc::CoverageControl,
	"coverage_get_max" => BuiltinSysFunc::CoverageGetMax,
	"coverage_get" => BuiltinSysFunc::CoverageGet,
	"coverage_merge" => BuiltinSysFunc::CoverageMerge,
	"coverage_save" => BuiltinSysFunc::CoverageSave,
	"dimensions" => BuiltinSysFunc::Dimensions,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"error" => BuiltinSysFunc::Error,
	"exit" => BuiltinSysFunc::Exit,
	"exp" => BuiltinSysFunc::Exp,
	"fatal" => BuiltinSysFunc::Fatal,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"fell" => BuiltinSysFunc::Fell,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"get_coverage" => BuiltinSysFunc::GetCoverage,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"high" => BuiltinSysFunc::High,
	"hypot" => BuiltinSysFunc::Hypot,
	"increment" => BuiltinSysFunc::Increment,
	"incsave" => BuiltinSysFunc::IncSave,
	"info" => BuiltinSysFunc::Info,
	"input" => BuiltinSysFunc::Input,
	"isunbounded" => BuiltinSysFunc::IsUnbounded,
	"isunknown" => BuiltinSysFunc::IsUnknown,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"left" => BuiltinSysFunc::Left,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"load_coverage_db" => BuiltinSysFunc::LoadCoverageDB,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"low" => BuiltinSysFunc::Low,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"onehot" => BuiltinSysFunc::OneHot,
	"onehot0" => BuiltinSysFunc::OneHot0,
	"past" => BuiltinSysFunc::Past,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"right" => BuiltinSysFunc::Right,
	"rose" => BuiltinSysFunc::Rose,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"sampled" => BuiltinSysFunc::Sampled,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"set_coverage_db" => BuiltinSysFunc::SetCoverageDb,
	"sformat" => BuiltinSysFunc::Sformat,
	"shortrealtobits" => BuiltinSysFunc::ShortRealToBits,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"sin" => BuiltinSysFunc::Sin,
	"size" => BuiltinSysFunc::Size,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stable" => BuiltinSysFunc::Stable,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"typename" => BuiltinSysFunc::TypeName,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unpacked_dimensions" => BuiltinSysFunc::UnpackedDimensions,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"urandom" => BuiltinSysFunc::Urandom,
	"urandom_range" => BuiltinSysFunc::UrandomRange,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writememb" => BuiltinSysFunc::WritememB,
	"writememh" => BuiltinSysFunc::WritememH,
	"writeo	" => BuiltinSysFunc::WriteO,
};

/// IEEE 1800-2005 (SystemVerilog 2005) Builtin System Function set
pub static SYSTEM_VERILOG_05_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"acos",             "dist_erlang",      "fmonitoro",        "nokey",           "sreadmemh",       "writeo",
	"acosh",            "dist_exponential", "fopen",            "nolog",           "sscanf",
	"asin",             "dist_normal",      "fread",            "onehot",          "stable",
	"asinh",            "dist_poisson",     "fscanf",           "onehot0",         "stime",
	"assertkill",       "dist_t",           "fseek",            "past",            "stop",
	"assertoff",        "dist_uniform",     "fstrobe",          "printtimescale",  "strobe",
	"asserton",         "dumpall",          "fstrobeb",         "q_add",           "strobeb",
	"async$and$array",  "dumpfile",         "fstrobeh",         "q_exam",          "strobeh",
	"async$and$plane",  "dumpflush",        "fstrobeo",         "q_full",          "strobeo",
	"async$nand$array", "dumplimit",        "ftell",            "q_initialize",    "swrite",
	"async$nand$plane", "dumpoff",          "fwrite",           "q_remove",        "swriteb",
	"async$nor$array",  "dumpon",           "fwriteb",          "random",          "swriteh",
	"async$nor$plane",  "dumpports",        "fwriteh",          "readmemb",        "swriteo",
	"async$or$array",   "dumpportsall",     "fwriteo",          "readmemh",        "sync$and$array",
	"async$or$plane",   "dumpportsflush",   "get_coverage",     "realtime",        "sync$and$plane",
	"atan",             "dumpportslimit",   "getpattern",       "realtobits",      "sync$nand$array",
	"atan2",            "dumpportsoff",     "high",             "reset",           "sync$nand$plane",
	"atanh",            "dumpportson",      "hypot",            "reset_count",     "sync$nor$array",
	"bits",             "dumpvars",         "increment",        "reset_value",     "sync$nor$plane",
	"bitstoreal",       "error",            "incsave",          "restart",         "sync$or$array",
	"bitstoshortreal",  "exit",             "info",             "rewind",          "sync$or$plane",
	"cast",             "exp",              "input",            "right",           "tan",
	"ceil",             "fatal",            "isunbounded",      "rose",            "tanh",
	"clog2",            "fclose",           "isunknown",        "rtoi",            "test$plusargs",
	"cos",              "fdisplay",         "itor",             "sampled",         "time",
	"cosh",             "fdisplayb",        "key",              "save",            "timeformat",
	"countdrivers",     "fdisplayh",        "left",             "scale",           "typename",
	"countones",        "fdisplayo",        "list",             "scope",           "ungetc",
	"coverage_control", "fell",             "ln",               "sdf_annotate",    "unpacked_dimensions",
	"coverage_get",     "feof",             "load_coverage_db", "set_coverage_db", "unsigned",
	"coverage_get_max", "ferror",           "log",              "sformat",         "urandom",
	"coverage_merge",   "fflush",           "log10",            "shortrealtobits", "urandom_range",
	"coverage_save",    "fgetc",            "low",              "showscopes",      "value$plusargs",
	"dimensions",       "fgets",            "monitor",          "showvars",        "warning",
	"display",          "finish",           "monitorb",         "signed",          "write",
	"displayb",         "floor",            "monitorh",         "sin",             "writeb",
	"displayh",         "fmonitor",         "monitoro",         "size",            "writeh",
	"displayo",         "fmonitorb",        "monitoroff",       "sqrt",            "writememh",
	"dist_chi_square",  "fmonitorh",        "monitoron",        "sreadmemb",       "writememb",
};

/// IEEE 1800-2009 (SystemVerilog 2009) Builtin System Function to [`BuiltinSysFunc`] token map
pub static SYSTEM_VERILOG_09_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"assertfailoff" => BuiltinSysFunc::AssertFailOff,
	"assertfailon" => BuiltinSysFunc::AssertFailOn,
	"assertkill" => BuiltinSysFunc::AssertKill,
	"assertnonvacuouson" => BuiltinSysFunc::AssertNonVacuousOn,
	"assertoff" => BuiltinSysFunc::AssertOff,
	"asserton" => BuiltinSysFunc::AssertOn,
	"assertpassoff" => BuiltinSysFunc::AssertPassOff,
	"assertpasson" => BuiltinSysFunc::AssertPassOn,
	"assertvacuousoff" => BuiltinSysFunc::AssertVacuousOff,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bits" => BuiltinSysFunc::Bits,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"bitstoshortreal" => BuiltinSysFunc::BitsToShortReal,
	"cast" => BuiltinSysFunc::Cast,
	"ceil" => BuiltinSysFunc::Ceil,
	"changed_gclk" => BuiltinSysFunc::ChangedGclk,
	"changed" => BuiltinSysFunc::Changed,
	"changing_gclk" => BuiltinSysFunc::ChangingGclk,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"countones" => BuiltinSysFunc::CountOnes,
	"coverage_control" => BuiltinSysFunc::CoverageControl,
	"coverage_get_max" => BuiltinSysFunc::CoverageGetMax,
	"coverage_get" => BuiltinSysFunc::CoverageGet,
	"coverage_merge" => BuiltinSysFunc::CoverageMerge,
	"coverage_save" => BuiltinSysFunc::CoverageSave,
	"dimensions" => BuiltinSysFunc::Dimensions,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"error" => BuiltinSysFunc::Error,
	"exit" => BuiltinSysFunc::Exit,
	"exp" => BuiltinSysFunc::Exp,
	"falling_gclk" => BuiltinSysFunc::FallingGclk,
	"fatal" => BuiltinSysFunc::Fatal,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"fell_gclk" => BuiltinSysFunc::FellGclk,
	"fell" => BuiltinSysFunc::Fell,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"future_gclk" => BuiltinSysFunc::FutureGclk,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"get_coverage" => BuiltinSysFunc::GetCoverage,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"high" => BuiltinSysFunc::High,
	"hypot" => BuiltinSysFunc::Hypot,
	"increment" => BuiltinSysFunc::Increment,
	"incsave" => BuiltinSysFunc::IncSave,
	"info" => BuiltinSysFunc::Info,
	"input" => BuiltinSysFunc::Input,
	"isunbounded" => BuiltinSysFunc::IsUnbounded,
	"isunknown" => BuiltinSysFunc::IsUnknown,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"left" => BuiltinSysFunc::Left,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"load_coverage_db" => BuiltinSysFunc::LoadCoverageDB,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"low" => BuiltinSysFunc::Low,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"onehot" => BuiltinSysFunc::OneHot,
	"onehot0" => BuiltinSysFunc::OneHot0,
	"past_gclk" => BuiltinSysFunc::PastGclk,
	"past" => BuiltinSysFunc::Past,
	"pow" => BuiltinSysFunc::Pow,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"right" => BuiltinSysFunc::Right,
	"rising_gclk" => BuiltinSysFunc::RisingGclk,
	"rose_gclk" => BuiltinSysFunc::RoseGclk,
	"rose" => BuiltinSysFunc::Rose,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"sampled" => BuiltinSysFunc::Sampled,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"set_coverage_db_name" => BuiltinSysFunc::SetCoverageDbName,
	"sformat" => BuiltinSysFunc::Sformat,
	"sformatf" => BuiltinSysFunc::Sformatf,
	"shortrealtobits" => BuiltinSysFunc::ShortRealToBits,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"sin" => BuiltinSysFunc::Sin,
	"sinh" => BuiltinSysFunc::Sinh,
	"size" => BuiltinSysFunc::Size,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stable_gclk" => BuiltinSysFunc::StableGclk,
	"stable" => BuiltinSysFunc::Stable,
	"steady_gclk" => BuiltinSysFunc::SteadyGclk,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"system" => BuiltinSysFunc::System,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"typename" => BuiltinSysFunc::TypeName,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unpacked_dimensions" => BuiltinSysFunc::UnpackedDimensions,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"urandom" => BuiltinSysFunc::Urandom,
	"urandom_range" => BuiltinSysFunc::UrandomRange,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writememb" => BuiltinSysFunc::WritememB,
	"writememh" => BuiltinSysFunc::WritememH,
	"writeo	" => BuiltinSysFunc::WriteO,
};

/// IEEE 1800-2009 (SystemVerilog 2009) Builtin System Function set
pub static SYSTEM_VERILOG_09_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"acos",               "dumpall",        "increment",        "set_coverage_db_name", "writeo",
	"acosh",              "dumpfile",       "incsave",          "sformat",
	"asin",               "dumpflush",      "info",             "sformatf",
	"asinh",              "dumplimit",      "input",            "shortrealtobits",
	"assertfailoff",      "dumpoff",        "isunbounded",      "showscopes",
	"assertfailon",       "dumpon",         "isunknown",        "showvars",
	"assertkill",         "dumpports",      "itor",             "signed",
	"assertnonvacuouson", "dumpportsall",   "key",              "sin",
	"assertoff",          "dumpportsflush", "left",             "sinh",
	"asserton",           "dumpportslimit", "list",             "size",
	"assertpassoff",      "dumpportsoff",   "ln",               "sqrt",
	"assertpasson",       "dumpportson",    "load_coverage_db", "sreadmemb",
	"assertvacuousoff",   "dumpvars",       "log",              "sreadmemh",
	"async$and$array",    "error",          "log10",            "sscanf",
	"async$and$plane",    "exit",           "low",              "stable",
	"async$nand$array",   "exp",            "monitor",          "stable_gclk",
	"async$nand$plane",   "falling_gclk",   "monitorb",         "steady_gclk",
	"async$nor$array",    "fatal",          "monitorh",         "stime",
	"async$nor$plane",    "fclose",         "monitoro",         "stop",
	"async$or$array",     "fdisplay",       "monitoroff",       "strobe",
	"async$or$plane",     "fdisplayb",      "monitoron",        "strobeb",
	"atan",               "fdisplayh",      "nokey",            "strobeh",
	"atan2",              "fdisplayo",      "nolog",            "strobeo",
	"atanh",              "fell",           "onehot",           "swrite",
	"bits",               "fell_gclk",      "onehot0",          "swriteb",
	"bitstoreal",         "feof",           "past",             "swriteh",
	"bitstoshortreal",    "ferror",         "past_gclk",        "swriteo",
	"cast",               "fflush",         "pow",              "sync$and$array",
	"ceil",               "fgetc",          "printtimescale",   "sync$and$plane",
	"changed",            "fgets",          "q_add",            "sync$nand$array",
	"changed_gclk",       "finish",         "q_exam",           "sync$nand$plane",
	"changing_gclk",      "floor",          "q_full",           "sync$nor$array",
	"clog2",              "fmonitor",       "q_initialize",     "sync$nor$plane",
	"cos",                "fmonitorb",      "q_remove",         "sync$or$array",
	"cosh",               "fmonitorh",      "random",           "sync$or$plane",
	"countdrivers",       "fmonitoro",      "readmemb",         "system",
	"countones",          "fopen",          "readmemh",         "tan",
	"coverage_control",   "fread",          "realtime",         "tanh",
	"coverage_get",       "fscanf",         "realtobits",       "test$plusargs",
	"coverage_get_max",   "fseek",          "reset",            "time",
	"coverage_merge",     "fstrobe",        "reset_count",      "timeformat",
	"coverage_save",      "fstrobeb",       "reset_value",      "typename",
	"dimensions",         "fstrobeh",       "restart",          "ungetc",
	"display",            "fstrobeo",       "rewind",           "unpacked_dimensions",
	"displayb",           "ftell",          "right",            "unsigned",
	"displayh",           "future_gclk",    "rising_gclk",      "urandom",
	"displayo",           "fwrite",         "rose",             "urandom_range",
	"dist_chi_square",    "fwriteb",        "rose_gclk",        "value$plusargs",
	"dist_erlang",        "fwriteh",        "rtoi",             "warning",
	"dist_exponential",   "fwriteo",        "sampled",          "write",
	"dist_normal",        "get_coverage",   "save",             "writeb",
	"dist_poisson",       "getpattern",     "scale",            "writeh",
	"dist_t",             "high",           "scope",            "writememb",
	"dist_uniform",       "hypot",          "sdf_annotate",     "writememh",
};

/// IEEE 1800-2012 (SystemVerilog 2012) Builtin System Function to [`BuiltinSysFunc`] token map
pub static SYSTEM_VERILOG_12_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"assertcontrol" => BuiltinSysFunc::AssertControl,
	"assertfailoff" => BuiltinSysFunc::AssertFailOff,
	"assertfailon" => BuiltinSysFunc::AssertFailOn,
	"assertkill" => BuiltinSysFunc::AssertKill,
	"assertnonvacuouson" => BuiltinSysFunc::AssertNonVacuousOn,
	"assertoff" => BuiltinSysFunc::AssertOff,
	"asserton" => BuiltinSysFunc::AssertOn,
	"assertpassoff" => BuiltinSysFunc::AssertPassOff,
	"assertpasson" => BuiltinSysFunc::AssertPassOn,
	"assertvacuousoff" => BuiltinSysFunc::AssertVacuousOff,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bits" => BuiltinSysFunc::Bits,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"bitstoshortreal" => BuiltinSysFunc::BitsToShortReal,
	"cast" => BuiltinSysFunc::Cast,
	"ceil" => BuiltinSysFunc::Ceil,
	"changed_gclk" => BuiltinSysFunc::ChangedGclk,
	"changed" => BuiltinSysFunc::Changed,
	"changing_gclk" => BuiltinSysFunc::ChangingGclk,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countbits" => BuiltinSysFunc::CountBits,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"countones" => BuiltinSysFunc::CountOnes,
	"coverage_control" => BuiltinSysFunc::CoverageControl,
	"coverage_get_max" => BuiltinSysFunc::CoverageGetMax,
	"coverage_get" => BuiltinSysFunc::CoverageGet,
	"coverage_merge" => BuiltinSysFunc::CoverageMerge,
	"coverage_save" => BuiltinSysFunc::CoverageSave,
	"dimensions" => BuiltinSysFunc::Dimensions,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"error" => BuiltinSysFunc::Error,
	"exit" => BuiltinSysFunc::Exit,
	"exp" => BuiltinSysFunc::Exp,
	"falling_gclk" => BuiltinSysFunc::FallingGclk,
	"fatal" => BuiltinSysFunc::Fatal,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"fell_gclk" => BuiltinSysFunc::FellGclk,
	"fell" => BuiltinSysFunc::Fell,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"future_gclk" => BuiltinSysFunc::FutureGclk,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"get_coverage" => BuiltinSysFunc::GetCoverage,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"high" => BuiltinSysFunc::High,
	"hypot" => BuiltinSysFunc::Hypot,
	"increment" => BuiltinSysFunc::Increment,
	"incsave" => BuiltinSysFunc::IncSave,
	"info" => BuiltinSysFunc::Info,
	"input" => BuiltinSysFunc::Input,
	"isunbounded" => BuiltinSysFunc::IsUnbounded,
	"isunknown" => BuiltinSysFunc::IsUnknown,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"left" => BuiltinSysFunc::Left,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"load_coverage_db" => BuiltinSysFunc::LoadCoverageDB,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"low" => BuiltinSysFunc::Low,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"onehot" => BuiltinSysFunc::OneHot,
	"onehot0" => BuiltinSysFunc::OneHot0,
	"past_gclk" => BuiltinSysFunc::PastGclk,
	"past" => BuiltinSysFunc::Past,
	"pow" => BuiltinSysFunc::Pow,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"right" => BuiltinSysFunc::Right,
	"rising_gclk" => BuiltinSysFunc::RisingGclk,
	"rose_gclk" => BuiltinSysFunc::RoseGclk,
	"rose" => BuiltinSysFunc::Rose,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"sampled" => BuiltinSysFunc::Sampled,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"set_coverage_db_name" => BuiltinSysFunc::SetCoverageDbName,
	"sformat" => BuiltinSysFunc::Sformat,
	"sformatf" => BuiltinSysFunc::Sformatf,
	"shortrealtobits" => BuiltinSysFunc::ShortRealToBits,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"sin" => BuiltinSysFunc::Sin,
	"sinh" => BuiltinSysFunc::Sinh,
	"size" => BuiltinSysFunc::Size,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stable_gclk" => BuiltinSysFunc::StableGclk,
	"stable" => BuiltinSysFunc::Stable,
	"steady_gclk" => BuiltinSysFunc::SteadyGclk,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"system" => BuiltinSysFunc::System,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"typename" => BuiltinSysFunc::TypeName,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unpacked_dimensions" => BuiltinSysFunc::UnpackedDimensions,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"urandom" => BuiltinSysFunc::Urandom,
	"urandom_range" => BuiltinSysFunc::UrandomRange,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writememb" => BuiltinSysFunc::WritememB,
	"writememh" => BuiltinSysFunc::WritememH,
	"writeo	" => BuiltinSysFunc::WriteO,
};

/// IEEE 1800-2012 (SystemVerilog 2012) Builtin System Function set
pub static SYSTEM_VERILOG_12_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"acos",               "dist_uniform",   "increment",            "sformat",
	"acosh",              "dumpall",        "incsave",              "sformatf",
	"asin",               "dumpfile",       "info",                 "shortrealtobits",
	"asinh",              "dumpflush",      "input",                "showscopes",
	"assertcontrol",      "dumplimit",      "isunbounded",          "showvars",
	"assertfailoff",      "dumpoff",        "isunknown",            "signed",
	"assertfailon",       "dumpon",         "itor",                 "sin",
	"assertkill",         "dumpports",      "key",                  "sinh",
	"assertnonvacuouson", "dumpportsall",   "left",                 "size",
	"assertoff",          "dumpportsflush", "list",                 "sqrt",
	"asserton",           "dumpportslimit", "ln",                   "sreadmemb",
	"assertpassoff",      "dumpportsoff",   "load_coverage_db",     "sreadmemh",
	"assertpasson",       "dumpportson",    "log",                  "sscanf",
	"assertvacuousoff",   "dumpvars",       "log10",                "stable",
	"async$and$array",    "error",          "low",                  "stable_gclk",
	"async$and$plane",    "exit",           "monitor",              "steady_gclk",
	"async$nand$array",   "exp",            "monitorb",             "stime",
	"async$nand$plane",   "falling_gclk",   "monitorh",             "stop",
	"async$nor$array",    "fatal",          "monitoro",             "strobe",
	"async$nor$plane",    "fclose",         "monitoroff",           "strobeb",
	"async$or$array",     "fdisplay",       "monitoron",            "strobeh",
	"async$or$plane",     "fdisplayb",      "nokey",                "strobeo",
	"atan",               "fdisplayh",      "nolog",                "swrite",
	"atan2",              "fdisplayo",      "onehot",               "swriteb",
	"atanh",              "fell",           "onehot0",              "swriteh",
	"bits",               "fell_gclk",      "past",                 "swriteo",
	"bitstoreal",         "feof",           "past_gclk",            "sync$and$array",
	"bitstoshortreal",    "ferror",         "pow",                  "sync$and$plane",
	"cast",               "fflush",         "printtimescale",       "sync$nand$array",
	"ceil",               "fgetc",          "q_add",                "sync$nand$plane",
	"changed",            "fgets",          "q_exam",               "sync$nor$array",
	"changed_gclk",       "finish",         "q_full",               "sync$nor$plane",
	"changing_gclk",      "floor",          "q_initialize",         "sync$or$array",
	"clog2",              "fmonitor",       "q_remove",             "sync$or$plane",
	"cos",                "fmonitorb",      "random",               "system",
	"cosh",               "fmonitorh",      "readmemb",             "tan",
	"countbits",          "fmonitoro",      "readmemh",             "tanh",
	"countdrivers",       "fopen",          "realtime",             "test$plusargs",
	"countones",          "fread",          "realtobits",           "time",
	"coverage_control",   "fscanf",         "reset",                "timeformat",
	"coverage_get",       "fseek",          "reset_count",          "typename",
	"coverage_get_max",   "fstrobe",        "reset_value",          "ungetc",
	"coverage_merge",     "fstrobeb",       "restart",              "unpacked_dimensions",
	"coverage_save",      "fstrobeh",       "rewind",               "unsigned",
	"dimensions",         "fstrobeo",       "right",                "urandom",
	"display",            "ftell",          "rising_gclk",          "urandom_range",
	"displayb",           "future_gclk",    "rose",                 "value$plusargs",
	"displayh",           "fwrite",         "rose_gclk",            "warning",
	"displayo",           "fwriteb",        "rtoi",                 "write",
	"dist_chi_square",    "fwriteh",        "sampled",              "writeb",
	"dist_erlang",        "fwriteo",        "save",                 "writeh",
	"dist_exponential",   "get_coverage",   "scale",                "writememb",
	"dist_normal",        "getpattern",     "scope",                "writememh",
	"dist_poisson",       "high",           "sdf_annotate",         "writeo",
	"dist_t",             "hypot",          "set_coverage_db_name",
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_12_BUILTIN_SYSFUNC_MAP`
/// IEEE 1800-2017 (SystemVerilog 2017) Builtin System Function to [`BuiltinSysFunc`] token map
pub static SYSTEM_VERILOG_17_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"assertcontrol" => BuiltinSysFunc::AssertControl,
	"assertfailoff" => BuiltinSysFunc::AssertFailOff,
	"assertfailon" => BuiltinSysFunc::AssertFailOn,
	"assertkill" => BuiltinSysFunc::AssertKill,
	"assertnonvacuouson" => BuiltinSysFunc::AssertNonVacuousOn,
	"assertoff" => BuiltinSysFunc::AssertOff,
	"asserton" => BuiltinSysFunc::AssertOn,
	"assertpassoff" => BuiltinSysFunc::AssertPassOff,
	"assertpasson" => BuiltinSysFunc::AssertPassOn,
	"assertvacuousoff" => BuiltinSysFunc::AssertVacuousOff,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bits" => BuiltinSysFunc::Bits,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"bitstoshortreal" => BuiltinSysFunc::BitsToShortReal,
	"cast" => BuiltinSysFunc::Cast,
	"ceil" => BuiltinSysFunc::Ceil,
	"changed_gclk" => BuiltinSysFunc::ChangedGclk,
	"changed" => BuiltinSysFunc::Changed,
	"changing_gclk" => BuiltinSysFunc::ChangingGclk,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countbits" => BuiltinSysFunc::CountBits,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"countones" => BuiltinSysFunc::CountOnes,
	"coverage_control" => BuiltinSysFunc::CoverageControl,
	"coverage_get_max" => BuiltinSysFunc::CoverageGetMax,
	"coverage_get" => BuiltinSysFunc::CoverageGet,
	"coverage_merge" => BuiltinSysFunc::CoverageMerge,
	"coverage_save" => BuiltinSysFunc::CoverageSave,
	"dimensions" => BuiltinSysFunc::Dimensions,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"error" => BuiltinSysFunc::Error,
	"exit" => BuiltinSysFunc::Exit,
	"exp" => BuiltinSysFunc::Exp,
	"falling_gclk" => BuiltinSysFunc::FallingGclk,
	"fatal" => BuiltinSysFunc::Fatal,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"fell_gclk" => BuiltinSysFunc::FellGclk,
	"fell" => BuiltinSysFunc::Fell,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"future_gclk" => BuiltinSysFunc::FutureGclk,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"get_coverage" => BuiltinSysFunc::GetCoverage,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"high" => BuiltinSysFunc::High,
	"hypot" => BuiltinSysFunc::Hypot,
	"increment" => BuiltinSysFunc::Increment,
	"incsave" => BuiltinSysFunc::IncSave,
	"info" => BuiltinSysFunc::Info,
	"input" => BuiltinSysFunc::Input,
	"isunbounded" => BuiltinSysFunc::IsUnbounded,
	"isunknown" => BuiltinSysFunc::IsUnknown,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"left" => BuiltinSysFunc::Left,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"load_coverage_db" => BuiltinSysFunc::LoadCoverageDB,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"low" => BuiltinSysFunc::Low,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"onehot" => BuiltinSysFunc::OneHot,
	"onehot0" => BuiltinSysFunc::OneHot0,
	"past_gclk" => BuiltinSysFunc::PastGclk,
	"past" => BuiltinSysFunc::Past,
	"pow" => BuiltinSysFunc::Pow,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"right" => BuiltinSysFunc::Right,
	"rising_gclk" => BuiltinSysFunc::RisingGclk,
	"rose_gclk" => BuiltinSysFunc::RoseGclk,
	"rose" => BuiltinSysFunc::Rose,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"sampled" => BuiltinSysFunc::Sampled,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"set_coverage_db_name" => BuiltinSysFunc::SetCoverageDbName,
	"sformat" => BuiltinSysFunc::Sformat,
	"sformatf" => BuiltinSysFunc::Sformatf,
	"shortrealtobits" => BuiltinSysFunc::ShortRealToBits,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"sin" => BuiltinSysFunc::Sin,
	"sinh" => BuiltinSysFunc::Sinh,
	"size" => BuiltinSysFunc::Size,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stable_gclk" => BuiltinSysFunc::StableGclk,
	"stable" => BuiltinSysFunc::Stable,
	"steady_gclk" => BuiltinSysFunc::SteadyGclk,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"system" => BuiltinSysFunc::System,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"typename" => BuiltinSysFunc::TypeName,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unpacked_dimensions" => BuiltinSysFunc::UnpackedDimensions,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"urandom" => BuiltinSysFunc::Urandom,
	"urandom_range" => BuiltinSysFunc::UrandomRange,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writememb" => BuiltinSysFunc::WritememB,
	"writememh" => BuiltinSysFunc::WritememH,
	"writeo	" => BuiltinSysFunc::WriteO,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_12_BUILTIN_SYSFUNC_SET`
/// IEEE 1800-2017 (SystemVerilog 2017) Builtin System Function set
pub static SYSTEM_VERILOG_17_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"acos",               "dist_uniform",   "increment",            "sformat",
	"acosh",              "dumpall",        "incsave",              "sformatf",
	"asin",               "dumpfile",       "info",                 "shortrealtobits",
	"asinh",              "dumpflush",      "input",                "showscopes",
	"assertcontrol",      "dumplimit",      "isunbounded",          "showvars",
	"assertfailoff",      "dumpoff",        "isunknown",            "signed",
	"assertfailon",       "dumpon",         "itor",                 "sin",
	"assertkill",         "dumpports",      "key",                  "sinh",
	"assertnonvacuouson", "dumpportsall",   "left",                 "size",
	"assertoff",          "dumpportsflush", "list",                 "sqrt",
	"asserton",           "dumpportslimit", "ln",                   "sreadmemb",
	"assertpassoff",      "dumpportsoff",   "load_coverage_db",     "sreadmemh",
	"assertpasson",       "dumpportson",    "log",                  "sscanf",
	"assertvacuousoff",   "dumpvars",       "log10",                "stable",
	"async$and$array",    "error",          "low",                  "stable_gclk",
	"async$and$plane",    "exit",           "monitor",              "steady_gclk",
	"async$nand$array",   "exp",            "monitorb",             "stime",
	"async$nand$plane",   "falling_gclk",   "monitorh",             "stop",
	"async$nor$array",    "fatal",          "monitoro",             "strobe",
	"async$nor$plane",    "fclose",         "monitoroff",           "strobeb",
	"async$or$array",     "fdisplay",       "monitoron",            "strobeh",
	"async$or$plane",     "fdisplayb",      "nokey",                "strobeo",
	"atan",               "fdisplayh",      "nolog",                "swrite",
	"atan2",              "fdisplayo",      "onehot",               "swriteb",
	"atanh",              "fell",           "onehot0",              "swriteh",
	"bits",               "fell_gclk",      "past",                 "swriteo",
	"bitstoreal",         "feof",           "past_gclk",            "sync$and$array",
	"bitstoshortreal",    "ferror",         "pow",                  "sync$and$plane",
	"cast",               "fflush",         "printtimescale",       "sync$nand$array",
	"ceil",               "fgetc",          "q_add",                "sync$nand$plane",
	"changed",            "fgets",          "q_exam",               "sync$nor$array",
	"changed_gclk",       "finish",         "q_full",               "sync$nor$plane",
	"changing_gclk",      "floor",          "q_initialize",         "sync$or$array",
	"clog2",              "fmonitor",       "q_remove",             "sync$or$plane",
	"cos",                "fmonitorb",      "random",               "system",
	"cosh",               "fmonitorh",      "readmemb",             "tan",
	"countbits",          "fmonitoro",      "readmemh",             "tanh",
	"countdrivers",       "fopen",          "realtime",             "test$plusargs",
	"countones",          "fread",          "realtobits",           "time",
	"coverage_control",   "fscanf",         "reset",                "timeformat",
	"coverage_get",       "fseek",          "reset_count",          "typename",
	"coverage_get_max",   "fstrobe",        "reset_value",          "ungetc",
	"coverage_merge",     "fstrobeb",       "restart",              "unpacked_dimensions",
	"coverage_save",      "fstrobeh",       "rewind",               "unsigned",
	"dimensions",         "fstrobeo",       "right",                "urandom",
	"display",            "ftell",          "rising_gclk",          "urandom_range",
	"displayb",           "future_gclk",    "rose",                 "value$plusargs",
	"displayh",           "fwrite",         "rose_gclk",            "warning",
	"displayo",           "fwriteb",        "rtoi",                 "write",
	"dist_chi_square",    "fwriteh",        "sampled",              "writeb",
	"dist_erlang",        "fwriteo",        "save",                 "writeh",
	"dist_exponential",   "get_coverage",   "scale",                "writememb",
	"dist_normal",        "getpattern",     "scope",                "writememh",
	"dist_poisson",       "high",           "sdf_annotate",         "writeo",
	"dist_t",             "hypot",          "set_coverage_db_name",
};

/// IEEE 1800-2023 (SystemVerilog 2023) Builtin System Function to [`BuiltinSysFunc`] token map
pub static SYSTEM_VERILOG_23_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"assertcontrol" => BuiltinSysFunc::AssertControl,
	"assertfailoff" => BuiltinSysFunc::AssertFailOff,
	"assertfailon" => BuiltinSysFunc::AssertFailOn,
	"assertkill" => BuiltinSysFunc::AssertKill,
	"assertnonvacuouson" => BuiltinSysFunc::AssertNonVacuousOn,
	"assertoff" => BuiltinSysFunc::AssertOff,
	"asserton" => BuiltinSysFunc::AssertOn,
	"assertpassoff" => BuiltinSysFunc::AssertPassOff,
	"assertpasson" => BuiltinSysFunc::AssertPassOn,
	"assertvacuousoff" => BuiltinSysFunc::AssertVacuousOff,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bits" => BuiltinSysFunc::Bits,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"bitstoshortreal" => BuiltinSysFunc::BitsToShortReal,
	"cast" => BuiltinSysFunc::Cast,
	"ceil" => BuiltinSysFunc::Ceil,
	"changed_gclk" => BuiltinSysFunc::ChangedGclk,
	"changed" => BuiltinSysFunc::Changed,
	"changing_gclk" => BuiltinSysFunc::ChangingGclk,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countbits" => BuiltinSysFunc::CountBits,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"countones" => BuiltinSysFunc::CountOnes,
	"coverage_control" => BuiltinSysFunc::CoverageControl,
	"coverage_get_max" => BuiltinSysFunc::CoverageGetMax,
	"coverage_get" => BuiltinSysFunc::CoverageGet,
	"coverage_merge" => BuiltinSysFunc::CoverageMerge,
	"coverage_save" => BuiltinSysFunc::CoverageSave,
	"dimensions" => BuiltinSysFunc::Dimensions,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"error" => BuiltinSysFunc::Error,
	"exit" => BuiltinSysFunc::Exit,
	"exp" => BuiltinSysFunc::Exp,
	"falling_gclk" => BuiltinSysFunc::FallingGclk,
	"fatal" => BuiltinSysFunc::Fatal,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"fell_gclk" => BuiltinSysFunc::FellGclk,
	"fell" => BuiltinSysFunc::Fell,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"future_gclk" => BuiltinSysFunc::FutureGclk,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"get_coverage" => BuiltinSysFunc::GetCoverage,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"high" => BuiltinSysFunc::High,
	"hypot" => BuiltinSysFunc::Hypot,
	"increment" => BuiltinSysFunc::Increment,
	"incsave" => BuiltinSysFunc::IncSave,
	"info" => BuiltinSysFunc::Info,
	"input" => BuiltinSysFunc::Input,
	"isunbounded" => BuiltinSysFunc::IsUnbounded,
	"isunknown" => BuiltinSysFunc::IsUnknown,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"left" => BuiltinSysFunc::Left,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"load_coverage_db" => BuiltinSysFunc::LoadCoverageDB,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"low" => BuiltinSysFunc::Low,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"onehot" => BuiltinSysFunc::OneHot,
	"onehot0" => BuiltinSysFunc::OneHot0,
	"past_gclk" => BuiltinSysFunc::PastGclk,
	"past" => BuiltinSysFunc::Past,
	"pow" => BuiltinSysFunc::Pow,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"right" => BuiltinSysFunc::Right,
	"rising_gclk" => BuiltinSysFunc::RisingGclk,
	"rose_gclk" => BuiltinSysFunc::RoseGclk,
	"rose" => BuiltinSysFunc::Rose,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"sampled" => BuiltinSysFunc::Sampled,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"set_coverage_db_name" => BuiltinSysFunc::SetCoverageDbName,
	"sformat" => BuiltinSysFunc::Sformat,
	"sformatf" => BuiltinSysFunc::Sformatf,
	"shortrealtobits" => BuiltinSysFunc::ShortRealToBits,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"sin" => BuiltinSysFunc::Sin,
	"sinh" => BuiltinSysFunc::Sinh,
	"size" => BuiltinSysFunc::Size,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stable_gclk" => BuiltinSysFunc::StableGclk,
	"stable" => BuiltinSysFunc::Stable,
	"stacktrace" => BuiltinSysFunc::Stacktrace,
	"steady_gclk" => BuiltinSysFunc::SteadyGclk,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"system" => BuiltinSysFunc::System,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"timeprecision" => BuiltinSysFunc::TimePrecision,
	"timeunit" => BuiltinSysFunc::TimeUnit,
	"typename" => BuiltinSysFunc::TypeName,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unpacked_dimensions" => BuiltinSysFunc::UnpackedDimensions,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"urandom" => BuiltinSysFunc::Urandom,
	"urandom_range" => BuiltinSysFunc::UrandomRange,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writememb" => BuiltinSysFunc::WritememB,
	"writememh" => BuiltinSysFunc::WritememH,
	"writeo	" => BuiltinSysFunc::WriteO,
};

/// IEEE 1800-2023 (SystemVerilog 2023) Builtin System Function set
pub static SYSTEM_VERILOG_23_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"acos",               "dumpall",        "info",                 "shortrealtobits",
	"acosh",              "dumpfile",       "input",                "showscopes",
	"asin",               "dumpflush",      "isunbounded",          "showvars",
	"asinh",              "dumplimit",      "isunknown",            "signed",
	"assertcontrol",      "dumpoff",        "itor",                 "sin",
	"assertfailoff",      "dumpon",         "key",                  "sinh",
	"assertfailon",       "dumpports",      "left",                 "size",
	"assertkill",         "dumpportsall",   "list",                 "sqrt",
	"assertnonvacuouson", "dumpportsflush", "ln",                   "sreadmemb",
	"assertoff",          "dumpportslimit", "load_coverage_db",     "sreadmemh",
	"asserton",           "dumpportsoff",   "log",                  "sscanf",
	"assertpassoff",      "dumpportson",    "log10",                "stable",
	"assertpasson",       "dumpvars",       "low",                  "stable_gclk",
	"assertvacuousoff",   "error",          "monitor",              "stacktrace",
	"async$and$array",    "exit",           "monitorb",             "steady_gclk",
	"async$and$plane",    "exp",            "monitorh",             "stime",
	"async$nand$array",   "falling_gclk",   "monitoro",             "stop",
	"async$nand$plane",   "fatal",          "monitoroff",           "strobe",
	"async$nor$array",    "fclose",         "monitoron",            "strobeb",
	"async$nor$plane",    "fdisplay",       "nokey",                "strobeh",
	"async$or$array",     "fdisplayb",      "nolog",                "strobeo",
	"async$or$plane",     "fdisplayh",      "onehot",               "swrite",
	"atan",               "fdisplayo",      "onehot0",              "swriteb",
	"atan2",              "fell",           "past",                 "swriteh",
	"atanh",              "fell_gclk",      "past_gclk",            "swriteo",
	"bits",               "feof",           "pow",                  "sync$and$array",
	"bitstoreal",         "ferror",         "printtimescale",       "sync$and$plane",
	"bitstoshortreal",    "fflush",         "q_add",                "sync$nand$array",
	"cast",               "fgetc",          "q_exam",               "sync$nand$plane",
	"ceil",               "fgets",          "q_full",               "sync$nor$array",
	"changed",            "finish",         "q_initialize",         "sync$nor$plane",
	"changed_gclk",       "floor",          "q_remove",             "sync$or$array",
	"changing_gclk",      "fmonitor",       "random",               "sync$or$plane",
	"clog2",              "fmonitorb",      "readmemb",             "system",
	"cos",                "fmonitorh",      "readmemh",             "tan",
	"cosh",               "fmonitoro",      "realtime",             "tanh",
	"countbits",          "fopen",          "realtobits",           "test$plusargs",
	"countdrivers",       "fread",          "rell",                 "time",
	"countones",          "fscanf",         "reset",                "timeformat",
	"coverage_control",   "fseek",          "reset_count",          "timeprecision",
	"coverage_get",       "fstrobe",        "reset_value",          "timeunit",
	"coverage_get_max",   "fstrobeb",       "restart",              "typename",
	"coverage_merge",     "fstrobeh",       "rewind",               "ungetc",
	"coverage_save",      "fstrobeo",       "right",                "unpacked_dimensions",
	"dimensions",         "ftell",          "rising_gclk",          "unsigned",
	"display",            "future_gclk",    "rose",                 "urandom",
	"displayb",           "fwrite",         "rose_gclk",            "urandom_range",
	"displayh",           "fwriteb",        "rtoi",                 "value$plusargs",
	"displayo",           "fwriteh",        "sampled",              "warning",
	"dist_chi_square",    "fwriteo",        "save",                 "write",
	"dist_erlang",        "get_coverage",   "scale",                "writeb",
	"dist_exponential",   "getpattern",     "scope",                "writeh",
	"dist_normal",        "high",           "sdf_annotate",         "writememb",
	"dist_poisson",       "hypot",          "set_coverage_db_name", "writememh",
	"dist_t",             "increment",      "sformat",              "writeo",
	"dist_uniform",       "incsave",        "sformatf",
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Builtin System Function to [`BuiltinSysFunc`] token map
pub static VERILOG_AMS_09_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"ceil" => BuiltinSysFunc::Ceil,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"exp" => BuiltinSysFunc::Exp,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"hypot" => BuiltinSysFunc::Hypot,
	"incsave" => BuiltinSysFunc::IncSave,
	"input" => BuiltinSysFunc::Input,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"sformat" => BuiltinSysFunc::Sformat,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"simparam" => BuiltinSysFunc::SimParam,
	"simparamstr" => BuiltinSysFunc::SimParamStr,
	"simprobe" => BuiltinSysFunc::SimProbe,
	"sin" => BuiltinSysFunc::Sin,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"table_model" => BuiltinSysFunc::TableModel,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"temperature" => BuiltinSysFunc::Temperature,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"vflip" => BuiltinSysFunc::Vflip,
	"vt" => BuiltinSysFunc::Vt,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writeo	" => BuiltinSysFunc::WriteO,
	"xposition" => BuiltinSysFunc::XPosition,
	"yposition" => BuiltinSysFunc::YPosition,
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Builtin System Function set
pub static VERILOG_AMS_09_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"abstime",              "dumpoff",        "key",               "simparam",
	"acos",                 "dumpon",         "limit",             "simparam$str",
	"acosh",                "dumpports",      "list",              "simprobe",
	"angle",                "dumpportsall",   "ln",                "sin",
	"arandom",              "dumpportsflush", "log",               "sqrt",
	"asin",                 "dumpportslimit", "log10",             "sreadmemb",
	"asinh",                "dumpportsoff",   "mfactor",           "sreadmemh",
	"async$and$array",      "dumpportson",    "monitor",           "sscanf",
	"async$and$plane",      "dumpvars",       "monitorb",          "stime",
	"async$nand$array",     "error",          "monitorh",          "stop",
	"async$nand$plane",     "exp",            "monitoro",          "strobe",
	"async$nor$array",      "fatal",          "monitoroff",        "strobeb",
	"async$nor$plane",      "fclose",         "monitoron",         "strobeh",
	"async$or$array",       "fdebug",         "nokey",             "strobeo",
	"async$or$plane",       "fdisplay",       "nolog",             "swrite",
	"atan",                 "fdisplayb",      "param_given",       "swriteb",
	"atan2",                "fdisplayh",      "port_connected",    "swriteh",
	"atanh",                "fdisplayo",      "printtimescale",    "swriteo",
	"bitstoreal",           "feof",           "q_add",             "sync$and$array",
	"bound_step",           "ferror",         "q_exam",            "sync$and$plane",
	"ceil",                 "fflush",         "q_full",            "sync$nand$array",
	"clog2",                "fgetc",          "q_initialize",      "sync$nand$plane",
	"cos",                  "fgets",          "q_remove",          "sync$nor$array",
	"cosh",                 "finish",         "random",            "sync$nor$plane",
	"countdrivers",         "floor",          "rdist_chi_square",  "sync$or$array",
	"debug",                "fmonitor",       "rdist_erlang",      "sync$or$plane",
	"discontinuity",        "fmonitorb",      "rdist_exponential", "table_model",
	"display",              "fmonitorh",      "rdist_normal",      "tan",
	"displayb",             "fmonitoro",      "rdist_poisson",     "tanh",
	"displayh",             "fopen",          "rdist_t",           "temperature",
	"displayo",             "fread",          "rdist_uniform",     "test$plusargs",
	"dist_chi_square",      "fscanf",         "readmemb",          "time",
	"dist_erlang",          "fseek",          "readmemh",          "timeformat",
	"dist_exponential",     "fstrobe",        "realtime",          "ungetc",
	"dist_normal",          "fstrobeb",       "realtobits",        "unsigned",
	"dist_poisson",         "fstrobeh",       "reset",             "value$plusargs",
	"dist_t",               "fstrobeo",       "reset_count",       "vflip",
	"dist_uniform",         "ftell",          "reset_value",       "vt",
	"driver_count",         "fwrite",         "restart",           "warning",
	"driver_delay",         "fwriteb",        "rewind",            "write",
	"driver_next_state",    "fwriteh",        "rtoi",              "writeb",
	"driver_next_strength", "fwriteo",        "save",              "writeh",
	"driver_state",         "getpattern",     "scale",             "writeo",
	"driver_strength",      "hflip",          "scope",             "xposition",
	"driver_type",          "hypot",          "sdf_annotate",      "yposition",
	"dumpall",              "incsave",        "sformat",
	"dumpfile",             "info",           "showscopes",
	"dumpflush",            "input",          "showvars",
	"dumplimit",            "itor",           "signed",
};

/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Builtin System Function to [`BuiltinSysFunc`] token map
pub static VERILOG_AMS_14_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"analog_node_alias" => BuiltinSysFunc::AnalogNodeAlias,
	"analog_port_alias" => BuiltinSysFunc::AnalogPortAlias,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"ceil" => BuiltinSysFunc::Ceil,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"exp" => BuiltinSysFunc::Exp,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"hypot" => BuiltinSysFunc::Hypot,
	"incsave" => BuiltinSysFunc::IncSave,
	"input" => BuiltinSysFunc::Input,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"sformat" => BuiltinSysFunc::Sformat,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"simparam" => BuiltinSysFunc::SimParam,
	"simparamstr" => BuiltinSysFunc::SimParamStr,
	"simprobe" => BuiltinSysFunc::SimProbe,
	"sin" => BuiltinSysFunc::Sin,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"table_model" => BuiltinSysFunc::TableModel,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"temperature" => BuiltinSysFunc::Temperature,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"vflip" => BuiltinSysFunc::Vflip,
	"vt" => BuiltinSysFunc::Vt,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writeo	" => BuiltinSysFunc::WriteO,
	"xposition" => BuiltinSysFunc::XPosition,
	"yposition" => BuiltinSysFunc::YPosition,
};

/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Builtin System Function set
pub static VERILOG_AMS_14_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"abstime",              "dumpflush",      "input",             "showvars",
	"acos",                 "dumplimit",      "itor",              "signed",
	"acosh",                "dumpoff",        "key",               "simparam",
	"analog_node_alias",    "dumpon",         "limit",             "simparam$str",
	"analog_port_alias",    "dumpports",      "list",              "simprobe",
	"angle",                "dumpportsall",   "ln",                "sin",
	"arandom",              "dumpportsflush", "log",               "sqrt",
	"asin",                 "dumpportslimit", "log10",             "sreadmemb",
	"asinh",                "dumpportsoff",   "mfactor",           "sreadmemh",
	"async$and$array",      "dumpportson",    "monitor",           "sscanf",
	"async$and$plane",      "dumpvars",       "monitorb",          "stime",
	"async$nand$array",     "error",          "monitorh",          "stop",
	"async$nand$plane",     "exp",            "monitoro",          "strobe",
	"async$nor$array",      "fatal",          "monitoroff",        "strobeb",
	"async$nor$plane",      "fclose",         "monitoron",         "strobeh",
	"async$or$array",       "fdebug",         "nokey",             "strobeo",
	"async$or$plane",       "fdisplay",       "nolog",             "swrite",
	"atan",                 "fdisplayb",      "param_given",       "swriteb",
	"atan2",                "fdisplayh",      "port_connected",    "swriteh",
	"atanh",                "fdisplayo",      "printtimescale",    "swriteo",
	"bitstoreal",           "feof",           "q_add",             "sync$and$array",
	"bound_step",           "ferror",         "q_exam",            "sync$and$plane",
	"ceil",                 "fflush",         "q_full",            "sync$nand$array",
	"clog2",                "fgetc",          "q_initialize",      "sync$nand$plane",
	"cos",                  "fgets",          "q_remove",          "sync$nor$array",
	"cosh",                 "finish",         "random",            "sync$nor$plane",
	"countdrivers",         "floor",          "rdist_chi_square",  "sync$or$array",
	"debug",                "fmonitor",       "rdist_erlang",      "sync$or$plane",
	"discontinuity",        "fmonitorb",      "rdist_exponential", "table_model",
	"display",              "fmonitorh",      "rdist_normal",      "tan",
	"displayb",             "fmonitoro",      "rdist_poisson",     "tanh",
	"displayh",             "fopen",          "rdist_t",           "temperature",
	"displayo",             "fread",          "rdist_uniform",     "test$plusargs",
	"dist_chi_square",      "fscanf",         "readmemb",          "time",
	"dist_erlang",          "fseek",          "readmemh",          "timeformat",
	"dist_exponential",     "fstrobe",        "realtime",          "ungetc",
	"dist_normal",          "fstrobeb",       "realtobits",        "unsigned",
	"dist_poisson",         "fstrobeh",       "reset",             "value$plusargs",
	"dist_t",               "fstrobeo",       "reset_count",       "vflip",
	"dist_uniform",         "ftell",          "reset_value",       "vt",
	"driver_count",         "fwrite",         "restart",           "warning",
	"driver_delay",         "fwriteb",        "rewind",            "write",
	"driver_next_state",    "fwriteh",        "rtoi",              "writeb",
	"driver_next_strength", "fwriteo",        "save",              "writeh",
	"driver_state",         "getpattern",     "scale",             "writeo",
	"driver_strength",      "hflip",          "scope",             "xposition",
	"driver_type",          "hypot",          "sdf_annotate",      "yposition",
	"dumpall",              "incsave",        "sformat",
	"dumpfile",             "info",           "showscopes",
};

/// Verilog-AMS 2023 Builtin System Function to [`BuiltinSysFunc`] token map
pub static VERILOG_AMS_23_BUILTIN_SYSFUNC_MAP: phf::Map<&'static str, BuiltinSysFunc> = phf_map! {
	"acos" => BuiltinSysFunc::Acos,
	"acosh" => BuiltinSysFunc::Acosh,
	"analog_node_alias" => BuiltinSysFunc::AnalogNodeAlias,
	"analog_port_alias" => BuiltinSysFunc::AnalogPortAlias,
	"asin" => BuiltinSysFunc::Asin,
	"asinh" => BuiltinSysFunc::Asinh,
	"async$and$array" => BuiltinSysFunc::AsyncAndArray,
	"async$and$plane" => BuiltinSysFunc::AsyncAndPlane,
	"async$nand$array" => BuiltinSysFunc::AsyncNandArray,
	"async$nand$plane" => BuiltinSysFunc::AsyncNandPlane,
	"async$nor$array" => BuiltinSysFunc::AsyncNorArray,
	"async$nor$plane" => BuiltinSysFunc::AsyncNorPlane,
	"async$or$array" => BuiltinSysFunc::AsyncOrArray,
	"async$or$plane" => BuiltinSysFunc::AsyncOrPlane,
	"atan" => BuiltinSysFunc::Atan,
	"atan2" => BuiltinSysFunc::Atan2,
	"atanh" => BuiltinSysFunc::Atanh,
	"bitstoreal" => BuiltinSysFunc::BitsToReal,
	"ceil" => BuiltinSysFunc::Ceil,
	"clog2" => BuiltinSysFunc::Clog2,
	"cos" => BuiltinSysFunc::Cos,
	"cosh" => BuiltinSysFunc::Cosh,
	"countdrivers" => BuiltinSysFunc::CountDrivers,
	"display" => BuiltinSysFunc::Display,
	"displayb" => BuiltinSysFunc::DisplayB,
	"displayh" => BuiltinSysFunc::DisplayH,
	"displayo" => BuiltinSysFunc::DisplayO,
	"dist_chi_square" => BuiltinSysFunc::DistChiSquare,
	"dist_erlang" => BuiltinSysFunc::DistErlang,
	"dist_exponential" => BuiltinSysFunc::DistExponential,
	"dist_normal" => BuiltinSysFunc::DistNormal,
	"dist_poisson" => BuiltinSysFunc::DistPoisson,
	"dist_t" => BuiltinSysFunc::DistT,
	"dist_uniform" => BuiltinSysFunc::DistUniform,
	"dumpall" => BuiltinSysFunc::DumpAll,
	"dumpfile" => BuiltinSysFunc::DumpFile,
	"dumpflush" => BuiltinSysFunc::DumpFlush,
	"dumplimit" => BuiltinSysFunc::DumpLimit,
	"dumpoff" => BuiltinSysFunc::DumpOff,
	"dumpon" => BuiltinSysFunc::DumpOn,
	"dumpports" => BuiltinSysFunc::DumpPorts,
	"dumpportsall" => BuiltinSysFunc::DumpPortsAll,
	"dumpportsflush" => BuiltinSysFunc::DumpPortsFlush,
	"dumpportslimit" => BuiltinSysFunc::DumpPortsLimit,
	"dumpportsoff" => BuiltinSysFunc::DumpPortsOff,
	"dumpportson" => BuiltinSysFunc::DumpPortsOn,
	"dumpvars" => BuiltinSysFunc::DumpVars,
	"exp" => BuiltinSysFunc::Exp,
	"fclose" => BuiltinSysFunc::Fclose,
	"fdisplay" => BuiltinSysFunc::Fdisplay,
	"fdisplayb" => BuiltinSysFunc::FdisplayB,
	"fdisplayh" => BuiltinSysFunc::FdisplayH,
	"fdisplayo" => BuiltinSysFunc::FdisplayO,
	"feof" => BuiltinSysFunc::Feof,
	"ferror" => BuiltinSysFunc::Ferror,
	"fflush" => BuiltinSysFunc::Fflush,
	"fgetc" => BuiltinSysFunc::Fgetc,
	"fgets" => BuiltinSysFunc::Fgets,
	"finish" => BuiltinSysFunc::Finish,
	"floor" => BuiltinSysFunc::Floor,
	"fmonitor" => BuiltinSysFunc::Fmonitor,
	"fmonitorb" => BuiltinSysFunc::FmonitorB,
	"fmonitorh" => BuiltinSysFunc::FmonitorH,
	"fmonitoro" => BuiltinSysFunc::FmonitorO,
	"fopen" => BuiltinSysFunc::Fopen,
	"fread" => BuiltinSysFunc::Fread,
	"fscanf" => BuiltinSysFunc::Fscanf,
	"fseek" => BuiltinSysFunc::Fseek,
	"fstrobe" => BuiltinSysFunc::Fstrobe,
	"fstrobeb" => BuiltinSysFunc::FstrobeB,
	"fstrobeh" => BuiltinSysFunc::FstrobeH,
	"fstrobeo" => BuiltinSysFunc::FstrobeO,
	"ftell" => BuiltinSysFunc::Ftell,
	"fwrite" => BuiltinSysFunc::Fwrite,
	"fwriteb" => BuiltinSysFunc::FwriteB,
	"fwriteh" => BuiltinSysFunc::FwriteH,
	"fwriteo" => BuiltinSysFunc::FwriteO,
	"getpattern" => BuiltinSysFunc::GetPattern,
	"hypot" => BuiltinSysFunc::Hypot,
	"incsave" => BuiltinSysFunc::IncSave,
	"input" => BuiltinSysFunc::Input,
	"itor" => BuiltinSysFunc::Itor,
	"key" => BuiltinSysFunc::Key,
	"list" => BuiltinSysFunc::List,
	"ln" => BuiltinSysFunc::Ln,
	"log" => BuiltinSysFunc::Log,
	"log10" => BuiltinSysFunc::Log10,
	"monitor" => BuiltinSysFunc::Monitor,
	"monitorb" => BuiltinSysFunc::MonitorB,
	"monitorh" => BuiltinSysFunc::MonitorH,
	"monitoro" => BuiltinSysFunc::MonitorO,
	"monitoroff" => BuiltinSysFunc::MonitorOff,
	"monitoron" => BuiltinSysFunc::MonitorOn,
	"nokey" => BuiltinSysFunc::NoKey,
	"nolog" => BuiltinSysFunc::NoLog,
	"printtimescale" => BuiltinSysFunc::PrintTimescale,
	"q_add" => BuiltinSysFunc::QAdd,
	"q_exam" => BuiltinSysFunc::QExam,
	"q_full" => BuiltinSysFunc::QFull,
	"q_initialize" => BuiltinSysFunc::QInitialize,
	"q_remove" => BuiltinSysFunc::QRemove,
	"random" => BuiltinSysFunc::Random,
	"readmemb" => BuiltinSysFunc::ReadmemB,
	"readmemh" => BuiltinSysFunc::ReadmemH,
	"realtime" => BuiltinSysFunc::RealTime,
	"realtobits" => BuiltinSysFunc::RealToBits,
	"receiver_count" => BuiltinSysFunc::ReceiverCount,
	"reset_count" => BuiltinSysFunc::ResetCount,
	"reset_value" => BuiltinSysFunc::ResetValue,
	"reset" => BuiltinSysFunc::Reset,
	"restart" => BuiltinSysFunc::Restart,
	"rewind" => BuiltinSysFunc::Rewind,
	"rtoi" => BuiltinSysFunc::Rtoi,
	"save" => BuiltinSysFunc::Save,
	"scale" => BuiltinSysFunc::Scale,
	"scope" => BuiltinSysFunc::Scope,
	"sdf_annotate" => BuiltinSysFunc::SdfAnnotate,
	"sformat" => BuiltinSysFunc::Sformat,
	"showscopes" => BuiltinSysFunc::Showscopes,
	"showvars" => BuiltinSysFunc::Showvars,
	"signed" => BuiltinSysFunc::Signed,
	"simparam" => BuiltinSysFunc::SimParam,
	"simparamstr" => BuiltinSysFunc::SimParamStr,
	"simprobe" => BuiltinSysFunc::SimProbe,
	"sin" => BuiltinSysFunc::Sin,
	"sqrt" => BuiltinSysFunc::Sqrt,
	"sreadmemb" => BuiltinSysFunc::SreadmemB,
	"sreadmemh" => BuiltinSysFunc::SreadmemH,
	"sscanf" => BuiltinSysFunc::Sscanf,
	"stime" => BuiltinSysFunc::Stime,
	"stop" => BuiltinSysFunc::Stop,
	"strobe" => BuiltinSysFunc::Strobe,
	"strobeb" => BuiltinSysFunc::StrobeB,
	"strobeh" => BuiltinSysFunc::StrobeH,
	"strobeo" => BuiltinSysFunc::StrobeO,
	"swrite" => BuiltinSysFunc::Swrite,
	"swriteb" => BuiltinSysFunc::SwriteB,
	"swriteh" => BuiltinSysFunc::SwriteH,
	"swriteo" => BuiltinSysFunc::SwriteO,
	"sync$and$array" => BuiltinSysFunc::SyncAndArray,
	"sync$and$plane" => BuiltinSysFunc::SyncAndPlane,
	"sync$nand$array" => BuiltinSysFunc::SyncNandArray,
	"sync$nand$plane" => BuiltinSysFunc::SyncNandPlane,
	"sync$nor$array" => BuiltinSysFunc::SyncNorArray,
	"sync$nor$plane" => BuiltinSysFunc::SyncNorPlane,
	"sync$or$array" => BuiltinSysFunc::SyncOrArray,
	"sync$or$plane" => BuiltinSysFunc::SyncOrPlane,
	"table_model" => BuiltinSysFunc::TableModel,
	"tan" => BuiltinSysFunc::Tan,
	"tanh" => BuiltinSysFunc::Tanh,
	"temperature" => BuiltinSysFunc::Temperature,
	"test$plusargs" => BuiltinSysFunc::TestPlusArgs,
	"time" => BuiltinSysFunc::Time,
	"timeformat" => BuiltinSysFunc::TimeFormat,
	"ungetc" => BuiltinSysFunc::Ungetc,
	"unsigned" => BuiltinSysFunc::Unsigned,
	"value$plusargs" => BuiltinSysFunc::ValuePlusArgs,
	"vflip" => BuiltinSysFunc::Vflip,
	"vt" => BuiltinSysFunc::Vt,
	"warning" => BuiltinSysFunc::Warning,
	"write" => BuiltinSysFunc::Write,
	"writeb" => BuiltinSysFunc::WriteB,
	"writeh" => BuiltinSysFunc::WriteH,
	"writeo	" => BuiltinSysFunc::WriteO,
	"xposition" => BuiltinSysFunc::XPosition,
	"yposition" => BuiltinSysFunc::YPosition,
};

/// Verilog-AMS 2023 Builtin System Function set
pub static VERILOG_AMS_23_BUILTIN_SYSFUNC_SET: phf::Set<&'static str> = phf_set! {
	"abstime",              "dumpflush",      "input",             "showscopes",
	"acos",                 "dumplimit",      "itor",              "showvars",
	"acosh",                "dumpoff",        "key",               "signed",
	"analog_node_alias",    "dumpon",         "limit",             "simparam",
	"analog_port_alias",    "dumpports",      "list",              "simparam$str",
	"angle",                "dumpportsall",   "ln",                "simprobe",
	"arandom",              "dumpportsflush", "log",               "sin",
	"asin",                 "dumpportslimit", "log10",             "sqrt",
	"asinh",                "dumpportsoff",   "mfactor",           "sreadmemb",
	"async$and$array",      "dumpportson",    "monitor",           "sreadmemh",
	"async$and$plane",      "dumpvars",       "monitorb",          "sscanf",
	"async$nand$array",     "error",          "monitorh",          "stime",
	"async$nand$plane",     "exp",            "monitoro",          "stop",
	"async$nor$array",      "fatal",          "monitoroff",        "strobe",
	"async$nor$plane",      "fclose",         "monitoron",         "strobeb",
	"async$or$array",       "fdebug",         "nokey",             "strobeh",
	"async$or$plane",       "fdisplay",       "nolog",             "strobeo",
	"atan",                 "fdisplayb",      "param_given",       "swrite",
	"atan2",                "fdisplayh",      "port_connected",    "swriteb",
	"atanh",                "fdisplayo",      "printtimescale",    "swriteh",
	"bitstoreal",           "feof",           "q_add",             "swriteo",
	"bound_step",           "ferror",         "q_exam",            "sync$and$array",
	"ceil",                 "fflush",         "q_full",            "sync$and$plane",
	"clog2",                "fgetc",          "q_initialize",      "sync$nand$array",
	"cos",                  "fgets",          "q_remove",          "sync$nand$plane",
	"cosh",                 "finish",         "random",            "sync$nor$array",
	"countdrivers",         "floor",          "rdist_chi_square",  "sync$nor$plane",
	"debug",                "fmonitor",       "rdist_erlang",      "sync$or$array",
	"discontinuity",        "fmonitorb",      "rdist_exponential", "sync$or$plane",
	"display",              "fmonitorh",      "rdist_normal",      "table_model",
	"displayb",             "fmonitoro",      "rdist_poisson",     "tan",
	"displayh",             "fopen",          "rdist_t",           "tanh",
	"displayo",             "fread",          "rdist_uniform",     "temperature",
	"dist_chi_square",      "fscanf",         "readmemb",          "test$plusargs",
	"dist_erlang",          "fseek",          "readmemh",          "time",
	"dist_exponential",     "fstrobe",        "realtime",          "timeformat",
	"dist_normal",          "fstrobeb",       "realtobits",        "ungetc",
	"dist_poisson",         "fstrobeh",       "receiver_count",    "unsigned",
	"dist_t",               "fstrobeo",       "reset",             "value$plusargs",
	"dist_uniform",         "ftell",          "reset_count",       "vflip",
	"driver_count",         "fwrite",         "reset_value",       "vt",
	"driver_delay",         "fwriteb",        "restart",           "warning",
	"driver_next_state",    "fwriteh",        "rewind",            "write",
	"driver_next_strength", "fwriteo",        "rtoi",              "writeb",
	"driver_state",         "getpattern",     "save",              "writeh",
	"driver_strength",      "hflip",          "scale",             "writeo",
	"driver_type",          "hypot",          "scope",             "xposition",
	"dumpall",              "incsave",        "sdf_annotate",      "yposition",
	"dumpfile",             "info",           "sformat",
};

/// Get the Verilog 1995 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_95_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	VERILOG_95_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 1995 builtin system function
///
/// This is used rather than [`get_verilog_95_builtin_sysfunc`] to test if the
/// found identifier for a lower Verilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_verilog_95_builtin_sysfunc(ident: &str) -> bool {
	VERILOG_95_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the Verilog 2001 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_01_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	VERILOG_01_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2001 builtin system function
///
/// This is used rather than [`get_verilog_01_builtin_sysfunc`] to test if the
/// found identifier for a lower Verilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_verilog_01_builtin_sysfunc(ident: &str) -> bool {
	VERILOG_01_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the Verilog 2005 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_05_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	VERILOG_05_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2005 builtin system function
///
/// This is used rather than [`get_verilog_05_builtin_sysfunc`] to test if the
/// found identifier for a lower Verilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_verilog_05_builtin_sysfunc(ident: &str) -> bool {
	VERILOG_05_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the Verilog builtin system function for the given standard if it exists
#[inline(always)]
pub fn get_verilog_builtin_sysfunc(ident: &str, std: LanguageStd) -> Option<BuiltinSysFunc> {
	match std {
		LanguageStd::Vl95 => get_verilog_95_builtin_sysfunc(ident),
		LanguageStd::Vl01 => get_verilog_01_builtin_sysfunc(ident),
		LanguageStd::Vl05 => get_verilog_05_builtin_sysfunc(ident),
		_ => None,
	}
}

/// Get the SystemVerilog 2005 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_05_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	SYSTEM_VERILOG_05_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2005 builtin system function
///
/// This is used rather than [`get_system_verilog_05_builtin_sysfunc`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_system_verilog_05_builtin_sysfunc(ident: &str) -> bool {
	SYSTEM_VERILOG_05_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the SystemVerilog 2009 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_09_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	SYSTEM_VERILOG_09_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2009 builtin system function
///
/// This is used rather than [`get_system_verilog_09_builtin_sysfunc`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_system_verilog_09_builtin_sysfunc(ident: &str) -> bool {
	SYSTEM_VERILOG_09_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the SystemVerilog 2012 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_12_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	SYSTEM_VERILOG_12_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2012 builtin system function
///
/// This is used rather than [`get_system_verilog_12_builtin_sysfunc`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_system_verilog_12_builtin_sysfunc(ident: &str) -> bool {
	SYSTEM_VERILOG_12_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the SystemVerilog 2017 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_17_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	SYSTEM_VERILOG_17_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2017 builtin system function
///
/// This is used rather than [`get_system_verilog_17_builtin_sysfunc`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_system_verilog_17_builtin_sysfunc(ident: &str) -> bool {
	SYSTEM_VERILOG_17_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the SystemVerilog 2023 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_23_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	SYSTEM_VERILOG_23_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2023 builtin system function
///
/// This is used rather than [`get_system_verilog_23_builtin_sysfunc`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_system_verilog_23_builtin_sysfunc(ident: &str) -> bool {
	SYSTEM_VERILOG_23_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the SystemVerilog builtin system function for the given standard if it exists
#[inline(always)]
pub fn get_system_verilog_builtin_sysfunc(ident: &str, std: LanguageStd) -> Option<BuiltinSysFunc> {
	match std {
		LanguageStd::Sv05 => get_system_verilog_05_builtin_sysfunc(ident),
		LanguageStd::Sv09 => get_system_verilog_09_builtin_sysfunc(ident),
		LanguageStd::Sv12 => get_system_verilog_12_builtin_sysfunc(ident),
		LanguageStd::Sv17 => get_system_verilog_17_builtin_sysfunc(ident),
		LanguageStd::Sv23 => get_system_verilog_23_builtin_sysfunc(ident),
		_ => None,
	}
}

/// Get the Verilog-AMS 2009 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_ams_09_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	VERILOG_AMS_09_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2009 builtin system function
///
/// This is used rather than [`get_verilog_ams_09_builtin_sysfunc`] to test if the
/// found identifier for a lower Verilog-AMS standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_verilog_ams_09_builtin_sysfunc(ident: &str) -> bool {
	VERILOG_AMS_09_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the Verilog-AMS 2014 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_ams_14_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	VERILOG_AMS_14_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2014 builtin system function
///
/// This is used rather than [`get_verilog_ams_14_builtin_sysfunc`] to test if the
/// found identifier for a lower Verilog-AMS standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_verilog_ams_14_builtin_sysfunc(ident: &str) -> bool {
	VERILOG_AMS_14_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the Verilog-AMS 2023 builtin system function for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_ams_23_builtin_sysfunc(ident: &str) -> Option<BuiltinSysFunc> {
	VERILOG_AMS_23_BUILTIN_SYSFUNC_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2023 builtin system function
///
/// This is used rather than [`get_verilog_ams_23_builtin_sysfunc`] to test if the
/// found identifier for a lower Verilog-AMS standard is a builtin system function in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin system function.
#[inline(always)]
pub fn is_verilog_ams_23_builtin_sysfunc(ident: &str) -> bool {
	VERILOG_AMS_23_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the Verilog-AMS builtin system function for the given standard if it exists
#[inline(always)]
pub fn get_verilog_ams_builtin_sysfunc(ident: &str, std: LanguageStd) -> Option<BuiltinSysFunc> {
	match std {
		LanguageStd::Vams09 => get_verilog_ams_09_builtin_sysfunc(ident),
		LanguageStd::Vams14 => get_verilog_ams_14_builtin_sysfunc(ident),
		LanguageStd::Vams23 => get_verilog_ams_23_builtin_sysfunc(ident),
		_ => None,
	}
}

/// Get the [`BuiltinSysFunc`] token for the given identifier if it appears in
/// the provided language standard.
///
/// If you are only checking if the given system task is present in the langauge
/// standard then use one of the `is_` methods directly, or [`is_builtin_sysfunc`]
/// for a faster hit-test.
#[inline(always)]
pub fn get_builtin_sysfunc(ident: &str, std: LanguageStd) -> Option<BuiltinSysFunc> {
	match std {
		LanguageStd::Vl95 => get_verilog_95_builtin_sysfunc(ident),
		LanguageStd::Vl01 => get_verilog_01_builtin_sysfunc(ident),
		LanguageStd::Vl05 => get_verilog_05_builtin_sysfunc(ident),
		LanguageStd::Sv05 => get_system_verilog_05_builtin_sysfunc(ident),
		LanguageStd::Sv09 => get_system_verilog_09_builtin_sysfunc(ident),
		LanguageStd::Sv12 => get_system_verilog_12_builtin_sysfunc(ident),
		LanguageStd::Sv17 => get_system_verilog_17_builtin_sysfunc(ident),
		LanguageStd::Sv23 => get_system_verilog_23_builtin_sysfunc(ident),
		LanguageStd::Vams09 => get_verilog_ams_09_builtin_sysfunc(ident),
		LanguageStd::Vams14 => get_verilog_ams_14_builtin_sysfunc(ident),
		LanguageStd::Vams23 => get_verilog_ams_23_builtin_sysfunc(ident),
		_ => None,
	}
}

/// Check to see if the given task name is a timing task or not.
///
/// In IEEE 1364-1995 the timing tasks were classified as system tasks, but the were
/// removed and specialized as part of the grammar in every version of Verilog, SystemVerilog
/// and Verilog-AMS after that, but due to backwards compatibility, they maintain the `$` prefix.
///
/// As such, we need to be able to check if it is specifically a timing task, because they're valid
/// in context where normal system tasks/functions are not.
#[inline(always)]
pub fn is_timing_task(ident: &str) -> bool {
	TIMING_TASKS_SET.contains(ident)
}

/// Check to see if the given name is one of the optional system functions
///
/// There are a collection of optional system functions that are listed in the
/// standards, but are not required to be implemented by tools.
///
/// This allows us to check to see if the given name is one of those, if so, we can
/// possibly emit a diagnostic if requested.
#[inline(always)]
pub fn is_optional_builtin_sysfunc(ident: &str) -> bool {
	OPTIONAL_BUILTIN_SYSFUNC_SET.contains(ident)
}

/// Get the bitmap of Verilog/SystemVerilog/Verilog-AMS standards that
/// have the given system task identifier.
///
/// This can be used to generate appropriate diagnostics and list which
/// language and which standards of said langauge support the given
/// system task.
#[inline(always)]
pub fn is_builtin_sysfunc(ident: &str) -> LanguageStd {
	let mut supported = LanguageStd::none();

	if is_verilog_95_builtin_sysfunc(ident) {
		supported |= LanguageStd::Vl95;
	}

	if is_verilog_01_builtin_sysfunc(ident) {
		supported |= LanguageStd::Vl01;
	}

	if is_verilog_05_builtin_sysfunc(ident) {
		supported |= LanguageStd::Vl05;
	}

	if is_system_verilog_05_builtin_sysfunc(ident) {
		supported |= LanguageStd::Sv05;
	}

	if is_system_verilog_09_builtin_sysfunc(ident) {
		supported |= LanguageStd::Sv09;
	}

	if is_system_verilog_12_builtin_sysfunc(ident) {
		supported |= LanguageStd::Sv12;
	}

	if is_system_verilog_17_builtin_sysfunc(ident) {
		supported |= LanguageStd::Sv17;
	}

	if is_system_verilog_23_builtin_sysfunc(ident) {
		supported |= LanguageStd::Sv23;
	}

	if is_verilog_ams_09_builtin_sysfunc(ident) {
		supported |= LanguageStd::Vams09;
	}

	if is_verilog_ams_14_builtin_sysfunc(ident) {
		supported |= LanguageStd::Vams14;
	}

	if is_verilog_ams_23_builtin_sysfunc(ident) {
		supported |= LanguageStd::Vams23;
	}

	supported
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for BuiltinSysFunc {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"BuiltinSysFunc(${})",
			match self {
				Self::Acos => "acos",                             // Added: IEEE 1364-2005
				Self::Acosh => "acosh",                           // Added: IEEE 1364-2005
				Self::AnalogNodeAlias => "analog_node_alias",     // Added: Verilog-AMS 2.4.0
				Self::AnalogPortAlias => "analog_port_alias",     // Added: Verilog-AMS 2.4.0
				Self::Asin => "asin",                             // Added: IEEE 1364-2005
				Self::Asinh => "asinh",                           // Added: IEEE 1364-2005
				Self::AssertControl => "assertcontrol",           // Added: IEEE 1800-2012
				Self::AssertFailOff => "assertfailoff",           // Added: IEEE 1800-2009
				Self::AssertFailOn => "assertfailon",             // Added: IEEE 1800-2009
				Self::AssertKill => "assertkill",                 // Added: IEEE 1800-2005
				Self::AssertNonVacuousOn => "assertnonvacuouson", // Added: IEEE 1800-2009
				Self::AssertOff => "assertoff",                   // Added: IEEE 1800-2005
				Self::AssertOn => "asserton",                     // Added: IEEE 1800-2005
				Self::AssertPassOff => "assertpassoff",           // Added: IEEE 1800-2009
				Self::AssertPassOn => "assertpasson",             // Added: IEEE 1800-2009
				Self::AssertVacuousOff => "assertvacuousoff",     // Added: IEEE 1800-2009
				Self::AsyncAndArray => "async$and$array",
				Self::AsyncAndPlane => "async$and$plane",
				Self::AsyncNandArray => "async$nand$array",
				Self::AsyncNandPlane => "async$nand$plane",
				Self::AsyncNorArray => "async$nor$array",
				Self::AsyncNorPlane => "async$nor$plane",
				Self::AsyncOrArray => "async$or$array",
				Self::AsyncOrPlane => "async$or$plane",
				Self::Atan => "atan",   // Added: IEEE 1364-2005
				Self::Atan2 => "atan2", // Added: IEEE 1364-2005
				Self::Atanh => "atanh", // Added: IEEE 1364-2005
				Self::Bits => "bits",   // Added: IEEE 1800-2005
				Self::BitsToReal => "bitstoreal",
				Self::BitsToShortReal => "bitstoshortreal", // Added: IEEE 1800-2005
				Self::Cast => "cast",                       // Added: IEEE 1800-2005
				Self::Ceil => "ceil",                       // Added: IEEE 1364-2005
				Self::Changed => "changed",                 // Added: IEEE 1800-2009
				Self::ChangedGclk => "changed_gclk",        // Added: IEEE 1800-2009
				Self::ChangingGclk => "changing_gclk",      // Added: IEEE 1800-2009
				Self::Clog2 => "clog2",                     // Added: IEEE 1364-2005
				Self::Cos => "cos",                         // Added: IEEE 1364-2005
				Self::Cosh => "cosh",                       // Added: IEEE 1364-2005
				Self::CountBits => "countbits",             // Added: IEEE 1800-2012
				Self::CountDrivers => "countdrivers",
				Self::CountOnes => "countones", // Added: IEEE 1800-2005
				Self::CoverageControl => "coverage_control", // Added: IEEE 1800-2005
				Self::CoverageGet => "coverage_get", // Added: IEEE 1800-2005
				Self::CoverageGetMax => "coverage_get_max", // Added: IEEE 1800-2005
				Self::CoverageMerge => "coverage_merge", // Added: IEEE 1800-2005
				Self::CoverageSave => "coverage_save", // Added: IEEE 1800-2005
				Self::Dimensions => "dimensions", // Added: IEEE 1800-2005
				Self::Display => "display",
				Self::DisplayB => "displayb",
				Self::DisplayH => "displayh",
				Self::DisplayO => "displayo",
				Self::DistChiSquare => "dist_chi_square",
				Self::DistErlang => "dist_erlang",
				Self::DistExponential => "dist_exponential",
				Self::DistNormal => "dist_normal",
				Self::DistPoisson => "dist_poisson",
				Self::DistT => "dist_t",
				Self::DistUniform => "dist_uniform",
				Self::DumpAll => "dumpall",
				Self::DumpFile => "dumpfile",
				Self::DumpFlush => "dumpflush",
				Self::DumpLimit => "dumplimit",
				Self::DumpOff => "dumpoff",
				Self::DumpOn => "dumpon",
				Self::DumpPorts => "dumpports",           // Added: IEEE 1364-2001
				Self::DumpPortsAll => "dumpportsall",     // Added: IEEE 1364-2001
				Self::DumpPortsFlush => "dumpportsflush", // Added: IEEE 1364-2001
				Self::DumpPortsLimit => "dumpportslimit", // Added: IEEE 1364-2001
				Self::DumpPortsOff => "dumpportsoff",     // Added: IEEE 1364-2001
				Self::DumpPortsOn => "dumpportson",       // Added: IEEE 1364-2001
				Self::DumpVars => "dumpvars",
				Self::Error => "error",              // Added: IEEE 1800-2005
				Self::Exit => "exit",                // Added: IEEE 1800-2005
				Self::Exp => "exp",                  // Added: IEEE 1364-2005
				Self::FallingGclk => "falling_gclk", // Added: IEEE 1800-2009
				Self::Fatal => "fatal",              // Added: IEEE 1800-2005
				Self::Fclose => "fclose",
				Self::Fdisplay => "fdisplay",
				Self::FdisplayB => "fdisplayb",
				Self::FdisplayH => "fdisplayh",
				Self::FdisplayO => "fdisplayo",
				Self::Fell => "fell",          // Added: IEEE 1800-2005
				Self::FellGclk => "fell_gclk", // Added: IEEE 1800-2009
				Self::Feof => "feof",          // Added: IEEE 1364-2005
				Self::Ferror => "ferror",      // Added: IEEE 1364-2001
				Self::Fflush => "fflush",      // Added: IEEE 1364-2001
				Self::Fgetc => "fgetc",        // Added: IEEE 1364-2001
				Self::Fgets => "fgets",        // Added: IEEE 1364-2001
				Self::Finish => "finish",
				Self::Floor => "floor", // Added: IEEE 1364-2005
				Self::Fmonitor => "fmonitor",
				Self::FmonitorB => "fmonitorb",
				Self::FmonitorH => "fmonitorh",
				Self::FmonitorO => "fmonitoro",
				Self::Fopen => "fopen",
				Self::Fread => "fread",   // Added: IEEE 1364-2001
				Self::Fscanf => "fscanf", // Added: IEEE 1364-2001
				Self::Fseek => "fseek",   // Added: IEEE 1364-2001
				Self::Fstrobe => "fstrobe",
				Self::FstrobeB => "fstrobeb",
				Self::FstrobeH => "fstrobeh",
				Self::FstrobeO => "fstrobeo",
				Self::Ftell => "ftell",            // Added: IEEE 1364-2001
				Self::FutureGclk => "future_gclk", // Added: IEEE 1800-2009
				Self::Fwrite => "fwrite",
				Self::FwriteB => "fwriteb",
				Self::FwriteH => "fwriteh",
				Self::FwriteO => "fwriteo",
				Self::GetCoverage => "get_coverage", // Added: IEEE 1800-2005
				Self::GetPattern => "getpattern",
				Self::High => "high",           // Added: IEEE 1800-2005
				Self::Hold => "hold",           // Removed: IEEE 1364-2001
				Self::Hypot => "hypot",         // Added: IEEE 1364-2005
				Self::Increment => "increment", // Added: IEEE 1800-2005
				Self::IncSave => "incsave",
				Self::Info => "info", // Added: IEEE 1800-2005
				Self::Input => "input",
				Self::IsUnbounded => "isunbounded", // Added: IEEE 1800-2005
				Self::IsUnknown => "isunknown",     // Added: IEEE 1800-2005
				Self::Itor => "itor",
				Self::Key => "key",
				Self::Left => "left", // Added: IEEE 1800-2005
				Self::List => "list",
				Self::Ln => "ln",                           // Added: IEEE 1364-2005
				Self::LoadCoverageDB => "load_coverage_db", // Added: IEEE 1800-2005
				Self::Log => "log",
				Self::Log10 => "log10", // Added: IEEE 1364-2005
				Self::Low => "low",     // Added: IEEE 1800-2005
				Self::Monitor => "monitor",
				Self::MonitorB => "monitorb",
				Self::MonitorH => "monitorh",
				Self::MonitorO => "monitoro",
				Self::MonitorOff => "monitoroff",
				Self::MonitorOn => "monitoron",
				Self::NoChange => "nochange", // Removed: IEEE 1364-2001
				Self::NoKey => "nokey",
				Self::NoLog => "nolog",
				Self::OneHot => "onehot",      // Added: IEEE 1800-2005
				Self::OneHot0 => "onehot0",    // Added: IEEE 1800-2005
				Self::Past => "past",          // Added: IEEE 1800-2005
				Self::PastGclk => "past_gclk", // Added: IEEE 1800-2009
				Self::Period => "period",      // Removed: IEEE 1364-2001
				Self::Pow => "pow",            // Added: IEEE 1800-2009
				Self::PrintTimescale => "printtimescale",
				Self::QAdd => "q_add",
				Self::QExam => "q_exam",
				Self::QFull => "q_full",
				Self::QInitialize => "q_initialize",
				Self::QRemove => "q_remove",
				Self::Random => "random",
				Self::ReadmemB => "readmemb",
				Self::ReadmemH => "readmemh",
				Self::RealTime => "realtime",
				Self::RealToBits => "realtobits",
				Self::Recovery => "recovery",            // Removed: IEEE 1364-2001
				Self::ReceiverCount => "receiver_count", // Added: Verilog-AMS 23
				Self::Reset => "reset",
				Self::ResetCount => "reset_count",
				Self::ResetValue => "reset_value",
				Self::Restart => "restart",
				Self::Rewind => "rewind",          // Added: IEEE 1364-2001
				Self::Right => "right",            // Added: IEEE 1800-2005
				Self::RisingGclk => "rising_gclk", // Added: IEEE 1800-2009
				Self::Rose => "rose",              // Added: IEEE 1800-2005
				Self::RoseGclk => "rose_gclk",     // Added: IEEE 1800-2009
				Self::Rtoi => "rtoi",
				Self::Sampled => "sampled", // Added: IEEE 1800-2005
				Self::Save => "save",
				Self::Scale => "scale",
				Self::Scope => "scope",
				Self::SdfAnnotate => "sdf_annotate", // Added: IEEE 1364-2001
				Self::SetCoverageDb => "set_coverage_db", // Added: IEEE 1800-2005-IEEE 1800-2009
				Self::SetCoverageDbName => "set_coverage_db_name", // Added: IEEE 1800-2009
				Self::Setup => "setup",              // Removed: IEEE 1364-2001
				Self::Setuphold => "setuphold",      // Removed: IEEE 1364-2001
				Self::Sformat => "sformat",          // Added: IEEE 1364-2001
				Self::Sformatf => "sformatf",        // Added: IEEE 1800-2009
				Self::ShortRealToBits => "shortrealtobits", // Added: IEEE 1800-2005
				Self::Showscopes => "showscopes",
				Self::Showvars => "showvars",
				Self::Signed => "signed",           // Added: IEEE 1364-2001
				Self::SimParam => "simparam",       // Added: Verilog-AMS 2.3.1
				Self::SimParamStr => "simparamstr", // Added: Verilog-AMS 2.3.1
				Self::SimProbe => "simprobe",       // Added: Verilog-AMS 2.3.1
				Self::Sin => "sin",                 // Added: IEEE 1364-2005
				Self::Sinh => "sinh",               // Added: IEEE 1800-2009
				Self::Size => "size",               // Added: IEEE 1800-2005
				Self::Skew => "skew",               // Removed: IEEE 1364-2001
				Self::Sqrt => "sqrt",               // Added: IEEE 1364-2005
				Self::SreadmemB => "sreadmemb",
				Self::SreadmemH => "sreadmemh",
				Self::Sscanf => "sscanf",          // Added: IEEE 1364-2001
				Self::Stable => "stable",          // Added: IEEE 1800-2005
				Self::StableGclk => "stable_gclk", // Added: IEEE 1800-2009
				Self::Stacktrace => "stacktrace",  // Added: IEEE 1800-2023
				Self::SteadyGclk => "steady_gclk", // Added: IEEE 1800-2009
				Self::Stime => "stime",
				Self::Stop => "stop",
				Self::Strobe => "strobe",
				Self::StrobeB => "strobeb",
				Self::StrobeH => "strobeh",
				Self::StrobeO => "strobeo",
				Self::Swrite => "swrite",   // Added: IEEE 1364-2001
				Self::SwriteB => "swriteb", // Added: IEEE 1364-2001
				Self::SwriteH => "swriteh", // Added: IEEE 1364-2001
				Self::SwriteO => "swriteo", // Added: IEEE 1364-2001
				Self::SyncAndArray => "sync$and$array",
				Self::SyncAndPlane => "sync$and$plane",
				Self::SyncNandArray => "sync$nand$array",
				Self::SyncNandPlane => "sync$nand$plane",
				Self::SyncNorArray => "sync$nor$array",
				Self::SyncNorPlane => "sync$nor$plane",
				Self::SyncOrArray => "sync$or$array",
				Self::SyncOrPlane => "sync$or$plane",
				Self::System => "system",              // Added: IEEE 1800-2009
				Self::TableModel => "table_model",     // Added: Verilog-AMS 2.3.1
				Self::Tan => "tan",                    // Added: IEEE 1364-2005
				Self::Tanh => "tanh",                  // Added: IEEE 1364-2005
				Self::Temperature => "temperature",    // Added: Verilog-AMS 2.3.1
				Self::TestPlusArgs => "test$plusargs", // Added: IEEE 1364-2001
				Self::Time => "time",
				Self::TimeFormat => "timeformat",
				Self::TimePrecision => "timeprecision", // Added: IEEE 1800-2023
				Self::TimeUnit => "timeunit",           // Added: IEEE 1800-2023
				Self::TypeName => "typename",           // Added: IEEE 1800-2005
				Self::Ungetc => "ungetc",               // Added: IEEE 1364-2001
				Self::UnpackedDimensions => "unpacked_dimensions", // Added: IEEE 1800-2005
				Self::Unsigned => "unsigned",           // Added: IEEE 1364-2001
				Self::Urandom => "urandom",             // Added: IEEE 1800-2005
				Self::UrandomRange => "urandom_range",  // Added: IEEE 1800-2005
				Self::ValuePlusArgs => "value$plusargs", // Added: IEEE 1364-2001
				Self::Vflip => "vflip",                 // Added: Verilog-AMS 2.3.1
				Self::Vt => "vt",                       // Added: Verilog-AMS 2.3.1
				Self::Warning => "warning",             // Added: IEEE 1800-2005 & Verilog-AMS
				// 2.3.1
				Self::Width => "width", // Removed: IEEE 1364-2001
				Self::Write => "write",
				Self::WriteB => "writeb",
				Self::WriteH => "writeh",
				Self::WritememB => "writememb", // Added: IEEE 1800-2009
				Self::WritememH => "writememh", // Added: IEEE 1800-2009
				Self::WriteO => "writeo",
				Self::XPosition => "xposition", // Added: Verilog-AMS 2.3.1
				Self::YPosition => "yposition", // Added: Verilog-AMS 2.3.1
			}
		)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sysfunc_verilog_95() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_verilog_95_builtin_sysfunc("nya"));
		assert_eq!(is_builtin_sysfunc("nya"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_verilog_95_builtin_sysfunc("async$and$array"));
		assert!(is_builtin_sysfunc("async$and$array").contains(LanguageStd::Vl95));

		// A system function/task that is at one version up
		assert!(!is_verilog_95_builtin_sysfunc("dumpports"));
		assert!(is_builtin_sysfunc("dumpports").contains(LanguageStd::Vl01));

		// Two versions up
		assert!(!is_verilog_95_builtin_sysfunc("acos"));
		assert!(is_builtin_sysfunc("acos").contains(LanguageStd::Vl05));

		// Three versions up
		assert!(!is_verilog_95_builtin_sysfunc("assertkill"));
		assert!(is_builtin_sysfunc("assertkill").contains(LanguageStd::Sv05));

		// Four versions up
		assert!(!is_verilog_95_builtin_sysfunc("assertfailoff"));
		assert!(is_builtin_sysfunc("assertfailoff").contains(LanguageStd::Sv09));

		// Five versions up
		assert!(!is_verilog_95_builtin_sysfunc("assertcontrol"));
		assert!(is_builtin_sysfunc("assertcontrol").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_verilog_01() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_verilog_01_builtin_sysfunc("meow"));
		assert_eq!(is_builtin_sysfunc("meow"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_verilog_01_builtin_sysfunc("ferror"));
		assert!(is_builtin_sysfunc("ferror").contains(LanguageStd::Vl01));

		// A system function/task that is at one version up
		assert!(!is_verilog_01_builtin_sysfunc("feof"));
		assert!(is_builtin_sysfunc("feof").contains(LanguageStd::Vl05));

		// Two versions up
		assert!(!is_verilog_01_builtin_sysfunc("error"));
		assert!(is_builtin_sysfunc("error").contains(LanguageStd::Sv05));

		// Three versions up
		assert!(!is_verilog_01_builtin_sysfunc("falling_gclk"));
		assert!(is_builtin_sysfunc("falling_gclk").contains(LanguageStd::Sv09));

		// Four versions up
		assert!(!is_verilog_01_builtin_sysfunc("countbits"));
		assert!(is_builtin_sysfunc("countbits").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_verilog_05() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_verilog_05_builtin_sysfunc("awoo"));
		assert_eq!(is_builtin_sysfunc("awoo"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_verilog_05_builtin_sysfunc("ceil"));
		assert!(is_builtin_sysfunc("ceil").contains(LanguageStd::Vl05));

		// One version up
		assert!(!is_verilog_05_builtin_sysfunc("bitstoshortreal"));
		assert!(is_builtin_sysfunc("bitstoshortreal").contains(LanguageStd::Sv05));

		// Two versions up
		assert!(!is_verilog_05_builtin_sysfunc("changed"));
		assert!(is_builtin_sysfunc("changed").contains(LanguageStd::Sv09));

		// Three versions up
		assert!(!is_verilog_05_builtin_sysfunc("assertcontrol"));
		assert!(is_builtin_sysfunc("assertcontrol").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_system_verilog_05() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_system_verilog_05_builtin_sysfunc("kon"));
		assert_eq!(is_builtin_sysfunc("kon"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_system_verilog_05_builtin_sysfunc("assertoff"));
		assert!(is_builtin_sysfunc("assertoff").contains(LanguageStd::Sv05));

		// One up
		assert!(!is_system_verilog_05_builtin_sysfunc("assertfailon"));
		assert!(is_builtin_sysfunc("assertfailon").contains(LanguageStd::Sv09));

		// Two versions up
		assert!(!is_system_verilog_05_builtin_sysfunc("countbits"));
		assert!(is_builtin_sysfunc("countbits").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_system_verilog_09() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_system_verilog_09_builtin_sysfunc("wah"));
		assert_eq!(is_builtin_sysfunc("wah"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_system_verilog_09_builtin_sysfunc("assertnonvacuouson"));
		assert!(is_builtin_sysfunc("assertnonvacuouson").contains(LanguageStd::Sv09));

		// One version up
		assert!(!is_system_verilog_09_builtin_sysfunc("assertcontrol"));
		assert!(is_builtin_sysfunc("assertcontrol").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_system_verilog_12() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_system_verilog_12_builtin_sysfunc("meow"));
		assert_eq!(is_builtin_sysfunc("meow"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_system_verilog_12_builtin_sysfunc("countbits"));
		assert!(is_builtin_sysfunc("countbits").contains(LanguageStd::Sv12));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_system_verilog_17() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_system_verilog_17_builtin_sysfunc("nya"));
		assert_eq!(is_builtin_sysfunc("nya"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_system_verilog_17_builtin_sysfunc("assertcontrol"));
		assert!(is_builtin_sysfunc("assertcontrol").contains(LanguageStd::Sv17));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_system_verilog_23() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_system_verilog_23_builtin_sysfunc("kon"));
		assert_eq!(is_builtin_sysfunc("kon"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_system_verilog_23_builtin_sysfunc("countbits"));
		assert!(is_builtin_sysfunc("countbits").contains(LanguageStd::Sv23));

		// SystemVerilog 2012, 2017, and 2023 all have the same system functions
	}

	#[test]
	fn test_sysfunc_verilog_ams_09() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_verilog_ams_09_builtin_sysfunc("awoo"));
		assert_eq!(is_builtin_sysfunc("awoo"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_verilog_ams_09_builtin_sysfunc("simparam"));
		assert!(is_builtin_sysfunc("simparam").contains(LanguageStd::Vams09));

		// One version up
		assert!(!is_verilog_ams_09_builtin_sysfunc("analog_node_alias"));
		assert!(is_builtin_sysfunc("analog_node_alias").contains(LanguageStd::Vams14));

		// Two versions up
		assert!(!is_verilog_ams_09_builtin_sysfunc("receiver_count"));
		assert!(is_builtin_sysfunc("receiver_count").contains(LanguageStd::Vams23));
	}

	#[test]
	fn test_sysfunc_verilog_ams_14() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_verilog_ams_14_builtin_sysfunc("wah"));
		assert_eq!(is_builtin_sysfunc("wah"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_verilog_ams_14_builtin_sysfunc("analog_port_alias"));
		assert!(is_builtin_sysfunc("analog_port_alias").contains(LanguageStd::Vams14));

		// One version up
		assert!(!is_verilog_ams_14_builtin_sysfunc("receiver_count"));
		assert!(is_builtin_sysfunc("receiver_count").contains(LanguageStd::Vams23));
	}

	#[test]
	fn test_sysfunc_verilog_ams_23() {
		// If it's an unknown system func we shouldn't get anything back
		assert!(!is_verilog_ams_23_builtin_sysfunc("meow"));
		assert_eq!(is_builtin_sysfunc("meow"), LanguageStd::none());

		// A system function/task that is at least in the current version
		assert!(is_verilog_ams_23_builtin_sysfunc("receiver_count"));
		assert!(is_builtin_sysfunc("receiver_count").contains(LanguageStd::Vams23));
	}
}
