// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// All configuration information for a contained component or channel instance.
///
/// schema-type: `spirit:configuration`
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Configuration {
	pub(crate) element: Vec<ConfigurableElement>,
}

/// Describes the configurable content of an element in the instance.
///
/// May contain text and/or child elements.
///
/// The required referenceId attribute refers to the ID attribute of the configurable element.
///
/// schema-type: `spirit:configurableElement`
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
/// schema-type: `spirit:instanceName`
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
/// schema-type: `spirit:componentInstance`
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
/// schema-type: `spirit:componentInstances`
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ComponentInstances {
	pub(crate) instances: Vec<spirit::ComponentInstance>,
}

/// schema-type: `spirit:pinReference`
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
/// schema-type: `spirit:export`
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Export {
	#[serde(flatten)]
	pub(crate) attrs: spirit::BoolPromptAtt,
}

/// Represents an ad-hoc connection between component pins.
///
/// schema-type: `spirit:adHocConnection`
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
/// schema-type: `spirit:adHocConnections`
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
/// schema-type: `spirit:interconnection`
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
/// schema-type: `spirit:interconnections`
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Interconnections {
	pub(crate) connections: Vec<spirit::Interconnection>,
}
