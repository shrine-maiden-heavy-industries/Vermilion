// SPDX-License-Identifier: BSD-3-Clause

use vermilion_macros::cfg_schema;

use crate::{
	LanguageStd,
	fmt::workspace::{SystemVerilogFormat, VerilogAmsFormat, VerilogFormat},
	lint::workspace::{SystemVerilogLint, VerilogAmsLint, VerilogLint},
};

#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
pub struct VerilogWorkspace {
	std:  LanguageStd,
	fmt:  Option<VerilogFormat>,
	lint: Option<VerilogLint>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
pub struct SystemVerilogWorkspace {
	std:  LanguageStd,
	fmt:  Option<SystemVerilogFormat>,
	lint: Option<SystemVerilogLint>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
pub struct VerilogAmsWorkspace {
	std:  LanguageStd,
	fmt:  Option<VerilogAmsFormat>,
	lint: Option<VerilogAmsLint>,
}

impl Default for VerilogWorkspace {
	fn default() -> Self {
		Self {
			std:  LanguageStd::Vl95,
			fmt:  Default::default(),
			lint: Default::default(),
		}
	}
}

impl Default for SystemVerilogWorkspace {
	fn default() -> Self {
		Self {
			std:  LanguageStd::Sv05,
			fmt:  Default::default(),
			lint: Default::default(),
		}
	}
}

impl Default for VerilogAmsWorkspace {
	fn default() -> Self {
		Self {
			std:  LanguageStd::Vams09,
			fmt:  Default::default(),
			lint: Default::default(),
		}
	}
}

cfg_schema! {
	#[cfg_attr(coverage_nightly, coverage(off))]
	impl schemars::JsonSchema for VerilogWorkspace {
		fn schema_name() -> std::borrow::Cow<'static, str> {
			"VerilogWorkspace".into()
		}

		fn schema_id() -> std::borrow::Cow<'static, str> {
			concat!(module_path!(), "::VerilogWorkspace").into()
		}

		fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
			let fmt_schema = generator.subschema_for::<Option<VerilogFormat>>();
			let lint_schema = generator.subschema_for::<Option<VerilogLint>>();
			let std_schema = schemars::json_schema!({
				"description": "Verilog Standard",
				"type": "string",
				"enum": [
					"Vl95",
					"Vl01",
					"Vl05"
				]
			});

			schemars::json_schema!({
				"description": "Verilog Workspace Configuration",
				"type": "object",
				"properties": {
					"std": std_schema,
					"fmt": fmt_schema,
					"lint": lint_schema,
				}
			})
		}
	}

	#[cfg_attr(coverage_nightly, coverage(off))]
	impl schemars::JsonSchema for SystemVerilogWorkspace {
		fn schema_name() -> std::borrow::Cow<'static, str> {
			"SystemVerilogWorkspace".into()
		}

		fn schema_id() -> std::borrow::Cow<'static, str> {
			concat!(module_path!(), "::SystemVerilogWorkspace").into()
		}

		fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
			let fmt_schema = generator.subschema_for::<Option<SystemVerilogFormat>>();
			let lint_schema = generator.subschema_for::<Option<SystemVerilogLint>>();
			let std_schema = schemars::json_schema!({
				"description": "SystemVerilog Standard",
				"type": "string",
				"enum": [
					"Sv05",
					"Sv09",
					"Sv12",
					"Sv17",
					"Sv23"
				]
			});

			schemars::json_schema!({
				"description": "SystemVerilog Workspace Configuration",
				"type": "object",
				"properties": {
					"std": std_schema,
					"fmt": fmt_schema,
					"lint": lint_schema,
				}
			})
		}
	}

	#[cfg_attr(coverage_nightly, coverage(off))]
	impl schemars::JsonSchema for VerilogAmsWorkspace {
		fn schema_name() -> std::borrow::Cow<'static, str> {
			"VerilogAmsWorkspace".into()
		}

		fn schema_id() -> std::borrow::Cow<'static, str> {
			concat!(module_path!(), "::VerilogAmsWorkspace").into()
		}

		fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
			let fmt_schema = generator.subschema_for::<Option<VerilogAmsFormat>>();
			let lint_schema = generator.subschema_for::<Option<VerilogAmsLint>>();
			let std_schema = schemars::json_schema!({
				"description": "Verilog-AMS Standard",
				"type": "string",
				"enum": [
					"Vams09",
					"Vams14",
					"Vams23",
				]
			});

			schemars::json_schema!({
				"description": "Verilog-AMS Workspace Configuration",
				"type": "object",
				"properties": {
					"std": std_schema,
					"fmt": fmt_schema,
					"lint": lint_schema,
				}
			})
		}
	}
}
