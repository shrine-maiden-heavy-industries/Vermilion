// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fmt::Display;

use bitmask_enum::bitmask;
use eyre::eyre;
use vermilion_lang::LanguageMetadata;

pub mod error;
pub mod fmt;
pub mod lang;
pub mod lint;
pub mod workspace;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum VerilogStd {
	#[default]
	Vl95,
	Vl01,
	Vl05,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum SystemVerilogStd {
	#[default]
	Sv05,
	Sv09,
	Sv12,
	Sv17,
	Sv23,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum VerilogAmsStd {
	#[default]
	Vams09,
	Vams14,
	Vams23,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum VerilogVariant {
	Verilog(VerilogStd),
	SystemVerilog(SystemVerilogStd),
	VerilogAms(VerilogAmsStd),
}

// NOTE(aki):
// In order to save space in things such as `Tokens` and other diagnostics, we have an
// internal representation of the language standard packed from a `VerilogVariant` into
// a 16-bit enum where we can set multiple standards at once.
//
// This is not super ideal to do the conversions all the time, and we should likely
// eventually just use this rather than `VerilogVariant` but for now as an internal repr
// it should be fine:tm:
#[bitmask(u16)]
#[bitmask_config(flags_iter)]
pub enum LanguageSet {
	Vl95,
	Vl01,
	Vl05,
	Sv05,
	Sv09,
	Sv12,
	Sv17,
	Sv23,
	Vams09,
	Vams14,
	Vams23,
}

impl VerilogStd {
	pub const KNOWN_FILE_EXTS: [&'static str; 4] = ["v", "vh", "vm", "vg"];
}

impl LanguageMetadata for VerilogStd {
	fn file_extensions<'a, 'b: 'a>() -> &'a [&'b str] {
		&VerilogStd::KNOWN_FILE_EXTS
	}
}

impl Display for VerilogStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Vl95 => write!(f, "Verilog 1995 (IEEE 1364-1995)"),
			Self::Vl01 => write!(f, "Verilog 2001 (IEEE 1364-2001)"),
			Self::Vl05 => write!(f, "Verilog 2005 (IEEE 1364-2005)"),
		}
	}
}

impl SystemVerilogStd {
	pub const KNOWN_FILE_EXTS: [&'static str; 1] = ["sv"];
}

impl LanguageMetadata for SystemVerilogStd {
	fn file_extensions<'a, 'b: 'a>() -> &'a [&'b str] {
		&SystemVerilogStd::KNOWN_FILE_EXTS
	}
}

impl Display for SystemVerilogStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Sv05 => write!(f, "SystemVerilog 2005 (IEEE 1800-2005)"),
			Self::Sv09 => write!(f, "SystemVerilog 2009 (IEEE 1800-2009)"),
			Self::Sv12 => write!(f, "SystemVerilog 2012 (IEEE 1800-2012)"),
			Self::Sv17 => write!(f, "SystemVerilog 2017 (IEEE 1800-2017)"),
			Self::Sv23 => write!(f, "SystemVerilog 2023 (IEEE 1800-2023)"),
		}
	}
}

impl VerilogAmsStd {
	pub const KNOWN_FILE_EXTS: [&'static str; 1] = ["vams"];
}

impl LanguageMetadata for VerilogAmsStd {
	fn file_extensions<'a, 'b: 'a>() -> &'a [&'b str] {
		&VerilogAmsStd::KNOWN_FILE_EXTS
	}
}

impl Display for VerilogAmsStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Vams09 => write!(f, "Verilog-AMS 2009 (Accellera Verilog-AMS 2.3.1)"),
			Self::Vams14 => write!(f, "Verilog-AMS 2014 (Accellera Verilog-AMS 2.4)"),
			Self::Vams23 => write!(f, "Verilog-AMS 2023 (Accellera Verilog-AMS 2023)"),
		}
	}
}

impl Display for VerilogVariant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Verilog(std) => std.fmt(f),
			Self::SystemVerilog(std) => std.fmt(f),
			Self::VerilogAms(std) => std.fmt(f),
		}
	}
}

impl LanguageSet {
	pub const SYSTEM_VERILOG_STDS: LanguageSet = LanguageSet::Sv05
		.or(LanguageSet::Sv09)
		.or(LanguageSet::Sv12)
		.or(LanguageSet::Sv17)
		.or(LanguageSet::Sv23);
	pub const VERILOG_AMS_STDS: LanguageSet = LanguageSet::Vams09
		.or(LanguageSet::Vams14)
		.or(LanguageSet::Vams23);
	pub const VERILOG_STDS: LanguageSet = LanguageSet::Vl95
		.or(LanguageSet::Vl01)
		.or(LanguageSet::Vl05);

