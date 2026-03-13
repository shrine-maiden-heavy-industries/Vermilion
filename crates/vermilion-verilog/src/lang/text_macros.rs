// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use phf::{phf_map, phf_set};

use crate::LanguageStd;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BuiltinTextMacro {
	DunderFile,          // Added: Verilog-AMS 2023 & IEEE 1800-2009
	DunderLine,          // Added: Verilog-AMS 2023 & IEEE 1800-2009
	VamsCompactModeling, // Added: Verilog-AMS 2.3.1
	VamsEnable,          // Added: Verilog-AMS 2.3.1
}

/// IEEE 1364-1995 (Verilog 1995) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static VERILOG_95_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> =
	phf_map! {};

/// IEEE 1364-1995 (Verilog 1995) Builtin text macro set
pub static VERILOG_95_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {};

// NOTE(aki): Identical to `VERILOG_95_BUILTIN_TEXT_MACRO_MAP`
/// IEEE 1364-2001 (Verilog 2001) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static VERILOG_01_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> =
	phf_map! {};

// NOTE(aki): Identical to `VERILOG_95_BUILTIN_TEXT_MACRO_SET`
/// IEEE 1364-2001 (Verilog 2001) Builtin text macro set
pub static VERILOG_01_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {};

// NOTE(aki): Identical to `VERILOG_95_BUILTIN_TEXT_MACRO_MAP`
/// IEEE 1364-2005 (Verilog 2005) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static VERILOG_05_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> =
	phf_map! {};

// NOTE(aki): Identical to `VERILOG_95_BUILTIN_TEXT_MACRO_SET`
/// IEEE 1364-2005 (Verilog 2005) Builtin text macro set
pub static VERILOG_05_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {};

/// IEEE 1800-2005 (SystemVerilog 2005) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static SYSTEM_VERILOG_05_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> =
	phf_map! {};

/// IEEE 1800-2005 (SystemVerilog 2005) Builtin text macro set
pub static SYSTEM_VERILOG_05_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {};

/// IEEE 1800-2009 (SystemVerilog 2009) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> = phf_map! {
	"__FILE__" => BuiltinTextMacro::DunderFile,
	"__LINE__" => BuiltinTextMacro::DunderLine,
};

