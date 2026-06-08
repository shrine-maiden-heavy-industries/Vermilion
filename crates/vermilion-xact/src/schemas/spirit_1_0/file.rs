// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// Path to the file or directory.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="name">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="spirit:spiritURI">
///         <xs:attributeGroup ref="spirit:autoConfig"/>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:name")]
pub struct FileName {
	#[serde(flatten)]
	pub(crate) attrs: spirit::AutoConfig,
	#[serde(rename = "$text")]
	pub(crate) name:  spirit::SpiritUri,
}

/// Logical name for this file or directory e.g. VHDL library name.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="logicalName">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:Name">
///         <xs:attribute name="default" type="xs:string" use="optional"/>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:logicalName")]
pub struct LogicalName {
	#[serde(
		rename = "@spirit:default",
		alias = "@default",
		skip_serializing_if = "Option::is_none"
	)]
	pub(crate) default: Option<xs::String>,
	#[serde(rename = "$text")]
	pub(crate) name:    xs::Name,
}

/// Defines exported names that can be accessed externally, e.g. exported function names from a C
/// source file.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="exportedName" type="xs:Name" />
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:exportedName")]
pub struct ExportedName {
	#[serde(rename = "$value")]
	pub(crate) value: xs::Name,
}

/// Relates the current file to a certain executable image type in the design.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="imageType" type="xs:string" />
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:imageType")]
pub struct ImageType {
	#[serde(rename = "$value")]
	pub(crate) value: xs::String,
}

/// Command used to build this file.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="command">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:string">
///         <xs:attributeGroup ref="spirit:autoConfig"/>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:command")]
pub struct Command {
	#[serde(flatten)]
	pub(crate) attrs: spirit::AutoConfig,
	#[serde(rename = "$value")]
	pub(crate) value: xs::String,
}

/// Flags given to the build command when building this file. If the optional attribute "append" is
/// "true", this string will be appended to any existing flags, otherwise these flags will replace
/// any existing default flags.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="flags">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:string">
///         <xs:attribute name="append" type="xs:boolean" use="optional"/>
///         <xs:attributeGroup ref="spirit:autoConfig"/>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:flags")]
pub struct Flags {
	#[serde(flatten)]
	pub(crate) attrs:  spirit::AutoConfig,
	#[serde(
		rename = "@spirit:append",
		alias = "@append",
		skip_serializing_if = "Option::is_none"
	)]
	pub(crate) append: Option<xs::Boolean>,
	#[serde(rename = "$value")]
	pub(crate) value:  xs::String,
}

/// If true, the value of the sibling element "flags" should replace any default flags specified at
/// a more global level. If this is true and the sibling element "flags" is empty or missing, this
/// has the effect of clearing any default flags.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="replaceDefaultFlags">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:boolean">
///         <xs:attributeGroup ref="spirit:autoConfig"/>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:replaceDefaultFlags")]
pub struct ReplaceDefaultFlags {
	#[serde(flatten)]
	pub(crate) attrs: spirit::AutoConfig,
	#[serde(rename = "$value")]
	pub(crate) value: xs::Boolean,
}

/// Pathname to the file that is derived (built) from the source file.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="targetName">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="spirit:spiritURI">
///         <xs:attributeGroup ref="spirit:autoConfig"/>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:targetName")]
pub struct TargetName {
	#[serde(flatten)]
	pub(crate) attrs: spirit::AutoConfig,
	#[serde(rename = "$value")]
	pub(crate) value: spirit::SpiritUri,
}

