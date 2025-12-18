/* SPDX-License-Identifier: BSD-3-Clause */
//!
//!
//!

use std::collections::HashMap;

pub type Object = HashMap<String, Any>;
pub type Array = Vec<Any>;
pub type Uri = fluent_uri::Uri<String>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Any {
	Object(Object),
	Array(Array),
	String(String),
	Integer(i32),
	UInteger(u32),
	Decimal(i32),
	Boolean(bool),
	Null,
}

// `integer | string` LSP Type
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum IntegerOrString {
	Integer(i32),
	String(String),
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Position {
	line: u32,
	character: u32,
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Range {
	start: Position,
	end: Position,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Location {
	uri: Uri,
	range: Range,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LocationLink {
	target_uri: Uri,
	target_range: Range,
	target_selection_range: Range,
	#[serde(skip_serializing_if = "Option::is_none")]
	origin_selection_range: Option<Range>,
}

// TODO(aki):
// Figure out a way to turn a Chumsky `Span` into this.
// It's annoying because they use glyph offsets rather than lines + char, so
// we need to likely have a custom `Span` type that keeps track of this as extra
// context information.
impl Position {
	pub const fn new(line: u32, character: u32) -> Self {
		Self { line, character }
	}

	pub const fn line(&self) -> &u32 {
		&self.line
	}

	pub const fn character(&self) -> &u32 {
		&self.character
	}
}

// TODO(aki): See TODO on [`Position`]
impl Range {
	pub const fn new(start: Position, end: Position) -> Self {
		Self { start, end }
	}

	pub const fn start(&self) -> &Position {
		&self.start
	}

	pub const fn end(&self) -> &Position {
		&self.end
	}
}

impl Location {
	pub const fn new(uri: Uri, range: Range) -> Self {
		Self { uri, range }
	}

	pub const fn uri(&self) -> &Uri {
		&self.uri
	}

	pub const fn range(&self) -> &Range {
		&self.range
	}
}

impl LocationLink {
	pub const fn new(
		target_uri: Uri,
		target_range: Range,
		target_selection: Range,
		origin_selection: Option<Range>,
	) -> Self {
		Self {
			origin_selection_range: origin_selection,
			target_uri,
			target_range,
			target_selection_range: target_selection,
		}
	}

	pub const fn target_uri(&self) -> &Uri {
		&self.target_uri
	}

	pub const fn target_range(&self) -> &Range {
		&self.target_range
	}

	pub const fn target_selection(&self) -> &Range {
		&self.target_selection_range
	}

	pub const fn origin_selection(&self) -> Option<&Range> {
		self.origin_selection_range.as_ref()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
