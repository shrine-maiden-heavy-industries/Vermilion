// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// Used wherever a name value pair is appropriate.
///
/// The name is given by the attribute while the value is the element content.
///
/// Supports configurability attributes and a cross reference XPath expression.
///
/// schema-type: `spirit:nameValuePairType`
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct NameValuePairType {
	/// The name in a name-value pair.
	#[serde(rename = "@spirit:name")]
	pub(crate) name:      xs::String,
	#[serde(rename = "@spirit:crossRef", skip_serializing_if = "Option::is_none", default)]
	pub(crate) cross_ref: Option<xs::String>,
	#[serde(flatten)]
	pub(crate) auto_cfg:  spirit::AutoConfig,
}

/// Name value pair with data type information.
///
/// schema-type: `spirit:nameValueTypeType`
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct NameValueTypeType {
	#[serde(flatten)]
	pub(crate) nvp_type:  spirit::NameValuePairType,
	/// The data type of the argument as pertains to the language. Example: "int", "double", "char
	/// *".
	#[serde(rename = "@spirit:dataType", skip_serializing_if = "Option::is_none", default)]
	pub(crate) data_type: Option<xs::String>,
}

/// A name value pair.
///
/// The name is in given by the name attribute. The value is in the text content of the element.
///
/// This element supports all configurability attributes. It also supports a cross reference
/// attribute which allows it to be associated with other elements in the document
/// through an XPath expression.
///
/// schema-type: `spirit:parameter`
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Parameter {
	#[serde(rename = "$text")]
	pub(crate) value:    xs::String,
	#[serde(flatten)]
	pub(crate) nvp_type: spirit::NameValuePairType,
}

/// A group of parameters.
///
/// schema-type: `spirit:parameterGroup`
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ParameterGroup {
	#[serde(default)]
	pub(crate) params: Vec<spirit::Parameter>,
}

/// schema-type: `spirit:persistentDataType`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct PersistentDataType {
	#[serde(rename = "@spirit:id")]
	pub(crate) id: spirit::Id,
}

/// A container for any data that is specific to this instance of the design object.
///
/// The contents are not interpreted or validated by the Design Environment.
///
/// This element will be saved with the design and restored when the design is loaded.
///
/// It is intended to be used by generators to store and retrieve instance specific data.
///
/// schema-type: `spirit:persistentInstanceData`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct PersistentInstanceData {
	#[serde(flatten)]
	pub(crate) pd_type: spirit::PersistentDataType,
	#[serde(rename = "$text")]
	pub(crate) data:    xs::String,
}

/// Container for vendor specific extensions.
///
/// schema-type: `spirit:vendorExtension`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct VendorExtension {
	// TODO(aki): This is not correct, should be arbitrary elements?
	/// Accepts any element(s) the content provider wants to put here, including elements from the
	/// SPIRIT namespace.
	#[serde(rename = "$text")]
	pub(crate) data: xs::String,
}
