// SPDX-License-Identifier: BSD-3-Clause

use std::{
	error::Error,
	fmt::{Debug, Display},
	str::FromStr,
};

const SHIFT_CATEGORY: u32 = 28;
const SHIFT_LANG_TYPE: u32 = 24;
const SHIFT_LANG_VALUE: u32 = 20;
const SHIFT_RESERVED: u32 = 14;
const MASK_CATEGORY: u32 = 0xF << SHIFT_CATEGORY;
const MASK_LANG_TYPE: u32 = 0xF << SHIFT_LANG_TYPE;
const MASK_LANG_VALUE: u32 = 0xF << SHIFT_LANG_VALUE;
const MASK_VALUE: u32 = 0x3FFF;
const MASK_RESERVED: u32 = 0x3F << SHIFT_RESERVED;

const CAT_INFO: u32 = 0x0 << SHIFT_CATEGORY;
const CAT_LINT: u32 = 0x1 << SHIFT_CATEGORY;
const CAT_WARN: u32 = 0x2 << SHIFT_CATEGORY;
const CAT_ERROR: u32 = 0x3 << SHIFT_CATEGORY;
const CAT_DEBUG: u32 = 0x4 << SHIFT_CATEGORY;

const PREFIX_GENERIC: &str = "VIR";
const LANG_TYPE_GENERIC: u32 = 0x0 << SHIFT_LANG_TYPE;
const LANG_TYPE_HDL: u32 = 0x1 << SHIFT_LANG_TYPE;
const LANG_TYPE_CONSTRAINT: u32 = 0x2 << SHIFT_LANG_TYPE;
const PREFIX_IP_XACT: &str = "IP";
const LANG_TYPE_LIBERTY: u32 = 0x3 << SHIFT_LANG_TYPE;
const PREFIX_LIBERTY: &str = "LIB";
const LANG_TYPE_IP_XACT: u32 = 0x4 << SHIFT_LANG_TYPE;

const PREFIX_HDL_GENERIC: &str = "HDL";
const PREFIX_HDL_SYSTEM_VERILOG: &str = "SV";
const PREFIX_HDL_VERILOG: &str = "V";
const PREFIX_HDL_VERILOG_AMS: &str = "VA";
const PREFIX_HDL_VHDL: &str = "VH";
const PREFIX_HDL_VHDL_AMS: &str = "VHA";
const LANG_VAL_HDL_GENERIC: u32 = 0x0 << SHIFT_LANG_VALUE;
const LANG_VAL_HDL_VERILOG: u32 = 0x1 << SHIFT_LANG_VALUE;
const LANG_VAL_HDL_SYSTEM_VERILOG: u32 = 0x2 << SHIFT_LANG_VALUE;
const LANG_VAL_HDL_VERILOG_AMS: u32 = 0x3 << SHIFT_LANG_VALUE;
const LANG_VAL_HDL_VHDL: u32 = 0x4 << SHIFT_LANG_VALUE;
const LANG_VAL_HDL_VHDL_AMS: u32 = 0x5 << SHIFT_LANG_VALUE;

const PREFIX_CONSTRAINT_GENERIC: &str = "CON";
const PREFIX_CONSTRAINT_LPF: &str = "LPF";
const PREFIX_CONSTRAINT_PDC: &str = "PDC";
const PREFIX_CONSTRAINT_SDC: &str = "SDC";
const PREFIX_CONSTRAINT_XDC: &str = "XDC";
const LANG_VAL_CONSTRAINT_GENERIC: u32 = 0x0 << SHIFT_LANG_VALUE;
const LANG_VAL_CONSTRAINT_SDC: u32 = 0x1 << SHIFT_LANG_VALUE;
const LANG_VAL_CONSTRAINT_XDC: u32 = 0x2 << SHIFT_LANG_VALUE;
const LANG_VAL_CONSTRAINT_PDC: u32 = 0x3 << SHIFT_LANG_VALUE;
const LANG_VAL_CONSTRAINT_LPF: u32 = 0x4 << SHIFT_LANG_VALUE;

const MAX_CODE_VALUE: u32 = 9999;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticCodeError {
	IdOverflow(u16),
	InvalidCategory(u8),
	InvalidCodeFormat,
	InvalidLanguageType(u8),
	InvalidLanguageValue(u8, u8),
	ReservedValueSet,
}

