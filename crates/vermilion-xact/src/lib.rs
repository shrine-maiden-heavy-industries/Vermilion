// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

use std::fmt::Display;

pub mod config;
pub mod diagnostics;
pub mod schemas;
pub mod workspace;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum XactVersion {
	Spirit1_0,
	Spirit1_1,
	Spirit1_2,
	Spirit1_4,
	Spirit1_5,
	IpXact2009,
	IpXact2014,
	IpXact2022,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for XactVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Spirit1_0 => "SPIRIT 1.0",
				Self::Spirit1_1 => "SPIRIT 1.1",
				Self::Spirit1_2 => "SPIRIT 1.2",
				Self::Spirit1_4 => "SPIRIT 1.4",
				Self::Spirit1_5 => "SPIRIT 1.5",
				Self::IpXact2009 => "IP-XACT 2009 (IEEE 1685-2009)",
				Self::IpXact2014 => "IP-XACT 2014 (IEEE 1685-2014)",
				Self::IpXact2022 => "IP-XACT 2022 (IEEE 1685-2022)",
			}
		)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for XactVersion {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		static VALUES: [&str; 8] = [
			"SPIRIT 1.0",
			"SPIRIT 1.1",
			"SPIRIT 1.2",
			"SPIRIT 1.4",
			"SPIRIT 1.5",
			"IP-XACT 2009",
			"IP-XACT 2014",
			"IP-XACT 2022",
		];

		struct ValueVisitor;
		impl<'de> serde::de::Visitor<'de> for ValueVisitor {
			type Value = XactVersion;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str(
					"`SPIRIT 1.0`, `SPIRIT 1.1`, `SPIRIT 1.2`, `SPIRIT 1.4`, `SPIRIT 1.5`, \
					 `IP-XACT 2009`, `IP-XACT 2014`, or `IP-XACT 2022`",
				)
			}

			fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				match value {
					"SPIRIT 1.0" => Ok(XactVersion::Spirit1_0),
					"SPIRIT 1.1" => Ok(XactVersion::Spirit1_1),
					"SPIRIT 1.2" => Ok(XactVersion::Spirit1_2),
					"SPIRIT 1.4" => Ok(XactVersion::Spirit1_4),
					"SPIRIT 1.5" => Ok(XactVersion::Spirit1_5),
					"IP-XACT 2009" => Ok(XactVersion::IpXact2009),
					"IP-XACT 2014" => Ok(XactVersion::IpXact2014),
					"IP-XACT 2022" => Ok(XactVersion::IpXact2022),
					_ => Err(serde::de::Error::unknown_variant(value, &VALUES)),
				}
			}
		}

		deserializer.deserialize_str(ValueVisitor)
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for XactVersion {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match *self {
			Self::Spirit1_0 => serializer.serialize_str("SPIRIT 1.0"),
			Self::Spirit1_1 => serializer.serialize_str("SPIRIT 1.1"),
			Self::Spirit1_2 => serializer.serialize_str("SPIRIT 1.2"),
			Self::Spirit1_4 => serializer.serialize_str("SPIRIT 1.4"),
			Self::Spirit1_5 => serializer.serialize_str("SPIRIT 1.5"),
			Self::IpXact2009 => serializer.serialize_str("IP-XACT 2009"),
			Self::IpXact2014 => serializer.serialize_str("IP-XACT 2014"),
			Self::IpXact2022 => serializer.serialize_str("IP-XACT 2022"),
		}
	}
}

#[cfg(feature = "schema")]
#[cfg_attr(coverage_nightly, coverage(off))]
impl schemars::JsonSchema for XactVersion {
	fn schema_name() -> std::borrow::Cow<'static, str> {
		"XactVersion".into()
	}

	fn schema_id() -> std::borrow::Cow<'static, str> {
		concat!(module_path!(), "::XactVersion").into()
	}

	fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
		schemars::json_schema!({
			"description": "IP-XACT Schema Version",
			"type": "string",
			"enum": [
				"SPIRIT 1.0",
				"SPIRIT 1.1",
				"SPIRIT 1.2",
				"SPIRIT 1.4",
				"SPIRIT 1.5",
				"IP-XACT 2009",
				"IP-XACT 2014",
				"IP-XACT 2022",
			]
		})
	}
}

#[cfg(test)]
mod test {
	#[cfg(feature = "serde")]
	use serde_test::{Token, assert_tokens};

	use super::*;

	#[test]
	#[cfg(feature = "serde")]
	fn test_xact_version_serialize() {
		assert_tokens(&XactVersion::Spirit1_0, &[Token::Str("SPIRIT 1.0")]);
		assert_tokens(&XactVersion::Spirit1_1, &[Token::Str("SPIRIT 1.1")]);
		assert_tokens(&XactVersion::Spirit1_2, &[Token::Str("SPIRIT 1.2")]);
		assert_tokens(&XactVersion::Spirit1_4, &[Token::Str("SPIRIT 1.4")]);
		assert_tokens(&XactVersion::Spirit1_5, &[Token::Str("SPIRIT 1.5")]);
		assert_tokens(&XactVersion::IpXact2009, &[Token::Str("IP-XACT 2009")]);
		assert_tokens(&XactVersion::IpXact2014, &[Token::Str("IP-XACT 2014")]);
		assert_tokens(&XactVersion::IpXact2022, &[Token::Str("IP-XACT 2022")]);
	}
}
