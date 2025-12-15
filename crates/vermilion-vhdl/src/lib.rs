/* SPDX-License-Identifier: BSD-3-Clause */
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fmt::Display;

pub mod error;
pub mod fmt;
pub mod lang;
pub mod lint;
pub mod settings;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum VhdlStd {
	#[default]
	Vh87,
	Vh93,
	Vh2k,
	Vh02,
	Vh07,
	Vh08,
	Vh11,
	Vh19,
	Vh23,
}

impl Display for VhdlStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Vh87 => write!(f, "VHDL 1987 (IEEE 1076-1987)"),
			Self::Vh93 => write!(f, "VHDL 1993 (IEEE 1076-1993)"),
			Self::Vh2k => write!(f, "VHDL 2000 (IEEE 1076-2000)"),
			Self::Vh02 => write!(f, "VHDL 2002 (IEEE 1076-2002)"),
			Self::Vh07 => write!(f, "VHDL 2007 (IEEE 1076-2007)"),
			Self::Vh08 => write!(f, "VHDL 2008 (IEEE 1076-2008)"),
			Self::Vh11 => write!(f, "VHDL 2011 (IEEE 1076-2011)"),
			Self::Vh19 => write!(f, "VHDL 2019 (IEEE 1076-2019)"),
			Self::Vh23 => write!(f, "VHDL 2023 (IEEE 1076-2023)"),
		}
	}
}
