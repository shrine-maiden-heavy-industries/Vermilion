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
	pub fn new<T, U>(filter: T, label: U) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			filter:                filter.to_string(),
			label:                 label.to_string(),
			description:           None,
			default:               None,
			supports_condition:    None,
			condition_description: None,
		}
	}

	pub fn with_description<T>(self, description: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.description = Some(description.to_string());
		this
	}

	pub fn with_default(self, default: bool) -> Self {
		let mut this = self;
		this.default = Some(default);
		this
	}

	pub fn with_supports_condition(self, supports_condition: bool) -> Self {
		let mut this = self;
		this.supports_condition = Some(supports_condition);
		this
	}

	pub fn with_condition_description<T>(self, condition_description: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.condition_description = Some(condition_description.to_string());
		this
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

impl Default for ExceptionDetails {
	fn default() -> Self {
		Self::new()
	}
}

impl ExceptionDetails {
	pub fn new() -> Self {
		Self {
			message:         None,
			type_name:       None,
			full_type_name:  None,
			evaluate_name:   None,
			stack_trace:     None,
			inner_exception: None,
		}
	}

	pub fn with_message<T>(self, message: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.message = Some(message.to_string());
		this
	}

	pub fn with_type_name<T>(self, type_name: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.type_name = Some(type_name.to_string());
		this
	}

	pub fn with_full_type_name<T>(self, full_type_name: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.full_type_name = Some(full_type_name.to_string());
		this
	}

	pub fn with_evaluate_name<T>(self, evaluate_name: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.evaluate_name = Some(evaluate_name.to_string());
		this
	}

	pub fn with_stack_trace<T>(self, stack_trace: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.stack_trace = Some(stack_trace.to_string());
		this
	}

	pub fn with_inner_exception(self, inner_exception: Vec<Self>) -> Self {
		let mut this = self;
		this.inner_exception = Some(inner_exception);
		this
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
	pub fn new(names: Vec<String>) -> Self {
		Self { negate: None, names }
	}

	pub fn with_negate(self, negate: bool) -> Self {
		let mut this = self;
		this.negate = Some(negate);
		this
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
