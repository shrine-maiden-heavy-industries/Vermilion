// SPDX-License-Identifier: BSD-3-Clause
//! Serialization implementations for XML Schema types

macro_rules! thin_serialize {
	($type:ty, $inner:ty) => {
		impl serde::Serialize for $type {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: serde::Serializer,
			{
				// Dispatch to the Serialize impl for our inner type
				<$inner as serde::Serialize>::serialize(&self.inner, serializer)
			}
		}
	};
}

thin_serialize!(super::Double, f64);
