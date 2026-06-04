// SPDX-License-Identifier: BSD-3-Clause
//! Deserialization implementations for XML Schema types

macro_rules! thin_deserialize {
	($type:ty, $inner:ty) => {
		impl<'de> serde::Deserialize<'de> for $type {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				// Dispatch to the Deserialize impl for our inner type
				Ok(Self {
					inner: <$inner as serde::Deserialize>::deserialize(deserializer)?,
				})
			}
		}
	};
}

thin_deserialize!(super::Double, f64);