/// Vermilion Diagnostic Code
///
///
/// ## Format
///
/// The in-memory format of the diagnostic code is as follows:
///
/// ```text
///  3       2      2      2        1            0
///  2       8      4      0        4            0
/// ╭┴─────┬─┴────┬─┴────┬─┴──────┬─┴────────────┴─╮
/// │ CCCC │ TTTT │ LLLL │ XXXXXX │ NNNNNNNNNNNNNN │
/// ╰─┬────┴─┬────┴─┬────┴─┬──────┴──────────────┬─╯
///   │      │      │      │                     ╰── Numeric ID Part
///   │      │      │      ╰──────────────────────── Reserved
///   │      │      ╰─────────────────────────────── Lang ID (Language Type Dependant)
///   │      ╰────────────────────────────────────── Lang Type
///   │                                              ├─ 0b0000 - Generic (No lang)
///   │                                              ├─ 0b0001 - HDL
///   │                                              │  ├─ 0b0000 - Non-Specific
///   │                                              │  ├─ 0b0001 - Verilog
///   │                                              │  ├─ 0b0010 - SystemVerilog
///   │                                              │  ├─ 0b0011 - Verilog-AMS
///   │                                              │  ├─ 0b0100 - VHDL
///   │                                              │  ╰─ 0b0101 - VHDL-AMS
///   │                                              │
///   │                                              ├─ 0b0010 - Constraints
///   │                                              │  ├─ 0b0000 - Non-Specific
///   │                                              │  ├─ 0b0001 - SDC
///   │                                              │  ├─ 0b0010 - XDC
///   │                                              │  ├─ 0b0011 - PDC
///   │                                              │  ╰─ 0b0100 - LPF
///   │                                              │
///   │                                              ├─ 0b0011 - Liberty
///   │                                              ╰─ 0b0100 - IP-XACT
///   │
///   ╰───────────────────────────────────────────── Code Category
///                                                  ├─ 0b0000 - Informational
///                                                  ├─ 0b0001 - Lint
///                                                  ├─ 0b0010 - Warning
///                                                  ├─ 0b0011 - Error
///                                                  ╰─ 0b0100 - Debug
/// ```
///
/// When being rendered for the end user, the format is like so:
///
/// ```text
/// PPPCNNNN
/// │  ││
/// │  │╰──── Code Value
/// │  ╰───── Code Category
/// │         ├─ 0 - Informational
/// │         ├─ 1 - Lint
/// │         ├─ 2 - Warning
/// │         ├─ 3 - Error
/// │         ╰─ 4 - Debug
/// │
/// ╰──────── Language Type Prefix
///           ├─ `HDL` - Generic/Non-specific HDL
///           ├─   `V` - Verilog
///           ├─  `SV` - SystemVerilog
///           ├─  `VA` - Verilog-AMS
///           ├─  `VH` - VHDL
///           ├─ `VHA` - VHDL-AMS
///           ├─ `CON` - Generic/Non-Specific Constraints
///           ├─ `SDC` - Synopsys Design Constraints
///           ├─ `XDC` - Xilinx Design Constraints
///           ├─ `PDX` - Lattice Radiant Design Constraints
///           ├─ `LPF` - Lattice Diamond Logical Properties
///           ├─ `LIB` - Liberty
///           ╰─ `IP` - IP-XACT
/// ```
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Code(u32);

impl Code {
	/// Condition the input [`u16`]
	///
	/// This extends it to a [`u32`] and chops of the top 2-bits so it fits within the code value
	/// region
	#[inline(always)]
	const fn _condition_code(code: u16) -> u32 {
		(code as u32) & MASK_VALUE
	}

	/// Construct a new [`Code`] from a raw [`u32`]
	#[inline(always)]
	pub const fn from_raw(raw_value: u32) -> Result<Self, DiagnosticCodeError> {
		let cat = (raw_value & MASK_CATEGORY) >> SHIFT_CATEGORY;
		let lang_typ = (raw_value & MASK_LANG_TYPE) >> SHIFT_LANG_TYPE;
		let lang_val = (raw_value & MASK_LANG_VALUE) >> SHIFT_LANG_VALUE;
		let reserved = raw_value & MASK_RESERVED;
		// NOTE(aki):
		// We use `0xFFFF` here because we need to see if the upper 2 bits of the code value are set
		let code = raw_value & 0xFFFF;

		// Check to make sure the category is within range
		if !matches!(cat, 0..=4) {
			return Err(DiagnosticCodeError::InvalidCategory(cat as u8));
		}

		// ... and that we have a valid language type
		if !matches!(lang_typ, 0..=5) {
			return Err(DiagnosticCodeError::InvalidLanguageType(lang_typ as u8));
		}

		// ... and that the language value is within range for the given type
		if match lang_typ {
			0x0 => lang_val != 0,
			0x1 => !matches!(lang_val, 0..=5),
			0x2 => !matches!(lang_val, 0..=4),
			0x3 => lang_val != 0,
			0x4 => lang_val != 0,
			_ => unreachable!(),
		} {
			return Err(DiagnosticCodeError::InvalidLanguageValue(
				lang_val as u8,
				lang_typ as u8,
			));
		}

		// ... and that our reserved bits are not set
		if reserved != 0 {
			return Err(DiagnosticCodeError::ReservedValueSet);
		}

		// ... and finally that the actual code value is within range
		if code > MAX_CODE_VALUE {
			return Err(DiagnosticCodeError::IdOverflow(code as u16));
		}

		// Assuming that all checks out okay, then we can construct a new code type
		Ok(Self(raw_value))
	}

