// SPDX-License-Identifier: BSD-3-Clause

use crate::types::Source;

/// Information about a breakpoint created in `setBreakpoints`, `setFunctionBreakpoints`,
/// `setInstructionBreakpoints`, or `setDataBreakpoints` requests.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoint {
	/// The identifier for the breakpoint.
	///
	/// It is needed if breakpoint events are used to update or remove breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) id:                    Option<i32>,
	/// If true, the breakpoint could be set (but not necessarily at the desired location).
	pub(crate) verified:              bool,
	/// A message about the state of the breakpoint.
	///
	/// This is shown to the user and can be used to explain why a breakpoint could not be
	/// verified.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message:               Option<String>,
	/// The source where the breakpoint is located.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source:                Option<Box<Source>>,
	/// The start line of the actual range covered by the breakpoint.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line:                  Option<u64>,
	/// Start position of the source range covered by the breakpoint.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:                Option<u64>,
	/// The end line of the actual range covered by the breakpoint.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line:              Option<u64>,
	/// End position of the source range covered by the breakpoint.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	///
	/// If no end line is given, then the end column is assumed to be in the start line.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column:            Option<u64>,
	/// A memory reference to where the breakpoint is set
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) instruction_reference: Option<String>,
	/// The offset from the instruction reference.
	///
	/// This can be negative.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) offset:                Option<i64>,
	/// A machine-readable explanation of why a breakpoint may not be verified.
	///
	/// If a breakpoint is verified or a specific reason is not known, the adapter should omit this
	/// property.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) reason:                Option<BreakpointReason>,
}

/// A machine-readable explanation of why a breakpoint may not be verified.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum BreakpointReason {
	/// Indicates a breakpoint might be verified in the future, but the adapter cannot verify it in
	/// the current state.
	Pending,
	/// Indicates a breakpoint was not able to be verified, and the adapter does not believe it can
	/// be verified without intervention.
	Failed,
}

/// Properties of a breakpoint location returned from the `breakpointLocations` request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocation {
	/// Start line of breakpoint location.
	pub(crate) line:       u64,
	/// The start position of a breakpoint location.
	///
	/// Position is measured in UTF-16 code units and the client capability `columnsStartAt1`
	/// determines whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:     Option<u64>,
	/// The end line of breakpoint location if the location covers a range.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line:   Option<u64>,
	/// The end position of a breakpoint location (if the location covers a range).
	///
	/// Position is measured in UTF-16 code units and the client capability `columnsStartAt1`
	/// determines whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column: Option<u64>,
}

/// A [`BreakpointMode`] is provided as a option when setting breakpoints on sources or
/// instructions.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointMode {
	/// The internal ID of the mode. This value is passed to the `setBreakpoints` request.
	pub(crate) mode:        String,
	/// The name of the breakpoint mode. This is shown in the UI.
	pub(crate) label:       String,
	/// A help text providing additional information about the breakpoint mode.
	///
	/// This string is typically shown as a hover and can be translated.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) description: Option<String>,
	/// Describes one or more type of breakpoint this mode applies to.
	pub(crate) applies_to:  Vec<BreakpointModeApplicability>,
}

/// Describes one or more type of breakpoint a [`BreakpointMode`] applies to.
///
/// This is a non-exhaustive enumeration and may expand as future breakpoint types are added.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum BreakpointModeApplicability {
	/// In [`SourceBreakpoint`]s
	Source,
	/// In exception breakpoints applied in the [`ExceptionFilterOptions`]
	///
	/// [`ExceptionFilterOptions`]: crate::types::options::ExceptionFilterOptions
	Exception,
	/// In data breakpoints requested in the [`DataBreakpointInfo`] request
	Data,
	/// In [`InstructionBreakpoint`]s
	Instruction,
	Other(String),
}

/// Properties of a data breakpoint passed to the `setDataBreakpoints` request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpoint {
	/// An id representing the data. This id is returned from the `dataBreakpointInfo` request.
	pub(crate) data_id:       String,
	/// The access type of the data.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) access_type:   Option<DataBreakpointAccessType>,
	/// An expression for conditional breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) condition:     Option<String>,
	/// An expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hit_condition: Option<String>,
}

/// This enumeration defines all possible access types for data breakpoints.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum DataBreakpointAccessType {
	Read,
	Write,
	ReadWrite,
}