/// Command and flags used to build derived files from the sourceName files. If this element is
/// present, the command and/or flags used to to build the file will override or augment any default
/// builders at a higher level.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="buildCommand">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="command" ref="spirit:command" minOccurs="0" />
///       <xs:element name="flags" ref="spirit:flags" minOccurs="0" />
///       <xs:element name="replaceDefaultFlags" ref="spirit:replaceDefaultFlags" minOccurs="0" />
///       <xs:element name="targetName" ref="spirit:targetName" minOccurs="0" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:buildCommand")]
pub struct BuildCommand {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) command:               Option<spirit::Command>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) flags:                 Option<spirit::Flags>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) replace_default_flags: Option<spirit::ReplaceDefaultFlags>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) target_name:           Option<spirit::TargetName>,
}

/// SPIRIT reference to a file or directory.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="file">
///   <xs:complexType>
///     <xs:attribute name="fileId" type="xs:ID"/>
///     <xs:anyAttribute namespace="##other" processContents="lax"/>
///     <xs:sequence>
///       <xs:element name="name" ref="spirit:name" />
///       <xs:group ref="spirit:fileType"/>
///       <xs:element name="logicalName" ref="spirit:logicalName" minOccurs="0" />
///       <xs:element name="exportedName" ref="spirit:exportedName" minOccurs="0" maxOccurs="unbounded" />
///       <xs:element name="buildCommand" ref="spirit:buildCommand" minOccurs="0" />
///       <xs:element name="define" type="spirit:nameValuePairType" minOccurs="0" maxOccurs="unbounded" />
///       <xs:element ref="spirit:dependency" minOccurs="0" maxOccurs="unbounded"/>
///       <xs:element name="imageType" ref="spirit:imageType" minOccurs="0" maxOccurs="unbounded" />
///       <xs:element ref="spirit:vendorExtensions" minOccurs="0"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:file")]
pub struct File {
	#[serde(rename = "@spirit:fileId", alias = "@fileId")]
	pub(crate) file_id:           xs::Id,
	pub(crate) name:              spirit::FileName,
	pub(crate) file_type:         spirit::FileType,
	pub(crate) logical_name:      spirit::LogicalName,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) exported_names:    Option<Vec<xs::Name>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) build_command:     Option<spirit::BuildCommand>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) dependencies:      Option<Vec<spirit::Dependency>>,
	/// Specifies define symbols that are used in the source file.  The spirit:name attribute gives
	/// the name to be defined and the text content of the element holds the value.  This element
	/// supports full configurability.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) defines:           Option<Vec<spirit::NameValuePairType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) image_types:       Option<Vec<spirit::ImageType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) vendor_extensions: Option<spirit::VendorExtension>,
}

/// This element specifies a list of unique pathnames to files and directories. It may also include
/// build instructions for the files. If compilation order is important, e.g. for VHDL files, the
/// files have to be provided in compilation order.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="fileSet" type="spirit:fileSetType" />
/// ```
pub type FileSet = spirit::FileSetType; // TODO(aki): Properly new-type and wrap/validate/serde

/// Contains a group of file set references that indicates the set of file sets complying with the
/// tool set of the current executable image.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="fileSetRefGroup">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element ref="spirit:fileSetRef" minOccurs="0" maxOccurs="unbounded"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "spirit:fileSetRefGroup")]
pub struct FileSetRefGroup {
	#[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
	pub(crate) refs: Option<Vec<spirit::FileSetRef>>,
}

/// Specifies an executable software image to be loaded into a processors address space. The format
/// of the image is not specified. It could, for example, be an ELF loadfile, or it could be raw
/// binary or ascii hex data for loading directly into a memory model instance.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="executableImage">
///   <xs:complexType>
///     <xs:attribute name="id" type="xs:ID" use="required"/>
///     <xs:attribute name="imageType" type="xs:Name" use="optional"/>
///     <xs:sequence>
///       <xs:element name="name" ref="spirit:fileName" />
///       <xs:element ref="spirit:parameter" minOccurs="0" maxOccurs="unbounded" />
///
///       <xs:element ref="spirit:fileSetRefGroup" minOccurs="0" />
///       <xs:element ref="spirit:vendorExtensions" minOccurs="0"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ExecutableImage {
	#[serde(
		rename = "@spirit:imageType",
		alias = "@imageType",
		skip_serializing_if = "Option::is_none"
	)]
	pub(crate) image_type:        Option<xs::Name>,
	#[serde(rename = "@spirit:id", alias = "@id", skip_serializing_if = "Option::is_none")]
	pub(crate) id:                Option<xs::Id>,
	pub(crate) name:              spirit::FileName,
	/// Additional information about the load module, e.g. stack base addresses, table addresses,
	/// etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) parameters:        Option<Vec<spirit::Parameter>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) file_set_refs:     Option<spirit::FileSetRefGroup>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) vendor_extensions: Option<spirit::VendorExtension>,
}

