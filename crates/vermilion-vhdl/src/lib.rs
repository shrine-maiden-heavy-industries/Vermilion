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
	Vh87,
	Vh93,
	Vh2k,
	Vh02,
	Vh04,
	Vh07,
	Vh08,
	Vh11,
	Vh19,
	Vh23,
	Vhams99,
	Vhams07,
	Vhams09,
	Vhams17,
	Vhams21,
}

impl LanguageStd {
	pub const VHDL_AMS_KNOWN_EXTS: [&'static str; 2] = ["vhd", "vhdl"];
	pub const VHDL_AMS_STDS: LanguageStd = LanguageStd::Vh87
		.or(LanguageStd::Vh93)
		.or(LanguageStd::Vh2k)
		.or(LanguageStd::Vh02)
		.or(LanguageStd::Vh04)
		.or(LanguageStd::Vh07)
		.or(LanguageStd::Vh08)
		.or(LanguageStd::Vh11)
		.or(LanguageStd::Vh19)
		.or(LanguageStd::Vh23);
	pub const VHDL_KNOWN_EXTS: [&'static str; 2] = ["vhd", "vhdl"];
	pub const VHDL_STDS: LanguageStd = LanguageStd::Vhams99
		.or(LanguageStd::Vhams07)
		.or(LanguageStd::Vhams09)
		.or(LanguageStd::Vhams17)
		.or(LanguageStd::Vhams21);

	pub fn has_single_std(&self) -> bool {
		self.bits.count_ones() == 1
	}

	pub fn popcount(&self) -> u32 {
		self.bits.count_ones()
	}

	pub fn is_vhdl(&self) -> bool {
		self.contains(Self::VHDL_STDS)
	}

	pub fn is_vhdl_ams(&self) -> bool {
		self.contains(Self::VHDL_AMS_STDS)
	}
}

impl Display for LanguageStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(LanguageStd::Vh87) {
			write!(f, "VHDL 1987 (IEEE 1076-1987)")?;
		}

		if self.contains(LanguageStd::Vh93) {
			write!(f, "VHDL 1993 (IEEE 1076-1993)")?;
		}

		if self.contains(LanguageStd::Vh2k) {
			write!(f, "VHDL 2000 (IEEE 1076-2000)")?;
		}

		if self.contains(LanguageStd::Vh02) {
			write!(f, "VHDL 2002 (IEEE 1076-2002)")?;
		}

		if self.contains(LanguageStd::Vh04) {
			write!(f, "VHDL 2004 (IEC 61691-1-1:2004)")?;
		}

		if self.contains(LanguageStd::Vh07) {
			write!(f, "VHDL 2007 (IEEE 1076-2007)")?;
		}

		if self.contains(LanguageStd::Vh08) {
			write!(f, "VHDL 2008 (IEEE 1076-2008)")?;
		}

		if self.contains(LanguageStd::Vh11) {
			write!(f, "VHDL 2011 (IEC 61691-1-1:2011)")?;
		}

		if self.contains(LanguageStd::Vh19) {
			write!(f, "VHDL 2019 (IEEE 1076-2019)")?;
		}

		if self.contains(LanguageStd::Vh23) {
			write!(f, "VHDL 2023 (IEC 61691-1-1:2023)")?;
		}

		if self.contains(LanguageStd::Vhams99) {
			write!(f, "VHDL-AMS 1999 (IEEE 1076.1-1999)")?;
		}

		if self.contains(LanguageStd::Vhams07) {
			write!(f, "VHDL-AMS 2007 (IEEE 1076.1-2007)")?;
		}

		if self.contains(LanguageStd::Vhams09) {
			write!(f, "VHDL-AMS 2009 (IEC 61691-6:2009)")?;
		}

		if self.contains(LanguageStd::Vhams17) {
			write!(f, "VHDL-AMS 2017 (IEEE 1076.1-2017)")?;
		}

		if self.contains(LanguageStd::Vhams21) {
			write!(f, "VHDL-AMS 2021 (IEC 61691-6:2021)")?;
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
		static VALUES: [&str; 14] = [
			"Vh87", "Vh93", "Vh2k", "Vh04", "Vh07", "Vh08", "Vh11", "Vh19", "Vh23", "Vhams99",
			"Vhams07", "Vhams09", "Vhams17", "Vhams21",
		];

		struct ValueVisitor;
		impl<'de> serde::de::Visitor<'de> for ValueVisitor {
			type Value = LanguageStd;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str(
					"`Vh87`, `Vh93`, `Vh2k`, `Vh04`, `Vh07`, `Vh08`, `Vh11`, `Vh19`, `Vh23`, \
					 `Vhams99`, `Vhams07`, `Vhams09`, `Vhams17`, or `Vhams21`",
				)
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				match value {
					"Vh87" => Ok(LanguageStd::Vh87),
					"Vh93" => Ok(LanguageStd::Vh93),
					"Vh2k" => Ok(LanguageStd::Vh2k),
					"Vh04" => Ok(LanguageStd::Vh04),
					"Vh07" => Ok(LanguageStd::Vh07),
					"Vh08" => Ok(LanguageStd::Vh08),
					"Vh11" => Ok(LanguageStd::Vh11),
					"Vh19" => Ok(LanguageStd::Vh19),
					"Vh23" => Ok(LanguageStd::Vh23),
					"Vhams99" => Ok(LanguageStd::Vhams99),
					"Vhams07" => Ok(LanguageStd::Vhams07),
					"Vhams09" => Ok(LanguageStd::Vhams09),
					"Vhams17" => Ok(LanguageStd::Vhams17),
					"Vhams21" => Ok(LanguageStd::Vhams21),
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
			LanguageStd::Vh87 => serializer.serialize_str("Vh87"),
			LanguageStd::Vh93 => serializer.serialize_str("Vh93"),
			LanguageStd::Vh2k => serializer.serialize_str("Vh2k"),
			LanguageStd::Vh04 => serializer.serialize_str("Vh04"),
			LanguageStd::Vh07 => serializer.serialize_str("Vh07"),
			LanguageStd::Vh08 => serializer.serialize_str("Vh08"),
			LanguageStd::Vh11 => serializer.serialize_str("Vh11"),
			LanguageStd::Vh19 => serializer.serialize_str("Vh19"),
			LanguageStd::Vh23 => serializer.serialize_str("Vh23"),
			LanguageStd::Vhams99 => serializer.serialize_str("Vhams99"),
			LanguageStd::Vhams07 => serializer.serialize_str("Vhams07"),
			LanguageStd::Vhams09 => serializer.serialize_str("Vhams09"),
			LanguageStd::Vhams17 => serializer.serialize_str("Vhams17"),
			LanguageStd::Vhams21 => serializer.serialize_str("Vhams21"),
			_ => Err(serde::ser::Error::custom(
				"Unable to serialize `LanguageStd` with more than one bit set",
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
		assert_tokens(&LanguageStd::Vh87, &[Token::Str("Vh87")]);
		assert_tokens(&LanguageStd::Vh93, &[Token::Str("Vh93")]);
		assert_tokens(&LanguageStd::Vh2k, &[Token::Str("Vh2k")]);
		assert_tokens(&LanguageStd::Vh04, &[Token::Str("Vh04")]);
		assert_tokens(&LanguageStd::Vh07, &[Token::Str("Vh07")]);
		assert_tokens(&LanguageStd::Vh08, &[Token::Str("Vh08")]);
		assert_tokens(&LanguageStd::Vh11, &[Token::Str("Vh11")]);
		assert_tokens(&LanguageStd::Vh19, &[Token::Str("Vh19")]);
		assert_tokens(&LanguageStd::Vh23, &[Token::Str("Vh23")]);
		assert_tokens(&LanguageStd::Vhams99, &[Token::Str("Vhams99")]);
		assert_tokens(&LanguageStd::Vhams07, &[Token::Str("Vhams07")]);
		assert_tokens(&LanguageStd::Vhams09, &[Token::Str("Vhams09")]);
		assert_tokens(&LanguageStd::Vhams17, &[Token::Str("Vhams17")]);
		assert_tokens(&LanguageStd::Vhams21, &[Token::Str("Vhams21")]);
	}
}
