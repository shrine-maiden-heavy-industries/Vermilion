// SPDX-License-Identifier: BSD-3-Clause

use vermilion_diagnostics::workspace::CommonLintWorkspace;

/// Workspace specific common VHDL/VHDL-AMS lint options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlCommonLint {
	/// Workspace specific common Vermilion lint options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub workspace_common: CommonLintWorkspace,
}

/// Workspace specific VHDL lint options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlLint {
	/// Common VHDL family lint options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VhdlCommonLint,
}

/// Workspace specific VHDL-AMS lint options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlAmsLint {
	/// Common VHDL family lint options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub common: VhdlCommonLint,
}
