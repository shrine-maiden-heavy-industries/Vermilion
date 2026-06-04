// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// Base SPIRIT identifier.
///
/// Identify a SPIRIT document by its by vendor, library and name.
///
/// schema-type: `spirit:baseIdentifier`
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
/// schema-type: `spirit:identifier`
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
/// schema-type: `spirit:versionedIdentifier`
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
/// schema-type: `spirit:libraryRefGroup`
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
/// schema-type: `spirit:libraryRefType`
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
/// schema-type: `spirit:resolvedLibraryRefType``
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ResolvedLibraryRefType {
	#[serde(flatten)]
	pub(crate) attributes: spirit::LibraryRefGroup,
}