	/// Create a new generic informational diagnostic code
	#[inline(always)]
	pub const fn new_generic_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_GENERIC | Self::_condition_code(code))
	}

	/// Create a new generic HDL informational diagnostic code
	#[inline(always)]
	pub const fn new_hdl_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_HDL | LANG_VAL_HDL_GENERIC | Self::_condition_code(code))
	}

	/// Create a new Verilog informational diagnostic code
	#[inline(always)]
	pub const fn new_verilog_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG | Self::_condition_code(code))
	}

	/// Create a new SystemVerilog informational diagnostic code
	#[inline(always)]
	pub const fn new_system_verilog_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_HDL | LANG_VAL_HDL_SYSTEM_VERILOG | Self::_condition_code(code))
	}

	/// Create a new Verilog-AMS informational diagnostic code
	#[inline(always)]
	pub const fn new_verilog_ams_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG_AMS | Self::_condition_code(code))
	}

	/// Create a new VHDL informational diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL | Self::_condition_code(code))
	}

	/// Create a new VHDL-AMS informational diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_ams_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL_AMS | Self::_condition_code(code))
	}

	/// Create a new generic constraint informational diagnostic code
	#[inline(always)]
	pub const fn new_constraint_info(code: u16) -> Self {
		Self(
			CAT_INFO |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_GENERIC |
				Self::_condition_code(code),
		)
	}

	/// Create a new SDC constraint informational diagnostic code
	#[inline(always)]
	pub const fn new_sdc_info(code: u16) -> Self {
		Self(
			CAT_INFO | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_SDC | Self::_condition_code(code),
		)
	}

	/// Create a new XDC constraint informational diagnostic code
	#[inline(always)]
	pub const fn new_xdc_info(code: u16) -> Self {
		Self(
			CAT_INFO | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_XDC | Self::_condition_code(code),
		)
	}

	/// Create a new PDC constraint informational diagnostic code
	#[inline(always)]
	pub const fn new_pdc_info(code: u16) -> Self {
		Self(
			CAT_INFO | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_PDC | Self::_condition_code(code),
		)
	}

	/// Create a new LPF constraint informational diagnostic code
	#[inline(always)]
	pub const fn new_lpf_info(code: u16) -> Self {
		Self(
			CAT_INFO | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_LPF | Self::_condition_code(code),
		)
	}

	/// Create a new Liberty informational diagnostic code
	#[inline(always)]
	pub const fn new_liberty_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_LIBERTY | Self::_condition_code(code))
	}

	/// Create a new IP-XACT informational diagnostic code
	#[inline(always)]
	pub const fn new_xact_info(code: u16) -> Self {
		Self(CAT_INFO | LANG_TYPE_IP_XACT | Self::_condition_code(code))
	}

	/// Create a new new generic lint diagnostic code
	#[inline(always)]
	pub const fn new_generic_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_GENERIC | Self::_condition_code(code))
	}

	/// Create a new generic HDL lint diagnostic code
	#[inline(always)]
	pub const fn new_hdl_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_HDL | LANG_VAL_HDL_GENERIC | Self::_condition_code(code))
	}

	/// Create a new Verilog lint diagnostic code
	#[inline(always)]
	pub const fn new_verilog_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG | Self::_condition_code(code))
	}

	/// Create a new SystemVerilog lint diagnostic code
	#[inline(always)]
	pub const fn new_system_verilog_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_HDL | LANG_VAL_HDL_SYSTEM_VERILOG | Self::_condition_code(code))
	}

	/// Create a new Verilog-AMS lint diagnostic code
	#[inline(always)]
	pub const fn new_verilog_ams_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG_AMS | Self::_condition_code(code))
	}

	/// Create a new VHDL lint diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL | Self::_condition_code(code))
	}

	/// Create a new VHDL-AMS lint diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_ams_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL_AMS | Self::_condition_code(code))
	}

	/// Create a new generic constraint lint diagnostic code
	#[inline(always)]
	pub const fn new_constraint_lint(code: u16) -> Self {
		Self(
			CAT_LINT |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_GENERIC |
				Self::_condition_code(code),
		)
	}

	/// Create a new SDC constraint lint diagnostic code
	#[inline(always)]
	pub const fn new_sdc_lint(code: u16) -> Self {
		Self(
			CAT_LINT | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_SDC | Self::_condition_code(code),
		)
	}

	/// Create a new XDC constraint lint diagnostic code
	#[inline(always)]
	pub const fn new_xdc_lint(code: u16) -> Self {
		Self(
			CAT_LINT | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_XDC | Self::_condition_code(code),
		)
	}

	/// Create a new PDC constraint lint diagnostic code
	#[inline(always)]
	pub const fn new_pdc_lint(code: u16) -> Self {
		Self(
			CAT_LINT | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_PDC | Self::_condition_code(code),
		)
	}

	/// Create a new LPF constraint lint diagnostic code
	#[inline(always)]
	pub const fn new_lpf_lint(code: u16) -> Self {
		Self(
			CAT_LINT | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_LPF | Self::_condition_code(code),
		)
	}

	/// Create a new Liberty lint diagnostic code
	#[inline(always)]
	pub const fn new_liberty_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_LIBERTY | Self::_condition_code(code))
	}

	/// Create a new IP-XACT lint diagnostic code
	#[inline(always)]
	pub const fn new_xact_lint(code: u16) -> Self {
		Self(CAT_LINT | LANG_TYPE_IP_XACT | Self::_condition_code(code))
	}

	/// Create a new generic warning diagnostic code
	#[inline(always)]
	pub const fn new_generic_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_GENERIC | Self::_condition_code(code))
	}

	/// Create a new generic HDL warning diagnostic code
	#[inline(always)]
	pub const fn new_hdl_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_HDL | LANG_VAL_HDL_GENERIC | Self::_condition_code(code))
	}

	/// Create a new Verilog warning diagnostic code
	#[inline(always)]
	pub const fn new_verilog_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG | Self::_condition_code(code))
	}

	/// Create a new SystemVerilog warning diagnostic code
	#[inline(always)]
	pub const fn new_system_verilog_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_HDL | LANG_VAL_HDL_SYSTEM_VERILOG | Self::_condition_code(code))
	}

	/// Create a new Verilog-AMS warning diagnostic code
	#[inline(always)]
	pub const fn new_verilog_ams_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG_AMS | Self::_condition_code(code))
	}

	/// Create a new VHDL warning diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL | Self::_condition_code(code))
	}

	/// Create a new VHDL-AMS warning diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_ams_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL_AMS | Self::_condition_code(code))
	}

	/// Create a new generic constraint warning diagnostic code
	#[inline(always)]
	pub const fn new_constraint_warn(code: u16) -> Self {
		Self(
			CAT_WARN |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_GENERIC |
				Self::_condition_code(code),
		)
	}

	/// Create a new SDC constraint warning diagnostic code
	#[inline(always)]
	pub const fn new_sdc_warn(code: u16) -> Self {
		Self(
			CAT_WARN | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_SDC | Self::_condition_code(code),
		)
	}

	/// Create a new XDC constraint warning diagnostic code
	#[inline(always)]
	pub const fn new_xdc_warn(code: u16) -> Self {
		Self(
			CAT_WARN | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_XDC | Self::_condition_code(code),
		)
	}

	/// Create a new PDC constraint warning diagnostic code
	#[inline(always)]
	pub const fn new_pdc_warn(code: u16) -> Self {
		Self(
			CAT_WARN | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_PDC | Self::_condition_code(code),
		)
	}

	/// Create a new LPF constraint warning diagnostic code
	#[inline(always)]
	pub const fn new_lpf_warn(code: u16) -> Self {
		Self(
			CAT_WARN | LANG_TYPE_CONSTRAINT | LANG_VAL_CONSTRAINT_LPF | Self::_condition_code(code),
		)
	}

	/// Create a new Liberty warning diagnostic code
	#[inline(always)]
	pub const fn new_liberty_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_LIBERTY | Self::_condition_code(code))
	}

	/// Create a new IP-XACT warning diagnostic code
	#[inline(always)]
	pub const fn new_xact_warn(code: u16) -> Self {
		Self(CAT_WARN | LANG_TYPE_IP_XACT | Self::_condition_code(code))
	}

	/// Create a new generic error diagnostic code
	#[inline(always)]
	pub const fn new_generic_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_GENERIC | Self::_condition_code(code))
	}

	/// Create a new generic HDL error diagnostic code
	#[inline(always)]
	pub const fn new_hdl_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_HDL | LANG_VAL_HDL_GENERIC | Self::_condition_code(code))
	}

	/// Create a new Verilog error diagnostic code
	#[inline(always)]
	pub const fn new_verilog_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG | Self::_condition_code(code))
	}

	/// Create a new SystemVerilog error diagnostic code
	#[inline(always)]
	pub const fn new_system_verilog_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_HDL | LANG_VAL_HDL_SYSTEM_VERILOG | Self::_condition_code(code))
	}

	/// Create a new Verilog-AMS error diagnostic code
	#[inline(always)]
	pub const fn new_verilog_ams_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG_AMS | Self::_condition_code(code))
	}

	/// Create a new VHDL error diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL | Self::_condition_code(code))
	}

	/// Create a new VHDL-AMS error diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_ams_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL_AMS | Self::_condition_code(code))
	}

	/// Create a new generic constraint error diagnostic code
	#[inline(always)]
	pub const fn new_constraint_error(code: u16) -> Self {
		Self(
			CAT_ERROR |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_GENERIC |
				Self::_condition_code(code),
		)
	}

	/// Create a new SDC constraint error diagnostic code
	#[inline(always)]
	pub const fn new_sdc_error(code: u16) -> Self {
		Self(
			CAT_ERROR |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_SDC |
				Self::_condition_code(code),
		)
	}

	/// Create a new XDC constraint error diagnostic code
	#[inline(always)]
	pub const fn new_xdc_error(code: u16) -> Self {
		Self(
			CAT_ERROR |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_XDC |
				Self::_condition_code(code),
		)
	}

	/// Create a new PDC constraint error diagnostic code
	#[inline(always)]
	pub const fn new_pdc_error(code: u16) -> Self {
		Self(
			CAT_ERROR |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_PDC |
				Self::_condition_code(code),
		)
	}

	/// Create a new LPF constraint error diagnostic code
	#[inline(always)]
	pub const fn new_lpf_error(code: u16) -> Self {
		Self(
			CAT_ERROR |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_LPF |
				Self::_condition_code(code),
		)
	}

	/// Create a new Liberty error diagnostic code
	#[inline(always)]
	pub const fn new_liberty_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_LIBERTY | Self::_condition_code(code))
	}

	/// Create a new IP-XACT error diagnostic code
	#[inline(always)]
	pub const fn new_xact_error(code: u16) -> Self {
		Self(CAT_ERROR | LANG_TYPE_IP_XACT | Self::_condition_code(code))
	}

	/// Create a new generic debug diagnostic code
	#[inline(always)]
	pub const fn new_generic_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_GENERIC | Self::_condition_code(code))
	}

	/// Create a new generic HDL debug diagnostic code
	#[inline(always)]
	pub const fn new_hdl_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_HDL | LANG_VAL_HDL_GENERIC | Self::_condition_code(code))
	}

	/// Create a new Verilog debug diagnostic code
	#[inline(always)]
	pub const fn new_verilog_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG | Self::_condition_code(code))
	}

	/// Create a new SystemVerilog debug diagnostic code
	#[inline(always)]
	pub const fn new_system_verilog_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_HDL | LANG_VAL_HDL_SYSTEM_VERILOG | Self::_condition_code(code))
	}

	/// Create a new Verilog-AMS debug diagnostic code
	#[inline(always)]
	pub const fn new_verilog_ams_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_HDL | LANG_VAL_HDL_VERILOG_AMS | Self::_condition_code(code))
	}

	/// Create a new VHDL debug diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL | Self::_condition_code(code))
	}

	/// Create a new VHDL-AMS debug diagnostic code
	#[inline(always)]
	pub const fn new_vhdl_ams_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_HDL | LANG_VAL_HDL_VHDL_AMS | Self::_condition_code(code))
	}

	/// Create a new generic constraint debug diagnostic code
	#[inline(always)]
	pub const fn new_constraint_debug(code: u16) -> Self {
		Self(
			CAT_DEBUG |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_GENERIC |
				Self::_condition_code(code),
		)
	}

	/// Create a new SDC constraint debug diagnostic code
	#[inline(always)]
	pub const fn new_sdc_debug(code: u16) -> Self {
		Self(
			CAT_DEBUG |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_SDC |
				Self::_condition_code(code),
		)
	}

	/// Create a new XDC constraint debug diagnostic code
	#[inline(always)]
	pub const fn new_xdc_debug(code: u16) -> Self {
		Self(
			CAT_DEBUG |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_XDC |
				Self::_condition_code(code),
		)
	}

	/// Create a new PDC constraint debug diagnostic code
	#[inline(always)]
	pub const fn new_pdc_debug(code: u16) -> Self {
		Self(
			CAT_DEBUG |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_PDC |
				Self::_condition_code(code),
		)
	}

	/// Create a new LPF constraint debug diagnostic code
	#[inline(always)]
	pub const fn new_lpf_debug(code: u16) -> Self {
		Self(
			CAT_DEBUG |
				LANG_TYPE_CONSTRAINT |
				LANG_VAL_CONSTRAINT_LPF |
				Self::_condition_code(code),
		)
	}

	/// Create a new Liberty debug diagnostic code
	#[inline(always)]
	pub const fn new_liberty_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_LIBERTY | Self::_condition_code(code))
	}

	/// Create a new IP-XACT debug diagnostic code
	#[inline(always)]
	pub const fn new_xact_debug(code: u16) -> Self {
		Self(CAT_DEBUG | LANG_TYPE_IP_XACT | Self::_condition_code(code))
	}

	/// Create a new code with all the same metadata, just increment the actual value.
	#[inline(always)]
	pub const fn next_code(&self) -> Result<Self, DiagnosticCodeError> {
		let value = (self.0 & MASK_VALUE) + 1;

		if value > MAX_CODE_VALUE {
			Err(DiagnosticCodeError::IdOverflow(value as u16))
		} else {
			Ok(Self((self.0 & !MASK_VALUE) | value))
		}
	}

	#[inline(always)]
	pub const fn with_value(self, code: u16) -> Self {
		let code = Self::_condition_code(code);

		Self(code | (self.0 & !MASK_VALUE))
	}

	/// Get the value of the current code
	#[inline(always)]
	pub const fn value(&self) -> u16 {
		(MASK_VALUE & self.0) as u16
	}

	/// Get the raw value
	#[inline(always)]
	pub const fn raw(&self) -> u32 {
		self.0
	}

	/// Check to see if this given code is a generic diagnostic
	#[inline(always)]
	pub const fn is_generic(&self) -> bool {
		(MASK_LANG_TYPE & self.0) == LANG_TYPE_GENERIC
	}

	/// Check to see if this given code belongs to an HDL diagnostic
	#[inline(always)]
	pub const fn is_hdl(&self) -> bool {
		(MASK_LANG_TYPE & self.0) == LANG_TYPE_HDL
	}

	/// Check to see if this given code belongs to a constraint diagnostic
	#[inline(always)]
	pub const fn is_constraint(&self) -> bool {
		(MASK_LANG_TYPE & self.0) == LANG_TYPE_CONSTRAINT
	}

	/// Check to see if this given code belongs to a Liberty diagnostic
	#[inline(always)]
	pub const fn is_liberty(&self) -> bool {
		(MASK_LANG_TYPE & self.0) == LANG_TYPE_LIBERTY
	}

	/// Check to see if this given diagnostic code belongs to an IP-XACT diagnostic
	#[inline(always)]
	pub const fn is_xact(&self) -> bool {
		(MASK_LANG_TYPE & self.0) == LANG_TYPE_IP_XACT
	}

	/// Check to see if this given code is for HDL code in general
	#[inline(always)]
	pub const fn is_generic_hdl(&self) -> bool {
		self.is_hdl() && (MASK_LANG_VALUE & self.0) == LANG_VAL_HDL_GENERIC
	}

	/// Check to see if this given code is for Verilog
	#[inline(always)]
	pub const fn is_verilog(&self) -> bool {
		self.is_hdl() && (MASK_LANG_VALUE & self.0) == LANG_VAL_HDL_VERILOG
	}

	/// Check to see if this given code is for SystemVerilog
	#[inline(always)]
	pub const fn is_system_verilog(&self) -> bool {
		self.is_hdl() && (MASK_LANG_VALUE & self.0) == LANG_VAL_HDL_SYSTEM_VERILOG
	}

	/// Check to see if this given code is for Verilog-AMS
	#[inline(always)]
	pub const fn is_verilog_ams(&self) -> bool {
		self.is_hdl() && (MASK_LANG_VALUE & self.0) == LANG_VAL_HDL_VERILOG_AMS
	}

	/// Check to see if this given code is for VHDL
	#[inline(always)]
	pub const fn is_vhdl(&self) -> bool {
		self.is_hdl() && (MASK_LANG_VALUE & self.0) == LANG_VAL_HDL_VHDL
	}

	/// Check to see if this given code is for VHDL-AMS
	#[inline(always)]
	pub const fn is_vhdl_ams(&self) -> bool {
		self.is_hdl() && (MASK_LANG_VALUE & self.0) == LANG_VAL_HDL_VHDL_AMS
	}

	/// Check to see if this given code is for constraints in general
	#[inline(always)]
	pub const fn is_generic_constraint(&self) -> bool {
		self.is_constraint() && (MASK_LANG_VALUE & self.0) == LANG_VAL_CONSTRAINT_GENERIC
	}

	/// Check to see if this given code is for sdc constraints
	#[inline(always)]
	pub const fn is_sdc(&self) -> bool {
		self.is_constraint() && (MASK_LANG_VALUE & self.0) == LANG_VAL_CONSTRAINT_SDC
	}

	/// Check to see if this given code is for xdc constraints
	#[inline(always)]
	pub const fn is_xdc(&self) -> bool {
		self.is_constraint() && (MASK_LANG_VALUE & self.0) == LANG_VAL_CONSTRAINT_XDC
	}

	/// Check to see if this given code is for pdc constraints
	#[inline(always)]
	pub const fn is_pdc(&self) -> bool {
		self.is_constraint() && (MASK_LANG_VALUE & self.0) == LANG_VAL_CONSTRAINT_PDC
	}

	/// Check to see if this given code is for lpf constraints
	#[inline(always)]
	pub const fn is_lpf(&self) -> bool {
		self.is_constraint() && (MASK_LANG_VALUE & self.0) == LANG_VAL_CONSTRAINT_LPF
	}

	/// Check to see if this given code is informational
	#[inline(always)]
	pub const fn is_info(&self) -> bool {
		(MASK_CATEGORY & self.0) == CAT_INFO
	}

	/// Check to see if this given code is a linter code
	#[inline(always)]
	pub const fn is_lint(&self) -> bool {
		(MASK_CATEGORY & self.0) == CAT_LINT
	}

	/// Check to see if this given code is a warning code
	#[inline(always)]
	pub const fn is_warn(&self) -> bool {
		(MASK_CATEGORY & self.0) == CAT_WARN
	}

	/// Check to see if this given code is an error code
	#[inline(always)]
	pub const fn is_error(&self) -> bool {
		(MASK_CATEGORY & self.0) == CAT_ERROR
	}

	/// Check to see if this given code is a debug code
	#[inline(always)]
	pub const fn is_debug(&self) -> bool {
		(MASK_CATEGORY & self.0) == CAT_DEBUG
	}

	/// Get the raw value of the category
	#[inline(always)]
	pub(crate) const fn raw_category(&self) -> u32 {
		MASK_CATEGORY & self.0
	}

	/// Get the raw value of the language type
	#[inline(always)]
	pub(crate) const fn raw_lang_type(&self) -> u32 {
		MASK_LANG_TYPE & self.0
	}

	/// Get the raw value of the language
	#[inline(always)]
	pub(crate) const fn raw_lang_value(&self) -> u32 {
		MASK_LANG_VALUE & self.0
	}
}

