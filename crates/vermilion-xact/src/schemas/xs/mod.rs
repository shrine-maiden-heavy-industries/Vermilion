// SPDX-License-Identifier: BSD-3-Clause
//! # Common XML Schema Types
//!
//! This module implements the fundamental types defined in the W3C
//! [XML Schema] standard that are used in the IP-XACT objects.
//!
//! [XML Schema]: https://www.w3.org/TR/xmlschema-2/

mod deserialize;
#[cfg(feature = "schema")]
mod schema;
mod serialize;

macro_rules! thin_impl {
	($type:ty,const $inner:ty) => {
		impl $type {
			pub const fn new(inner: $inner) -> Self {
				Self { inner }
			}

			pub const fn value(self) -> $inner {
				self.inner
			}
		}

		impl From<$inner> for $type {
			fn from(value: $inner) -> Self {
				Self { inner: value }
			}
		}

		impl From<$type> for $inner {
			fn from(value: $type) -> Self {
				value.value()
			}
		}
	};
	($type:ty, $inner:ty) => {
		impl $type {
			pub const fn new(inner: $inner) -> Self {
				Self { inner }
			}

			pub fn value(self) -> $inner {
				self.inner
			}
		}

		impl From<$inner> for $type {
			fn from(value: $inner) -> Self {
				Self { inner: value }
			}
		}

		impl From<$type> for $inner {
			fn from(value: $type) -> Self {
				value.value()
			}
		}
	};
}

/// A type analogous to the XML Schema [`xs:anyURI`] type.
///
/// [`xs:anyURI`]: https://www.w3.org/TR/xmlschema-2/#anyURI
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnyURI {
	inner: fluent_uri::Uri<std::string::String>,
}

thin_impl!(AnyURI, fluent_uri::Uri<std::string::String>);

/// A type analogous to the XML Schema [`xs:base64Binary`] type.
///
/// [`xs:base64Binary`]: https://www.w3.org/TR/xmlschema-2/#base64Binary
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64Binary {
	inner: Vec<u8>,
}

thin_impl!(Base64Binary, Vec<u8>);

/// A type analogous to the XML Schema [`xs:boolean`] type.
///
/// [`xs:boolean`]: https://www.w3.org/TR/xmlschema-2/#boolean
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Boolean {
	inner: bool,
}

thin_impl!(Boolean, const bool);

/// A type analogous to the XML Schema [`xs:byte`] type.
///
/// [`xs:byte`]: https://www.w3.org/TR/xmlschema-2/#byte
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Byte {
	inner: i8,
}

thin_impl!(Byte, const i8);

/// A type analogous to the XML Schema [`xs:date`] type.
///
/// [`xs:date`]: https://www.w3.org/TR/xmlschema-2/#date
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Date {
	inner: chrono::NaiveDate,
}

thin_impl!(Date, const chrono::NaiveDate);

/// A type analogous to the XML Schema [`xs:dateTime`] type.
///
/// [`xs:dateTime`]: https://www.w3.org/TR/xmlschema-2/#dateTime
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTime {
	inner: chrono::DateTime<chrono::Utc>,
}

thin_impl!(DateTime, const chrono::DateTime<chrono::Utc>);

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

thin_impl!(Decimal, const f64);

/// A type analogous to the XML Schema [`xs:double`] type.
///
/// [`xs:double`]: https://www.w3.org/TR/xmlschema-2/#double
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Double {
	inner: f64,
}

thin_impl!(Double, const f64);

/// A type analogous to the XML Schema [`xs:duration`] type.
///
/// [`xs:duration`]: https://www.w3.org/TR/xmlschema-2/#duration
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration {
	inner: chrono::TimeDelta,
}

thin_impl!(Duration, const chrono::TimeDelta);

/// A type analogous to the XML Schema [`xs:ENTITIES`] type.
///
/// [`xs:ENTITIES`]: https://www.w3.org/TR/xmlschema-2/#ENTITIES
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ENTITIES {
	inner: Vec<std::string::String>,
}

thin_impl!(ENTITIES, Vec<std::string::String>);

/// A type analogous to the XML Schema [`xs:ENTITY`] type.
///
/// [`xs:ENTITY`]: https://www.w3.org/TR/xmlschema-2/#ENTITY
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ENTITY {
	inner: std::string::String,
}

thin_impl!(ENTITY, std::string::String);

