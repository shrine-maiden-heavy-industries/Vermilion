// SPDX-License-Identifier: BSD-3-Clause

use crate::{
	SystemVerilogStd, VerilogAmsStd, VerilogStd,
	fmt::workspace::{SystemVerilogFormat, VerilogAmsFormat, VerilogFormat},
	lint::workspace::{SystemVerilogLint, VerilogAmsLint, VerilogLint},
};

#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogConfig {
	std:  VerilogStd,
	fmt:  Option<VerilogFormat>,
	lint: Option<VerilogLint>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SystemVerilogConfig {
	std:  SystemVerilogStd,
	fmt:  Option<SystemVerilogFormat>,
	lint: Option<SystemVerilogLint>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VerilogAmsConfig {
	std:  VerilogAmsStd,
	fmt:  Option<VerilogAmsFormat>,
	lint: Option<VerilogAmsLint>,
}
