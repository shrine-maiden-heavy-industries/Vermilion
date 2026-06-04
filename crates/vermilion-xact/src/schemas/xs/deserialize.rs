// SPDX-License-Identifier: BSD-3-Clause
//! Deserialization implementations for XML Schema types

macro_rules! thin_deserialize {
	($type:ty, $inner:ty) => {
		impl<'de> serde::Deserialize<'de> for $type {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				// Dispatch to the Deserialize impl for our inner type
				Ok(Self {
					inner: <$inner as serde::Deserialize>::deserialize(deserializer)?,
				})
			}
		}
	};
}

thin_deserialize!(super::AnyURI, fluent_uri::Uri<std::string::String>);
thin_deserialize!(super::Base64Binary, Vec<u8>);
thin_deserialize!(super::Boolean, bool);
thin_deserialize!(super::Byte, i8);
thin_deserialize!(super::Date, chrono::NaiveDate);
thin_deserialize!(super::DateTime, chrono::DateTime<chrono::Utc>);
thin_deserialize!(super::Decimal, f64);
thin_deserialize!(super::Double, f64);
thin_deserialize!(super::Duration, chrono::TimeDelta);
thin_deserialize!(super::Entities, Vec<std::string::String>);
thin_deserialize!(super::Entity, std::string::String);
thin_deserialize!(super::Float, f32);
thin_deserialize!(super::GDay, ());
thin_deserialize!(super::GMonth, ());
thin_deserialize!(super::GMonthDay, ());
thin_deserialize!(super::GYear, ());
thin_deserialize!(super::GYearMonth, ());
thin_deserialize!(super::HexBinary, Vec<u8>);
thin_deserialize!(super::Id, std::string::String);
thin_deserialize!(super::IdRef, std::string::String);
thin_deserialize!(super::IdRefs, Vec<std::string::String>);
thin_deserialize!(super::Int, i32);
thin_deserialize!(super::Integer, i64);
thin_deserialize!(super::Language, std::string::String);
thin_deserialize!(super::Long, i64);
thin_deserialize!(super::Name, std::string::String);
thin_deserialize!(super::NCName, std::string::String);
thin_deserialize!(super::NegativeInteger, i64);
thin_deserialize!(super::NMToken, std::string::String);
thin_deserialize!(super::NMTokens, Vec<std::string::String>);
thin_deserialize!(super::NonNegativeInteger, u64);
thin_deserialize!(super::NonPositiveInteger, i64);
thin_deserialize!(super::NormalizedString, std::string::String);
thin_deserialize!(super::Notation, Vec<std::string::String>);
thin_deserialize!(super::PositiveInteger, u64);
thin_deserialize!(super::QName, std::string::String);
thin_deserialize!(super::Short, i16);
thin_deserialize!(super::String, std::string::String);
thin_deserialize!(super::Time, chrono::NaiveTime);
thin_deserialize!(super::Token, std::string::String);
thin_deserialize!(super::UnsignedByte, u8);
thin_deserialize!(super::UnsignedInt, u32);
thin_deserialize!(super::UnsignedLong, u64);
thin_deserialize!(super::UnsignedShort, u16);