impl Display for Code {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}{:04}",
			match self.raw_lang_type() {
				LANG_TYPE_GENERIC => PREFIX_GENERIC,
				LANG_TYPE_HDL => match self.raw_lang_value() {
					LANG_VAL_HDL_GENERIC => PREFIX_HDL_GENERIC,
					LANG_VAL_HDL_VERILOG => PREFIX_HDL_VERILOG,
					LANG_VAL_HDL_SYSTEM_VERILOG => PREFIX_HDL_SYSTEM_VERILOG,
					LANG_VAL_HDL_VERILOG_AMS => PREFIX_HDL_VERILOG_AMS,
					LANG_VAL_HDL_VHDL => PREFIX_HDL_VHDL,
					LANG_VAL_HDL_VHDL_AMS => PREFIX_HDL_VHDL_AMS,
					_ => unreachable!(),
				},
				LANG_TYPE_CONSTRAINT => match self.raw_lang_value() {
					LANG_VAL_CONSTRAINT_GENERIC => PREFIX_CONSTRAINT_GENERIC,
					LANG_VAL_CONSTRAINT_SDC => PREFIX_CONSTRAINT_SDC,
					LANG_VAL_CONSTRAINT_XDC => PREFIX_CONSTRAINT_XDC,
					LANG_VAL_CONSTRAINT_PDC => PREFIX_CONSTRAINT_PDC,
					LANG_VAL_CONSTRAINT_LPF => PREFIX_CONSTRAINT_LPF,
					_ => unreachable!(),
				},
				LANG_TYPE_LIBERTY => PREFIX_LIBERTY,
				LANG_TYPE_IP_XACT => PREFIX_IP_XACT,
				_ => unreachable!(),
			},
			(self.raw_category() >> SHIFT_CATEGORY),
			self.value()
		)
	}
}

