// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::xs;

/// Free form file type, not yet known by SPIRIT.
///
/// schema-type: `spirit:userFileType`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct UserFileType {
	#[serde(rename = "$text")]
	pub(crate) value: xs::String,
}

/// Enumerated file types known by SPIRIT.
///
/// schema-type: `spirit:fileType`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "camelCase")]
pub enum KnownFileType {
	Unknown,
	CSource,
	CppSource,
	AsmSource,
	VhdlSource,
	#[serde(rename = "vhdlSource-87")]
	Vhdl87Source,
	#[serde(rename = "vhdlSource-93")]
	Vhdl93Source,
	VerilogSource,
	#[serde(rename = "verilogSource-95")]
	Verilog95Source,
	#[serde(rename = "verilogSource-2001")]
	Verilog2kSource,
	SwObject,
	SwObjectLibrary,
	VhdlBinaryLibrary,
	VerilogBinaryLibrary,
	UnelaboratedHdl,
	ExecutableHdl,
	SystemVerilogSource,
	#[serde(rename = "systemVerilogSource-3.0")]
	SystemVerilog3_0Source,
	#[serde(rename = "systemVerilogSource-3.1")]
	SystemVerilog3_1Source,
	SystemCSource,
	#[serde(rename = "systemCSource-2.0")]
	SystemC2Source,
	VeraSource,
	ESource,
}

/// The type of a file referenced by SPIRIT.
///
/// schema-type: `spirit:fileType`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum FileType {
	/// A known SPIRIT file type
	#[serde(rename = "spirit:fileType")]
	FileType(KnownFileType),
	/// An unknown file type
	#[serde(rename = "spirit:userFileType")]
	User(UserFileType),
}
