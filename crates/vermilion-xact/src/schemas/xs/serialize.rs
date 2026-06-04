// SPDX-License-Identifier: BSD-3-Clause
//! Serialization implementations for XML Schema types

impl serde::Serialize for super::Double {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_f64(self.inner)
	}
}