impl Debug for Code {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Code(category: {}, type: {}, value: {}, code: {})",
			self.raw_category() >> SHIFT_CATEGORY,
			self.raw_lang_type() >> SHIFT_LANG_TYPE,
			self.raw_lang_value() >> SHIFT_LANG_VALUE,
			self.value()
		)
	}
}

impl TryFrom<u32> for Code {
	type Error = DiagnosticCodeError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		Self::from_raw(value)
	}
}

impl FromStr for Code {
	type Err = DiagnosticCodeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Find where the prefix ends
		let idx = s
			.find(char::is_numeric)
			.ok_or(DiagnosticCodeError::InvalidCodeFormat)?;

		// Split the string into the prefix and then the code portion
		let (prefix, code) = s.split_at(idx);
		// ... and then split the code into the category and the value
		let (category, value) = code.split_at(1);

		// Try to figure out the type and value for the language
		let (lang_type, lang_val) = match prefix {
			PREFIX_GENERIC => (LANG_TYPE_GENERIC, 0),
			PREFIX_HDL_GENERIC => (LANG_TYPE_HDL, LANG_VAL_HDL_GENERIC),
			PREFIX_HDL_VERILOG => (LANG_TYPE_HDL, LANG_VAL_HDL_VERILOG),
			PREFIX_HDL_SYSTEM_VERILOG => (LANG_TYPE_HDL, LANG_VAL_HDL_SYSTEM_VERILOG),
			PREFIX_HDL_VERILOG_AMS => (LANG_TYPE_HDL, LANG_VAL_HDL_VERILOG_AMS),
			PREFIX_HDL_VHDL => (LANG_TYPE_HDL, LANG_VAL_HDL_VHDL),
			PREFIX_HDL_VHDL_AMS => (LANG_TYPE_HDL, LANG_VAL_HDL_VHDL_AMS),
			PREFIX_CONSTRAINT_GENERIC => (LANG_TYPE_CONSTRAINT, LANG_VAL_CONSTRAINT_GENERIC),
			PREFIX_CONSTRAINT_SDC => (LANG_TYPE_CONSTRAINT, LANG_VAL_CONSTRAINT_SDC),
			PREFIX_CONSTRAINT_XDC => (LANG_TYPE_CONSTRAINT, LANG_VAL_CONSTRAINT_XDC),
			PREFIX_CONSTRAINT_PDC => (LANG_TYPE_CONSTRAINT, LANG_VAL_CONSTRAINT_PDC),
			PREFIX_CONSTRAINT_LPF => (LANG_TYPE_CONSTRAINT, LANG_VAL_CONSTRAINT_LPF),
			PREFIX_LIBERTY => (LANG_TYPE_LIBERTY, 0),
			PREFIX_IP_XACT => (LANG_TYPE_IP_XACT, 0),
			_ => return Err(DiagnosticCodeError::InvalidCodeFormat),
		};

