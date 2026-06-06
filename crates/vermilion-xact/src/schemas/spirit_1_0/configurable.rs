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
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="resolve"/>
///   <xs:simpleType>
///    <xs:restriction base="xs:token">
///      <xs:enumeration value="immediate" />
///      <xs:enumeration value="user" />
///      <xs:enumeration value="dependent" />
///      <xs:enumeration value="generated" />
///    </xs:restriction>
///   </xs:simpleType>
/// </xs:attribute>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum Resolve {
	/// Property value is included in the XML file. It cannot be configured.
	#[serde(rename = "immediate")]
	Immediate,
	/// Property content can be modified through configuration. Modifications will be saved with the
	/// design.
	#[serde(rename = "user")]
	User,
	/// Property value is expressed as an XPath expression which may refer to other properties.  The
	/// expression must appear in the dependency attribute.
	#[serde(rename = "dependent")]
	Dependent,
	/// Generators may modify this property. Modifications do not get saved with the design.
	#[serde(rename = "generated")]
	Generated,
}

/// ID attribute for uniquely identifying an element within its document.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="id" type="xs:ID" />
/// ```
pub type Id = xs::Id; // TODO(aki): Properly new-type and wrap/validate/serde

/// Required on properties with [`ResolveType::Dependent`].
///
/// This is an XPath expression supplying the resultant value in terms of other properties
/// in the component file.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="dependency" type="xs:string" />
/// ```
pub type ConfigurableDependency = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// Base set of attributes for an element to be configurable.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="configurable">
///   <xs:attribute ref="spirit:resolve"/>
///   <xs:attribute ref="spirit:id"/>
///   <xs:attribute ref="spirit:dependency"/>
///   <xs:anyAttribute namespace="##other" processContents="lax"/>
/// </xs:attributeGroup>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ConfigurableAttributes {
	#[serde(
		rename = "@spirit:resolve",
		alias = "@resolve",
		skip_serializing_if = "Option::is_none"
	)]
	pub(crate) resolve:    Option<spirit::Resolve>,
	#[serde(rename = "@spirit:id", alias = "@id", skip_serializing_if = "Option::is_none")]
	pub(crate) id:         Option<spirit::Id>,
	#[serde(
		rename = "@spirit:dependency",
		alias = "@dependency",
		skip_serializing_if = "Option::is_none"
	)]
	pub(crate) dependency: Option<spirit::ConfigurableDependency>,
	// TODO(aki): Deal with vendor attributes
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_xml_serdes;

	#[derive(
		Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
	)]
	struct AttrTest {
		#[serde(
			rename = "@spirit:resolve",
			alias = "@resolve",
			skip_serializing_if = "Option::is_none"
		)]
		resolve: Option<spirit::Resolve>,
	}

	test_xml_serdes!(
		spirit_1_0,
		attr_resolve_dependent,
		"<AttrTest spirit:resolve=\"dependent\"/>",
		AttrTest {
			resolve: Some(spirit::Resolve::Dependent),
			..Default::default()
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_resolve_generated,
		"<AttrTest spirit:resolve=\"generated\"/>",
		AttrTest {
			resolve: Some(spirit::Resolve::Generated),
			..Default::default()
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_resolve_immediate,
		"<AttrTest spirit:resolve=\"immediate\"/>",
		AttrTest {
			resolve: Some(spirit::Resolve::Immediate),
			..Default::default()
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_resolve_user,
		"<AttrTest spirit:resolve=\"user\"/>",
		AttrTest {
			resolve: Some(spirit::Resolve::User),
			..Default::default()
		}
	);
}
