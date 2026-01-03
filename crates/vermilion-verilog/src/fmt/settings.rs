// SPDX-License-Identifier: BSD-3-Clause

use vermilion_fmt::settings::CommonFormat;

#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogFormat {
	#[cfg_attr(feature = "serde", serde(flatten))]
	common: CommonFormat,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SystemVerilogFormat {
	#[cfg_attr(feature = "serde", serde(flatten))]
	common: CommonFormat,
}
