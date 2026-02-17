// SPDX-License-Identifier: BSD-3-Clause

use phf::{phf_map, phf_set};

use crate::{
	SystemVerilogStd, VerilogAmsStd, VerilogStd, VerilogVariant, lang::tokenizer::token::Directive,
};

/// IEEE 1364-1995 (Verilog 1995) Directive to [`Directive`] token map
pub static VERILOG_95_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"ifdef" => Directive::IfDef,
	"include" => Directive::Include,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
};

/// IEEE 1364-1995 (Verilog 1995) Directive set
pub static VERILOG_95_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"celldefine",              "delay_mode_unit", "nounconnected_drive",
	"default_decay_time",      "delay_mode_zero", "resetall",
	"default_nettype",         "else",            "timescale",
	"default_trireg_strength", "endcelldefine",   "unconnected_drive",
	"define",                  "endif",           "undef",
	"delay_mode_distributed",  "ifdef",
	"delay_mode_path",         "include",

};

/// IEEE 1364-2001 (Verilog 2001) Directive to [`Directive`] token map
pub static VERILOG_01_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
};

/// IEEE 1364-2001 (Verilog 2001) Directive set
pub static VERILOG_01_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"celldefine",              "delay_mode_zero", "line",
	"default_decay_time",      "else",            "nounconnected_drive",
	"default_nettype",         "elsif",           "resetall",
	"default_trireg_strength", "endcelldefine",   "timescale",
	"define",                  "endif",           "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
	"delay_mode_path",         "ifndef",
	"delay_mode_unit",         "include",
};

/// IEEE 1364-2005 (Verilog 2005) Directive to [`Directive`] token map
pub static VERILOG_05_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
};

/// IEEE 1364-2005 (Verilog 2005) Directive set
pub static VERILOG_05_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_zero", "line",
	"celldefine",              "else",            "nounconnected_drive",
	"default_decay_time",      "elsif",           "pragma",
	"default_nettype",         "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
	"delay_mode_path",         "ifndef",
	"delay_mode_unit",         "include",
};

// NOTE(aki): Identical to `VERILOG_05_DIRECTIVE_MAP`
/// IEEE 1800-2005 (SystemVerilog 2005) Directive to [`Directive`] token map
pub static SYSTEM_VERILOG_05_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
};

// NOTE(aki): Identical to `VERILOG_05_DIRECTIVE_SET`
/// IEEE 1800-2005 (SystemVerilog 2005) Directive set
pub static SYSTEM_VERILOG_05_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_zero", "line",
	"celldefine",              "else",            "nounconnected_drive",
	"default_decay_time",      "elsif",           "pragma",
	"default_nettype",         "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
	"delay_mode_path",         "ifndef",
	"delay_mode_unit",         "include",
};

/// IEEE 1800-2009 (SystemVerilog 2009) Directive to [`Directive`] token map
pub static SYSTEM_VERILOG_09_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
	"undefineall" => Directive::UndefineAll,
};

/// IEEE 1800-2009 (SystemVerilog 2009) Directive set
pub static SYSTEM_VERILOG_09_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_zero", "line",
	"celldefine",              "else",            "nounconnected_drive",
	"default_decay_time",      "elsif",           "pragma",
	"default_nettype",         "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
	"delay_mode_path",         "ifndef",          "undefineall",
	"delay_mode_unit",         "include",
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_DIRECTIVE_MAP`
/// IEEE 1800-2012 (SystemVerilog 2012) Directive to [`Directive`] token map
pub static SYSTEM_VERILOG_12_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
	"undefineall" => Directive::UndefineAll,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_DIRECTIVE_SET`
/// IEEE 1800-2012 (SystemVerilog 2012) Directive set
pub static SYSTEM_VERILOG_12_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_zero", "line",
	"celldefine",              "else",            "nounconnected_drive",
	"default_decay_time",      "elsif",           "pragma",
	"default_nettype",         "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
	"delay_mode_path",         "ifndef",          "undefineall",
	"delay_mode_unit",         "include",
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_DIRECTIVE_MAP`
/// IEEE 1800-2017 (SystemVerilog 2017) Directive to [`Directive`] token map
pub static SYSTEM_VERILOG_17_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
	"undefineall" => Directive::UndefineAll,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_DIRECTIVE_SET`
/// IEEE 1800-2017 (SystemVerilog 2017) Directive set
pub static SYSTEM_VERILOG_17_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_zero", "line",
	"celldefine",              "else",            "nounconnected_drive",
	"default_decay_time",      "elsif",           "pragma",
	"default_nettype",         "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
	"delay_mode_path",         "ifndef",          "undefineall",
	"delay_mode_unit",         "include",
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_DIRECTIVE_MAP`
/// IEEE 1800-2023 (SystemVerilog 2023) Directive to [`Directive`] token map
pub static SYSTEM_VERILOG_23_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_nettype" => Directive::DefaultNetType,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
	"undefineall" => Directive::UndefineAll,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_DIRECTIVE_SET`
