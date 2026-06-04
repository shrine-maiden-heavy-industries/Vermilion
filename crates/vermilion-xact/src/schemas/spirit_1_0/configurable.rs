// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// Determines how a property is resolved.
///
/// Immediate means the value is included in the XML document and cannot be changed by the user.
///
/// User means the value must be obtained from the user.
///
/// Optional means the user is given the opportunity to supply a value, but can leave the entry
/// blank.
///
/// Dependent means the value depends on the value of other properties.
///
/// A dependency expression must be supplied in the dependency attribute.
///
/// Generated means the value will be provided by a generator.
///
/// schema-type: `spirit:resolveType`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
#[serde(rename = "camelCase")]
pub enum ResolveType {
	/// Property value is included in the XML file. It cannot be configured.
	Immediate,
	/// Property content can be modified through configuration. Modifications will be saved with the
	/// design.
	User,
	/// Property value is expressed as an XPath expression which may refer to other properties.  The
	/// expression must appear in the dependency attribute.
	Dependent,
	/// Generators may modify this property. Modifications do not get saved with the design.
	Generated,
}

/// Determines how a property value is resolved.
///
/// schema-type: `spirit:resolve`
pub type Resolve = ResolveType; // TODO(aki): Properly new-type and wrap/validate/serde

/// ID attribute for uniquely identifying an element within its document.
///
/// schema-type: `spirit:id`
pub type Id = xs::Id; // TODO(aki): Properly new-type and wrap/validate/serde

/// Required on properties with [`ResolveType::Dependent`].
///
/// This is an XPath expression supplying the resultant value in terms of other properties
/// in the component file.
///
/// schema-type: `spirit:dependency`
pub type Dependency = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// Base set of attributes for an element to be configurable.
///
/// schema-type: `spirit:configurable`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ConfigurableAttributes {
	#[serde(rename = "@spirit:resolve", skip_serializing_if = "Option::is_none")]
	pub(crate) resolve:    Option<spirit::ResolveType>,
	#[serde(rename = "@spirit:id", skip_serializing_if = "Option::is_none")]
	pub(crate) id:         Option<spirit::Id>,
	#[serde(rename = "@spirit:dependency", skip_serializing_if = "Option::is_none")]
	pub(crate) dependency: Option<spirit::Dependency>,
}
