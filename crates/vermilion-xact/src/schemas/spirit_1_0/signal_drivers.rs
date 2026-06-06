// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// Describes a driven clock signal.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="clockDriver">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="clockPeriod" type="spirit:clockDriver.period" />
///       <xs:element name="clockPulseOffset" type="spirit:clockDriver.offset" />
///       <xs:element name="clockPulseValue" type="spirit:clockDriver.value" />
///       <xs:element name="clockPulseDuration" type="spirit:clockDriver.duration" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ClockDriver {
	/// Clock period in nanoseconds
	#[serde(rename = "spirit:clockPeriod")]
	pub(crate) period:   spirit::ClockPeriod,
	/// Time in nanoseconds until first pulse
	#[serde(rename = "spirit:clockPulseOffset")]
	pub(crate) offset:   spirit::ClockPulseOffset,
	/// Value of signal after first clock edge
	#[serde(rename = "spirit:clockPulseValue")]
	pub(crate) value:    spirit::ClockPulseValue,
	/// Duration in nanoseconds of first state in cycle
	#[serde(rename = "spirit:clockPulseDuration")]
	pub(crate) duration: spirit::ClockPulseDuration,
}

/// Clock period in nanoseconds.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="clockDriver.period">
///   <xs:simpleContent>
///     <xs:extension base="spirit:configurableDouble">
///       <xs:attributeGroup ref="spirit:general.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ClockPeriod {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::ConfigurableDouble,
}

/// Time in nanoseconds until first pulse.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="clockDriver.offset">
///   <xs:simpleContent>
///     <xs:extension base="spirit:configurableDouble">
///       <xs:attributeGroup ref="spirit:general.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ClockPulseOffset {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::ConfigurableDouble,
}

/// Value of signal after first clock edge.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="clockDriver.value">
///   <xs:simpleContent>
///     <xs:extension base="spirit:spiritNumberType">
///       <xs:attributeGroup ref="spirit:general.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ClockPulseValue {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::ConfigurableDouble,
}

/// Duration in nanoseconds of first state in cycle.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="clockDriver.duration">
///   <xs:simpleContent>
///     <xs:extension base="spirit:configurableDouble">
///       <xs:attributeGroup ref="spirit:general.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ClockPulseDuration {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::ConfigurableDouble,
}

/// Describes a driven one-shot signal.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="singleShotDriver">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="singleShotOffset" type="spirit:singleShotDriver.offset" />
///       <xs:element name="singleShotValue" type="spirit:singleShotDriver.value" />
///       <xs:element name="singleShotDuration" type="spirit:singleShotDriver.duration" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SingleShotDriver {
	/// Time in nanoseconds until first pulse
	#[serde(rename = "spirit:singleShotOffset")]
	pub(crate) offset:   spirit::SingleShotOffset,
	/// Value of signal after first clock edge
	#[serde(rename = "spirit:singleShotValue")]
	pub(crate) value:    spirit::SingleShotValue,
	/// Duration in nanoseconds of first state in cycle
	#[serde(rename = "spirit:singleShotDuration")]
	pub(crate) duration: spirit::SingleShotDuration,
}

/// Time in nanoseconds until start of one-shot.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType>
///   <xs:simpleContent>
///     <xs:extension base="spirit:configurableDouble">
///       <xs:attributeGroup ref="spirit:general.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SingleShotOffset {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::ConfigurableDouble,
}

/// Value of signal after first  edge of one-shot.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name=="singleShotDriver.value">
///   <xs:simpleContent>
///     <xs:extension base="spirit:spiritNumberType">
///       <xs:attributeGroup ref="spirit:general.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SingleShotValue {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::SpiritNumber,
}

/// Duration in nanoseconds of the one shot.
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name=="singleShotDriver.duration">
///   <xs:simpleContent>
///     <xs:extension base="spirit:configurableDouble">
///       <xs:attributeGroup ref="spirit:general.att"/>
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct SingleShotDuration {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::ConfigurableDouble,
}

/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="driverType" use="optional" default="any">
///   <xs:simpleType>
///     <xs:restriction base="xs:token">
///       <xs:enumeration value="clock"/>
///       <xs:enumeration value="singleShot"/>
///       <xs:enumeration value="any"/>
///     </xs:restriction>
///   </xs:simpleType>
/// </xs:attribute>
/// ```
#[derive(
	Clone,
	Copy,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum DriverType {
	#[serde(rename = "clock")]
	Clock,
	#[serde(rename = "singleShot")]
	SingleShot,
	#[default]
	#[serde(rename = "any")]
	Any,
}

/// Specifies if a signal requires a driver.
///
/// Default is false.
///
/// The attribute driverType can further qualify what type of driver is required.
///
/// Undefined behaviour if direction is not input or inout.
///
/// Driver type any indicates that any unspecified type of driver must be connected
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="requiresDriver">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:boolean" />
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct RequiresDriver {
	#[serde(rename = "$text")]
	pub(crate) requires:    xs::Boolean,
	#[serde(rename = "@spirit:driverType", skip_serializing_if = "Option::is_none", default)]
	pub(crate) driver_type: Option<spirit::DriverType>,
}

/// Describes a signal strength.
///
/// ## XML Schema
///
/// ```xml
/// <xs:simpleType name="strengthType">
///   <xs:restriction base="xs:token">
///     <xs:enumeration value="strong" />
///     <xs:enumeration value="weak" />
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum StrengthType {
	#[default]
	#[serde(rename = "strong")]
	Strong,
	#[serde(rename = "weak")]
	Weak,
}

/// The strength of the signal.
///
/// ### XML Schema
///
/// ```xml
/// <xs:element name="strength">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="spirit:strengthType">
///         <xs:attributeGroup ref="spirit:general.att"/>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Strength {
	#[serde(flatten)]
	pub(crate) attrs: spirit::GeneralAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::StrengthType,
}

/// The value of a signal.
///
/// 1 or 0 for single bit signals, unsigned numeric otherwise.
///
/// ## XML Schema
///
/// ```xml
/// <xs:element name="value">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="spirit:spiritNumberType">
///         <xs:attributeGroup ref="spirit:long.prompt.att" />
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct Value {
	#[serde(flatten)]
	pub(crate) attr:  spirit::LongPromptAtt,
	#[serde(flatten)]
	pub(crate) value: spirit::SpiritNumber,
}

/// Describes a signal value.
///
/// The signal value can be just a value (number), a strength, or both.
///
/// If the "value" is not given, it should be considered an X (unknown).
///
/// A weak strength with no value given is considered Z (tristate).
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="signalValueType">
///   <xs:choice>
///     <xs:element ref="spirit:strength"/>
///     <xs:sequence>
///       <xs:element ref="spirit:value"/>
///       <xs:element ref="spirit:strength" minOccurs="0"/>
///     </xs:sequence>
///   </xs:choice>
/// </xs:complexType>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum ValueType {
	Value(Box<spirit::Value>),
	Strength(Box<spirit::Strength>),
	Both(Box<spirit::Value>, Option<Box<spirit::Strength>>),
}
