// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::xs;

/// Free form file type, not yet known by SPIRIT.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="userFileType" type="xs:string" />
/// ```
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
/// ## XML Schema
///
/// ```xml
/// <xs:element name="fileType">
///    <xs:simpleType>
///      <xs:restriction base="xs:string">
///        <xs:enumeration value="unknown"/>
///        <xs:enumeration value="cSource"/>
///        <xs:enumeration value="cppSource"/>
///        <xs:enumeration value="asmSource"/>
///        <xs:enumeration value="vhdlSource"/>
///        <xs:enumeration value="vhdlSource-87"/>
///        <xs:enumeration value="vhdlSource-93"/>
///        <xs:enumeration value="verilogSource"/>
///        <xs:enumeration value="verilogSource-95"/>
///        <xs:enumeration value="verilogSource-2001"/>
///        <xs:enumeration value="swObject"/>
///        <xs:enumeration value="swObjectLibrary"/>
///        <xs:enumeration value="vhdlBinaryLibrary"/>
///        <xs:enumeration value="verilogBinaryLibrary"/>
///        <xs:enumeration value="unelaboratedHdl"/>
///        <xs:enumeration value="executableHdl"/>
///        <xs:enumeration value="systemVerilogSource"/>
///        <xs:enumeration value="systemVerilogSource-3.0"/>
///        <xs:enumeration value="systemVerilogSource-3.1"/>
///        <xs:enumeration value="systemCSource"/>
///        <xs:enumeration value="systemCSource-2.0"/>
///        <xs:enumeration value="veraSource"/>
///        <xs:enumeration value="eSource"/>
///      </xs:restriction>
///    </xs:simpleType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum KnownFileType {
	#[serde(rename = "unknown")]
	Unknown,
	#[serde(rename = "cSource")]
	CSource,
	#[serde(rename = "cppSource")]
	CppSource,
	#[serde(rename = "asmSource")]
	AsmSource,
	#[serde(rename = "vhdlSource")]
	VhdlSource,
	#[serde(rename = "vhdlSource-87")]
	Vhdl87Source,
	#[serde(rename = "vhdlSource-93")]
	Vhdl93Source,
	#[serde(rename = "verilogSource")]
	VerilogSource,
	#[serde(rename = "verilogSource-95")]
	Verilog95Source,
	#[serde(rename = "verilogSource-2001")]
	Verilog2kSource,
	#[serde(rename = "swObject")]
	SwObject,
	#[serde(rename = "swObjectLibrary")]
	SwObjectLibrary,
	#[serde(rename = "vhdlBinaryLibrary")]
	VhdlBinaryLibrary,
	#[serde(rename = "verilogBinaryLibrary")]
	VerilogBinaryLibrary,
	#[serde(rename = "unelaboratedHdl")]
	UnelaboratedHdl,
	#[serde(rename = "executableHdl")]
	ExecutableHdl,
	#[serde(rename = "systemVerilogSource")]
	SystemVerilogSource,
	#[serde(rename = "systemVerilogSource-3.0")]
	SystemVerilog3_0Source,
	#[serde(rename = "systemVerilogSource-3.1")]
	SystemVerilog3_1Source,
	#[serde(rename = "systemCSource")]
	SystemCSource,
	#[serde(rename = "systemCSource-2.0")]
	SystemC2Source,
	#[serde(rename = "veraSource")]
	VeraSource,
	#[serde(rename = "eSource")]
	ESource,
}

/// The type of a file referenced by SPIRIT.
///
/// ## XML Schema
///
/// ```xml
/// <xs:group name="fileType">
///   <xs:choice>
///     <xs:element name="fileType" type="spirit:fileType" />
///     <xs:element name="userFileType" type="spirit:userFileType" />
///   </xs:choice>
/// </xs:group>
/// ```
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
