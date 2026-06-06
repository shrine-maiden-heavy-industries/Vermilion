// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::{spirit_1_0 as spirit, xs};

/// This is a hint to the user interface on how to obtain the value for user defined properties.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="format">
///   <xs:simpleType>
///     <xs:restriction base="xs:token">
///       <xs:enumeration value="float"/>
///       <xs:enumeration value="long"/>
///       <xs:enumeration value="bool"/>
///       <xs:enumeration value="choice"/>
///       <xs:enumeration value="string"/>
///     </xs:restriction>
///   s</xs:simpleType>
/// </xs:attribute>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum Format {
	/// A floating point value is expected
	#[serde(rename = "float")]
	Float,
	/// An integer is expected
	#[serde(rename = "long")]
	Long,
	/// A boolean is expected
	#[serde(rename = "bool")]
	Bool,
	/// One or more choices from a list are expected
	#[serde(rename = "choice")]
	Choice,
	/// Any value is allowed
	#[serde(rename = "string")]
	String,
}

/// Provides a string used to prompt the user for user-resolved property values.
///
/// ### XML Schema
///
/// ```xml
/// <xs:attribute name="prompt" type="xs:string" />
/// ```
pub type Prompt = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// For user-resolved properties with numeric values, this indicates the minimum value allowed.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="minimum" type="xs:string" />
/// ```
pub type Minimum = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// For user-resolved properties with numeric values, this indicates the maximum value allowed.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="maximum" type="xs:string" />
/// ```
pub type Maximum = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// This type is used to indicate how the minimum and maximum attributes values should be
/// interpreted.
///
/// For reasons of backward compatibility, this attribute is assumed to have the value 'double'
/// if not present.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="rangeType">
///   <xs:simpleType>
///     <xs:restriction base="xs:token">
///       <xs:enumeration value="float"/>
///       <xs:enumeration value="int"/>
///       <xs:enumeration value="unsigned int"/>
///       <xs:enumeration value="long"/>
///       <xs:enumeration value="unsigned long"/>
///     </xs:restriction>
///   </xs:simpleType>
/// </xs:attribute>
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
pub enum RangeType {
	/// A floating point number
	#[default]
	#[serde(rename = "float")]
	Float,
	/// A signed 32-bit integer
	#[serde(rename = "int")]
	Int,
	/// An unsigned 32-bit integer
	#[serde(rename = "unsigned int")]
	UnsignedInt,
	/// A signed 64-bit integer
	#[serde(rename = "long")]
	Long,
	/// An unsigned 64-bit integer
	#[serde(rename = "unsigned long")]
	UnsignedLong,
}

/// For components with auto-generated configuration forms, the user-resolved properties with order
/// attributes will be presented in ascending order.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="order" type="xs:float" />
/// ```
pub type Order = xs::Float; // TODO(aki): Properly new-type and wrap/validate/serde

/// For user defined properties with a "choice" format, refers the choice element enumerating the
/// values to choose from.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="choiceRef" type="xs:Name" />
/// ```
pub type ChoiceRef = xs::Name; // TODO(aki): Properly new-type and wrap/validate/serde

/// For user resolved properties with a "choice" formats.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="choiceStyle">
///   <xs:simpleType>
///     <xs:restriction base="xs:token">
///       <xs:enumeration value="radio" />
///       <xs:enumeration value="combo" />
///     </xs:restriction>
///   </xs:simpleType>
/// </xs:attribute>
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
pub enum ChoiceStyle {
	/// Display choice as radio buttons.
	#[default]
	#[serde(rename = "radio")]
	Radio,
	/// Display choice as combo box.
	#[serde(rename = "combo")]
	Combo,
}

/// For user resolved properties with a "choice" format and a radio button presentation style.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="direction">
///   <xs:simpleType>
///     <xs:restriction base="xs:token">
///       <xs:enumeration value="vertical" />
///       <xs:enumeration value="horizontal" />
///     </xs:restriction>
///   </xs:simpleType>
/// </xs:attribute>
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
pub enum Direction {
	/// Display radio buttons vertically
	#[serde(rename = "vertical")]
	Vertical,
	/// Display radio buttons horizontally
	#[default]
	#[serde(rename = "horizontal")]
	Horizontal,
}

/// Tags configurable properties so that they may be auto-configured by different ui generators.
///
/// Can also be applied to top-level uiIcons to designate which auto-configured forms they will be
/// added to.
///
/// This is a white-space delimited list of groups so a property may be configured by more
/// than one configurator.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attribute name="configGroups" type="xs:NMTOKENS" />
/// ```
pub type ConfigGroups = xs::NMTokens; // TODO(aki): Properly new-type and wrap/validate/serde