/// IEEE 1800-2009 (SystemVerilog 2009) Builtin text macro set
pub static SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {
	"__FILE__", "__LINE__"
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_MAP`
/// IEEE 1800-2012 (SystemVerilog 2012) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static SYSTEM_VERILOG_12_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> = phf_map! {
	"__FILE__" => BuiltinTextMacro::DunderFile,
	"__LINE__" => BuiltinTextMacro::DunderLine,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_SET`
/// IEEE 1800-2012 (SystemVerilog 2012) Builtin text macro set
pub static SYSTEM_VERILOG_12_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {
	"__FILE__", "__LINE__"
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_MAP`
/// IEEE 1800-2017 (SystemVerilog 2017) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static SYSTEM_VERILOG_17_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> = phf_map! {
	"__FILE__" => BuiltinTextMacro::DunderFile,
	"__LINE__" => BuiltinTextMacro::DunderLine,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_SET`
/// IEEE 1800-2017 (SystemVerilog 2017) Builtin text macro set
pub static SYSTEM_VERILOG_17_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {
	"__FILE__", "__LINE__"
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_MAP`
/// IEEE 1800-2023 (SystemVerilog 2023) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static SYSTEM_VERILOG_23_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> = phf_map! {
	"__FILE__" => BuiltinTextMacro::DunderFile,
	"__LINE__" => BuiltinTextMacro::DunderLine,
};

// NOTE(aki): Identical to `SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_SET`
/// IEEE 1800-2023 (SystemVerilog 2023) Builtin text macro set
pub static SYSTEM_VERILOG_23_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {
	"__FILE__", "__LINE__"
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static VERILOG_AMS_09_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> = phf_map! {
	"__VAMS_COMPACT_MODELING__" => BuiltinTextMacro::VamsCompactModeling,
	"__VAMS_ENABLE__" => BuiltinTextMacro::VamsEnable,
};

/// Verilog-AMS 2.3.1 (Verilog-AMS 2009) Builtin text macro set
pub static VERILOG_AMS_09_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {
	"__VAMS_COMPACT_MODELING__", "__VAMS_ENABLE__"
};

// NOTE(aki): Identical to `VERILOG_AMS_09_BUILTIN_TEXT_MACRO_MAP`
/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static VERILOG_AMS_14_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> = phf_map! {
	"__VAMS_COMPACT_MODELING__" => BuiltinTextMacro::VamsCompactModeling,
	"__VAMS_ENABLE__" => BuiltinTextMacro::VamsEnable,
};

// NOTE(aki): Identical to `VERILOG_AMS_09_BUILTIN_TEXT_MACRO_SET`
/// Verilog-AMS 2.4.0 (Verilog-AMS 2014) Builtin text macro set
pub static VERILOG_AMS_14_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {
	"__VAMS_COMPACT_MODELING__", "__VAMS_ENABLE__"
};

/// Verilog-AMS 2023 Builtin Text Macro to [`BuiltinTextMacro`] token map
pub static VERILOG_AMS_23_BUILTIN_TEXT_MACRO_MAP: phf::Map<&'static str, BuiltinTextMacro> = phf_map! {
	"__FILE__" => BuiltinTextMacro::DunderFile,
	"__LINE__" => BuiltinTextMacro::DunderLine,
	"__VAMS_COMPACT_MODELING__" => BuiltinTextMacro::VamsCompactModeling,
	"__VAMS_ENABLE__" => BuiltinTextMacro::VamsEnable,
};

/// Verilog-AMS 2023 Builtin text macro set
pub static VERILOG_AMS_23_BUILTIN_TEXT_MACRO_SET: phf::Set<&'static str> = phf_set! {
	"__FILE__", "__LINE__", "__VAMS_COMPACT_MODELING__", "__VAMS_ENABLE__"
};

/// Get the Verilog 1995 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_95_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	VERILOG_95_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 1995 builtin text macro
///
/// This is used rather than [`get_verilog_95_builtin_text_macro`] to test if the
/// found identifier for a lower Verilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_verilog_95_builtin_text_macro(ident: &str) -> bool {
	VERILOG_95_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the Verilog 2001 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_01_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	VERILOG_01_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2001 builtin text macro
///
/// This is used rather than [`get_verilog_01_builtin_text_macro`] to test if the
/// found identifier for a lower Verilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_verilog_01_builtin_text_macro(ident: &str) -> bool {
	VERILOG_01_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the Verilog 2005 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_05_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	VERILOG_05_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog 2005 builtin text macro
///
/// This is used rather than [`get_verilog_05_builtin_text_macro`] to test if the
/// found identifier for a lower Verilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_verilog_05_builtin_text_macro(ident: &str) -> bool {
	VERILOG_05_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the Verilog builtin text macro for the given standard if it exists
#[inline(always)]
pub fn get_verilog_builtin_text_macro(ident: &str, std: LanguageStd) -> Option<BuiltinTextMacro> {
	match std {
		LanguageStd::Vl95 => get_verilog_95_builtin_text_macro(ident),
		LanguageStd::Vl01 => get_verilog_01_builtin_text_macro(ident),
		LanguageStd::Vl05 => get_verilog_05_builtin_text_macro(ident),
		_ => None,
	}
}

/// Get the SystemVerilog 2005 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_05_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	SYSTEM_VERILOG_05_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2005 builtin text macro
///
/// This is used rather than [`get_system_verilog_05_builtin_text_macro`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_system_verilog_05_builtin_text_macro(ident: &str) -> bool {
	SYSTEM_VERILOG_05_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the SystemVerilog 2009 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_09_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2009 builtin text macro
///
/// This is used rather than [`get_system_verilog_09_builtin_text_macro`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_system_verilog_09_builtin_text_macro(ident: &str) -> bool {
	SYSTEM_VERILOG_09_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the SystemVerilog 2012 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_12_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	SYSTEM_VERILOG_12_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2012 builtin text macro
///
/// This is used rather than [`get_system_verilog_12_builtin_text_macro`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_system_verilog_12_builtin_text_macro(ident: &str) -> bool {
	SYSTEM_VERILOG_12_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the SystemVerilog 2017 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_17_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	SYSTEM_VERILOG_17_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2017 builtin text macro
///
/// This is used rather than [`get_system_verilog_17_builtin_text_macro`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_system_verilog_17_builtin_text_macro(ident: &str) -> bool {
	SYSTEM_VERILOG_17_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the SystemVerilog 2023 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_system_verilog_23_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	SYSTEM_VERILOG_23_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SystemVerilog 2023 builtin text macro
///
/// This is used rather than [`get_system_verilog_23_builtin_text_macro`] to test if the
/// found identifier for a lower SystemVerilog standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_system_verilog_23_builtin_text_macro(ident: &str) -> bool {
	SYSTEM_VERILOG_23_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the SystemVerilog builtin text macro for the given standard if it exists
#[inline(always)]
pub fn get_system_verilog_builtin_text_macro(
	ident: &str,
	std: LanguageStd,
) -> Option<BuiltinTextMacro> {
	match std {
		LanguageStd::Sv05 => get_system_verilog_05_builtin_text_macro(ident),
		LanguageStd::Sv09 => get_system_verilog_09_builtin_text_macro(ident),
		LanguageStd::Sv12 => get_system_verilog_12_builtin_text_macro(ident),
		LanguageStd::Sv17 => get_system_verilog_17_builtin_text_macro(ident),
		LanguageStd::Sv23 => get_system_verilog_23_builtin_text_macro(ident),
		_ => None,
	}
}

/// Get the Verilog-AMS 2009 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_ams_09_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	VERILOG_AMS_09_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2009 builtin text macro
///
/// This is used rather than [`get_verilog_ams_09_builtin_text_macro`] to test if the
/// found identifier for a lower Verilog-AMS standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_verilog_ams_09_builtin_text_macro(ident: &str) -> bool {
	VERILOG_AMS_09_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the Verilog-AMS 2014 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_ams_14_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	VERILOG_AMS_14_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2014 builtin text macro
///
/// This is used rather than [`get_verilog_ams_14_builtin_text_macro`] to test if the
/// found identifier for a lower Verilog-AMS standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_verilog_ams_14_builtin_text_macro(ident: &str) -> bool {
	VERILOG_AMS_14_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the Verilog-AMS 2023 builtin text macro for the given identifier if it exists
#[inline(always)]
pub fn get_verilog_ams_23_builtin_text_macro(ident: &str) -> Option<BuiltinTextMacro> {
	VERILOG_AMS_23_BUILTIN_TEXT_MACRO_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a Verilog-AMS 2023 builtin text macro
///
/// This is used rather than [`get_verilog_ams_23_builtin_text_macro`] to test if the
/// found identifier for a lower Verilog-AMS standard is a builtin text macro in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a builtin text macro.
#[inline(always)]
pub fn is_verilog_ams_23_builtin_text_macro(ident: &str) -> bool {
	VERILOG_AMS_23_BUILTIN_TEXT_MACRO_SET.contains(ident)
}

/// Get the Verilog-AMS text macro for the given standard if it exists
#[inline(always)]
pub fn get_verilog_ams_builtin_text_macro(
	ident: &str,
	std: LanguageStd,
) -> Option<BuiltinTextMacro> {
	match std {
		LanguageStd::Vams09 => get_verilog_ams_09_builtin_text_macro(ident),
		LanguageStd::Vams14 => get_verilog_ams_14_builtin_text_macro(ident),
		LanguageStd::Vams23 => get_verilog_ams_23_builtin_text_macro(ident),
		_ => None,
	}
}

/// Get the Verilog/Verilog-AMS/SystemVerilog builtin text macro for the given standard if it exists
#[inline(always)]
pub fn get_builtin_text_macro(ident: &str, std: LanguageStd) -> Option<BuiltinTextMacro> {
	match std {
		LanguageStd::Vl95 => get_verilog_95_builtin_text_macro(ident),
		LanguageStd::Vl01 => get_verilog_01_builtin_text_macro(ident),
		LanguageStd::Vl05 => get_verilog_05_builtin_text_macro(ident),
		LanguageStd::Sv05 => get_system_verilog_05_builtin_text_macro(ident),
		LanguageStd::Sv09 => get_system_verilog_09_builtin_text_macro(ident),
		LanguageStd::Sv12 => get_system_verilog_12_builtin_text_macro(ident),
		LanguageStd::Sv17 => get_system_verilog_17_builtin_text_macro(ident),
		LanguageStd::Sv23 => get_system_verilog_23_builtin_text_macro(ident),
		LanguageStd::Vams09 => get_verilog_ams_09_builtin_text_macro(ident),
		LanguageStd::Vams14 => get_verilog_ams_14_builtin_text_macro(ident),
		LanguageStd::Vams23 => get_verilog_ams_23_builtin_text_macro(ident),
		_ => None,
	}
}

/// Check to see if the given identifier is a builtin text macro in a future Verilog standard,
/// returning the standard version if so.
#[inline(always)]
pub fn is_builtin_text_macro(ident: &str) -> LanguageStd {
	let mut supported = LanguageStd::none();

	if is_verilog_95_builtin_text_macro(ident) {
		supported |= LanguageStd::Vl95;
	}

	if is_verilog_01_builtin_text_macro(ident) {
		supported |= LanguageStd::Vl01;
	}

	if is_verilog_05_builtin_text_macro(ident) {
		supported |= LanguageStd::Vl05;
	}

	if is_system_verilog_05_builtin_text_macro(ident) {
		supported |= LanguageStd::Sv05;
	}

	if is_system_verilog_09_builtin_text_macro(ident) {
		supported |= LanguageStd::Sv09;
	}

	if is_system_verilog_12_builtin_text_macro(ident) {
		supported |= LanguageStd::Sv12;
	}

	if is_system_verilog_17_builtin_text_macro(ident) {
		supported |= LanguageStd::Sv17;
	}

	if is_system_verilog_23_builtin_text_macro(ident) {
		supported |= LanguageStd::Sv23;
	}

	if is_verilog_ams_09_builtin_text_macro(ident) {
		supported |= LanguageStd::Vams09;
	}

	if is_verilog_ams_14_builtin_text_macro(ident) {
		supported |= LanguageStd::Vams14;
	}

	if is_verilog_ams_23_builtin_text_macro(ident) {
		supported |= LanguageStd::Vams23;
	}

	supported
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for BuiltinTextMacro {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"BuiltinTextMacro({})",
			match self {
				Self::DunderFile => "__FILE__", // Added: Verilog-AMS 2023 & IEEE 1800-2009
				Self::DunderLine => "__LINE__", // Added: Verilog-AMS 2023 & IEEE 1800-2009
				Self::VamsCompactModeling => "__VAMS_COMPACT_MODELING__", // Added: Verilog-AMS 2.3.1
				Self::VamsEnable => "__VAMS_ENABLE__", // Added: Verilog-AMS 2.3.1
			}
		)
	}
}

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