/// Properties of a breakpoint passed to the `setFunctionBreakpoints` request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FunctionBreakpoint {
	/// The name of the function.
	pub(crate) name:          String,
	/// An expression for conditional breakpoints.
	///
	/// It is only honored by a debug adapter if the corresponding capability
	/// `supportsConditionalBreakpoints` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) condition:     Option<String>,
	/// An expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsHitConditionalBreakpoints` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hit_condition: Option<String>,
}

/// Properties of a breakpoint passed to the `setInstructionBreakpoints` request
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InstructionBreakpoint {
	/// The instruction reference of the breakpoint.
	///
	/// This should be a memory or instruction pointer reference from an `EvaluateResponse`,
	/// `Variable`, `StackFrame`, `GotoTarget`, or `Breakpoint`.
	pub(crate) instruction_reference: String,
	/// The offset from the instruction reference in bytes.
	///
	/// This can be negative.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) offset:                Option<i64>,
	/// An expression for conditional breakpoints.
	///
	/// It is only honored by a debug adapter if the corresponding capability
	/// `supportsConditionalBreakpoints` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) condition:             Option<String>,
	/// An expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsHitConditionalBreakpoints` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hit_condition:         Option<String>,
	/// The mode of this breakpoint.
	///
	/// If defined, this must be one of the `breakpointModes` the debug adapter advertised in its
	/// `Capabilities`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) mode:                  Option<String>,
}

/// Properties of a breakpoint or logpoint passed to the `setBreakpoints` request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SourceBreakpoint {
	/// The source line of the breakpoint or logpoint.
	pub(crate) line:          u64,
	/// Start position within source line of the breakpoint or logpoint.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:        Option<u64>,
	/// The expression for conditional breakpoints.
	///
	/// It is only honored by a debug adapter if the corresponding capability
	/// `supportsConditionalBreakpoints` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) condition:     Option<String>,
	/// The expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsHitConditionalBreakpoints` is true.
	///
	/// If both this property and `condition` are specified, `hitCondition` should be evaluated
	/// only if the `condition` is met, and the debug adapter should stop only if both conditions
	/// are met.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hit_condition: Option<String>,
	/// If this attribute exists and is non-empty, the debug adapter must not 'break' (stop) but
	/// log the message instead.
	///
	/// Expressions within `{}` are interpolated.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsLogPoints` is true.
	///
	/// If either `hitCondition` or `condition` is specified, then the message should only be
	/// logged if those conditions are met.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) log_message:   Option<String>,
	/// The mode of this breakpoint. If defined, this must be one of the `breakpointModes` the
	/// debug adapter advertised in its `Capabilities`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) mode:          Option<String>,
}

impl Breakpoint {
	pub fn new(verified: bool) -> Self {
		Self {
			id: None,
			verified,
			message: None,
			source: None,
			line: None,
			column: None,
			end_line: None,
			end_column: None,
			instruction_reference: None,
			offset: None,
			reason: None,
		}
	}

	pub fn with_id(self, id: i32) -> Self {
		let mut this = self;
		this.id = Some(id);
		this
	}

	pub fn with_message<T>(self, message: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.message = Some(message.to_string());
		this
	}

	pub fn with_source(self, source: Box<Source>) -> Self {
		let mut this = self;
		this.source = Some(source);
		this
	}

