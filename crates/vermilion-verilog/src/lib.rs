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
pub enum VerilogStd {
	#[default]
	Vl95,
	Vl01,
	Vl05,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum SystemVerilogStd {
	#[default]
	Sv05,
	Sv09,
	Sv12,
	Sv17,
	Sv23,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum VerilogAmsStd {
	#[default]
	Vams09,
	Vams14,
	Vams23,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum VerilogVariant {
	Verilog(VerilogStd),
	SystemVerilog(SystemVerilogStd),
	VerilogAms(VerilogAmsStd),
}

impl Display for VerilogStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Vl95 => write!(f, "Verilog 1995 (IEEE 1364-1995)"),
			Self::Vl01 => write!(f, "Verilog 2001 (IEEE 1364-2001)"),
			Self::Vl05 => write!(f, "Verilog 2005 (IEEE 1364-2005)"),
		}
	}
}

impl Display for SystemVerilogStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Sv05 => write!(f, "SystemVerilog 2005 (IEEE 1800-2005)"),
			Self::Sv09 => write!(f, "SystemVerilog 2009 (IEEE 1800-2009)"),
			Self::Sv12 => write!(f, "SystemVerilog 2012 (IEEE 1800-2012)"),
			Self::Sv17 => write!(f, "SystemVerilog 2017 (IEEE 1800-2017)"),
			Self::Sv23 => write!(f, "SystemVerilog 2023 (IEEE 1800-2023)"),
		}
	}
}

impl Display for VerilogAmsStd {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Vams09 => write!(f, "Verilog-AMS 2009 (Accellera Verilog-AMS 2.3.1)"),
			Self::Vams14 => write!(f, "Verilog-AMS 2014 (Accellera Verilog-AMS 2.4)"),
			Self::Vams23 => write!(f, "Verilog-AMS 2023 (Accellera Verilog-AMS 2023)"),
		}
	}
}

impl Display for VerilogVariant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Verilog(verilog_std) => write!(f, "{}", verilog_std),
			Self::SystemVerilog(system_verilog_std) => write!(f, "{}", system_verilog_std),
			Self::VerilogAms(verilog_ams_std) => write!(f, "{}", verilog_ams_std),
		}
	}
}
