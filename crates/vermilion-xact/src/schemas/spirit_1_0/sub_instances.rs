// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// All configuration information for a contained component or channel instance.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="configuration">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element ref="spirit:configurableElement" minOccurs="0" maxOccurs="unbounded"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Configuration {
	pub(crate) element: Vec<spirit::ConfigurableElement>,
}

/// Describes the configurable content of an element in the instance.
///
/// May contain text and/or child elements.
///
/// The required referenceId attribute refers to the ID attribute of the configurable element.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="configurableElement">
///   <xs:complexType mixed="true">
///     <xs:sequence>
///       <xs:any namespace="##any" processContents="skip" minOccurs="0" maxOccurs="unbounded"/>
///     </xs:sequence>
///     <xs:attribute name="referenceId" type="xs:Name" use="required"/>
///     <xs:attributeGroup ref="spirit:autoConfig"/>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ConfigurableElement {
	#[serde(rename = "$text")]
	pub(crate) text:         xs::String,
	#[serde(rename = "@spirit:referenceId")]
	pub(crate) reference_id: xs::Name,
	#[serde(flatten)]
	pub(crate) auto_config:  spirit::AutoConfig,
}

/// An instance name assigned to subcomponent instances and contained channels, that is unique
/// within the parent component.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="instanceName" type="xs:Name" />
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct InstanceName {
	#[serde(rename = "$text")]
	pub(crate) value: xs::String,
}

/// Component instance element.
///
/// The instance name is contained in the unique-value instanceName
/// attribute.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="componentInstance">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element ref="spirit:instanceName"/>
///       <xs:element name="componentRef" type="spirit:libraryRefType" />
///       <xs:element ref="spirit:configuration" minOccurs="0"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ComponentInstance {
	#[serde(rename = "spirit:instanceName")]
	pub(crate) name:          spirit::InstanceName,
	/// References a component to be found in an external library.  The name attribute gives the
	/// name of the component and the version attribute speicifies which version of the component
	/// to use.
	#[serde(rename = "spirit:componentRef")]
	pub(crate) component_ref: spirit::LibraryRefType,
	#[serde(rename = "spirit:configuration")]
	pub(crate) configuration: spirit::Configuration,
}

/// Sub instances of internal components.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="componentInstances">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element ref="spirit:componentInstance" minOccurs="0" maxOccurs="unbounded"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ComponentInstances {
	pub(crate) instances: Vec<spirit::ComponentInstance>,
}

/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="adHocConnection.pinReference">
///   <xs:attribute name="componentRef" type="xs:Name" use="required"/>
///   <xs:attribute name="signalRef" type="xs:Name" use="required"/>
///   <xs:attribute name="left" type="xs:nonNegativeInteger"/>
///   <xs:attribute name="right" type="xs:nonNegativeInteger"/>
/// </xs:complexType>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct PinReference {
	#[serde(rename = "@spirit:componentRef")]
	pub(crate) component_ref: xs::Name,
	#[serde(rename = "@spirit:signalRef")]
	pub(crate) signal_ref:    xs::Name,
	#[serde(rename = "@spirit:left", skip_serializing_if = "Option::is_none")]
	pub(crate) left:          Option<xs::NonNegativeInteger>,
	#[serde(rename = "@spirit:right", skip_serializing_if = "Option::is_none")]
	pub(crate) right:         Option<xs::NonNegativeInteger>,
}

/// Specifies whether this ad-hoc connection will be exported out of the design.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="adHocConnection.export">
///   <xs:simpleContent>
///     <xs:extension base="xs:boolean">
///       <xs:attributeGroup ref="spirit:bool.prompt.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Export {
	#[serde(rename = "$text")]
	pub(crate) value: xs::Boolean,
	#[serde(flatten)]
	pub(crate) attrs: spirit::BoolPromptAtt,
}

/// Represents an ad-hoc connection between component pins.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="adHocConnection">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="export" type="spirit:adHocConnection.export" minOccurs="0" />
///       <xs:element name="pinReference" type="spirit:adHocConnection.pinReference" maxOccurs="unbounded" />
///     </xs:sequence>
///     <xs:attribute name="name" type="xs:string" use="optional"/>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct AdHocConnection {
	#[serde(rename = "@spirit:name", skip_serializing_if = "Option::is_none")]
	pub(crate) name: Option<xs::String>,
}

/// Defines the set of ad-hoc connections in a design.
///
/// An ad-hoc connection represents a connection between two component pins which were not connected
/// as a result of interface connections (i.e.the pin to pin connection was made explicitly and is
/// represented explicitly).
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="adHocConnections">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element ref="spirit:adHocConnection" minOccurs="0" maxOccurs="unbounded"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct AdHocConnections {
	pub(crate) connections: Vec<spirit::AdHocConnection>,
}

/// Describes a connection from the interface of one component to the interface of another component
/// through its attributes.
///
/// The component1Ref and busInterface1Ref attributes indicate the instance name and bus interface
/// name of one end of the connection.
///
/// The component2Ref and busInterface2Ref attributes indicate the instance name and bus interface
/// name of the other end of he connection.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="interconnection">
///   <xs:complexType>
///     <xs:attribute name="component1Ref" type="xs:Name" use="required"/>
///     <xs:attribute name="busInterface1Ref" type="xs:Name" use="required"/>
///     <xs:attribute name="component2Ref" type="xs:Name" use="required"/>
///     <xs:attribute name="busInterface2Ref" type="xs:Name" use="required"/>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Interconnection {
	#[serde(rename = "@spirit:component1Ref")]
	pub(crate) component1_ref:     xs::Name,
	#[serde(rename = "@spirit:busInterface1Ref")]
	pub(crate) bus_interface1_ref: xs::Name,
	#[serde(rename = "@spirit:component2Ref")]
	pub(crate) component2_ref:     xs::Name,
	#[serde(rename = "@spirit:busInterface2Ref")]
	pub(crate) bus_interface2_ref: xs::Name,
}

/// Connections between internal sub components.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="interconnections">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element ref="spirit:interconnection" minOccurs="0" maxOccurs="unbounded"/>
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Interconnections {
	pub(crate) connections: Vec<spirit::Interconnection>,
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_xml_serdes;
}