/// IEEE 1800-2023 (SystemVerilog 2023) Directive set
pub static SYSTEM_VERILOG_23_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_zero", "line",
	"celldefine",              "else",            "nounconnected_drive",
	"default_decay_time",      "elsif",           "pragma",
	"default_nettype",         "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
	"delay_mode_path",         "ifndef",          "undefineall",
	"delay_mode_unit",         "include",
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Directive to [`Directive`] token map
pub static VERILOG_AMS_09_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_discipline" => Directive::DefaultDiscipline,
	"default_nettype" => Directive::DefaultNetType,
	"default_transition" => Directive::DefaultTransition,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Directive set
pub static VERILOG_AMS_09_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_path", "ifndef",
	"celldefine",              "delay_mode_unit", "include",
	"default_decay_time",      "delay_mode_zero", "line",
	"default_discipline",      "else",            "nounconnected_drive",
	"default_nettype",         "elsif",           "pragma",
	"default_transition",      "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
};

// NOTE(aki): Identical to `VERILOG_AMS_09_DIRECTIVE_MAP`
/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Directive to [`Directive`] token map
pub static VERILOG_AMS_14_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_discipline" => Directive::DefaultDiscipline,
	"default_nettype" => Directive::DefaultNetType,
	"default_transition" => Directive::DefaultTransition,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
};

// NOTE(aki): Identical to `VERILOG_AMS_09_DIRECTIVE_SET`
/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Directive set
pub static VERILOG_AMS_14_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_path", "ifndef",
	"celldefine",              "delay_mode_unit", "include",
	"default_decay_time",      "delay_mode_zero", "line",
	"default_discipline",      "else",            "nounconnected_drive",
	"default_nettype",         "elsif",           "pragma",
	"default_transition",      "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
};

// NOTE(aki): Identical to `VERILOG_AMS_09_DIRECTIVE_MAP`
/// Verilog-AMS 2023 Directive to [`Directive`] token map
pub static VERILOG_AMS_23_DIRECTIVE_MAP: phf::Map<&'static str, Directive> = phf_map! {
	"begin_keywords" => Directive::BeginKeywords,
	"celldefine" => Directive::CellDefine,
	"default_decay_time" => Directive::DefaultDecayTime,
	"default_discipline" => Directive::DefaultDiscipline,
	"default_nettype" => Directive::DefaultNetType,
	"default_transition" => Directive::DefaultTransition,
	"default_trireg_strength" => Directive::DefaultTriRegStrength,
	"define" => Directive::Define,
	"delay_mode_distributed" => Directive::DelayModeDistributed,
	"delay_mode_path" => Directive::DelayModePath,
	"delay_mode_unit" => Directive::DelayModeUnit,
	"delay_mode_zero" => Directive::DelayModeZero,
	"else" => Directive::Else,
	"elsif" => Directive::ElsIf,
	"endcelldefine" => Directive::EndCellDefine,
	"endif" => Directive::EndIf,
	"end_keywords" => Directive::EndKeywords,
	"ifdef" => Directive::IfDef,
	"ifndef" => Directive::IfNotDef,
	"include" => Directive::Include,
	"line" => Directive::Line,
	"nounconnected_drive" => Directive::NoUnconnectedDrive,
	"pragma" => Directive::Pragma,
	"resetall" => Directive::ResetAll,
	"timescale" => Directive::TimeScale,
	"unconnected_drive" => Directive::UnconnectedDrive,
	"undef" => Directive::Undef,
};

// NOTE(aki): Identical to `VERILOG_AMS_09_DIRECTIVE_SET`s
/// Verilog-AMS 2023 Directive set
pub static VERILOG_AMS_23_DIRECTIVE_SET: phf::Set<&'static str> = phf_set! {
	"begin_keywords",          "delay_mode_path", "ifndef",
	"celldefine",              "delay_mode_unit", "include",
	"default_decay_time",      "delay_mode_zero", "line",
	"default_discipline",      "else",            "nounconnected_drive",
	"default_nettype",         "elsif",           "pragma",
	"default_transition",      "endcelldefine",   "resetall",
	"default_trireg_strength", "endif",           "timescale",
	"define",                  "end_keywords",    "unconnected_drive",
	"delay_mode_distributed",  "ifdef",           "undef",
};

/// Get the Verilog 1995 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_95_directive(ident: &str) -> Option<Directive> {
	VERILOG_95_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 1995 directive
///
/// This is used rather than [`get_verilog_95_directive`] to test if the
/// found identifier for a lower Verilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_verilog_95_directive(ident: &str) -> bool {
	VERILOG_95_DIRECTIVE_SET.contains(ident)
}