		let category = category
			.parse::<u32>()
			.map_err(|_| DiagnosticCodeError::InvalidCodeFormat)? <<
			SHIFT_CATEGORY;

		let value = value
			.parse::<u32>()
			.map_err(|_| DiagnosticCodeError::InvalidCodeFormat)? &
			MASK_VALUE;

		Self::from_raw(category | lang_type | lang_val | value)
	}
}

impl Error for DiagnosticCodeError {}

impl Display for DiagnosticCodeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::IdOverflow(val) => write!(
				f,
				"Attempted to create a diagnostic code with the value {}. The maximum value for \
				 any diagnostic code is {}.",
				val, MAX_CODE_VALUE
			),
			Self::InvalidCategory(cat) => write!(f, "Invalid diagnostic code category: {}", cat),
			Self::InvalidCodeFormat => {
				write!(f, "The textual format of the diagnostic code is invalid")
			},
			Self::InvalidLanguageType(typ) => {
				write!(f, "Invalid diagnostic code language type: {}", typ)
			},
			Self::InvalidLanguageValue(val, typ) => write!(
				f,
				"Invalid diagnostic code language value {} for language type {}",
				val, typ
			),
			Self::ReservedValueSet => write!(f, "Reserved bits set in raw diagnostic code"),
		}
	}
}

