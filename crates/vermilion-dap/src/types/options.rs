// SPDX-License-Identifier: BSD-3-Clause

use crate::types::exception::{ExceptionBreakMode, ExceptionPathSegment};

/// An `ExceptionFilterOptions` is used to specify an exception filter together with a condition for
/// the `setExceptionBreakpoints` request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionFilterOptions {
	/// ID of an exception filter returned by the `exceptionBreakpointFilters` capability.
	pub(crate) filter_id: String,
	/// An expression for conditional exceptions.
	///
	/// The exception breaks into the debugger if the result of the condition is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) condition: Option<String>,
	/// The mode of this exception breakpoint.
	///
	/// If defined, this must be one of the `breakpointModes` the debug adapter advertised in its
	/// `Capabilities`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) mode:      Option<String>,
}

/// An `ExceptionOptions` assigns configuration options to a set of exceptions.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionOptions {
	/// A path that selects a single or multiple exceptions in a tree.
	///
	/// If `path` is missing, the whole tree is selected.
	///
	/// By convention the first segment of the path is a category that is used to group exceptions
	/// in the UI.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) path:       Option<Vec<ExceptionPathSegment>>,
	/// Condition when a thrown exception should result in a break.
	pub(crate) break_mode: ExceptionBreakMode,
}

impl ExceptionFilterOptions {
	pub fn new<T>(filter_id: T) -> Self
	where
		T: ToString,
	{
		Self {
			filter_id: filter_id.to_string(),
			condition: None,
			mode:      None,
		}
	}

	pub fn with_condition<T>(self, condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.condition = Some(condition.to_string());
		this
	}

	pub fn with_mode<T>(self, mode: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.mode = Some(mode.to_string());
		this
	}

	/// ID of an exception filter returned by the `exceptionBreakpointFilters` capability.
	pub fn filter_id(&self) -> &String {
		&self.filter_id
	}

	/// An expression for conditional exceptions.
	///
	/// The exception breaks into the debugger if the result of the condition is true.
	pub fn condition(&self) -> Option<&String> {
		self.condition.as_ref()
	}

	/// The mode of this exception breakpoint.
	///
	/// If defined, this must be one of the `breakpointModes` the debug adapter advertised in its
	/// `Capabilities`.
	pub fn mode(&self) -> Option<&String> {
		self.mode.as_ref()
	}
}

impl ExceptionOptions {
	pub fn new(break_mode: ExceptionBreakMode) -> Self {
		Self { path: None, break_mode }
	}

	pub fn with_path(self, path: Vec<ExceptionPathSegment>) -> Self {
		let mut this = self;
		this.path = Some(path);
		this
	}

	/// A path that selects a single or multiple exceptions in a tree.
	///
	/// If `path` is missing, the whole tree is selected.
	///
	/// By convention the first segment of the path is a category that is used to group exceptions
	/// in the UI.
	pub fn path(&self) -> Option<&Vec<ExceptionPathSegment>> {
		self.path.as_ref()
	}

	/// Condition when a thrown exception should result in a break.
	pub fn break_mode(&self) -> ExceptionBreakMode {
		self.break_mode
	}
}