/// Get the Verilog 2001 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_01_directive(ident: &str) -> Option<Directive> {
	VERILOG_01_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2001 directive
///
/// This is used rather than [`get_verilog_01_directive`] to test if the
/// found identifier for a lower Verilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_verilog_01_directive(ident: &str) -> bool {
	VERILOG_01_DIRECTIVE_SET.contains(ident)
}

/// Get the Verilog 2005 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_05_directive(ident: &str) -> Option<Directive> {
	VERILOG_05_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2005 directive
///
/// This is used rather than [`get_verilog_05_directive`] to test if the
/// found identifier for a lower Verilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_verilog_05_directive(ident: &str) -> bool {
	VERILOG_05_DIRECTIVE_SET.contains(ident)
}

/// Get the Verilog directive for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_directive(ident: &str, std: VerilogStd) -> Option<Directive> {
	match std {
		VerilogStd::Vl95 => get_verilog_95_directive(ident),
		VerilogStd::Vl01 => get_verilog_01_directive(ident),
		VerilogStd::Vl05 => get_verilog_05_directive(ident),
	}
}

/// Get the SystemVerilog 2005 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_05_directive(ident: &str) -> Option<Directive> {
	SYSTEM_VERILOG_05_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2005 directive
///
/// This is used rather than [`get_system_verilog_05_directive`] to test if the
/// found identifier for a lower SystemVerilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_05_directive(ident: &str) -> bool {
	SYSTEM_VERILOG_05_DIRECTIVE_SET.contains(ident)
}

/// Get the SystemVerilog 2009 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_09_directive(ident: &str) -> Option<Directive> {
	SYSTEM_VERILOG_09_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2009 directive
///
/// This is used rather than [`get_system_verilog_09_directive`] to test if the
/// found identifier for a lower SystemVerilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_09_directive(ident: &str) -> bool {
	SYSTEM_VERILOG_09_DIRECTIVE_SET.contains(ident)
}

/// Get the SystemVerilog 2012 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_12_directive(ident: &str) -> Option<Directive> {
	SYSTEM_VERILOG_12_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2012 directive
///
/// This is used rather than [`get_system_verilog_12_directive`] to test if the
/// found identifier for a lower SystemVerilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_12_directive(ident: &str) -> bool {
	SYSTEM_VERILOG_12_DIRECTIVE_SET.contains(ident)
}

/// Get the SystemVerilog 2017 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_17_directive(ident: &str) -> Option<Directive> {
	SYSTEM_VERILOG_17_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2017 directive
///
/// This is used rather than [`get_system_verilog_17_directive`] to test if the
/// found identifier for a lower SystemVerilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_17_directive(ident: &str) -> bool {
	SYSTEM_VERILOG_17_DIRECTIVE_SET.contains(ident)
}

/// Get the SystemVerilog 2023 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_23_directive(ident: &str) -> Option<Directive> {
	SYSTEM_VERILOG_23_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2023 directive
///
/// This is used rather than [`get_system_verilog_23_directive`] to test if the
/// found identifier for a lower SystemVerilog standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_system_verilog_23_directive(ident: &str) -> bool {
	SYSTEM_VERILOG_23_DIRECTIVE_SET.contains(ident)
}

/// Get the SystemVerilog directive for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_system_verilog_directive(ident: &str, std: SystemVerilogStd) -> Option<Directive> {
	match std {
		SystemVerilogStd::Sv05 => get_system_verilog_05_directive(ident),
		SystemVerilogStd::Sv09 => get_system_verilog_09_directive(ident),
		SystemVerilogStd::Sv12 => get_system_verilog_12_directive(ident),
		SystemVerilogStd::Sv17 => get_system_verilog_17_directive(ident),
		SystemVerilogStd::Sv23 => get_system_verilog_23_directive(ident),
	}
}

/// Get the Verilog-AMS 2009 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_09_directive(ident: &str) -> Option<Directive> {
	VERILOG_AMS_09_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2009 directive
///
/// This is used rather than [`get_verilog_ams_09_directive`] to test if the
/// found identifier for a lower Verilog-AMS standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_verilog_ams_09_directive(ident: &str) -> bool {
	VERILOG_AMS_09_DIRECTIVE_SET.contains(ident)
}

/// Get the Verilog-AMS 2014 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_14_directive(ident: &str) -> Option<Directive> {
	VERILOG_AMS_14_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2014 directive
///
/// This is used rather than [`get_verilog_ams_14_directive`] to test if the
/// found identifier for a lower Verilog-AMS standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_verilog_ams_14_directive(ident: &str) -> bool {
	VERILOG_AMS_14_DIRECTIVE_SET.contains(ident)
}

