// SPDX-License-Identifier: BSD-3-Clause
//! Serialization implementations for XML Schema types

macro_rules! thin_serialize {
	($type:ty, $inner:ty) => {
		impl serde::Serialize for $type {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: serde::Serializer,
			{
				// Dispatch to the Serialize impl for our inner type
				<$inner as serde::Serialize>::serialize(&self.inner, serializer)
			}
		}
	};
}

thin_serialize!(super::AnyURI, fluent_uri::Uri<std::string::String>);
thin_serialize!(super::Base64Binary, Vec<u8>);
thin_serialize!(super::Boolean, bool);
thin_serialize!(super::Byte, i8);
thin_serialize!(super::Date, chrono::NaiveDate);
thin_serialize!(super::DateTime, chrono::DateTime<chrono::Utc>);
thin_serialize!(super::Decimal, f64);
thin_serialize!(super::Double, f64);
thin_serialize!(super::Duration, chrono::TimeDelta);
thin_serialize!(super::ENTITIES, Vec<std::string::String>);
thin_serialize!(super::ENTITY, std::string::String);
thin_serialize!(super::Float, f32);
thin_serialize!(super::GDay, ());
thin_serialize!(super::GMonth, ());
thin_serialize!(super::GMonthDay, ());
thin_serialize!(super::GYear, ());
thin_serialize!(super::GYearMonth, ());
thin_serialize!(super::HexBinary, Vec<u8>);
thin_serialize!(super::ID, std::string::String);
thin_serialize!(super::IDREF, std::string::String);
thin_serialize!(super::IDREFS, Vec<std::string::String>);
thin_serialize!(super::Int, i32);
thin_serialize!(super::Integer, i64);
thin_serialize!(super::Language, std::string::String);
thin_serialize!(super::Long, i64);
thin_serialize!(super::Name, std::string::String);
thin_serialize!(super::NCName, std::string::String);
thin_serialize!(super::NegativeInteger, i64);
thin_serialize!(super::NMTOKEN, std::string::String);
thin_serialize!(super::NMTOKENS, Vec<std::string::String>);
thin_serialize!(super::NonNegativeInteger, u64);
thin_serialize!(super::NonPositiveInteger, i64);
thin_serialize!(super::NormalizedString, std::string::String);
thin_serialize!(super::NOTATION, Vec<std::string::String>);
thin_serialize!(super::PositiveInteger, u64);
thin_serialize!(super::QName, std::string::String);
thin_serialize!(super::Short, i16);
thin_serialize!(super::String, std::string::String);
thin_serialize!(super::Time, chrono::NaiveTime);
thin_serialize!(super::Token, std::string::String);
thin_serialize!(super::UnsignedByte, u8);
thin_serialize!(super::UnsignedInt, u32);
thin_serialize!(super::UnsignedLong, u64);
thin_serialize!(super::UnsignedShort, u16);