/// A reference to a fileSet.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="fileSetRef" type="xs:NMTOKEN" />
/// ```
pub type FileSetRef = xs::NMTokens; // TODO(aki): Properly new-type and wrap/validate/serde

/// Specifies a location on which  files or fileSets may be dependent.
///
/// Typically, this would be a directory that would contain included files.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="dependency" type="spirit:spiritURI" />
/// ```
pub type Dependency = spirit::SpiritUri; // TODO(aki): Properly new-type and wrap/validate/serde

/// List of file sets associated with component.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="fileSets">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element ref="spirit:fileSet" minOccurs="0" maxOccurs="unbounded"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
pub type FileSets = Vec<spirit::FileSet>; // TODO(aki): Properly new-type and wrap/validate/serde

/// SPIRIT URI, like a standard [`xs::AnyURI`] except that it can contain environment variables
/// in the `${ }`` form, to be replaced by their value to provide the underlying URI.
///
/// ## XML Schema
///
/// ```xml
/// <xs:simpleType name="spiritURI">
///   <xs:union memberTypes="xs:anyURI"/>
/// </xs:simpleType>
/// ```
pub type SpiritUri = xs::AnyURI; // TODO(aki): Properly new-type and wrap/validate/serde

/// Enumerates C argument data types.
///
/// ## XML Schema
///
/// ```xml
/// <xs:simpleType name="dataTypeType">
///   <xs:restriction base="xs:string">
///     <xs:enumeration value="int"/>
///     <xs:enumeration value="unsigned int"/>
///     <xs:enumeration value="long"/>
///     <xs:enumeration value="unsigned long"/>
///     <xs:enumeration value="float"/>
///     <xs:enumeration value="double"/>
///     <xs:enumeration value="char *"/>
///     <xs:enumeration value="void *"/>
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum DataTypeType {
	#[serde(rename = "int")]
	Int,
	#[serde(rename = "unsigned int")]
	UnsignedInt,
	#[serde(rename = "long")]
	Long,
	#[serde(rename = "unsigned long")]
	UnsignedLong,
	#[serde(rename = "float")]
	Float,
	#[serde(rename = "double")]
	Double,
	#[serde(rename = "char *")]
	CharPtr,
	#[serde(rename = "void *")]
	VoidPtr,
}

/// A reference to a configurator element.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="configuratorRef" type="xs:string" />
/// ```
pub type ConfiguratorRef = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// A reference to a generator element.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="generatorRef" type="xs:string" />
/// ```
pub type GeneratorRef = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// Nya
///
/// ## XML Schema
///
/// ```xml
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct FileSetType {
	/// Specifies the original owner of the fileSet, used when importing fileSets from other
	/// components, as in Hierarchical components, to allow the parent component to correctly
	/// locate files imported from subcomponents
	#[serde(rename = "spirit:owner")]
	pub(crate) owner: Option<spirit::LibraryRefType>,
	/// Identifies this filleSet as belonging to a particular group or having a particular purpose.
	/// Examples might be "diagnostics", "boot", "application", "interrupt", "deviceDriver", etc.
	#[serde(rename = "spirit:group")]
	pub(crate) group: Option<Vec<xs::Name>>,
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_xml_serdes;
}
