// SPDX-License-Identifier: BSD-3-Clause

use vermilion_fmt::workspace::CommonFormatWorkspace;

/// Workspace specific common VHDL/VHDL-AMS formatting options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlCommonFormat {
	/// Common VHDL family formatting options
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub workspace_common: CommonFormatWorkspace,
}

/// Workspace specific VHDL formatting options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlFormat {
	/// Common VHDL family formatting options
	#[cfg_attr(feature = "serde", serde(flatten))]
	common: VhdlCommonFormat,
}

/// Workspace specific VHDL formatting options
#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlAmsFormat {
	/// Common VHDL family formatting options
	#[cfg_attr(feature = "serde", serde(flatten))]
	common: VhdlCommonFormat,
}
