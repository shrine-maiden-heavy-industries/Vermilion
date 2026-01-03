/* SPDX-License-Identifier: BSD-3-Clause */

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
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum SemanticTokenModifiers {
	Abstract,
	Async,
	Declaration,
	DefaultLibrary,
	Definition,
	Deprecated,
	Documentation,
	Modification,
	#[serde(rename = "readonly")]
	ReadOnly,
	Static,
	#[serde(untagged)]
	Other(String),
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokens {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) result_id: Option<String>,
	pub(crate) data: Vec<u32>,
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

impl SemanticTokens {
	pub fn new(data: Vec<u32>) -> Self {
		Self { data, result_id: None }
	}

	pub fn with_result_id(mut self, result_id: String) -> Self {
		self.result_id = Some(result_id);
		self
	}

	/// The actual tokens
	pub fn data(&self) -> &Vec<u32> {
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
