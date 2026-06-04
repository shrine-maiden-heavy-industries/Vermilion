// SPDX-License-Identifier: BSD-3-Clause
//! JSON Schema implementations for XML Schema types.

macro_rules! thin_schema {
	($type:ty, $name:literal, $desc:literal, $schema_type:literal) => {
		impl schemars::JsonSchema for $type {
			fn schema_name() -> std::borrow::Cow<'static, str> {
				$name.into()
			}

			fn schema_id() -> std::borrow::Cow<'static, str> {
				concat!("xs::", $name).into()
			}

			fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
				schemars::json_schema!({
					"description": $desc,
					"type": $schema_type,
				})
			}
		}
	};
}

thin_schema!(super::Double, "double", "W3C XML Schema double", "number");
