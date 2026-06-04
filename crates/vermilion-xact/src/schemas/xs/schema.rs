// SPDX-License-Identifier: BSD-3-Clause
//! JSON Schema implementations for XML Schema types.

macro_rules! thin_schema {
	($type:ty, $name:literal, $desc:literal, $schema_type:literal) => {
		impl schemars::JsonSchema for $type {
			fn schema_name() -> std::borrow::Cow<'static, str> {
				$name.into()
			}

			fn schema_id() -> std::borrow::Cow<'static, str> {
				concat!("xs::", $name).into()
			}

			fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
				schemars::json_schema!({
					"description": $desc,
					"type": $schema_type,
				})
			}
		}
	};
}

thin_schema!(super::AnyURI, "anyURI", "W3C XML Schema anyURI", "");
thin_schema!(
	super::Base64Binary,
	"base64Binary",
	"W3C XML Schema base64Binary",
	""
);
thin_schema!(super::Boolean, "boolean", "W3C XML Schema boolean", "");
thin_schema!(super::Byte, "byte", "W3C XML Schema byte", "");
thin_schema!(super::Date, "date", "W3C XML Schema date", "");
thin_schema!(super::DateTime, "dateTime", "W3C XML Schema dateTime", "");
thin_schema!(super::Decimal, "decimal", "W3C XML Schema decimal", "");
thin_schema!(super::Double, "double", "W3C XML Schema double", "number");
thin_schema!(super::Duration, "duration", "W3C XML Schema duration", "");
thin_schema!(super::Entities, "ENTITIES", "W3C XML Schema ENTITIES", "");
thin_schema!(super::Entity, "ENTITY", "W3C XML Schema ENTITY", "");
thin_schema!(super::Float, "float", "W3C XML Schema float", "");
thin_schema!(super::GDay, "gDay", "W3C XML Schema gDay", "");
thin_schema!(super::GMonth, "gMonth", "W3C XML Schema gMonth", "");
thin_schema!(
	super::GMonthDay,
	"gMonthDay",
	"W3C XML Schema gMonthDay",
	""
);
thin_schema!(super::GYear, "gYear", "W3C XML Schema gYear", "");
thin_schema!(
	super::GYearMonth,
	"gYearMonth",
	"W3C XML Schema gYearMonth",
	""
);
thin_schema!(
	super::HexBinary,
	"hexBinary",
	"W3C XML Schema hexBinary",
	""
);
thin_schema!(super::Id, "ID", "W3C XML Schema ID", "");
thin_schema!(super::IdRef, "IDREF", "W3C XML Schema IDREF", "");
thin_schema!(super::IdRefs, "IDREFS", "W3C XML Schema IDREFS", "");
thin_schema!(super::Int, "int", "W3C XML Schema int", "");
thin_schema!(super::Integer, "integer", "W3C XML Schema integer", "");
thin_schema!(super::Language, "language", "W3C XML Schema language", "");
thin_schema!(super::Long, "long", "W3C XML Schema long", "");
thin_schema!(super::Name, "name", "W3C XML Schema name", "");
thin_schema!(super::NCName, "NCName", "W3C XML Schema NCName", "");
thin_schema!(
	super::NegativeInteger,
	"negativeInteger",
	"W3C XML Schema negativeInteger",
	""
);
thin_schema!(super::NMToken, "NMTOKEN", "W3C XML Schema NMTOKEN", "");
thin_schema!(super::NMTokens, "NMTOKENS", "W3C XML Schema NMTOKENS", "");
thin_schema!(
	super::NonNegativeInteger,
	"nonNegativeInteger",
	"W3C XML Schema nonNegativeInteger",
	""
);
thin_schema!(
	super::NonPositiveInteger,
	"nonPositiveInteger",
	"W3C XML Schema nonPositiveInteger",
	""
);
thin_schema!(
	super::NormalizedString,
	"normalizedString",
	"W3C XML Schema normalizedString",
	""
);
thin_schema!(super::Notation, "NOTATION", "W3C XML Schema NOTATION", "");
thin_schema!(
	super::PositiveInteger,
	"positiveInteger",
	"W3C XML Schema positiveInteger",
	""
);
thin_schema!(super::QName, "QName", "W3C XML Schema QName", "");
thin_schema!(super::Short, "short", "W3C XML Schema short", "");
thin_schema!(super::String, "string", "W3C XML Schema string", "");
thin_schema!(super::Time, "time", "W3C XML Schema time", "");
thin_schema!(super::Token, "token", "W3C XML Schema token", "");
thin_schema!(
	super::UnsignedByte,
	"unsignedByte",
	"W3C XML Schema unsignedByte",
	""
);
thin_schema!(
	super::UnsignedInt,
	"unsignedInt",
	"W3C XML Schema unsignedInt",
	""
);
thin_schema!(
	super::UnsignedLong,
	"unsignedLong",
	"W3C XML Schema unsignedLong",
	""
);
thin_schema!(
	super::UnsignedShort,
	"unsignedShort",
	"W3C XML Schema unsignedShort",
	""
);
