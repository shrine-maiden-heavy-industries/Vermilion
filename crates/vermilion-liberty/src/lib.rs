// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fmt::Display;

use bitmask_enum::bitmask;

pub mod lang;

#[bitmask(u16)]
pub enum LanguageStd {
	Liberty2014_09,
	Liberty2015_12,
	Liberty2016_06,
	Liberty2016_12,
	Liberty2017_06,
	Liberty2018_06,
	Liberty2019_03,
	Liberty2019_12,
	Liberty2020_09,
}

impl LanguageStd {
	pub const LIBERTY_KNOWN_EXTS: [&'static str; 1] = ["lib"];

	pub fn has_single_std(&self) -> bool {
		self.bits.count_ones() == 1
	}

	pub fn popcount(&self) -> u32 {
		self.bits.count_ones()
	}
}

impl Display for LanguageStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(LanguageStd::Liberty2014_09) {
			f.write_str("Synopsys Liberty 2014.09")?;
		}

		if self.contains(LanguageStd::Liberty2015_12) {
			f.write_str("Synopsys Liberty 2015.12")?;
		}

		if self.contains(LanguageStd::Liberty2016_06) {
			f.write_str("Synopsys Liberty 2016.06")?;
		}

		if self.contains(LanguageStd::Liberty2016_12) {
			f.write_str("Synopsys Liberty 2016.12")?;
		}

		if self.contains(LanguageStd::Liberty2017_06) {
			f.write_str("Synopsys Liberty 2017.06")?;
		}

		if self.contains(LanguageStd::Liberty2018_06) {
			f.write_str("Synopsys Liberty 2018.06i")?;
		}

		if self.contains(LanguageStd::Liberty2019_03) {
			f.write_str("Synopsys Liberty 2019.03")?;
		}

		if self.contains(LanguageStd::Liberty2019_12) {
			f.write_str("Synopsys Liberty 2019.12")?;
		}

		if self.contains(LanguageStd::Liberty2020_09) {
			f.write_str("Synopsys Liberty 2020.09")?;
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
		static VALUES: [&str; 9] = [
			"2020.09", "2019.12", "2019.03", "2018.06", "2017.06", "2016.12", "2016.06", "2015.12",
			"2014.09",
		];

		struct ValueVisitor;
		impl<'de> serde::de::Visitor<'de> for ValueVisitor {
			type Value = LanguageStd;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str(
					"`2020.09`, `2019.12`, `2019.03`, `2018.06`, `2017.06`, `2016.12`, `2016.06`, \
					 `2015.12`, or `2014.09`",
				)
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				match value {
					"2014.09" => Ok(LanguageStd::Liberty2014_09),
					"2015.12" => Ok(LanguageStd::Liberty2015_12),
					"2016.06" => Ok(LanguageStd::Liberty2016_06),
					"2016.12" => Ok(LanguageStd::Liberty2016_12),
					"2017.06" => Ok(LanguageStd::Liberty2017_06),
					"2018.06" => Ok(LanguageStd::Liberty2018_06),
					"2019.03" => Ok(LanguageStd::Liberty2019_03),
					"2019.12" => Ok(LanguageStd::Liberty2019_12),
					"2020.09" => Ok(LanguageStd::Liberty2020_09),
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
			LanguageStd::Liberty2014_09 => serializer.serialize_str("2014.09"),
			LanguageStd::Liberty2015_12 => serializer.serialize_str("2015.12"),
			LanguageStd::Liberty2016_06 => serializer.serialize_str("2016.06"),
			LanguageStd::Liberty2016_12 => serializer.serialize_str("2016.12"),
			LanguageStd::Liberty2017_06 => serializer.serialize_str("2017.06"),
			LanguageStd::Liberty2018_06 => serializer.serialize_str("2018.06"),
			LanguageStd::Liberty2019_03 => serializer.serialize_str("2019.03"),
			LanguageStd::Liberty2019_12 => serializer.serialize_str("2019.12"),
			LanguageStd::Liberty2020_09 => serializer.serialize_str("2020.09"),
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
		assert_tokens(&LanguageStd::Liberty2014_09, &[Token::Str("2014.09")]);
		assert_tokens(&LanguageStd::Liberty2015_12, &[Token::Str("2015.12")]);
		assert_tokens(&LanguageStd::Liberty2016_06, &[Token::Str("2016.06")]);
		assert_tokens(&LanguageStd::Liberty2016_12, &[Token::Str("2016.12")]);
		assert_tokens(&LanguageStd::Liberty2017_06, &[Token::Str("2017.06")]);
		assert_tokens(&LanguageStd::Liberty2018_06, &[Token::Str("2018.06")]);
		assert_tokens(&LanguageStd::Liberty2019_03, &[Token::Str("2019.03")]);
		assert_tokens(&LanguageStd::Liberty2019_12, &[Token::Str("2019.12")]);
		assert_tokens(&LanguageStd::Liberty2020_09, &[Token::Str("2020.09")]);
	}
}