#[cfg(test)]
mod test {
	use proptest::prelude::*;

	use super::*;

	#[test]
	fn test_code_generic() {
		let code = Code::new_generic_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "VIR00001");

		let code = Code::new_generic_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "VIR10020");

		let code = Code::new_generic_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "VIR20300");

		let code = Code::new_generic_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "VIR34000");

		let code = Code::new_generic_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "VIR49999");
	}

	#[test]
	fn test_code_hdl() {
		let code = Code::new_hdl_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "HDL00001");

		let code = Code::new_hdl_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "HDL10020");

		let code = Code::new_hdl_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "HDL20300");

		let code = Code::new_hdl_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "HDL34000");

		let code = Code::new_hdl_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "HDL49999");
	}

	#[test]
	fn test_code_constraint() {
		let code = Code::new_constraint_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "CON00001");

		let code = Code::new_constraint_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "CON10020");

		let code = Code::new_constraint_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "CON20300");

		let code = Code::new_constraint_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "CON34000");

		let code = Code::new_constraint_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "CON49999");
	}

	#[test]
	fn test_code_liberty() {
		let code = Code::new_liberty_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "LIB00001");

		let code = Code::new_liberty_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "LIB10020");

		let code = Code::new_liberty_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "LIB20300");

		let code = Code::new_liberty_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "LIB34000");

		let code = Code::new_liberty_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(code.is_liberty());
		assert!(!code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "LIB49999");
	}

	#[test]
	fn test_code_xact() {
		let code = Code::new_xact_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "IP00001");

		let code = Code::new_xact_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "IP10020");

		let code = Code::new_xact_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "IP20300");

		let code = Code::new_xact_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "IP34000");

		let code = Code::new_xact_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(code.is_xact());
		assert_eq!((code.raw_lang_value() >> SHIFT_LANG_VALUE), 0);
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "IP49999");
	}

	#[test]
	fn test_code_verilog() {
		let code = Code::new_verilog_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "V00001");

		let code = Code::new_verilog_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "V10020");

		let code = Code::new_verilog_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "V20300");

		let code = Code::new_verilog_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "V34000");

		let code = Code::new_verilog_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "V49999");
	}

	#[test]
	fn test_code_system_verilog() {
		let code = Code::new_system_verilog_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "SV00001");

		let code = Code::new_system_verilog_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "SV10020");

		let code = Code::new_system_verilog_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "SV20300");

		let code = Code::new_system_verilog_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "SV34000");

		let code = Code::new_system_verilog_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "SV49999");
	}

	#[test]
	fn test_code_verilog_ams() {
		let code = Code::new_verilog_ams_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "VA00001");

		let code = Code::new_verilog_ams_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "VA10020");

		let code = Code::new_verilog_ams_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "VA20300");

		let code = Code::new_verilog_ams_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "VA34000");

		let code = Code::new_verilog_ams_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "VA49999");
	}

	#[test]
	fn test_code_vhdl() {
		let code = Code::new_vhdl_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "VH00001");

		let code = Code::new_vhdl_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "VH10020");

		let code = Code::new_vhdl_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "VH20300");

		let code = Code::new_vhdl_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "VH34000");

		let code = Code::new_vhdl_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(code.is_vhdl());
		assert!(!code.is_vhdl_ams());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "VH49999");
	}

	#[test]
	fn test_code_vhdl_ams() {
		let code = Code::new_vhdl_ams_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(code.is_vhdl_ams());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "VHA00001");

		let code = Code::new_vhdl_ams_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(code.is_vhdl_ams());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "VHA10020");

		let code = Code::new_vhdl_ams_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(code.is_vhdl_ams());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "VHA20300");

		let code = Code::new_vhdl_ams_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(code.is_vhdl_ams());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "VHA34000");

		let code = Code::new_vhdl_ams_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(code.is_hdl());
		assert!(!code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_hdl());
		assert!(!code.is_verilog());
		assert!(!code.is_system_verilog());
		assert!(!code.is_verilog_ams());
		assert!(!code.is_vhdl());
		assert!(code.is_vhdl_ams());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "VHA49999");
	}

	#[test]
	fn test_code_sdc() {
		let code = Code::new_sdc_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "SDC00001");

		let code = Code::new_sdc_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "SDC10020");

		let code = Code::new_sdc_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "SDC20300");

		let code = Code::new_sdc_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "SDC34000");

		let code = Code::new_sdc_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "SDC49999");
	}

	#[test]
	fn test_code_xdc() {
		let code = Code::new_xdc_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "XDC00001");

		let code = Code::new_xdc_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "XDC10020");

		let code = Code::new_xdc_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "XDC20300");

		let code = Code::new_xdc_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "XDC34000");

		let code = Code::new_xdc_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(code.is_xdc());
		assert!(!code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "XDC49999");
	}

	#[test]
	fn test_code_pdc() {
		let code = Code::new_pdc_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "PDC00001");

		let code = Code::new_pdc_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "PDC10020");

		let code = Code::new_pdc_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "PDC20300");

		let code = Code::new_pdc_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "PDC34000");

		let code = Code::new_pdc_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(code.is_pdc());
		assert!(!code.is_lpf());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "PDC49999");
	}

	#[test]
	fn test_code_lpf() {
		let code = Code::new_lpf_info(1);
		assert!(code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(code.is_lpf());
		assert_eq!(code.value(), 1);
		assert_eq!(format!("{}", code), "LPF00001");

		let code = Code::new_lpf_lint(20);
		assert!(!code.is_info());
		assert!(code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(code.is_lpf());
		assert_eq!(code.value(), 20);
		assert_eq!(format!("{}", code), "LPF10020");

		let code = Code::new_lpf_warn(300);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(code.is_warn());
		assert!(!code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(code.is_lpf());
		assert_eq!(code.value(), 300);
		assert_eq!(format!("{}", code), "LPF20300");

		let code = Code::new_lpf_error(4000);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(code.is_error());
		assert!(!code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(code.is_lpf());
		assert_eq!(code.value(), 4000);
		assert_eq!(format!("{}", code), "LPF34000");

		let code = Code::new_lpf_debug(9999);
		assert!(!code.is_info());
		assert!(!code.is_lint());
		assert!(!code.is_warn());
		assert!(!code.is_error());
		assert!(code.is_debug());
		assert!(!code.is_generic());
		assert!(!code.is_hdl());
		assert!(code.is_constraint());
		assert!(!code.is_liberty());
		assert!(!code.is_xact());
		assert!(!code.is_generic_constraint());
		assert!(!code.is_sdc());
		assert!(!code.is_xdc());
		assert!(!code.is_pdc());
		assert!(code.is_lpf());
		assert_eq!(code.value(), 9999);
		assert_eq!(format!("{}", code), "LPF49999");
	}

	proptest! {
		#[test]
		fn test_parse_code(s in "(VIR|IP|LIB|HDL|V|VA|VH|VHA|SV|CON|LPF|PDC|SDC|XDC)[0-3][0-9]{4}") {
			#[allow(clippy::unwrap_used, reason = "We want to crash in a test context")]
			let _: Code = s.parse().unwrap();
		}
	}
}