	pub fn has_verilog_variant(&self, variant: VerilogVariant) -> bool {
		self.contains(variant.into())
	}

	pub fn is_verilog(&self) -> bool {
		self.contains(Self::VERILOG_STDS)
	}

	pub fn is_system_verilog(&self) -> bool {
		self.contains(Self::SYSTEM_VERILOG_STDS)
	}

	pub fn is_verilog_ams(&self) -> bool {
		self.contains(Self::VERILOG_AMS_STDS)
	}
}

impl From<VerilogStd> for LanguageSet {
	fn from(value: VerilogStd) -> Self {
		match value {
			VerilogStd::Vl95 => LanguageSet::Vl95,
			VerilogStd::Vl01 => LanguageSet::Vl01,
			VerilogStd::Vl05 => LanguageSet::Vl05,
		}
	}
}

impl From<SystemVerilogStd> for LanguageSet {
	fn from(value: SystemVerilogStd) -> Self {
		match value {
			SystemVerilogStd::Sv05 => LanguageSet::Sv05,
			SystemVerilogStd::Sv09 => LanguageSet::Sv09,
			SystemVerilogStd::Sv12 => LanguageSet::Sv12,
			SystemVerilogStd::Sv17 => LanguageSet::Sv17,
			SystemVerilogStd::Sv23 => LanguageSet::Sv23,
		}
	}
}

impl From<VerilogAmsStd> for LanguageSet {
	fn from(value: VerilogAmsStd) -> Self {
		match value {
			VerilogAmsStd::Vams09 => LanguageSet::Vams09,
			VerilogAmsStd::Vams14 => LanguageSet::Vams14,
			VerilogAmsStd::Vams23 => LanguageSet::Vams23,
		}
	}
}

impl From<VerilogVariant> for LanguageSet {
	fn from(value: VerilogVariant) -> Self {
		match value {
			VerilogVariant::Verilog(std) => std.into(),
			VerilogVariant::SystemVerilog(std) => std.into(),
			VerilogVariant::VerilogAms(std) => std.into(),
		}
	}
}

impl TryFrom<LanguageSet> for VerilogStd {
	type Error = eyre::Report;

	fn try_from(value: LanguageSet) -> Result<Self, Self::Error> {
		if value.is_verilog() {
			Ok(if value.contains(LanguageSet::Vl05) {
				VerilogStd::Vl05
			} else if value.contains(LanguageSet::Vl01) {
				VerilogStd::Vl01
			} else {
				VerilogStd::Vl95
			})
		} else {
			Err(eyre!("No Verilog language standard set"))
		}
	}
}

impl TryFrom<LanguageSet> for SystemVerilogStd {
	type Error = eyre::Report;

	fn try_from(value: LanguageSet) -> Result<Self, Self::Error> {
		if value.is_system_verilog() {
			Ok(if value.contains(LanguageSet::Sv23) {
				SystemVerilogStd::Sv23
			} else if value.contains(LanguageSet::Sv17) {
				SystemVerilogStd::Sv17
			} else if value.contains(LanguageSet::Sv12) {
				SystemVerilogStd::Sv12
			} else if value.contains(LanguageSet::Sv09) {
				SystemVerilogStd::Sv09
			} else {
				SystemVerilogStd::Sv05
			})
		} else {
			Err(eyre!("No SystemVerilog language standard set"))
		}
	}
}

impl TryFrom<LanguageSet> for VerilogAmsStd {
	type Error = eyre::Report;

	fn try_from(value: LanguageSet) -> Result<Self, Self::Error> {
		if value.is_verilog_ams() {
			Ok(if value.contains(LanguageSet::Vams23) {
				VerilogAmsStd::Vams23
			} else if value.contains(LanguageSet::Vams14) {
				VerilogAmsStd::Vams14
			} else {
				VerilogAmsStd::Vams09
			})
		} else {
			Err(eyre!("No Verilog-AMS language standard set"))
		}
	}
}

impl TryFrom<LanguageSet> for VerilogVariant {
	type Error = eyre::Report;

	fn try_from(value: LanguageSet) -> Result<Self, Self::Error> {
		if value.is_verilog() {
			Ok(VerilogVariant::Verilog(value.try_into()?))
		} else if value.is_system_verilog() {
			Ok(VerilogVariant::SystemVerilog(value.try_into()?))
		} else if value.is_verilog() {
			Ok(VerilogVariant::VerilogAms(value.try_into()?))
		} else {
			Err(eyre!("No language standard set"))
		}
	}
}
