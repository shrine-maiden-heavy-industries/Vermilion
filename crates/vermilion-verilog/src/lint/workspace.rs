// SPDX-License-Identifier: BSD-3-Clause

use vermilion_diagnostics::workspace::CommonLintWorkspace;

/// Workspace specific common Verilog/SystemVerilog/Verilog-AMS lint options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogCommonLint {
	/// Workspace specific common Vermilion lint options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub workspace_common: CommonLintWorkspace,
}

/// Workspace specific Verilog lint options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogLint {
	/// Common Verilog family lint options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VerilogCommonLint,
}

/// Workspace specific SystemVerilog lint options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SystemVerilogLint {
	/// Common Verilog family lint options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VerilogCommonLint,
}

/// Workspace specific Verilog-AMS lint options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogAmsLint {
	/// Common Verilog family lint options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VerilogCommonLint,
}