/// A type analogous to the XML Schema [`xs:float`] type.
///
/// [`xs:float`]: https://www.w3.org/TR/xmlschema-2/#float
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Float {
	inner: f32,
}

thin_impl!(Float, const f32);

/// A type analogous to the XML Schema [`xs:gDay`] type.
///
/// [`xs:gDay`]: https://www.w3.org/TR/xmlschema-2/#gDay
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GDay {
	inner: (),
}

thin_impl!(GDay, ());

/// A type analogous to the XML Schema [`xs:gMonth`] type.
///
/// [`xs:gMonth`]: https://www.w3.org/TR/xmlschema-2/#gMonth
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GMonth {
	inner: (),
}

thin_impl!(GMonth, ());

/// A type analogous to the XML Schema [`xs:gMonthDay`] type.
///
/// [`xs:gMonthDay`]: https://www.w3.org/TR/xmlschema-2/#gMonthDay
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GMonthDay {
	inner: (),
}

thin_impl!(GMonthDay, ());

/// A type analogous to the XML Schema [`xs:gYear`] type.
///
/// [`xs:gYear`]: https://www.w3.org/TR/xmlschema-2/#gYear
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GYear {
	inner: (),
}

thin_impl!(GYear, ());

/// A type analogous to the XML Schema [`xs:gYearMonth`] type.
///
/// [`xs:gYearMonth`]: https://www.w3.org/TR/xmlschema-2/#gYearMonth
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GYearMonth {
	inner: (),
}

thin_impl!(GYearMonth, ());

/// A type analogous to the XML Schema [`xs:hexBinary`] type.
///
/// [`xs:hexBinary`]: https://www.w3.org/TR/xmlschema-2/#hexBinary
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HexBinary {
	inner: Vec<u8>,
}

thin_impl!(HexBinary, Vec<u8>);

/// A type analogous to the XML Schema [`xs:ID`] type.
///
/// [`xs:ID`]: https://www.w3.org/TR/xmlschema-2/#ID
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ID {
	inner: std::string::String,
}

thin_impl!(ID, std::string::String);

/// A type analogous to the XML Schema [`xs:IDREF`] type.
///
/// [`xs:IDREF`]: https://www.w3.org/TR/xmlschema-2/#IDREF
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IDREF {
	inner: std::string::String,
}

thin_impl!(IDREF, std::string::String);

/// A type analogous to the XML Schema [`xs:IDREFS`] type.
///
/// [`xs:IDREFS`]: https://www.w3.org/TR/xmlschema-2/#IDREFS
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IDREFS {
	inner: Vec<std::string::String>,
}

thin_impl!(IDREFS, Vec<std::string::String>);

/// A type analogous to the XML Schema [`xs:int`] type.
///
/// [`xs:int`]: https://www.w3.org/TR/xmlschema-2/#int
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int {
	inner: i32,
}

thin_impl!(Int, const i32);

/// A type analogous to the XML Schema [`xs:integer`] type.
///
/// [`xs:integer`]: https://www.w3.org/TR/xmlschema-2/#integer
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Integer {
	inner: i64,
}

thin_impl!(Integer, const i64);

/// A type analogous to the XML Schema [`xs:language`] type.
///
/// [`xs:language`]: https://www.w3.org/TR/xmlschema-2/#langauge
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Language {
	inner: std::string::String,
}

thin_impl!(Language, std::string::String);

/// A type analogous to the XML Schema [`xs:long`] type.
///
/// [`xs:long`]: https://www.w3.org/TR/xmlschema-2/#long
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Long {
	inner: i64,
}

thin_impl!(Long, const i64);

/// A type analogous to the XML Schema [`xs:Name`] type.
///
/// [`xs:Name`]: https://www.w3.org/TR/xmlschema-2/#Name
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name {
	inner: std::string::String,
}

thin_impl!(Name, std::string::String);

/// A type analogous to the XML Schema [`xs:NCName`] type.
///
/// [`xs:NCName`]: https://www.w3.org/TR/xmlschema-2/#NCName
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NCName {
	inner: std::string::String,
}

thin_impl!(NCName, std::string::String);

/// A type analogous to the XML Schema [`xs:negativeInteger`] type.
///
/// [`xs:negativeInteger`]: https://www.w3.org/TR/xmlschema-2/#negativeInteger
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NegativeInteger {
	inner: i64,
}

thin_impl!(NegativeInteger, const i64);

