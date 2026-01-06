// SPDX-License-Identifier: BSD-3-Clause

use crate::{VhdlStd, fmt::settings::VhdlFormat, lint::settings::VhdlLint};

#[derive(Clone, Debug, Default)]
#[cfg_attr(
	feature = "serde",
	derive(::serde::Serialize, ::serde::Deserialize),
	serde(deny_unknown_fields)
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VhdlConfig {
	std:  VhdlStd,
	fmt:  Option<VhdlFormat>,
	lint: Option<VhdlLint>,
}