/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="common.att">
///   <xs:attributeGroup ref="spirit:configurable"/>
///   <xs:attribute ref="spirit:minimum" />
///   <xs:attribute ref="spirit:maximum" />
///   <xs:attribute ref="spirit:rangeType"/>
///   <xs:attribute ref="spirit:order" />
///   <xs:attribute ref="spirit:choiceRef" />
///   <xs:attribute ref="spirit:choiceStyle"/>
///   <xs:attribute ref="spirit:direction"/>
///   <xs:attribute ref="spirit:configGroups"/>
/// </xs:attributeGroup>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct CommonAtt {
	#[serde(flatten)]
	pub(crate) configurable:  spirit::ConfigurableAttributes,
	#[serde(rename = "@spirit:minimum", skip_serializing_if = "Option::is_none", default)]
	pub(crate) minimum:       Option<spirit::Minimum>,
	#[serde(rename = "@spirit:maximum", skip_serializing_if = "Option::is_none", default)]
	pub(crate) maximum:       Option<spirit::Maximum>,
	/// Indicates the data type of the range attributes (minimum and maximum). For reasons of
	/// backward compatibility, this attribute is assumed to have the value 'double' if not
	/// present.
	#[serde(rename = "@spirit:rangeType", skip_serializing_if = "Option::is_none", default)]
	pub(crate) range_type:    Option<spirit::RangeType>,
	#[serde(rename = "@spirit:order", skip_serializing_if = "Option::is_none", default)]
	pub(crate) order:         Option<spirit::Order>,
	#[serde(rename = "@spirit:choiceRef", skip_serializing_if = "Option::is_none", default)]
	pub(crate) choice_ref:    Option<spirit::ChoiceRef>,
	#[serde(rename = "@spirit:choiceStyle", skip_serializing_if = "Option::is_none", default)]
	pub(crate) choice_style:  Option<spirit::ChoiceStyle>,
	#[serde(rename = "@spirit:direction", skip_serializing_if = "Option::is_none", default)]
	pub(crate) direction:     Option<spirit::Direction>,
	#[serde(
		rename = "@spirit:configGroups",
		skip_serializing_if = "Option::is_none",
		default
	)]
	pub(crate) config_groups: Option<spirit::ConfigGroups>,
}

/// Use this attribute group to allow all attributes associated with autoconfigurability.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="autoConfig">
///   <xs:attributeGroup ref="spirit:common.att"/>
///   <xs:attribute ref="spirit:format"/>
///   <xs:attribute ref="spirit:prompt"/>
/// </xs:attributeGroup>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct AutoConfig {
	#[serde(flatten)]
	pub(crate) common: spirit::CommonAtt,
	#[serde(rename = "@spirit:format", skip_serializing_if = "Option::is_none", default)]
	pub(crate) format: Option<spirit::Format>,
	#[serde(rename = "@spirit:prompt", skip_serializing_if = "Option::is_none", default)]
	pub(crate) prompt: Option<spirit::Prompt>,
}

/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="general.att">
///   <xs:attributeGroup ref="spirit:autoConfig"/>
/// </xs:attributeGroup>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct GeneralAtt {
	#[serde(flatten)]
	pub(crate) common: spirit::AutoConfig,
}

/// Use this attribute group on boolean elements for which the schema supplies a default prompt
/// attribute.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="bool.att">
///   <xs:attribute ref="spirit:format" default="bool" />
///   <xs:attributeGroup ref="spirit:common.att"/>
/// </xs:attributeGroup>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct BoolAtt {
	#[serde(flatten)]
	pub(crate) common: spirit::CommonAtt,
	/// This is a hint to the user interface about the data format to require for user resolved
	/// properties.
	#[serde(
		rename = "@spirit:format",
		skip_serializing_if = "Option::is_none",
		default = "BoolAtt::default_format"
	)]
	pub(crate) format: Option<spirit::Format>,
}

/// Use this attribute group on boolean elements.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="bool.prompt.att">
///   <xs:attributeGroup ref="spirit:bool.att"/>
///   <xs:attribute ref="spirit:prompt"/>
/// </xs:attributeGroup>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct BoolPromptAtt {
	#[serde(flatten)]
	pub(crate) base:   spirit::BoolAtt,
	#[serde(rename = "@spirit:prompt", skip_serializing_if = "Option::is_none", default)]
	pub(crate) prompt: Option<spirit::Prompt>,
}

/// Use this attribute group on long integer elements for which the schema supplies a default prompt
/// attribute.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="long.att">
///   <xs:attribute name="format" ref="spirit:format" default="long" />
///   <xs:attributeGroup ref="spirit:common.att"/>
/// </xs:attributeGroup>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct LongAtt {
	#[serde(flatten)]
	pub(crate) common: spirit::CommonAtt,
	/// This is a hint to the user interface about the data format to require for user resolved
	/// properties.
	#[serde(
		rename = "@spirit:format",
		skip_serializing_if = "Option::is_none",
		default = "LongAtt::default_format"
	)]
	pub(crate) format: Option<spirit::Format>,
}

