// SPDX-License-Identifier: BSD-3-Clause

use vermilion_macros::cfg_schema;

use crate::{
	LanguageStd,
	fmt::workspace::{VhdlAmsFormat, VhdlFormat},
	lint::workspace::{VhdlAmsLint, VhdlLint},
};

#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
pub struct VhdlWorkspace {
	std:  LanguageStd,
	fmt:  Option<VhdlFormat>,
	lint: Option<VhdlLint>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
pub struct VhdlAmsWorkspace {
	std:  LanguageStd,
	fmt:  Option<VhdlAmsFormat>,
	lint: Option<VhdlAmsLint>,
}

impl Default for VhdlWorkspace {
	fn default() -> Self {
		Self {
			std:  LanguageStd::Vh87,
			fmt:  Default::default(),
			lint: Default::default(),
		}
	}
}

impl Default for VhdlAmsWorkspace {
	fn default() -> Self {
		Self {
			std:  LanguageStd::Vhams99,
			fmt:  Default::default(),
			lint: Default::default(),
		}
	}
}

cfg_schema! {
	#[cfg_attr(coverage_nightly, coverage(off))]
	impl schemars::JsonSchema for VhdlWorkspace {
		fn schema_name() -> std::borrow::Cow<'static, str> {
			"VhdlWorkspace".into()
		}

		fn schema_id() -> std::borrow::Cow<'static, str> {
			concat!(module_path!(), "::VhdlWorkspace").into()
		}

		fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
			let fmt_schema = generator.subschema_for::<Option<VhdlAmsFormat>>();
			let lint_schema = generator.subschema_for::<Option<VhdlAmsLint>>();
			let std_schema = schemars::json_schema!({
				"description": "VHDL Standard",
				"type": "string",
				"enum": [
					"Vh87",
					"Vh93",
					"Vh2k",
					"Vh04",
					"Vh07",
					"Vh08",
					"Vh11",
					"Vh19",
					"Vh23",
				]
			});

			schemars::json_schema!({
				"description": "VHDL Workspace Configuration",
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
	impl schemars::JsonSchema for VhdlAmsWorkspace {
		fn schema_name() -> std::borrow::Cow<'static, str> {
			"VhdlAmsWorkspace".into()
		}

		fn schema_id() -> std::borrow::Cow<'static, str> {
			concat!(module_path!(), "::VhdlAmsWorkspace").into()
		}

		fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
			let fmt_schema = generator.subschema_for::<Option<VhdlAmsFormat>>();
			let lint_schema = generator.subschema_for::<Option<VhdlAmsLint>>();
			let std_schema = schemars::json_schema!({
				"description": "VHDL-AMS Standard",
				"type": "string",
				"enum": [
					"Vhams99",
					"Vhams07",
					"Vhams09",
					"Vhams17",
					"Vhams21",
				]
			});

			schemars::json_schema!({
				"description": "VHDL-AMS Workspace Configuration",
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
