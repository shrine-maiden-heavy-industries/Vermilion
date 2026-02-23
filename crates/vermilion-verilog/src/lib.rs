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

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LanguageSet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		static VALUES: [&str; 11] = [
			"Vl95", "Vl01", "Vl05", "Sv05", "Sv09", "Sv12", "Sv17", "Sv23", "Vams09", "Vams14",
			"Vams23",
		];

		struct ValueVisitor;
		impl<'de> serde::de::Visitor<'de> for ValueVisitor {
			type Value = LanguageSet;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str(
					"`Vl95`, `Vl01`, `Vl05`, `Sv05`, `Sv09`, `Sv12`, `Sv17`, `Sv23`, `Vams09`, \
					 `Vams14`, or `Vams23`",
				)
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				match value {
					"Vl95" => Ok(LanguageSet::Vl95),
					"Vl01" => Ok(LanguageSet::Vl01),
					"Vl05" => Ok(LanguageSet::Vl05),
					"Sv05" => Ok(LanguageSet::Sv05),
					"Sv09" => Ok(LanguageSet::Sv09),
					"Sv12" => Ok(LanguageSet::Sv12),
					"Sv17" => Ok(LanguageSet::Sv17),
					"Sv23" => Ok(LanguageSet::Sv23),
					"Vams09" => Ok(LanguageSet::Vams09),
					"Vams14" => Ok(LanguageSet::Vams14),
					"Vams23" => Ok(LanguageSet::Vams23),
					_ => Err(serde::de::Error::unknown_variant(value, &VALUES)),
				}
			}
		}

		deserializer.deserialize_str(ValueVisitor)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for LanguageSet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match *self {
			LanguageSet::Vl95 => serializer.serialize_str("Vl95"),
			LanguageSet::Vl01 => serializer.serialize_str("Vl01"),
			LanguageSet::Vl05 => serializer.serialize_str("Vl05"),
			LanguageSet::Sv05 => serializer.serialize_str("Sv05"),
			LanguageSet::Sv09 => serializer.serialize_str("Sv09"),
			LanguageSet::Sv12 => serializer.serialize_str("Sv12"),
			LanguageSet::Sv17 => serializer.serialize_str("Sv17"),
			LanguageSet::Sv23 => serializer.serialize_str("Sv23"),
			LanguageSet::Vams09 => serializer.serialize_str("Vams09"),
			LanguageSet::Vams14 => serializer.serialize_str("Vams14"),
			LanguageSet::Vams23 => serializer.serialize_str("Vams23"),
			_ => Err(serde::ser::Error::custom(
				"Unable to serialize `LanguageSet` with more than one bit set",
			)),
		}
	}
}

#[cfg(test)]
mod test {
	#[cfg(feature = "serde")]
	use serde_test::{Token, assert_tokens};

	use super::*;

	#[test]
	#[cfg(feature = "serde")]
	fn test_language_set_serialize() {
		assert_tokens(&LanguageSet::Vl95, &[Token::Str("Vl95")]);
		assert_tokens(&LanguageSet::Vl01, &[Token::Str("Vl01")]);
		assert_tokens(&LanguageSet::Vl05, &[Token::Str("Vl05")]);
		assert_tokens(&LanguageSet::Sv05, &[Token::Str("Sv05")]);
		assert_tokens(&LanguageSet::Sv09, &[Token::Str("Sv09")]);
		assert_tokens(&LanguageSet::Sv12, &[Token::Str("Sv12")]);
		assert_tokens(&LanguageSet::Sv17, &[Token::Str("Sv17")]);
		assert_tokens(&LanguageSet::Sv23, &[Token::Str("Sv23")]);
		assert_tokens(&LanguageSet::Vams09, &[Token::Str("Vams09")]);
		assert_tokens(&LanguageSet::Vams14, &[Token::Str("Vams14")]);
		assert_tokens(&LanguageSet::Vams23, &[Token::Str("Vams23")]);
	}
}
