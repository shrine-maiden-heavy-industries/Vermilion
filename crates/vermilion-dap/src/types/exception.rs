// SPDX-License-Identifier: BSD-3-Clause

/// This enumeration defines all possible conditions when a thrown exception should result in a
/// break.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ExceptionBreakMode {
	/// Never breaks,
	Never,
	/// Always breaks,
	Always,
	/// Breaks when exception unhandled,
	Unhandled,
	/// Breaks if the exception is not handled by user code.
	UserUnhandled,
}

/// An `ExceptionBreakpointsFilter` is shown in the UI as an filter option for configuring how
/// exceptions are dealt with.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionBreakpointsFilter {
	/// The internal ID of the filter option.
	///
	/// This value is passed to the `setExceptionBreakpoints` request.
	pub(crate) filter:                String,
	/// The name of the filter option. This is shown in the UI.
	pub(crate) label:                 String,
	/// A help text providing additional information about the exception filter.
	///
	/// This string is typically shown as a hover and can be translated.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) description:           Option<String>,
	/// Initial value of the filter option.
	///
	/// If not specified a value false is assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) default:               Option<bool>,
	/// Controls whether a condition can be specified for this filter option.
	///
	/// If false or missing, a condition can not be set.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_condition:    Option<bool>,
	/// A help text providing information about the condition.
	///
	/// This string is shown as the placeholder text for a text box and can be translated.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) condition_description: Option<String>,
}

/// Detailed information about an exception that has occurred.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
	/// Message contained in the exception
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message:         Option<String>,
	/// Short type name of the exception object.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) type_name:       Option<String>,
	/// Fully-qualified type name of the exception object.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) full_type_name:  Option<String>,
	/// An expression that can be evaluated in the current scope to obtain the exception object.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) evaluate_name:   Option<String>,
	/// Stack trace at the time the exception was thrown.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) stack_trace:     Option<String>,
	/// Details of the exception contained by this exception, if any.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inner_exception: Option<Vec<Self>>,
}

/// An `ExceptionPathSegment` represents a segment in a path that is used to match leafs or nodes in
/// a tree of exceptions.
///
/// If a segment consists of more than one name, it matches the names provided if `negate` is false
/// or missing, or it matches anything except the names provided if `negate` is true.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionPathSegment {
	/// If false or missing this segment matches the names provided, otherwise it matches anything
	/// except the names provided.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) negate: Option<bool>,
	/// Depending on the value of `negate` the names that should match or not match.
	pub(crate) names:  Vec<String>,
}

impl ExceptionBreakpointsFilter {
	pub fn new(
		filter: String,
		label: String,
		description: Option<String>,
		default: Option<bool>,
		supports_condition: Option<bool>,
		condition_description: Option<String>,
	) -> Self {
		Self {
			filter,
			label,
			description,
			default,
			supports_condition,
			condition_description,
		}
	}

	/// The internal ID of the filter option.
	///
	/// This value is passed to the `setExceptionBreakpoints` request.
	pub fn filter(&self) -> &String {
		&self.filter
	}

	/// The name of the filter option. This is shown in the UI.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// A help text providing additional information about the exception filter.
	///
	/// This string is typically shown as a hover and can be translated.
	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	/// Initial value of the filter option.
	///
	/// If not specified a value false is assumed.
	pub fn default(&self) -> bool {
		self.default.unwrap_or(false)
	}

	/// Controls whether a condition can be specified for this filter option.
	///
	/// If false a condition can not be set.
	pub fn supports_condition(&self) -> bool {
		self.supports_condition.unwrap_or(false)
	}

	/// A help text providing information about the condition.
	///
	/// This string is shown as the placeholder text for a text box and can be translated.
	pub fn condition_description(&self) -> Option<&String> {
		self.description.as_ref()
	}
}

impl ExceptionDetails {
	pub fn new(
		message: Option<String>,
		type_name: Option<String>,
		full_type_name: Option<String>,
		evaluate_name: Option<String>,
		stack_trace: Option<String>,
		inner_exception: Option<Vec<Self>>,
	) -> Self {
		Self {
			message,
			type_name,
			full_type_name,
			evaluate_name,
			stack_trace,
			inner_exception,
		}
	}

	/// Message contained in the exception
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	/// Short type name of the exception object.
	pub fn type_name(&self) -> Option<&String> {
		self.type_name.as_ref()
	}

	/// Fully-qualified type name of the exception object.
	pub fn full_type_name(&self) -> Option<&String> {
		self.full_type_name.as_ref()
	}

	/// An expression that can be evaluated in the current scope to obtain the exception object.
	pub fn evaluate_name(&self) -> Option<&String> {
		self.evaluate_name.as_ref()
	}

	/// Stack trace at the time the exception was thrown.
	pub fn stack_trace(&self) -> Option<&String> {
		self.stack_trace.as_ref()
	}

	/// Details of the exception contained by this exception, if any.
	pub fn inner_exception(&self) -> Option<&Vec<Self>> {
		self.inner_exception.as_ref()
	}
}

impl ExceptionPathSegment {
	pub fn new(negate: Option<bool>, names: Vec<String>) -> Self {
		Self { negate, names }
	}

	/// If false this segment matches the names provided, otherwise it matches anything
	/// except the names provided.
	pub fn negate(&self) -> bool {
		self.negate.unwrap_or(false)
	}

	/// Depending on the value of `negate` the names that should match or not match.
	pub fn names(&self) -> &Vec<String> {
		&self.names
	}
}