/// Get the Verilog-AMS 2023 directive for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_23_directive(ident: &str) -> Option<Directive> {
	VERILOG_AMS_23_DIRECTIVE_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2023 directive
///
/// This is used rather than [`get_verilog_ams_23_directive`] to test if the
/// found identifier for a lower Verilog-AMS standard is a directive in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a directive.
#[allow(unused)]
#[inline]
pub fn is_verilog_ams_23_directive(ident: &str) -> bool {
	VERILOG_AMS_23_DIRECTIVE_SET.contains(ident)
}

/// Get the Verilog-AMS directive for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_verilog_ams_directive(ident: &str, std: VerilogAmsStd) -> Option<Directive> {
	match std {
		VerilogAmsStd::Vams09 => get_verilog_ams_09_directive(ident),
		VerilogAmsStd::Vams14 => get_verilog_ams_14_directive(ident),
		VerilogAmsStd::Vams23 => get_verilog_ams_23_directive(ident),
	}
}

/// Get the Verilog/Verilog-AMS/SystemVerilog directive for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_directive(ident: &str, variant: VerilogVariant) -> Option<Directive> {
	match variant {
		VerilogVariant::Verilog(std) => get_verilog_directive(ident, std),
		VerilogVariant::SystemVerilog(std) => get_system_verilog_directive(ident, std),
		VerilogVariant::VerilogAms(std) => get_verilog_ams_directive(ident, std),
	}
}

/// Check to see if the given identifier is a directive in a future Verilog standard, returning
/// the standard version if so.
#[allow(unused)]
#[inline]
pub fn is_future_verilog_directive(ident: &str, variant: VerilogVariant) -> Option<VerilogVariant> {
	match variant {
		VerilogVariant::Verilog(std) => match std {
			VerilogStd::Vl95 => {
				if is_verilog_95_directive(ident) {
					Some(VerilogVariant::Verilog(VerilogStd::Vl95))
				} else if is_verilog_01_directive(ident) {
					Some(VerilogVariant::Verilog(VerilogStd::Vl01))
				} else if is_verilog_05_directive(ident) {
					Some(VerilogVariant::Verilog(VerilogStd::Vl05))
				} else if is_system_verilog_05_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
				} else if is_system_verilog_09_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
				} else if is_system_verilog_12_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
				} else if is_system_verilog_17_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
				} else if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
			VerilogStd::Vl01 => {
				if is_verilog_01_directive(ident) {
					Some(VerilogVariant::Verilog(VerilogStd::Vl01))
				} else if is_verilog_05_directive(ident) {
					Some(VerilogVariant::Verilog(VerilogStd::Vl05))
				} else if is_system_verilog_05_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
				} else if is_system_verilog_09_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
				} else if is_system_verilog_12_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
				} else if is_system_verilog_17_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
				} else if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
			VerilogStd::Vl05 => {
				if is_verilog_05_directive(ident) {
					Some(VerilogVariant::Verilog(VerilogStd::Vl05))
				} else if is_system_verilog_05_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
				} else if is_system_verilog_09_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
				} else if is_system_verilog_12_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
				} else if is_system_verilog_17_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
				} else if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
		},
		VerilogVariant::SystemVerilog(std) => match std {
			SystemVerilogStd::Sv05 => {
				if is_system_verilog_05_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
				} else if is_system_verilog_09_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
				} else if is_system_verilog_12_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
				} else if is_system_verilog_17_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
				} else if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
			SystemVerilogStd::Sv09 => {
				if is_system_verilog_09_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
				} else if is_system_verilog_12_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
				} else if is_system_verilog_17_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
				} else if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
			SystemVerilogStd::Sv12 => {
				if is_system_verilog_12_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
				} else if is_system_verilog_17_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
				} else if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
			SystemVerilogStd::Sv17 => {
				if is_system_verilog_17_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
				} else if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
			SystemVerilogStd::Sv23 => {
				if is_system_verilog_23_directive(ident) {
					Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
				} else {
					None
				}
			},
		},
		VerilogVariant::VerilogAms(std) => match std {
			VerilogAmsStd::Vams09 => {
				if is_verilog_ams_09_directive(ident) {
					Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams09))
				} else if is_verilog_ams_14_directive(ident) {
					Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams14))
				} else if is_verilog_ams_23_directive(ident) {
					Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams23))
				} else {
					None
				}
			},
			VerilogAmsStd::Vams14 => {
				if is_verilog_ams_14_directive(ident) {
					Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams14))
				} else if is_verilog_ams_23_directive(ident) {
					Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams23))
				} else {
					None
				}
			},
			VerilogAmsStd::Vams23 => {
				if is_verilog_ams_23_directive(ident) {
					Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams23))
				} else {
					None
				}
			},
		},
	}
}