	pub fn with_line(self, line: u64) -> Self {
		let mut this = self;
		this.line = Some(line);
		this
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	pub fn with_end_line(self, end_line: u64) -> Self {
		let mut this = self;
		this.end_line = Some(end_line);
		this
	}

	pub fn with_end_column(self, end_column: u64) -> Self {
		let mut this = self;
		this.end_column = Some(end_column);
		this
	}

	pub fn with_instruction_reference<T>(self, instruction_reference: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.instruction_reference = Some(instruction_reference.to_string());
		this
	}

	pub fn with_offset(self, offset: i64) -> Self {
		let mut this = self;
		this.offset = Some(offset);
		this
	}

	pub fn with_reason(self, reason: BreakpointReason) -> Self {
		let mut this = self;
		this.reason = Some(reason);
		this
	}

	/// The identifier for the breakpoint.
	///
	/// It is needed if breakpoint events are used to update or remove breakpoints.
	pub fn id(&self) -> Option<i32> {
		self.id
	}

	/// If true, the breakpoint could be set (but not necessarily at the desired location).
	pub fn verified(&self) -> bool {
		self.verified
	}

	/// A message about the state of the breakpoint.
	///
	/// This is shown to the user and can be used to explain why a breakpoint could not be
	/// verified.
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	/// The source where the breakpoint is located.
	pub fn source(&self) -> Option<&Source> {
		self.source.as_deref()
	}

	/// The start line of the actual range covered by the breakpoint.
	pub fn line(&self) -> Option<u64> {
		self.line
	}

	/// Start position of the source range covered by the breakpoint.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The end line of the actual range covered by the breakpoint.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// End position of the source range covered by the breakpoint.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	///
	/// If no end line is given, then the end column is assumed to be in the start line.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}

	/// A memory reference to where the breakpoint is set
	pub fn instruction_reference(&self) -> Option<&String> {
		self.instruction_reference.as_ref()
	}

	/// The offset from the instruction reference.
	///
	/// This can be negative.
	pub fn offset(&self) -> Option<i64> {
		self.offset
	}

	/// A machine-readable explanation of why a breakpoint may not be verified.
	///
	/// If a breakpoint is verified or a specific reason is not known, the adapter should omit this
	/// property.
	pub fn reason(&self) -> Option<BreakpointReason> {
		self.reason
	}
}

impl BreakpointLocation {
	pub fn new(line: u64) -> Self {
		Self {
			line,
			column: None,
			end_line: None,
			end_column: None,
		}
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	pub fn with_end_line(self, end_line: u64) -> Self {
		let mut this = self;
		this.end_line = Some(end_line);
		this
	}

	pub fn with_end_column(self, end_column: u64) -> Self {
		let mut this = self;
		this.end_column = Some(end_column);
		this
	}

	/// Start line of breakpoint location.
	pub fn line(&self) -> u64 {
		self.line
	}

	/// The start position of a breakpoint location.
	///
	/// Position is measured in UTF-16 code units and the client capability `columnsStartAt1`
	/// determines whether it is 0- or 1-based.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The end line of breakpoint location if the location covers a range.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// The end position of a breakpoint location (if the location covers a range).
	///
	/// Position is measured in UTF-16 code units and the client capability `columnsStartAt1`
	/// determines whether it is 0- or 1-based.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}
}

impl BreakpointMode {
	pub fn new<T, U>(mode: T, label: U, applies_to: Vec<BreakpointModeApplicability>) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			mode: mode.to_string(),
			label: label.to_string(),
			description: None,
			applies_to,
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

	/// The internal ID of the mode. This value is passed to the `setBreakpoints` request.
	pub fn mode(&self) -> &String {
		&self.mode
	}

	/// The name of the breakpoint mode. This is shown in the UI.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// A help text providing additional information about the breakpoint mode.
	///
	/// This string is typically shown as a hover and can be translated.
	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	/// Describes one or more type of breakpoint this mode applies to.
	pub fn applies_to(&self) -> &Vec<BreakpointModeApplicability> {
		&self.applies_to
	}
}

impl DataBreakpoint {
	pub fn new<T>(data_id: T) -> Self
	where
		T: ToString,
	{
		Self {
			data_id:       data_id.to_string(),
			access_type:   None,
			condition:     None,
			hit_condition: None,
		}
	}

	pub fn with_access_type(self, access_type: DataBreakpointAccessType) -> Self {
		let mut this = self;
		this.access_type = Some(access_type);
		this
	}

	pub fn with_condition<T>(self, condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.condition = Some(condition.to_string());
		this
	}

	pub fn with_hit_condition<T>(self, hit_condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.hit_condition = Some(hit_condition.to_string());
		this
	}

	/// An id representing the data. This id is returned from the `dataBreakpointInfo` request.
	pub fn data_id(&self) -> &String {
		&self.data_id
	}

	/// The access type of the data.
	pub fn access_type(&self) -> Option<DataBreakpointAccessType> {
		self.access_type
	}

	/// An expression for conditional breakpoints.
	pub fn condition(&self) -> Option<&String> {
		self.condition.as_ref()
	}

