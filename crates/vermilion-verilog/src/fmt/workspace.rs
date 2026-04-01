// SPDX-License-Identifier: BSD-3-Clause

use vermilion_fmt::workspace::CommonFormatWorkspace;

/// Workspace specific common Verilog/SystemVerilog/Verilog-AMS formatting options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogCommonFormat {
	/// Workspace specific common Vermilion formatting options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub workspace_common: CommonFormatWorkspace,
}

/// Workspace specific Verilog formatting options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogFormat {
	/// Common Verilog family formatting options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VerilogCommonFormat,
}

/// Workspace specific SystemVerilog formatting options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SystemVerilogFormat {
	/// Common Verilog family formatting options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VerilogCommonFormat,
}

/// Workspace specific Verilog-AMS formatting options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogAmsFormat {
	/// Common Verilog family formatting options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VerilogCommonFormat,
}
