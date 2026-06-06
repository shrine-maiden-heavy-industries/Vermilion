// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// Base SPIRIT identifier.
///
/// Identify a SPIRIT document by its by vendor, library and name.
///
/// ## XML Schema
///
/// ```xml
/// <xs:group name="baseIdentifier">
///   <xs:sequence>
///     <xs:element name="vendor" type="xs:Name" />
///     <xs:element name="library" type="xs:Name" />
///     <xs:element name="name" type="xs:NMTOKEN" />
///   </xs:sequence>
/// </xs:group>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct BaseIdentifier {
	/// Name of the vendor who supplies this file.
	#[serde(rename = "spirit:vendor")]
	pub(crate) vendor:  xs::Name,
	/// Name of the logical library this component belongs to.
	///
	/// Note that a physical library may contain components from multiple logical libraries.
	///
	/// Logical libraries are displays in component browser.
	#[serde(rename = "spirit:library")]
	pub(crate) library: xs::Name,
	/// The name of the object.
	///
	/// Must match the root name of the XML file and the directory name it or its version
	/// directory belongs to.
	#[serde(rename = "spirit:name")]
	pub(crate) name:    xs::NMToken,
}

/// Identifies a design and a PMD with an optional version number.
///
/// ## XML Schema
///
/// ```xml
/// <xs:group name="identifier">
///   <xs:sequence>
///     <xs:group ref="spirit:baseIdentifier"/>
///     <xs:element name="version" type="xs:NMTOKEN" minOccurs="0"/>
///   </xs:sequence>
/// </xs:group>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Identifier {
	#[serde(flatten)]
	pub(crate) base:    spirit::BaseIdentifier,
	#[serde(rename = "spirit:version", skip_serializing_if = "Option::is_none")]
	pub(crate) version: Option<xs::NMToken>,
}

/// Identifies a component of a bus definition - with a mandatory a version number.
///
/// ## XML Schema
///
/// ```xml
/// <xs:group name="versionedIdentifier">
///   <xs:sequence>
///     <xs:group ref="spirit:baseIdentifier"/>
///     <xs:element name="version" type="xs:NMTOKEN"/>
///   </xs:sequence>
/// </xs:group>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VersionedIdentifier {
	#[serde(flatten)]
	pub(crate) base:    spirit::BaseIdentifier,
	#[serde(rename = "spirit:version")]
	pub(crate) version: xs::NMToken,
}

/// Base SPIRIT document reference.
///
/// Contains vendor, library, name and optional version attributes.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="libraryRefGroup">
///   <xs:attribute name="vendor" type="xs:Name" use="required"/>
///   <xs:attribute name="library" type="xs:Name" use="required"/>
///   <xs:attribute name="name" type="xs:NMTOKEN" use="required"/>
///   <xs:attribute name="version" type="xs:NMTOKEN"/>
/// </xs:attributeGroup>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct LibraryRefGroup {
	#[serde(rename = "@spirit:vendor")]
	pub(crate) vendor:  xs::Name,
	#[serde(rename = "@spirit:library")]
	pub(crate) library: xs::Name,
	#[serde(rename = "@spirit:name")]
	pub(crate) name:    xs::NMToken,
	#[serde(rename = "@spirit:version")]
	pub(crate) version: Option<xs::NMToken>,
}

/// Base SPIRIT document reference type.
///
/// Contains vendor, library, name and optional version attributes.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="libraryRefType">
///   <xs:attributeGroup ref="spirit:libraryRefGroup"/>
/// </xs:complexType>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct LibraryRefType {
	#[serde(flatten)]
	pub(crate) attributes: spirit::LibraryRefGroup,
}

/// Resolved SPIRIT document reference type.
///
/// Contains vendor, library, name and optional version attributes and the URI of the referenced
/// SPIRIT document
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="resolvedLibraryRefType">
///   <xs:simpleContent>
///     <xs:extension base="xs:anyURI">
///       <xs:attributeGroup ref="spirit:libraryRefGroup"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ResolvedLibraryRefType {
	#[serde(flatten)]
	pub(crate) attributes: spirit::LibraryRefGroup,
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_xml_serdes;
}