/// Use this attribute group on long integer elements.
///
/// ## XML Schema
///
/// ```xml
/// <xs:attributeGroup name="long.prompt.att">
///   <xs:attributeGroup ref="spirit:long.att"/>
///   <xs:attribute ref="spirit:prompt"/>
/// </xs:attributeGroup>
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct LongPromptAtt {
	#[serde(flatten)]
	pub(crate) base:   spirit::LongAtt,
	#[serde(rename = "@spirit:prompt", skip_serializing_if = "Option::is_none", default)]
	pub(crate) prompt: Option<spirit::Prompt>,
}

/// A possible value of [`Choice`]
///
/// ## XML Schema
///
/// ```xml
/// <xs:complexType name="choice.enumeration">
///   <xs:simpleContent>
///     <xs:extension base="xs:string">
///       <xs:attribute name="text" type="xs:string" />
///       <xs:attribute name="help" type="xs:string" />
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub struct ChoiceEnumeration {
	#[serde(rename = "$text")]
	pub(crate) value: xs::String,
	/// When specified, displayed in place of the spirit:enumeration value
	#[serde(rename = "@spirit:text", skip_serializing_if = "Option::is_none", default)]
	pub(crate) text:  Option<xs::String>,
	/// Text that may be displayed if the user requests help about the meaning of an element
	#[serde(rename = "@spirit:help", skip_serializing_if = "Option::is_none", default)]
	pub(crate) help:  Option<xs::String>,
}

/// Non-empty set of legal values for a user defined property of type [`Format::Choice`].
///
/// ### XML Schema
///
/// ```xml
/// <xs:element name="choice" maxOccurs="unbounded">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="name" type="xs:Name" />
///       <xs:element name="enumeration" type="spirit:choice.enumeration" maxOccurs="unbounded" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum Choice {
	/// Choice key, available for reference by the spirit:choiceRef attribute of user defined
	/// properties of type [`Format::Choice`].
	#[serde(rename = "spirit:name")]
	Name(xs::Name),
	#[serde(rename = "spirit:enumeration")]
	Enumeration(ChoiceEnumeration),
}

/// Choices used by user defined properties of [`Format::Choice`]
///
/// ### XML Schema
///
/// ```xml
/// <xs:element name="choices">
///   <xs:complexType>
///     <xs:sequence>
///        <xs:element type="spirit:choice" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
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
pub struct Choices {
	#[serde(rename = "spirit:choice", skip_serializing_if = "Option::is_none")]
	pub(crate) choices: Option<Vec<spirit::Choice>>,
}

impl BoolAtt {
	fn default_format() -> Option<spirit::Format> {
		Some(spirit::Format::Bool)
	}
}

impl LongAtt {
	fn default_format() -> Option<spirit::Format> {
		Some(spirit::Format::Long)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_xml_serdes;

	#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
	struct AttrTest {
		#[serde(
			rename = "@spirit:format",
			alias = "@format",
			skip_serializing_if = "Option::is_none"
		)]
		format:  Option<spirit::Format>,
		#[serde(
			rename = "@spirit:prompt",
			alias = "@prompt",
			skip_serializing_if = "Option::is_none"
		)]
		prompt:  Option<spirit::Prompt>,
		#[serde(
			rename = "@spirit:minimum",
			alias = "@minimum",
			skip_serializing_if = "Option::is_none"
		)]
		minimum: Option<spirit::Minimum>,
	}

	test_xml_serdes!(
		spirit_1_0,
		attr_format_type_float,
		"<AttrTest spirit:format=\"float\"/>",
		AttrTest {
			format:  Some(spirit::Format::Float),
			prompt:  None,
			minimum: None,
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_format_type_long,
		"<AttrTest spirit:format=\"long\"/>",
		AttrTest {
			format:  Some(spirit::Format::Long),
			prompt:  None,
			minimum: None,
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_format_type_bool,
		"<AttrTest spirit:format=\"bool\"/>",
		AttrTest {
			format:  Some(spirit::Format::Bool),
			prompt:  None,
			minimum: None,
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		format_type_choice,
		"<AttrTest spirit:format=\"choice\"/>",
		AttrTest {
			format:  Some(spirit::Format::Choice),
			prompt:  None,
			minimum: None,
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_format_type_string,
		"<AttrTest spirit:format=\"string\"/>",
		AttrTest {
			format:  Some(spirit::Format::String),
			prompt:  None,
			minimum: None,
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_prompt_empty,
		"<AttrTest spirit:prompt=\"\"/>",
		AttrTest {
			format:  None,
			prompt:  Some("".into()),
			minimum: None,
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_prompt,
		"<AttrTest spirit:prompt=\"Meow!\"/>",
		AttrTest {
			format:  None,
			prompt:  Some("Meow!".into()),
			minimum: None,
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_minimum_empty,
		"<AttrTest spirit:minimum=\"\"/>",
		AttrTest {
			format:  None,
			prompt:  None,
			minimum: Some("".into()),
		}
	);

	test_xml_serdes!(
		spirit_1_0,
		attr_minimum,
		"<AttrTest spirit:minimum=\"0\"/>",
		AttrTest {
			format:  None,
			prompt:  None,
			minimum: Some("0".into()),
		}
	);
}
