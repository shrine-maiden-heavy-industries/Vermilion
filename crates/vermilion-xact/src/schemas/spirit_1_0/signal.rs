// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// The direction of a component signal.
///
/// ## XML Schema
///
/// ```xml
/// <xs:simpleType name="componentSignalDirectionType">
///   <xs:restriction base="xs:token">
///     <xs:enumeration value="in"/>
///     <xs:enumeration value="out"/>
///     <xs:enumeration value="inout"/>
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum ComponentSignalDirectionType {
	In,
	Out,
	InOut,
}

/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="signalType">
///   <xs:sequence>
///     <xs:element name="name" type="xs:NMTOKEN" />
///     <xs:element name="direction" type="spirit:componentSignalDirectionType" />
///     <xs:sequence minOccurs="0" ref="spirit:signalType.vector" />
///     <xs:choice minOccurs="0" ref="spirit:signalType.value" />
///     <xs:element name="export" minOccurs="0" />
///     <xs:element ref="spirit:vendorExtensions" minOccurs="0"/>
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SignalType {
	/// The name of the signal.
	#[serde(rename = "spirit:name")]
	pub(crate) name:       xs::NMToken,
	#[serde(rename = "spirit:direction")]
	pub(crate) direction:  spirit::ComponentSignalDirectionType,
	pub(crate) vector:     Option<spirit::SignalTypeVector>,
	pub(crate) value:      Option<spirit::SignalTypeValue>,
	pub(crate) export:     Option<spirit::SignalTypeExport>,
	pub(crate) extensions: Option<spirit::VendorExtension>,
}

/// Specific left and right vector bounds.
///
/// Signal width is max(left,right)-min(left,right)+1 When the bounds are not present, a scalar
/// signal is assumed.
///
/// ### XML Schema
///
/// ```xml
/// <xs:sequence minOccurs="0" name="spirit:signalType.vector" />
///   <xs:element name="left" type="spirit:signalType.vector.bound" />
///   <xs:element name="right" type="spirit:signalType.vector.bound" />
/// </xs:sequence>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SignalTypeVector {
	/// Left vector bound.
	#[serde(rename = "spirit:left")]
	pub(crate) left:  spirit::SignalTypeVectorBound,
	/// Right vector bound.
	#[serde(rename = "spirit:right")]
	pub(crate) right: spirit::SignalTypeVectorBound,
}

/// ### XML Schema
///
/// ```xml
/// <xs:complexType name="signalType.vector.bound">
///   <xs:simpleContent>
///     <xs:extension base="xs:nonNegativeInteger">
///       <xs:attributeGroup ref="spirit:long.prompt.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SignalTypeVectorBound {
	#[serde(rename = "$text")]
	pub(crate) value: xs::NonNegativeInteger,
	#[serde(flatten)]
	pub(crate) attrs: spirit::LongPromptAtt,
}

/// ## XML Schema
///
/// ```xml
/// <xs:choice minOccurs="0" name="spirit:signalType.value">
///   <xs:element name="defaultValue" type="spirit:signalValueType" minOccurs="0">
///     <xs:annotation>
///       <xs:documentation>The initial or default value on the signal.</xs:documentation>
///     </xs:annotation>
///   </xs:element>
///   <xs:element ref="spirit:clockDriver" minOccurs="0"/>
///   <xs:element ref="spirit:singleShotDriver" minOccurs="0"/>
/// </xs:choice>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum SignalTypeValue {
	Default(Box<spirit::ValueType>),
	Clock(Box<spirit::ClockDriver>),
	SingleShot(Box<spirit::SingleShotDriver>),
}

/// Specifies whether a signal is to be exported out of the module.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="signalType.export">
///   <xs:simpleContent>
///     <xs:extension base="xs:boolean">
///       <xs:attributeGroup ref="spirit:bool.prompt.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SignalTypeExport {
	#[serde(rename = "$text")]
	pub(crate) value: xs::Boolean,
	#[serde(flatten)]
	pub(crate) attrs: spirit::BoolPromptAtt,
}

/// Describes signal characteristics.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="signal" type="spirit:signalType" />
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Signal {
	#[serde(flatten)]
	pub(crate) value: spirit::SignalType,
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_xml_serdes;
}
