// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fmt::Display;

use bitmask_enum::bitmask;

pub mod error;
pub mod fmt;
pub mod lang;
pub mod lint;
pub mod workspace;

#[bitmask(u16)]
#[bitmask_config(flags_iter)]
pub enum LanguageStd {
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

impl LanguageStd {
	pub const SYSTEM_VERILOG_KNOWN_EXTS: [&'static str; 1] = ["sv"];
	pub const SYSTEM_VERILOG_STDS: LanguageStd = LanguageStd::Sv05
		.or(LanguageStd::Sv09)
		.or(LanguageStd::Sv12)
		.or(LanguageStd::Sv17)
		.or(LanguageStd::Sv23);
	pub const VERILOG_AMS_KNOWN_EXTS: [&'static str; 1] = ["vams"];
	pub const VERILOG_AMS_STDS: LanguageStd = LanguageStd::Vams09
		.or(LanguageStd::Vams14)
		.or(LanguageStd::Vams23);
	pub const VERILOG_KNOWN_EXTS: [&'static str; 4] = ["v", "vh", "vm", "vg"];
	pub const VERILOG_STDS: LanguageStd = LanguageStd::Vl95
		.or(LanguageStd::Vl01)
		.or(LanguageStd::Vl05);

	pub fn has_single_std(&self) -> bool {
		self.bits.count_ones() == 1
	}

	pub fn popcount(&self) -> u32 {
		self.bits.count_ones()
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

impl Display for LanguageStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(LanguageStd::Vl95) {
			f.write_str("Verilog 1995 (IEEE 1364-1995)")?;
		}
		if self.contains(LanguageStd::Vl01) {
			f.write_str("Verilog 2001 (IEEE 1364-2001)")?;
		}
		if self.contains(LanguageStd::Vl05) {
			f.write_str("Verilog 2005 (IEEE 1364-2005)")?;
		}
		if self.contains(LanguageStd::Sv05) {
			f.write_str("System Verilog 2005 (IEEE 1800-2005)")?;
		}
		if self.contains(LanguageStd::Sv09) {
			f.write_str("System Verilog 2009 (IEEE 1800-2009)")?;
		}
		if self.contains(LanguageStd::Sv12) {
			f.write_str("System Verilog 2012 (IEEE 1800-2012)")?;
		}
		if self.contains(LanguageStd::Sv17) {
			f.write_str("System Verilog 2017 (IEEE 1800-2017)")?;
		}
		if self.contains(LanguageStd::Sv23) {
			f.write_str("System Verilog 2023 (IEEE 1800-2023)")?;
		}
		if self.contains(LanguageStd::Vams09) {
			f.write_str("Verilog-AMS 2009 (Accellera Verilog-AMS 2.3.1)")?;
		}
		if self.contains(LanguageStd::Vams14) {
			f.write_str("Verilog-AMS 2014 (Accellera Verilog-AMS 2.4)")?;
		}
		if self.contains(LanguageStd::Vams23) {
			f.write_str("Verilog-AMS 2009 (Accellera Verilog-AMS 2.3.1)")?;
		}
		Ok(())
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LanguageStd {
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
			type Value = LanguageStd;

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
					"Vl95" => Ok(LanguageStd::Vl95),
					"Vl01" => Ok(LanguageStd::Vl01),
					"Vl05" => Ok(LanguageStd::Vl05),
					"Sv05" => Ok(LanguageStd::Sv05),
					"Sv09" => Ok(LanguageStd::Sv09),
					"Sv12" => Ok(LanguageStd::Sv12),
					"Sv17" => Ok(LanguageStd::Sv17),
					"Sv23" => Ok(LanguageStd::Sv23),
					"Vams09" => Ok(LanguageStd::Vams09),
					"Vams14" => Ok(LanguageStd::Vams14),
					"Vams23" => Ok(LanguageStd::Vams23),
					_ => Err(serde::de::Error::unknown_variant(value, &VALUES)),
				}
			}
		}

		deserializer.deserialize_str(ValueVisitor)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for LanguageStd {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match *self {
			LanguageStd::Vl95 => serializer.serialize_str("Vl95"),
			LanguageStd::Vl01 => serializer.serialize_str("Vl01"),
			LanguageStd::Vl05 => serializer.serialize_str("Vl05"),
			LanguageStd::Sv05 => serializer.serialize_str("Sv05"),
			LanguageStd::Sv09 => serializer.serialize_str("Sv09"),
			LanguageStd::Sv12 => serializer.serialize_str("Sv12"),
			LanguageStd::Sv17 => serializer.serialize_str("Sv17"),
			LanguageStd::Sv23 => serializer.serialize_str("Sv23"),
			LanguageStd::Vams09 => serializer.serialize_str("Vams09"),
			LanguageStd::Vams14 => serializer.serialize_str("Vams14"),
			LanguageStd::Vams23 => serializer.serialize_str("Vams23"),
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
		assert_tokens(&LanguageStd::Vl95, &[Token::Str("Vl95")]);
		assert_tokens(&LanguageStd::Vl01, &[Token::Str("Vl01")]);
		assert_tokens(&LanguageStd::Vl05, &[Token::Str("Vl05")]);
		assert_tokens(&LanguageStd::Sv05, &[Token::Str("Sv05")]);
		assert_tokens(&LanguageStd::Sv09, &[Token::Str("Sv09")]);
		assert_tokens(&LanguageStd::Sv12, &[Token::Str("Sv12")]);
		assert_tokens(&LanguageStd::Sv17, &[Token::Str("Sv17")]);
		assert_tokens(&LanguageStd::Sv23, &[Token::Str("Sv23")]);
		assert_tokens(&LanguageStd::Vams09, &[Token::Str("Vams09")]);
		assert_tokens(&LanguageStd::Vams14, &[Token::Str("Vams14")]);
		assert_tokens(&LanguageStd::Vams23, &[Token::Str("Vams23")]);
	}
}