	/// An expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	pub fn hit_condition(&self) -> Option<&String> {
		self.hit_condition.as_ref()
	}
}

impl FunctionBreakpoint {
	pub fn new<T>(name: T) -> Self
	where
		T: ToString,
	{
		Self {
			name:          name.to_string(),
			condition:     None,
			hit_condition: None,
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

	pub fn with_hit_condition<T>(self, hit_condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.hit_condition = Some(hit_condition.to_string());
		this
	}

	/// The name of the function.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// An expression for conditional breakpoints.
	///
	/// It is only honored by a debug adapter if the corresponding capability
	/// `supportsConditionalBreakpoints` is true.
	pub fn condition(&self) -> Option<&String> {
		self.condition.as_ref()
	}

	/// An expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsHitConditionalBreakpoints` is true.
	pub fn hit_condition(&self) -> Option<&String> {
		self.hit_condition.as_ref()
	}
}

impl InstructionBreakpoint {
	pub fn new<T>(instruction_reference: T) -> Self
	where
		T: ToString,
	{
		Self {
			instruction_reference: instruction_reference.to_string(),
			offset:                None,
			condition:             None,
			hit_condition:         None,
			mode:                  None,
		}
	}

	pub fn with_offset(self, offset: i64) -> Self {
		let mut this = self;
		this.offset = Some(offset);
		this
	}

	pub fn with_condition<T>(self, condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.condition = Some(condition.to_string());
		this
	}

	pub fn with_hit_condition<T>(self, hit_condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.hit_condition = Some(hit_condition.to_string());
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

	/// The instruction reference of the breakpoint.
	///
	/// This should be a memory or instruction pointer reference from an `EvaluateResponse`,
	/// `Variable`, `StackFrame`, `GotoTarget`, or `Breakpoint`.
	pub fn instruction_reference(&self) -> &String {
		&self.instruction_reference
	}

	/// The offset from the instruction reference in bytes.
	///
	/// This can be negative.
	pub fn offset(&self) -> Option<i64> {
		self.offset
	}

	/// An expression for conditional breakpoints.
	///
	/// It is only honored by a debug adapter if the corresponding capability
	/// `supportsConditionalBreakpoints` is true.
	pub fn condition(&self) -> Option<&String> {
		self.condition.as_ref()
	}

	/// An expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsHitConditionalBreakpoints` is true.
	pub fn hit_condition(&self) -> Option<&String> {
		self.hit_condition.as_ref()
	}

	/// The mode of this breakpoint.
	///
	/// If defined, this must be one of the `breakpointModes` the debug adapter advertised in its
	/// `Capabilities`.
	pub fn mode(&self) -> Option<&String> {
		self.mode.as_ref()
	}
}

impl SourceBreakpoint {
	pub fn new(line: u64) -> Self {
		Self {
			line,
			column: None,
			condition: None,
			hit_condition: None,
			log_message: None,
			mode: None,
		}
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	pub fn with_condition<T>(self, condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.condition = Some(condition.to_string());
		this
	}

	pub fn with_hit_condition<T>(self, hit_condition: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.hit_condition = Some(hit_condition.to_string());
		this
	}

	pub fn with_log_message<T>(self, log_message: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.log_message = Some(log_message.to_string());
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

	/// The source line of the breakpoint or logpoint.
	pub fn line(&self) -> u64 {
		self.line
	}

	/// Start position within source line of the breakpoint or logpoint.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The expression for conditional breakpoints.
	///
	/// It is only honored by a debug adapter if the corresponding capability
	/// `supportsConditionalBreakpoints` is true.
	pub fn condition(&self) -> Option<&String> {
		self.condition.as_ref()
	}

	/// The expression that controls how many hits of the breakpoint are ignored.
	///
	/// The debug adapter is expected to interpret the expression as needed.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsHitConditionalBreakpoints` is true.
	///
	/// If both this property and `condition` are specified, `hitCondition` should be evaluated
	/// only if the `condition` is met, and the debug adapter should stop only if both conditions
	/// are met.
	pub fn hit_condition(&self) -> Option<&String> {
		self.hit_condition.as_ref()
	}

	/// If this attribute exists and is non-empty, the debug adapter must not 'break' (stop) but
	/// log the message instead.
	///
	/// Expressions within `{}` are interpolated.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsLogPoints` is true.
	///
	/// If either `hitCondition` or `condition` is specified, then the message should only be
	/// logged if those conditions are met.
	pub fn log_message(&self) -> Option<&String> {
		self.log_message.as_ref()
	}

	/// The mode of this breakpoint. If defined, this must be one of the `breakpointModes` the
	/// debug adapter advertised in its `Capabilities`.
	pub fn mode(&self) -> Option<&String> {
		self.mode.as_ref()
	}
}
