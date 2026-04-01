// SPDX-License-Identifier: BSD-3-Clause

/// Workspace specific IP-XACT options
#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct XactWorkspace {}
