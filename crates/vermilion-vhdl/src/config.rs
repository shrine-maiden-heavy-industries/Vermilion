// SPDX-License-Identifier: BSD-3-Clause

/// VHDL specific user configuration options
#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlConfig {}

/// VHDL-AMS specific user configuration options
#[derive(Clone, Debug)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlAmsConfig {}
