// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fmt::Display;

use bitmask_enum::bitmask;

pub mod lpf;
pub mod sdc;

#[bitmask(u16)]
pub enum LanguageStd {
	Lpf,
	Pdc,
	Sdc,
	Xdc,
}

impl LanguageStd {
	pub const LPF_KNOWN_EXTS: [&'static str; 1] = ["lpf"];
	pub const PDC_KNOWN_EXTS: [&'static str; 1] = ["pdc"];
	pub const SDC_KNOWN_EXTS: [&'static str; 1] = ["sdc"];
	pub const XDC_KNOWN_EXTS: [&'static str; 1] = ["xdc"];

	pub fn has_single_std(&self) -> bool {
		self.bits.count_ones() == 1
	}

	pub fn popcount(&self) -> u32 {
		self.bits.count_ones()
	}

	pub fn is_lpf(&self) -> bool {
		self.contains(Self::Lpf)
	}

	pub fn is_pdc(&self) -> bool {
		self.contains(Self::Pdc)
	}

	pub fn is_sdc(&self) -> bool {
		self.contains(Self::Sdc)
	}

	pub fn is_xdc(&self) -> bool {
		self.contains(Self::Xdc)
	}
}

impl Display for LanguageStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(LanguageStd::Lpf) {
			f.write_str("Lattice Diamond Design Constraints")?;
		}

		if self.contains(LanguageStd::Pdc) {
			f.write_str("Lattice Radiant Design Constraints")?;
		}

		if self.contains(LanguageStd::Sdc) {
			f.write_str("Synopsys Design Constraints")?;
		}

		if self.contains(LanguageStd::Xdc) {
			f.write_str("Xilinx Design Constraints")?;
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
		static VALUES: [&str; 4] = ["lpf", "pdc", "sdc", "xdc"];

		struct ValueVisitor;
		impl<'de> serde::de::Visitor<'de> for ValueVisitor {
			type Value = LanguageStd;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("`lpf`, `pdc`, `sdc`, or `xdc`")
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				match value {
					"lpf" => Ok(LanguageStd::Lpf),
					"pdc" => Ok(LanguageStd::Pdc),
					"sdc" => Ok(LanguageStd::Sdc),
					"xdc" => Ok(LanguageStd::Xdc),
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
			LanguageStd::Lpf => serializer.serialize_str("lpf"),
			LanguageStd::Pdc => serializer.serialize_str("pdc"),
			LanguageStd::Sdc => serializer.serialize_str("sdc"),
			LanguageStd::Xdc => serializer.serialize_str("xdc"),
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
		assert_tokens(&LanguageStd::Lpf, &[Token::Str("lpf")]);
		assert_tokens(&LanguageStd::Pdc, &[Token::Str("pdc")]);
		assert_tokens(&LanguageStd::Sdc, &[Token::Str("sdc")]);
		assert_tokens(&LanguageStd::Xdc, &[Token::Str("xdc")]);
	}
}
