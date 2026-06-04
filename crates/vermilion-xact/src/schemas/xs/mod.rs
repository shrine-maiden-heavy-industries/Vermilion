// SPDX-License-Identifier: BSD-3-Clause
//! # Common XML Schema Types
//!
//! This module implements the fundamental types defined in the W3C
//! [XML Schema] standard that are used in the IP-XACT objects.
//!
//! [XML Schema]: https://www.w3.org/TR/xmlschema-2/

pub mod deserialize;
pub mod serialize;

/// A type analogous to the XML Schema [`xs:anyURI`] type.
///
/// [`xs:anyURI`]: https://www.w3.org/TR/xmlschema-2/#anyURI
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnyURI {
	inner: fluent_uri::Uri<std::string::String>,
}

impl AnyURI {}

/// A type analogous to the XML Schema [`xs:base64Binary`] type.
///
/// [`xs:base64Binary`]: https://www.w3.org/TR/xmlschema-2/#base64Binary
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64Binary {
	inner: Vec<u8>,
}

impl Base64Binary {}

/// A type analogous to the XML Schema [`xs:boolean`] type.
///
/// [`xs:boolean`]: https://www.w3.org/TR/xmlschema-2/#boolean
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Boolean {
	inner: bool,
}

impl Boolean {}

/// A type analogous to the XML Schema [`xs:byte`] type.
///
/// [`xs:byte`]: https://www.w3.org/TR/xmlschema-2/#byte
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Byte {
	inner: i8,
}

impl Byte {}

/// A type analogous to the XML Schema [`xs:date`] type.
///
/// [`xs:date`]: https://www.w3.org/TR/xmlschema-2/#date
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Date {
	inner: chrono::NaiveDate,
}

impl Date {}

/// A type analogous to the XML Schema [`xs:dateTime`] type.
///
/// [`xs:dateTime`]: https://www.w3.org/TR/xmlschema-2/#dateTime
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTime {
	inner: chrono::DateTime<chrono::Utc>,
}

impl DateTime {}

/// A type analogous to the XML Schema [`xs:decimal`] type.
///
/// # NOTE
///
/// This type has the same internal representation as the [`Double`] type,
/// that being the native [`f64`] type as [`f128`] is not stable, and
/// we don't pull in any arbitrary precision libraries.
///
/// [`xs:decimal`]: https://www.w3.org/TR/xmlschema-2/#decimal
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Decimal {
	inner: f64,
}

impl Decimal {}

/// A type analogous to the XML Schema [`xs:double`] type.
///
/// [`xs:double`]: https://www.w3.org/TR/xmlschema-2/#double
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Double {
	inner: f64,
}

impl Double {}

/// A type analogous to the XML Schema [`xs:duration`] type.
///
/// [`xs:duration`]: https://www.w3.org/TR/xmlschema-2/#duration
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration {
	inner: chrono::TimeDelta,
}

impl Duration {}

/// A type analogous to the XML Schema [`xs:ENTITIES`] type.
///
/// [`xs:ENTITIES`]: https://www.w3.org/TR/xmlschema-2/#ENTITIES
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ENTITIES {
	inner: Vec<std::string::String>,
}

impl ENTITIES {}

/// A type analogous to the XML Schema [`xs:ENTITY`] type.
///
/// [`xs:ENTITY`]: https://www.w3.org/TR/xmlschema-2/#ENTITY
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ENTITY {
	inner: std::string::String,
}

impl ENTITY {}

/// A type analogous to the XML Schema [`xs:float`] type.
///
/// [`xs:float`]: https://www.w3.org/TR/xmlschema-2/#float
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Float {
	inner: f32,
}

impl Float {}

/// A type analogous to the XML Schema [`xs:gDay`] type.
///
/// [`xs:gDay`]: https://www.w3.org/TR/xmlschema-2/#gDay
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GDay {
	inner: (),
}

impl GDay {}

/// A type analogous to the XML Schema [`xs:gMonth`] type.
///
/// [`xs:gMonth`]: https://www.w3.org/TR/xmlschema-2/#gMonth
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GMonth {
	inner: (),
}

impl GMonth {}

/// A type analogous to the XML Schema [`xs:gMonthDay`] type.
///
/// [`xs:gMonthDay`]: https://www.w3.org/TR/xmlschema-2/#gMonthDay
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GMonthDay {
	inner: (),
}

impl GMonthDay {}

/// A type analogous to the XML Schema [`xs:gYear`] type.
///
/// [`xs:gYear`]: https://www.w3.org/TR/xmlschema-2/#gYear
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GYear {
	inner: (),
}

impl GYear {}

/// A type analogous to the XML Schema [`xs:gYearMonth`] type.
///
/// [`xs:gYearMonth`]: https://www.w3.org/TR/xmlschema-2/#gYearMonth
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GYearMonth {
	inner: (),
}

impl GYearMonth {}

/// A type analogous to the XML Schema [`xs:hexBinary`] type.
///
/// [`xs:hexBinary`]: https://www.w3.org/TR/xmlschema-2/#hexBinary
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HexBinary {
	inner: Vec<u8>,
}

impl HexBinary {}

/// A type analogous to the XML Schema [`xs:ID`] type.
///
/// [`xs:ID`]: https://www.w3.org/TR/xmlschema-2/#ID
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ID {
	inner: std::string::String,
}

impl ID {}

/// A type analogous to the XML Schema [`xs:IDREF`] type.
///
/// [`xs:IDREF`]: https://www.w3.org/TR/xmlschema-2/#IDREF
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IDREF {
	inner: std::string::String,
}

impl IDREF {}

/// A type analogous to the XML Schema [`xs:IDREFS`] type.
///
/// [`xs:IDREFS`]: https://www.w3.org/TR/xmlschema-2/#IDREFS
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IDREFS {
	inner: Vec<std::string::String>,
}

