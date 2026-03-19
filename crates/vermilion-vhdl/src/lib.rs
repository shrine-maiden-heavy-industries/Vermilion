// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

use std::fmt::Display;

use bitmask_enum::bitmask;
use vermilion_macros::cfg_serde;

pub mod config;
pub mod diagnostics;
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
	pub const VHDL_AMS_STDS: Self = Self::Vhams99
		.or(Self::Vhams07)
		.or(Self::Vhams09)
		.or(Self::Vhams17)
		.or(Self::Vhams21);
	pub const VHDL_KNOWN_EXTS: [&'static str; 2] = ["vhd", "vhdl"];
	pub const VHDL_STDS: Self = Self::Vh87
		.or(Self::Vh93)
		.or(Self::Vh2k)
		.or(Self::Vh02)
		.or(Self::Vh04)
		.or(Self::Vh07)
		.or(Self::Vh08)
		.or(Self::Vh11)
		.or(Self::Vh19)
		.or(Self::Vh23);

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

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for LanguageStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(Self::Vh87) {
			write!(f, "VHDL 1987 (IEEE 1076-1987)")?;
		}

		if self.contains(Self::Vh93) {
			write!(f, "VHDL 1993 (IEEE 1076-1993)")?;
		}

		if self.contains(Self::Vh2k) {
			write!(f, "VHDL 2000 (IEEE 1076-2000)")?;
		}

		if self.contains(Self::Vh02) {
			write!(f, "VHDL 2002 (IEEE 1076-2002)")?;
		}

		if self.contains(Self::Vh04) {
			write!(f, "VHDL 2004 (IEC 61691-1-1:2004)")?;
		}

		if self.contains(Self::Vh07) {
			write!(f, "VHDL 2007 (IEEE 1076-2007)")?;
		}

		if self.contains(Self::Vh08) {
			write!(f, "VHDL 2008 (IEEE 1076-2008)")?;
		}

		if self.contains(Self::Vh11) {
			write!(f, "VHDL 2011 (IEC 61691-1-1:2011)")?;
		}

		if self.contains(Self::Vh19) {
			write!(f, "VHDL 2019 (IEEE 1076-2019)")?;
		}

		if self.contains(Self::Vh23) {
			write!(f, "VHDL 2023 (IEC 61691-1-1:2023)")?;
		}

		if self.contains(Self::Vhams99) {
			write!(f, "VHDL-AMS 1999 (IEEE 1076.1-1999)")?;
		}

		if self.contains(Self::Vhams07) {
			write!(f, "VHDL-AMS 2007 (IEEE 1076.1-2007)")?;
		}

		if self.contains(Self::Vhams09) {
			write!(f, "VHDL-AMS 2009 (IEC 61691-6:2009)")?;
		}

		if self.contains(Self::Vhams17) {
			write!(f, "VHDL-AMS 2017 (IEEE 1076.1-2017)")?;
		}

		if self.contains(Self::Vhams21) {
			write!(f, "VHDL-AMS 2021 (IEC 61691-6:2021)")?;
		}

		Ok(())
	}
}

cfg_serde! {
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


	impl serde::Serialize for LanguageStd {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
		{
			match *self {
				Self::Vh87 => serializer.serialize_str("Vh87"),
				Self::Vh93 => serializer.serialize_str("Vh93"),
				Self::Vh2k => serializer.serialize_str("Vh2k"),
				Self::Vh04 => serializer.serialize_str("Vh04"),
				Self::Vh07 => serializer.serialize_str("Vh07"),
				Self::Vh08 => serializer.serialize_str("Vh08"),
				Self::Vh11 => serializer.serialize_str("Vh11"),
				Self::Vh19 => serializer.serialize_str("Vh19"),
				Self::Vh23 => serializer.serialize_str("Vh23"),
				Self::Vhams99 => serializer.serialize_str("Vhams99"),
				Self::Vhams07 => serializer.serialize_str("Vhams07"),
				Self::Vhams09 => serializer.serialize_str("Vhams09"),
				Self::Vhams17 => serializer.serialize_str("Vhams17"),
				Self::Vhams21 => serializer.serialize_str("Vhams21"),
				_ => Err(serde::ser::Error::custom(
					"Unable to serialize `LanguageStd` with more than one bit set",
				)),
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	cfg_serde! {
		use serde_test::{Token, assert_tokens};

		#[test]
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
}
