// SPDX-License-Identifier: BSD-3-Clause
//! Deserialization implementations for XML Schema types

impl<'de> serde::Deserialize<'de> for super::Double {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct ValueVisitor;
		impl<'de> serde::de::Visitor<'de> for ValueVisitor {
			type Value = super::Double;

			fn expecting(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
				todo!()
			}

			fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				Ok(v.into())
			}
		}

		deserializer.deserialize_f64(ValueVisitor)
	}
}
