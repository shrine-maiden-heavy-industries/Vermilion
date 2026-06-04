// SPDX-License-Identifier: BSD-3-Clause
//! JSON Schema implementations for XML Schema types.

impl schemars::JsonSchema for super::Double {
	fn schema_name() -> std::borrow::Cow<'static, str> {
		"double".into()
	}

	fn schema_id() -> std::borrow::Cow<'static, str> {
		"xs::double".into()
	}

	fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
		schemars::json_schema!({
			"description": "W3C XML Schema Double",
			"type": "number",
		})
	}
}