impl IDREFS {}

/// A type analogous to the XML Schema [`xs:int`] type.
///
/// [`xs:int`]: https://www.w3.org/TR/xmlschema-2/#int
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int {
	inner: i32,
}

impl Int {}

/// A type analogous to the XML Schema [`xs:integer`] type.
///
/// [`xs:integer`]: https://www.w3.org/TR/xmlschema-2/#integer
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Integer {
	inner: i64,
}

impl Integer {}

/// A type analogous to the XML Schema [`xs:language`] type.
///
/// [`xs:language`]: https://www.w3.org/TR/xmlschema-2/#langauge
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Language {
	inner: std::string::String,
}

impl Language {}

/// A type analogous to the XML Schema [`xs:long`] type.
///
/// [`xs:long`]: https://www.w3.org/TR/xmlschema-2/#long
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Long {
	inner: i64,
}

impl Long {}

/// A type analogous to the XML Schema [`xs:Name`] type.
///
/// [`xs:Name`]: https://www.w3.org/TR/xmlschema-2/#Name
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name {
	inner: std::string::String,
}

impl Name {}

/// A type analogous to the XML Schema [`xs:NCName`] type.
///
/// [`xs:NCName`]: https://www.w3.org/TR/xmlschema-2/#NCName
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NCName {
	inner: std::string::String,
}

impl NCName {}

/// A type analogous to the XML Schema [`xs:negativeInteger`] type.
///
/// [`xs:negativeInteger`]: https://www.w3.org/TR/xmlschema-2/#negativeInteger
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NegativeInteger {
	inner: i64,
}

impl NegativeInteger {}

/// A type analogous to the XML Schema [`xs:NMTOKEN`] type.
///
/// [`xs:NMTOKEN`]: https://www.w3.org/TR/xmlschema-2/#NMTOKEN
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NMTOKEN {
	inner: std::string::String,
}

impl NMTOKEN {}

/// A type analogous to the XML Schema [`xs:NMTOKENS`] type.
///
/// [`xs:NMTOKENS`]: https://www.w3.org/TR/xmlschema-2/#NMTOKENS
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NMTOKENS {
	inner: Vec<std::string::String>,
}

impl NMTOKENS {}

/// A type analogous to the XML Schema [`xs:nonNegativeInteger`] type.
///
/// [`xs:nonNegativeInteger`]: https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonNegativeInteger {
	inner: u64,
}

impl NonNegativeInteger {}

/// A type analogous to the XML Schema [`xs:nonPositiveInteger`] type.
///
/// [`xs:nonPositiveInteger`]: https://www.w3.org/TR/xmlschema-2/#
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonPositiveInteger {
	inner: i64,
}

impl NonPositiveInteger {}

/// A type analogous to the XML Schema [`xs:normalizedString`] type.
///
/// [`xs:normalizedString`]: https://www.w3.org/TR/xmlschema-2/#normalizedString
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalizedString {
	inner: std::string::String,
}

impl NormalizedString {}

/// A type analogous to the XML Schema [`xs:NOTATION`] type.
///
/// [`xs:NOTATION`]: https://www.w3.org/TR/xmlschema-2/#NOTATION
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NOTATION {
	inner: Vec<std::string::String>,
}

impl NOTATION {}

/// A type analogous to the XML Schema [`xs:positiveInteger`] type.
///
/// [`xs:positiveInteger`]: https://www.w3.org/TR/xmlschema-2/#positiveInteger
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PositiveInteger {
	inner: u64,
}

impl PositiveInteger {}

/// A type analogous to the XML Schema [`xs:QName`] type.
///
/// [`xs:QName`]: https://www.w3.org/TR/xmlschema-2/#QName
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QName {
	inner: std::string::String,
}

impl QName {}

/// A type analogous to the XML Schema [`xs:short`] type.
///
/// [`xs:short`]: https://www.w3.org/TR/xmlschema-2/#short
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Short {
	inner: i16,
}

impl Short {}

/// A type analogous to the XML Schema [`xs:string`] type.
///
/// [`xs:string`]: https://www.w3.org/TR/xmlschema-2/#string
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct String {
	inner: std::string::String,
}

impl String {}

/// A type analogous to the XML Schema [`xs:time`] type.
///
/// [`xs:time`]: https://www.w3.org/TR/xmlschema-2/#time
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Time {
	inner: chrono::NaiveTime,
}

impl Time {}

/// A type analogous to the XML Schema [`xs:token`] type.
///
/// [`xs:token`]: https://www.w3.org/TR/xmlschema-2/#token
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Token {
	inner: std::string::String,
}

impl Token {}

/// A type analogous to the XML Schema [`xs:unsignedByte`] type.
///
/// [`xs:unsignedByte`]: https://www.w3.org/TR/xmlschema-2/#unsignedByte
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedByte {
	inner: u8,
}

impl UnsignedByte {}

/// A type analogous to the XML Schema [`xs:unsignedInt`] type.
///
/// [`xs:unsignedInt`]: https://www.w3.org/TR/xmlschema-2/#unsignedInt
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedInt {
	inner: u32,
}

impl UnsignedInt {}

/// A type analogous to the XML Schema [`xs:unsignedLong`] type.
///
/// [`xs:unsignedLong`]: https://www.w3.org/TR/xmlschema-2/#unsignedLong
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedLong {
	inner: u64,
}

impl UnsignedLong {}

/// A type analogous to the XML Schema [`xs:unsignedShort`] type.
///
/// [`xs:unsignedShort`]: https://www.w3.org/TR/xmlschema-2/#unsignedShort
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedShort {
	inner: u16,
}

impl UnsignedShort {}