/// A type analogous to the XML Schema [`xs:NMTOKEN`] type.
///
/// [`xs:NMTOKEN`]: https://www.w3.org/TR/xmlschema-2/#NMTOKEN
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NMTOKEN {
	inner: std::string::String,
}

thin_impl!(NMTOKEN, std::string::String);

/// A type analogous to the XML Schema [`xs:NMTOKENS`] type.
///
/// [`xs:NMTOKENS`]: https://www.w3.org/TR/xmlschema-2/#NMTOKENS
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NMTOKENS {
	inner: Vec<std::string::String>,
}

thin_impl!(NMTOKENS, Vec<std::string::String>);

/// A type analogous to the XML Schema [`xs:nonNegativeInteger`] type.
///
/// [`xs:nonNegativeInteger`]: https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonNegativeInteger {
	inner: u64,
}

thin_impl!(NonNegativeInteger, const u64);

/// A type analogous to the XML Schema [`xs:nonPositiveInteger`] type.
///
/// [`xs:nonPositiveInteger`]: https://www.w3.org/TR/xmlschema-2/#
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonPositiveInteger {
	inner: i64,
}

thin_impl!(NonPositiveInteger, const i64);

/// A type analogous to the XML Schema [`xs:normalizedString`] type.
///
/// [`xs:normalizedString`]: https://www.w3.org/TR/xmlschema-2/#normalizedString
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalizedString {
	inner: std::string::String,
}

thin_impl!(NormalizedString, std::string::String);

/// A type analogous to the XML Schema [`xs:NOTATION`] type.
///
/// [`xs:NOTATION`]: https://www.w3.org/TR/xmlschema-2/#NOTATION
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NOTATION {
	inner: Vec<std::string::String>,
}

thin_impl!(NOTATION, Vec<std::string::String>);

/// A type analogous to the XML Schema [`xs:positiveInteger`] type.
///
/// [`xs:positiveInteger`]: https://www.w3.org/TR/xmlschema-2/#positiveInteger
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PositiveInteger {
	inner: u64,
}

thin_impl!(PositiveInteger, const u64);

/// A type analogous to the XML Schema [`xs:QName`] type.
///
/// [`xs:QName`]: https://www.w3.org/TR/xmlschema-2/#QName
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QName {
	inner: std::string::String,
}

thin_impl!(QName, std::string::String);

/// A type analogous to the XML Schema [`xs:short`] type.
///
/// [`xs:short`]: https://www.w3.org/TR/xmlschema-2/#short
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Short {
	inner: i16,
}

thin_impl!(Short, const i16);

/// A type analogous to the XML Schema [`xs:string`] type.
///
/// [`xs:string`]: https://www.w3.org/TR/xmlschema-2/#string
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct String {
	inner: std::string::String,
}

thin_impl!(String, std::string::String);

/// A type analogous to the XML Schema [`xs:time`] type.
///
/// [`xs:time`]: https://www.w3.org/TR/xmlschema-2/#time
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Time {
	inner: chrono::NaiveTime,
}

thin_impl!(Time, const chrono::NaiveTime);

/// A type analogous to the XML Schema [`xs:token`] type.
///
/// [`xs:token`]: https://www.w3.org/TR/xmlschema-2/#token
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Token {
	inner: std::string::String,
}

thin_impl!(Token, std::string::String);

/// A type analogous to the XML Schema [`xs:unsignedByte`] type.
///
/// [`xs:unsignedByte`]: https://www.w3.org/TR/xmlschema-2/#unsignedByte
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedByte {
	inner: u8,
}

thin_impl!(UnsignedByte, const u8);

/// A type analogous to the XML Schema [`xs:unsignedInt`] type.
///
/// [`xs:unsignedInt`]: https://www.w3.org/TR/xmlschema-2/#unsignedInt
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedInt {
	inner: u32,
}

thin_impl!(UnsignedInt, const u32);

/// A type analogous to the XML Schema [`xs:unsignedLong`] type.
///
/// [`xs:unsignedLong`]: https://www.w3.org/TR/xmlschema-2/#unsignedLong
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedLong {
	inner: u64,
}

thin_impl!(UnsignedLong, const u64);

/// A type analogous to the XML Schema [`xs:unsignedShort`] type.
///
/// [`xs:unsignedShort`]: https://www.w3.org/TR/xmlschema-2/#unsignedShort
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsignedShort {
	inner: u16,
}

thin_impl!(UnsignedShort, const u16);
