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
pub mod lpf;
pub mod sdc;
pub mod workspace;

#[bitmask(u16)]
pub enum LanguageStd {
	Lpf,
	Pdc,
	Sdc,
	Sdc_1_3,
	Sdc_1_4,
	Sdc_1_5,
	Sdc_1_6,
	Sdc_1_7,
	Sdc_1_9,
	Sdc_2_0,
	Sdc_2_1,
	Sdc_2_2,
	Xdc,
}

impl LanguageStd {
	pub const LPF_KNOWN_EXTS: [&'static str; 1] = ["lpf"];
	pub const LPF_STDS: Self = Self::Lpf;
	pub const PDC_KNOWN_EXTS: [&'static str; 1] = ["pdc"];
	pub const PDC_STDS: Self = Self::Pdc;
	pub const SDC_KNOWN_EXTS: [&'static str; 1] = ["sdc"];
	pub const SDC_STDS: Self = Self::Sdc
		.or(Self::Sdc_1_3)
		.or(Self::Sdc_1_4)
		.or(Self::Sdc_1_5)
		.or(Self::Sdc_1_6)
		.or(Self::Sdc_1_7)
		.or(Self::Sdc_1_9)
		.or(Self::Sdc_2_0)
		.or(Self::Sdc_2_1)
		.or(Self::Sdc_2_2);
	pub const XDC_KNOWN_EXTS: [&'static str; 1] = ["xdc"];
	pub const XDC_STDS: Self = Self::Xdc;

	pub fn has_single_std(&self) -> bool {
		self.bits.count_ones() == 1
	}

	pub fn popcount(&self) -> u32 {
		self.bits.count_ones()
	}

	pub fn is_lpf(&self) -> bool {
		self.contains(Self::LPF_STDS)
	}

	pub fn is_pdc(&self) -> bool {
		self.contains(Self::PDC_STDS)
	}

	pub fn is_sdc(&self) -> bool {
		self.contains(Self::SDC_STDS)
	}

	pub fn is_xdc(&self) -> bool {
		self.contains(Self::XDC_STDS)
	}
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for LanguageStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(Self::Lpf) {
			f.write_str("Lattice Diamond Logical Preference File")?;
		}

		if self.contains(Self::Pdc) {
			f.write_str("Lattice Radiant Design Constraints")?;
		}

		if self.contains(Self::Sdc) {
			f.write_str("Synopsys Design Constraints")?;
		}

		if self.contains(Self::Sdc_1_3) {
			f.write_str("Synopsys Design Constraints 1.3")?;
		}

		if self.contains(Self::Sdc_1_4) {
			f.write_str("Synopsys Design Constraints 1.4")?;
		}

		if self.contains(Self::Sdc_1_5) {
			f.write_str("Synopsys Design Constraints 1.5")?;
		}

		if self.contains(Self::Sdc_1_6) {
			f.write_str("Synopsys Design Constraints 1.6")?;
		}

		if self.contains(Self::Sdc_1_7) {
			f.write_str("Synopsys Design Constraints 1.7")?;
		}

		if self.contains(Self::Sdc_1_9) {
			f.write_str("Synopsys Design Constraints 1.9")?;
		}

		if self.contains(Self::Sdc_2_0) {
			f.write_str("Synopsys Design Constraints 2.0")?;
		}

		if self.contains(Self::Sdc_2_1) {
			f.write_str("Synopsys Design Constraints 2.1")?;
		}

		if self.contains(Self::Sdc_2_2) {
			f.write_str("Synopsys Design Constraints 2.2")?;
		}

		if self.contains(Self::Xdc) {
			f.write_str("Xilinx Design Constraints")?;
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
			static VALUES: [&str; 13] = [
				"lpf", "pdc", "sdc", "sdc1.3", "sdc1.4", "sdc1.5", "sdc1.6", "sdc1.7", "sdc1.9",
				"sdc2.0", "sdc2.1", "sdc2.2",  "xdc"
			];

			struct ValueVisitor;
			impl<'de> serde::de::Visitor<'de> for ValueVisitor {
				type Value = LanguageStd;

				fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
					formatter.write_str(
						"`lpf`, `pdc`, `sdc`, `sdc1.3`, `sdc1.4`, `sdc1.5`, `sdc1.6`,  `sdc1.7`, \
						`sdc1.9`, `sdc2.0`, `sdc2.1`, `sdc2.2`, or `xdc`"
					)
				}

				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: serde::de::Error,
				{
					match value {
						"lpf" => Ok(LanguageStd::Lpf),
						"pdc" => Ok(LanguageStd::Pdc),
						"sdc" => Ok(LanguageStd::Sdc),
						"sdc1.3" => Ok(LanguageStd::Sdc_1_3),
						"sdc1.4" => Ok(LanguageStd::Sdc_1_4),
						"sdc1.5" => Ok(LanguageStd::Sdc_1_5),
						"sdc1.6" => Ok(LanguageStd::Sdc_1_6),
						"sdc1.7" => Ok(LanguageStd::Sdc_1_7),
						"sdc1.9" => Ok(LanguageStd::Sdc_1_9),
						"sdc2.0" => Ok(LanguageStd::Sdc_2_0),
						"sdc2.1" => Ok(LanguageStd::Sdc_2_1),
						"sdc2.2" => Ok(LanguageStd::Sdc_2_2),
						"xdc" => Ok(LanguageStd::Xdc),
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
				Self::Lpf => serializer.serialize_str("lpf"),
				Self::Pdc => serializer.serialize_str("pdc"),
				Self::Sdc => serializer.serialize_str("sdc"),
				Self::Sdc_1_3 => serializer.serialize_str("sdc1.3"),
				Self::Sdc_1_4 => serializer.serialize_str("sdc1.4"),
				Self::Sdc_1_5 => serializer.serialize_str("sdc1.5"),
				Self::Sdc_1_6 => serializer.serialize_str("sdc1.6"),
				Self::Sdc_1_7 => serializer.serialize_str("sdc1.7"),
				Self::Sdc_1_9 => serializer.serialize_str("sdc1.9"),
				Self::Sdc_2_0 => serializer.serialize_str("sdc2.0"),
				Self::Sdc_2_1 => serializer.serialize_str("sdc2.1"),
				Self::Sdc_2_2 => serializer.serialize_str("sdc2.2"),
				Self::Xdc => serializer.serialize_str("xdc"),
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
			assert_tokens(&LanguageStd::Lpf, &[Token::Str("lpf")]);
			assert_tokens(&LanguageStd::Pdc, &[Token::Str("pdc")]);
			assert_tokens(&LanguageStd::Sdc, &[Token::Str("sdc")]);
			assert_tokens(&LanguageStd::Sdc_1_3, &[Token::Str("sdc1.3")]);
			assert_tokens(&LanguageStd::Sdc_1_4, &[Token::Str("sdc1.4")]);
			assert_tokens(&LanguageStd::Sdc_1_5, &[Token::Str("sdc1.5")]);
			assert_tokens(&LanguageStd::Sdc_1_6, &[Token::Str("sdc1.6")]);
			assert_tokens(&LanguageStd::Sdc_1_7, &[Token::Str("sdc1.7")]);
			assert_tokens(&LanguageStd::Sdc_1_9, &[Token::Str("sdc1.9")]);
			assert_tokens(&LanguageStd::Sdc_2_0, &[Token::Str("sdc2.0")]);
			assert_tokens(&LanguageStd::Sdc_2_1, &[Token::Str("sdc2.1")]);
			assert_tokens(&LanguageStd::Sdc_2_2, &[Token::Str("sdc2.2")]);
			assert_tokens(&LanguageStd::Xdc, &[Token::Str("xdc")]);
		}
	}
}
