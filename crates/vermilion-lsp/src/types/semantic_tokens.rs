/* SPDX-License-Identifier: BSD-3-Clause */
use bitmask_enum::bitmask;

/// A set of predefined token types.
///
/// This set is not fixed a client can specify additional token types via the
/// corresponding client capabilities.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
#[repr(u32)]
pub enum SemanticTokenType {
	Class,
	Comment,
	/// since: 3.17.0
	Decorator,
	Enum,
	EnumMember,
	Event,
	Function,
	Interface,
	Keyword,
	Macro,
	Method,
	Modifier,
	Namespace,
	Number,
	Operator,
	Parameter,
	Property,
	Regexp,
	String,
	Struct,
	/// Represents a generic type.
	///
	/// Acts as a fallback for types which can't be mapped to a specific type like class or enum.
	Type,
	TypeParameter,
	Variable,
	#[serde(untagged)]
	Other(String),
}

/// A set of predefined token modifiers.
///
/// This set is not fixed a client can specify additional token types via the
/// corresponding client capabilities.
///
/// since: 3.16.0
#[bitmask(u32)]
pub enum SemanticTokenModifiers {
	Abstract,
	Async,
	Declaration,
	DefaultLibrary,
	Definition,
	Deprecated,
	Documentation,
	Modification,
	ReadOnly,
	Static,
	User1,
	User2,
	User3,
	User4,
	User5,
	User6,
	User7,
	User8,
	User9,
	User10,
	User11,
	User12,
	User13,
	User14,
	User15,
	User16,
	User17,
	User18,
	User19,
	User20,
	User21,
	User22,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensLegend {
	pub(crate) token_types: Vec<String>,
	pub(crate) token_modifiers: Vec<String>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokens {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) result_id: Option<String>,
	pub(crate) data: Vec<SemanticToken>,
}

/// See [`SemanticTokens`]
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensPartialResult {
	pub(crate) data: Vec<u32>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensDelta {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) result_id: Option<String>,
	pub(crate) edits: Vec<SemanticTokensEdit>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensDeltaPartialResult {
	pub(crate) edits: Vec<SemanticTokensEdit>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensEdit {
	pub(crate) start: u32,
	pub(crate) delete_count: u32,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<Vec<u32>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct SemanticToken {
	#[serde(flatten)]
	line: u32,
	#[serde(flatten)]
	character: u32,
	#[serde(flatten)]
	length: u32,
	#[serde(flatten)]
	token_type: SemanticTokenType,
	#[serde(flatten)]
	modifiers: SemanticTokenModifiers,
}

impl<'de> serde::Deserialize<'de> for SemanticTokenModifiers {
	fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
	where
		Deserializer: serde::Deserializer<'de> {
		struct SemanticTokenModifierVisitor;

		impl<'de> serde::de::Visitor<'de> for SemanticTokenModifierVisitor {
			type Value = SemanticTokenModifiers;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("A u32 containing packed SemanticTokenModifiers")
			}

			fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
			where
				E: serde::de::Error, {
				Ok(SemanticTokenModifiers::from(value))
			}
		}

		deserializer.deserialize_u32(SemanticTokenModifierVisitor)
	}
}

impl serde::Serialize for SemanticTokenModifiers {
	fn serialize<Serializer>(&self, serializer: Serializer) -> Result<Serializer::Ok, Serializer::Error>
	where
		Serializer: serde::Serializer {
		serializer.serialize_u32(self.bits)
	}
}

impl SemanticTokensLegend {
	pub fn new(token_types: Vec<String>, token_modifiers: Vec<String>) -> Self {
		Self { token_types, token_modifiers }
	}

	pub fn from_str_vec(token_types: Vec<&str>, token_modifiers: Vec<&str>) -> Self {
		Self::new(
			token_types.into_iter().map(String::from).collect(),
			token_modifiers.into_iter().map(String::from).collect()
		)
	}

	/// The token types a server uses.
	pub fn token_types(&self) -> &Vec<String> {
		&self.token_types
	}

	/// The token modifiers a server uses.
	pub fn token_modifiers(&self) -> &Vec<String> {
		&self.token_modifiers
	}
}

impl SemanticTokens {
	pub fn new(data: Vec<SemanticToken>) -> Self {
		Self { data, result_id: None }
	}

	pub fn with_result_id(mut self, result_id: String) -> Self {
		self.result_id = Some(result_id);
		self
	}

	/// The actual tokens
	pub fn data(&self) -> &Vec<SemanticToken> {
		&self.data
	}

	/// An optional result id.
	///
	/// If provided and clients support delta updating the client will include the result id in the next
	/// semantic token request.
	///
	/// A server can then instead of computing all semantic tokens again simply send a delta.
	pub fn result_id(&self) -> Option<&String> {
		self.result_id.as_ref()
	}
}

impl SemanticTokensPartialResult {
	pub fn new(data: Vec<u32>) -> Self {
		Self { data }
	}

	/// See [`SemanticTokens::data`]
	pub fn data(&self) -> &Vec<u32> {
		&self.data
	}
}

impl SemanticTokensDelta {
	pub fn new(edits: Vec<SemanticTokensEdit>) -> Self {
		Self { edits, result_id: None }
	}

	pub fn with_result_id(mut self, result_id: String) -> Self {
		self.result_id = Some(result_id);
		self
	}

	///The semantic token edits to transform a previous result into a new result
	pub fn edits(&self) -> &Vec<SemanticTokensEdit> {
		&self.edits
	}

	pub fn result_id(&self) -> Option<&String> {
		self.result_id.as_ref()
	}
}

impl SemanticTokensDeltaPartialResult {
	pub fn new(edits: Vec<SemanticTokensEdit>) -> Self {
		Self { edits }
	}

	pub fn edits(&self) -> &Vec<SemanticTokensEdit> {
		&self.edits
	}
}

impl SemanticTokensEdit {
	pub fn new(start: u32, delete_count: u32) -> Self {
		Self { start, delete_count, data: None }
	}
	pub fn with_data(mut self, data: Vec<u32>) -> Self {
		self.data = Some(data);
		self
	}

	/// The start offset of the edit
	pub fn start(&self) -> &u32 {
		&self.start
	}

	/// The count of elements to remove
	pub fn delete_count(&self) -> &u32 {
		&self.delete_count
	}

	/// The elements to insert
	pub fn data(&self) -> Option<&Vec<u32>> {
		self.data.as_ref()
	}
}
