// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

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
#[serde(rename = "spirit:userFileType")]
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
#[serde(rename = "spirit:fileType", tag = "$text")]
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
#[serde(untagged)]
pub enum FileType {
	/// A known SPIRIT file type
	Known(spirit::KnownFileType),
	/// An unknown file type
	User(spirit::UserFileType),
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_xml_serdes;

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_unknown,
		"<spirit:fileType>unknown</spirit:fileType>",
		spirit::KnownFileType::Unknown
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_csource,
		"<spirit:fileType>cSource</spirit:fileType>",
		spirit::KnownFileType::CSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_cppsource,
		"<spirit:fileType>cppSource</spirit:fileType>",
		spirit::KnownFileType::CppSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_asmsource,
		"<spirit:fileType>asmSource</spirit:fileType>",
		spirit::KnownFileType::AsmSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_vhdlsource,
		"<spirit:fileType>vhdlSource</spirit:fileType>",
		spirit::KnownFileType::VhdlSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_vhdl87source,
		"<spirit:fileType>vhdlSource-87</spirit:fileType>",
		spirit::KnownFileType::Vhdl87Source
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_vhdl93source,
		"<spirit:fileType>vhdlSource-93</spirit:fileType>",
		spirit::KnownFileType::Vhdl93Source
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_verilogsource,
		"<spirit:fileType>verilogSource</spirit:fileType>",
		spirit::KnownFileType::VerilogSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_verilog95source,
		"<spirit:fileType>verilogSource-95</spirit:fileType>",
		spirit::KnownFileType::Verilog95Source
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_verilog2ksource,
		"<spirit:fileType>verilogSource-2001</spirit:fileType>",
		spirit::KnownFileType::Verilog2kSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_swobject,
		"<spirit:fileType>swObject</spirit:fileType>",
		spirit::KnownFileType::SwObject
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_swobjectlibrary,
		"<spirit:fileType>swObjectLibrary</spirit:fileType>",
		spirit::KnownFileType::SwObjectLibrary
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_vhdlbinarylibrary,
		"<spirit:fileType>vhdlBinaryLibrary</spirit:fileType>",
		spirit::KnownFileType::VhdlBinaryLibrary
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_verilogbinarylibrary,
		"<spirit:fileType>verilogBinaryLibrary</spirit:fileType>",
		spirit::KnownFileType::VerilogBinaryLibrary
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_unelaboratedhdl,
		"<spirit:fileType>unelaboratedHdl</spirit:fileType>",
		spirit::KnownFileType::UnelaboratedHdl
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_executablehdl,
		"<spirit:fileType>executableHdl</spirit:fileType>",
		spirit::KnownFileType::ExecutableHdl
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_systemverilogsource,
		"<spirit:fileType>systemVerilogSource</spirit:fileType>",
		spirit::KnownFileType::SystemVerilogSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_systemverilog3_0source,
		"<spirit:fileType>systemVerilogSource-3.0</spirit:fileType>",
		spirit::KnownFileType::SystemVerilog3_0Source
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_systemverilog3_1source,
		"<spirit:fileType>systemVerilogSource-3.1</spirit:fileType>",
		spirit::KnownFileType::SystemVerilog3_1Source
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_systemcsource,
		"<spirit:fileType>systemCSource</spirit:fileType>",
		spirit::KnownFileType::SystemCSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_systemc2source,
		"<spirit:fileType>systemCSource-2.0</spirit:fileType>",
		spirit::KnownFileType::SystemC2Source
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_verasource,
		"<spirit:fileType>veraSource</spirit:fileType>",
		spirit::KnownFileType::VeraSource
	);

	test_xml_serdes!(
		spirit_1_0,
		known_file_type_esource,
		"<spirit:fileType>eSource</spirit:fileType>",
		spirit::KnownFileType::ESource
	);

	test_xml_serdes!(
		spirit_1_0,
		user_file_type,
		"<spirit:userFileType>nya!</spirit:userFileType>",
		spirit::UserFileType { value: "nya!".into() }
	);

}
