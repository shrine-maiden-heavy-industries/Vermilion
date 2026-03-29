// SPDX-License-Identifier: BSD-3-Clause

use crate::types::{
	CompletionItem, DisassembledInstruction, GotoTarget, Module, Scope, Source, StackFrame,
	StepInTarget, Thread,
	breakpoint::{Breakpoint, BreakpointLocation, DataBreakpointAccessType},
	capabilities::Capabilities,
	exception::{ExceptionBreakMode, ExceptionDetails},
	variable::{Variable, VariablePresentationHint},
};

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocationsParams {
	/// Sorted set of possible breakpoint locations.
	pub(crate) breakpoints: Vec<BreakpointLocation>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsParams {
	/// The possible completions for .
	pub(crate) targets: Vec<CompletionItem>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ContinueParams {
	/// If omitted or set to `true`, this response signals to the client that all threads have been
	/// resumed.
	///
	/// The value `false` indicates that not all threads were resumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) all_threads_continued: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpointInfoParams {
	/// An identifier for the data on which a data breakpoint can be registered with the
	/// `setDataBreakpoints` request or null if no data breakpoint is available.
	///
	/// If a `variablesReference` or `frameId` is passed, the `dataId` is valid in the current
	/// suspended state, otherwise it's valid indefinitely.
	///
	/// Breakpoints set using the `dataId` in the `setDataBreakpoints` request may outlive the
	/// lifetime of the associated `dataId`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data_id:      Option<String>,
	/// UI string that describes on what data the breakpoint is set on or why a data breakpoint is
	/// not available.
	pub(crate) description:  String,
	/// Attribute lists the available access types for a potential data breakpoint. A UI client
	/// could surface this information.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) access_types: Option<Vec<DataBreakpointAccessType>>,
	/// Attribute indicates that a potential data breakpoint could be persisted across sessions.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) can_persist:  Option<bool>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleParams {
	/// The list of disassembled instructions.
	pub(crate) instructions: Vec<DisassembledInstruction>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateParams {
	/// The result of the evaluate request.
	pub(crate) result:                   String,
	/// The type of the evaluate result.
	///
	/// This attribute should only be returned by a debug adapter if the corresponding capability
	/// `supportsVariableType` is true.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "type")]
	pub(crate) typ:                      Option<String>,
	/// Properties of an evaluate result that can be used to determine how to render the result in
	/// the UI.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) presentation_hint:        Option<VariablePresentationHint>,
	/// If `variablesReference` is > 0, the evaluate result is structured and its children can
	/// be retrieved by passing `variablesReference` to the `variables` request as long as
	/// execution remains suspended.
	pub(crate) variables_reference:      u32,
	/// The number of named child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) named_variables:          Option<u32>,
	/// The number of indexed child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) indexed_variables:        Option<u32>,
	/// A memory reference to a location appropriate for this result.
	///
	/// For pointer type eval results, this is generally a reference to the memory address
	/// contained in the pointer.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) memory_reference:         Option<String>,
	/// A reference that allows the client to request the location where the returned value is
	/// declared. For example, if a function pointer is returned, the adapter may be able to look
	/// up the function's location. This should be present only if the adapter is likely to be able
	/// to resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) value_location_reference: Option<i32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfoParams {
	/// ID of the exception that was thrown.
	pub(crate) exception_id: String,
	/// Descriptive text for the exception.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) description:  Option<String>,
	/// Mode that caused the exception notification to be raised.
	pub(crate) break_mode:   ExceptionBreakMode,
	/// Detailed information about the exception.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) details:      Option<ExceptionDetails>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GotoTargetsParams {
	/// The possible goto targets of the specified location.
	pub(crate) targets: Vec<GotoTarget>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
	/// The capabilities of this debug adapter.
	#[serde(flatten)]
	pub(crate) capabilities: Capabilities,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedSourcesParams {
	/// Set of loaded sources.
	pub(crate) sources: Vec<Source>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationsParams {
	/// The source containing the location; either `source.path` or `source.sourceReference` must
	/// be specified.
	pub(crate) source:     Source,
	/// The line number of the location. The client capability `linesStartAt1` determines whether
	/// it is 0- or 1-based.
	pub(crate) line:       u64,
	/// Position of the location within the `line`.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	///
	/// If no column is given, the first position in the start line is assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:     Option<u64>,
	/// End line of the location, present if the location refers to a range.
	///
	/// The client capability `linesStartAt1` determines whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line:   Option<u64>,
	/// End position of the location within `endLine`, present if the location refers to a range.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column: Option<u64>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ModuleParams {
	/// All modules or range of modules.
	pub(crate) modules:       Vec<Module>,
	/// The total number of modules available.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) total_modules: Option<u64>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReadMemoryParams {
	/// The address of the first byte of data returned.
	///
	/// Treated as a hex value if prefixed with `0x`, or as a decimal value otherwise.
	pub(crate) address:          String,
	/// The number of unreadable bytes encountered after the last successfully read byte.
	///
	/// This can be used to determine the number of bytes that should be skipped before a
	/// subsequent `readMemory` request succeeds.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) unreadable_bytes: Option<u64>,
	/// The bytes read from memory, encoded using base64.
	///
	/// If the decoded length of `data` is less than the requested `count` in the original
	/// `readMemory` request, and `unreadableBytes` is zero or omitted, then the client should
	/// assume it's reached the end of readable memory.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data:             Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesParams {
	/// The scopes of the stack frame. If the array has length zero, there are no scopes available.
	pub(crate) scopes: Vec<Scope>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsParams {
	/// Information about the breakpoints.
	///
	/// The array elements are in the same order as the elements of the `breakpoints` (or the
	/// deprecated `lines`) array in the arguments.
	pub(crate) breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDataBreakpointsParams {
	/// Information about the data breakpoints.
	///
	/// The array elements correspond to the elements of the input argument `breakpoints` array.
	pub(crate) breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExceptionBreakpointsParams {
	/// Information about the exception breakpoints or filters.
	///
	/// The breakpoints returned are in the same order as the elements of the `filters`,
	/// `filterOptions`, `exceptionOptions` arrays in the arguments.
	///
	/// If both `filters` and `filterOptions` are given, the returned array must start with
	/// `filters` information first, followed by `filterOptions` information.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) breakpoints: Option<Vec<Breakpoint>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetExpressionParams {
	/// The new value of the expression.
	pub(crate) value:                    String,
	/// The type of the value.
	///
	/// This attribute should only be returned by a debug adapter if the corresponding capability
	/// `supportsVariableType` is true.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "type")]
	pub(crate) typ:                      Option<String>,
	/// Properties of a value that can be used to determine how to render the result in the UI.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) presentation_hint:        Option<VariablePresentationHint>,
	/// If `variablesReference` is > 0, the evaluate result is structured and its children can be
	/// retrieved by passing `variablesReference` to the `variables` request as long as execution
	/// remains suspended.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) variables_reference:      Option<u32>,
	/// The number of named child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) named_variables:          Option<u32>,
	/// The number of indexed child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) indexed_variables:        Option<u32>,
	/// A memory reference to a location appropriate for this result.
	///
	/// For pointer type eval results, this is generally a reference to the memory address
	/// contained in the pointer.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) memory_reference:         Option<String>,
	/// A reference that allows the client to request the location where the new value is declared.
	/// For example, if the new value is function pointer, the adapter may be able to look up the
	/// function's location.
	///
	/// This should be present only if the adapter is likely to be able to
	/// resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) value_location_reference: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFunctionBreakpointsParams {
	/// Information about the breakpoints. The array elements correspond to the elements of the
	/// `breakpoints` array.
	pub(crate) breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInstructionBreakpointsParams {
	/// Information about the breakpoints. The array elements correspond to the elements of the
	/// `breakpoints` array.
	pub(crate) breakpoints: Vec<Breakpoint>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableParams {
	/// The new value of the variable.
	pub(crate) value:                    String,
	/// The type of the new value. Typically shown in the UI when hovering over the value.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "type")]
	pub(crate) typ:                      Option<String>,
	/// If `variablesReference` is > 0, the new value is structured and its children can be
	/// retrieved by passing `variablesReference` to the `variables` request as long as execution
	/// remains suspended.
	///
	/// If this property is included in the response, any `variablesReference` previously
	/// associated with the updated variable, and those of its children, are no longer valid.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) variables_reference:      Option<u32>,
	/// The number of named child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) named_variables:          Option<u32>,
	/// The number of indexed child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) indexed_variables:        Option<u32>,
	/// A memory reference to a location appropriate for this result.
	///
	/// For pointer type eval results, this is generally a reference to the memory address
	/// contained in the pointer.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) memory_reference:         Option<String>,
	/// A reference that allows the client to request the location where the new value is declared.
	/// For example, if the new value is function pointer, the adapter may be able to look up the
	/// function's location.
	///
	/// This should be present only if the adapter is likely to be able to resolve the
	/// location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) value_location_reference: Option<i32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SourceParams {
	/// Content of the source reference.
	pub(crate) content:   String,
	/// Content type (MIME type) of the source.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) mime_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceParams {
	/// The frames of the stack frame. If the array has length zero, there are no stack frames
	/// available.
	///
	/// This means that there is no location information available.
	pub(crate) stack_frames: Vec<StackFrame>,
	/// The total number of frames available in the stack. If omitted or if `totalFrames` is larger
	/// than the available frames, a client is expected to request frames until a request returns
	/// less frames than requested (which indicates the end of the stack).
	///
	/// Returning monotonically increasing `totalFrames` values for subsequent requests can be used
	/// to enforce paging in the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) total_frames: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StepInTargetsParams {
	/// The possible step-in targets of the specified source location.
	pub(crate) targets: Vec<StepInTarget>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ThreadsParams {
	/// All threads.
	pub(crate) threads: Vec<Thread>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct VariablesParams {
	/// All (or a range) of variables for the given variable reference.
	pub(crate) variables: Vec<Variable>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WriteMemoryParams {
	/// Property that should be returned when `allowPartial` is true to indicate the offset of the
	/// first byte of data successfully written. Can be negative.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) offset:        Option<i64>,
	/// Property that should be returned when `allowPartial` is true to indicate the number of
	/// bytes starting from address that were successfully written.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) bytes_written: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalParams {
	/// The process ID.
	pub(crate) process_id:       i32,
	/// The process ID of the terminal shell.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) shell_process_id: Option<i32>,
}

impl BreakpointLocationsParams {
	pub fn new(breakpoints: Vec<BreakpointLocation>) -> Self {
		Self { breakpoints }
	}

	/// Sorted set of possible breakpoint locations.
	pub fn breakpoints(&self) -> &Vec<BreakpointLocation> {
		&self.breakpoints
	}
}

impl CompletionsParams {
	pub fn new(targets: Vec<CompletionItem>) -> Self {
		Self { targets }
	}

	/// The possible completions for .
	pub fn targets(&self) -> &Vec<CompletionItem> {
		&self.targets
	}
}

impl Default for ContinueParams {
	fn default() -> Self {
		Self::new()
	}
}

impl ContinueParams {
	pub fn new() -> Self {
		Self { all_threads_continued: None }
	}

	pub fn with_all_threads_continued(self, all_threads_continued: bool) -> Self {
		let mut this = self;
		this.all_threads_continued = Some(all_threads_continued);
		this
	}

	/// If omitted or set to `true`, this response signals to the client that all threads have been
	/// resumed.
	///
	/// The value `false` indicates that not all threads were resumed.
	pub fn all_threads_continued(&self) -> Option<bool> {
		self.all_threads_continued
	}
}

impl DataBreakpointInfoParams {
	pub fn new<T>(description: T) -> Self
	where
		T: ToString,
	{
		Self {
			data_id:      None,
			description:  description.to_string(),
			access_types: None,
			can_persist:  None,
		}
	}

	pub fn with_data_id<T>(self, data_id: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.data_id = Some(data_id.to_string());
		this
	}

	pub fn with_access_types(self, access_types: Vec<DataBreakpointAccessType>) -> Self {
		let mut this = self;
		this.access_types = Some(access_types);
		this
	}

	pub fn with_can_persist(self, can_persist: bool) -> Self {
		let mut this = self;
		this.can_persist = Some(can_persist);
		this
	}

	/// An identifier for the data on which a data breakpoint can be registered with the
	/// `setDataBreakpoints` request or null if no data breakpoint is available.
	///
	/// If a `variablesReference` or `frameId` is passed, the `dataId` is valid in the current
	/// suspended state, otherwise it's valid indefinitely.
	///
	/// Breakpoints set using the `dataId` in the `setDataBreakpoints` request may outlive the
	/// lifetime of the associated `dataId`.
	pub fn data_id(&self) -> Option<&String> {
		self.data_id.as_ref()
	}

	/// UI string that describes on what data the breakpoint is set on or why a data breakpoint is
	/// not available.
	pub fn description(&self) -> &String {
		&self.description
	}

	/// Attribute lists the available access types for a potential data breakpoint. A UI client
	/// could surface this information.
	pub fn access_types(&self) -> Option<&Vec<DataBreakpointAccessType>> {
		self.access_types.as_ref()
	}

	/// Attribute indicates that a potential data breakpoint could be persisted across sessions.
	pub fn can_persist(&self) -> Option<bool> {
		self.can_persist
	}
}

impl DisassembleParams {
	pub fn new(instructions: Vec<DisassembledInstruction>) -> Self {
		Self { instructions }
	}

	/// The list of disassembled instructions.
	pub fn instructions(&self) -> &Vec<DisassembledInstruction> {
		&self.instructions
	}
}

impl EvaluateParams {
	pub fn new<T>(result: T, variables_reference: u32) -> Self
	where
		T: ToString,
	{
		Self {
			result: result.to_string(),
			typ: None,
			presentation_hint: None,
			variables_reference,
			named_variables: None,
			indexed_variables: None,
			memory_reference: None,
			value_location_reference: None,
		}
	}

	pub fn with_type<T>(self, typ: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.typ = Some(typ.to_string());
		this
	}

	pub fn with_presentation_hint(self, presentation_hint: VariablePresentationHint) -> Self {
		let mut this = self;
		this.presentation_hint = Some(presentation_hint);
		this
	}

	pub fn with_named_variables(self, named_variables: u32) -> Self {
		let mut this = self;
		this.named_variables = Some(named_variables);
		this
	}

	pub fn with_indexed_variables(self, indexed_variables: u32) -> Self {
		let mut this = self;
		this.indexed_variables = Some(indexed_variables);
		this
	}

	pub fn with_memory_reference<T>(self, memory_reference: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.memory_reference = Some(memory_reference.to_string());
		this
	}

	pub fn with_value_location_reference(self, value_location_reference: i32) -> Self {
		let mut this = self;
		this.value_location_reference = Some(value_location_reference);
		this
	}

	/// The result of the evaluate request.
	pub fn result(&self) -> &String {
		&self.result
	}

	/// The type of the evaluate result.
	///
	/// This attribute should only be returned by a debug adapter if the corresponding capability
	/// `supportsVariableType` is true.
	pub fn typ(&self) -> Option<&String> {
		self.typ.as_ref()
	}

	/// Properties of an evaluate result that can be used to determine how to render the result in
	/// the UI.
	pub fn presentation_hint(&self) -> Option<&VariablePresentationHint> {
		self.presentation_hint.as_ref()
	}

	/// If `variablesReference` is > 0, the evaluate result is structured and its children can
	/// be retrieved by passing `variablesReference` to the `variables` request as long as
	/// execution remains suspended.
	pub fn variables_reference(&self) -> u32 {
		self.variables_reference
	}

	/// The number of named child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	pub fn named_variables(&self) -> Option<u32> {
		self.named_variables
	}

	/// The number of indexed child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	pub fn indexed_variables(&self) -> Option<u32> {
		self.indexed_variables
	}

	/// A memory reference to a location appropriate for this result.
	///
	/// For pointer type eval results, this is generally a reference to the memory address
	/// contained in the pointer.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	pub fn memory_reference(&self) -> Option<&String> {
		self.memory_reference.as_ref()
	}

	/// A reference that allows the client to request the location where the returned value is
	/// declared. For example, if a function pointer is returned, the adapter may be able to look
	/// up the function's location. This should be present only if the adapter is likely to be able
	/// to resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	pub fn value_location_reference(&self) -> Option<i32> {
		self.value_location_reference
	}
}

impl ExceptionInfoParams {
	pub fn new<T>(exception_id: T, break_mode: ExceptionBreakMode) -> Self
	where
		T: ToString,
	{
		Self {
			exception_id: exception_id.to_string(),
			description: None,
			break_mode,
			details: None,
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

	pub fn with_details(self, details: ExceptionDetails) -> Self {
		let mut this = self;
		this.details = Some(details);
		this
	}

	/// ID of the exception that was thrown.
	pub fn exception_id(&self) -> &String {
		&self.exception_id
	}

	/// Descriptive text for the exception.
	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	/// Mode that caused the exception notification to be raised.
	pub fn break_mode(&self) -> ExceptionBreakMode {
		self.break_mode
	}

	/// Detailed information about the exception.
	pub fn details(&self) -> Option<&ExceptionDetails> {
		self.details.as_ref()
	}
}

impl GotoTargetsParams {
	pub fn new(targets: Vec<GotoTarget>) -> Self {
		Self { targets }
	}

	/// The possible goto targets of the specified location.
	pub fn targets(&self) -> &Vec<GotoTarget> {
		&self.targets
	}
}

impl InitializeParams {
	pub fn new(capabilities: Capabilities) -> Self {
		Self { capabilities }
	}

	/// The capabilities of this debug adapter.
	pub fn capabilities(&self) -> &Capabilities {
		&self.capabilities
	}
}

impl LoadedSourcesParams {
	pub fn new(sources: Vec<Source>) -> Self {
		Self { sources }
	}

	/// Set of loaded sources.
	pub fn sources(&self) -> &Vec<Source> {
		&self.sources
	}
}

impl LocationsParams {
	pub fn new(source: Source, line: u64) -> Self {
		Self {
			source,
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

	/// The source containing the location; either `source.path` or `source.sourceReference` must
	/// be specified.
	pub fn source(&self) -> &Source {
		&self.source
	}

	/// The line number of the location. The client capability `linesStartAt1` determines whether
	/// it is 0- or 1-based.
	pub fn line(&self) -> u64 {
		self.line
	}

	/// Position of the location within the `line`.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	///
	/// If no column is given, the first position in the start line is assumed.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// End line of the location, present if the location refers to a range.
	///
	/// The client capability `linesStartAt1` determines whether it is 0- or 1-based.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// End position of the location within `endLine`, present if the location refers to a range.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}
}

impl ModuleParams {
	pub fn new(modules: Vec<Module>) -> Self {
		Self { modules, total_modules: None }
	}

	pub fn with_total_modules(self, total_modules: u64) -> Self {
		let mut this = self;
		this.total_modules = Some(total_modules);
		this
	}

	/// All modules or range of modules.
	pub fn modules(&self) -> &Vec<Module> {
		&self.modules
	}

	/// The total number of modules available.
	pub fn total_modules(&self) -> Option<u64> {
		self.total_modules
	}
}

impl ReadMemoryParams {
	pub fn new<T>(address: T) -> Self
	where
		T: ToString,
	{
		Self {
			address:          address.to_string(),
			unreadable_bytes: None,
			data:             None,
		}
	}

	pub fn with_unreadable_bytes(self, unreadable_bytes: u64) -> Self {
		let mut this = self;
		this.unreadable_bytes = Some(unreadable_bytes);
		this
	}

	pub fn with_data<T>(self, data: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.data = Some(data.to_string());
		this
	}

	/// The address of the first byte of data returned.
	///
	/// Treated as a hex value if prefixed with `0x`, or as a decimal value otherwise.
	pub fn address(&self) -> &String {
		&self.address
	}

	/// The number of unreadable bytes encountered after the last successfully read byte.
	///
	/// This can be used to determine the number of bytes that should be skipped before a
	/// subsequent `readMemory` request succeeds.
	pub fn unreadable_bytes(&self) -> Option<u64> {
		self.unreadable_bytes
	}

	/// The bytes read from memory, encoded using base64.
	///
	/// If the decoded length of `data` is less than the requested `count` in the original
	/// `readMemory` request, and `unreadableBytes` is zero or omitted, then the client should
	/// assume it's reached the end of readable memory.
	pub fn data(&self) -> Option<&String> {
		self.data.as_ref()
	}
}

impl ScopesParams {
	pub fn new(scopes: Vec<Scope>) -> Self {
		Self { scopes }
	}

	/// The scopes of the stack frame. If the array has length zero, there are no scopes available.
	pub fn scopes(&self) -> &Vec<Scope> {
		&self.scopes
	}
}

impl SetBreakpointsParams {
	pub fn new(breakpoints: Vec<Breakpoint>) -> Self {
		Self { breakpoints }
	}

	/// Information about the breakpoints.
	///
	/// The array elements are in the same order as the elements of the `breakpoints` (or the
	/// deprecated `lines`) array in the arguments.
	pub fn breakpoints(&self) -> &Vec<Breakpoint> {
		&self.breakpoints
	}
}

impl SetDataBreakpointsParams {
	pub fn new(breakpoints: Vec<Breakpoint>) -> Self {
		Self { breakpoints }
	}

	/// Information about the data breakpoints.
	///
	/// The array elements correspond to the elements of the input argument `breakpoints` array.
	pub fn breakpoints(&self) -> &Vec<Breakpoint> {
		&self.breakpoints
	}
}

impl Default for SetExceptionBreakpointsParams {
	fn default() -> Self {
		Self::new()
	}
}

impl SetExceptionBreakpointsParams {
	pub fn new() -> Self {
		Self { breakpoints: None }
	}

	pub fn with_breakpoints(self, breakpoints: Vec<Breakpoint>) -> Self {
		let mut this = self;
		this.breakpoints = Some(breakpoints);
		this
	}

	/// Information about the exception breakpoints or filters.
	///
	/// The breakpoints returned are in the same order as the elements of the `filters`,
	/// `filterOptions`, `exceptionOptions` arrays in the arguments.
	///
	/// If both `filters` and `filterOptions` are given, the returned array must start with
	/// `filters` information first, followed by `filterOptions` information.
	pub fn breakpoints(&self) -> Option<&Vec<Breakpoint>> {
		self.breakpoints.as_ref()
	}
}

impl SetExpressionParams {
	pub fn new<T>(value: T) -> Self
	where
		T: ToString,
	{
		Self {
			value:                    value.to_string(),
			typ:                      None,
			presentation_hint:        None,
			variables_reference:      None,
			named_variables:          None,
			indexed_variables:        None,
			memory_reference:         None,
			value_location_reference: None,
		}
	}

	pub fn with_type<T>(self, typ: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.typ = Some(typ.to_string());
		this
	}

	pub fn with_presentation_hint(self, presentation_hint: VariablePresentationHint) -> Self {
		let mut this = self;
		this.presentation_hint = Some(presentation_hint);
		this
	}

	pub fn with_variables_reference(self, variables_reference: u32) -> Self {
		let mut this = self;
		this.variables_reference = Some(variables_reference);
		this
	}

	pub fn with_named_variables(self, named_variables: u32) -> Self {
		let mut this = self;
		this.named_variables = Some(named_variables);
		this
	}

	pub fn with_indexed_variables(self, indexed_variables: u32) -> Self {
		let mut this = self;
		this.indexed_variables = Some(indexed_variables);
		this
	}

	pub fn with_memory_reference<T>(self, memory_reference: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.memory_reference = Some(memory_reference.to_string());
		this
	}

	pub fn with_value_location_reference(self, value_location_reference: u32) -> Self {
		let mut this = self;
		this.value_location_reference = Some(value_location_reference);
		this
	}

	/// The new value of the expression.
	pub fn value(&self) -> &String {
		&self.value
	}

	/// The type of the value.
	///
	/// This attribute should only be returned by a debug adapter if the corresponding capability
	/// `supportsVariableType` is true.
	pub fn typ(&self) -> Option<&String> {
		self.typ.as_ref()
	}

	/// Properties of a value that can be used to determine how to render the result in the UI.
	pub fn presentation_hint(&self) -> Option<&VariablePresentationHint> {
		self.presentation_hint.as_ref()
	}

	/// If `variablesReference` is > 0, the evaluate result is structured and its children can be
	/// retrieved by passing `variablesReference` to the `variables` request as long as execution
	/// remains suspended.
	pub fn variables_reference(&self) -> Option<u32> {
		self.variables_reference
	}

	/// The number of named child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	pub fn named_variables(&self) -> Option<u32> {
		self.named_variables
	}

	/// The number of indexed child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	pub fn indexed_variables(&self) -> Option<u32> {
		self.indexed_variables
	}

	/// A memory reference to a location appropriate for this result.
	///
	/// For pointer type eval results, this is generally a reference to the memory address
	/// contained in the pointer.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	pub fn memory_reference(&self) -> Option<&String> {
		self.memory_reference.as_ref()
	}

	/// A reference that allows the client to request the location where the new value is declared.
	/// For example, if the new value is function pointer, the adapter may be able to look up the
	/// function's location.
	///
	/// This should be present only if the adapter is likely to be able to
	/// resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	pub fn value_location_reference(&self) -> Option<u32> {
		self.value_location_reference
	}
}

impl SetFunctionBreakpointsParams {
	pub fn new(breakpoints: Vec<Breakpoint>) -> Self {
		Self { breakpoints }
	}

	/// Information about the breakpoints. The array elements correspond to the elements of the
	/// `breakpoints` array.
	pub fn breakpoints(&self) -> &Vec<Breakpoint> {
		&self.breakpoints
	}
}

impl SetInstructionBreakpointsParams {
	pub fn new(breakpoints: Vec<Breakpoint>) -> Self {
		Self { breakpoints }
	}

	/// Information about the breakpoints. The array elements correspond to the elements of the
	/// `breakpoints` array.
	pub fn breakpoints(&self) -> &Vec<Breakpoint> {
		&self.breakpoints
	}
}

impl SetVariableParams {
	pub fn new<T>(value: T) -> Self
	where
		T: ToString,
	{
		Self {
			value:                    value.to_string(),
			typ:                      None,
			variables_reference:      None,
			named_variables:          None,
			indexed_variables:        None,
			memory_reference:         None,
			value_location_reference: None,
		}
	}

	pub fn with_type<T>(self, typ: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.typ = Some(typ.to_string());
		this
	}

	pub fn with_variables_reference(self, variables_reference: u32) -> Self {
		let mut this = self;
		this.variables_reference = Some(variables_reference);
		this
	}

	pub fn with_named_variables(self, named_variables: u32) -> Self {
		let mut this = self;
		this.named_variables = Some(named_variables);
		this
	}

	pub fn with_indexed_variables(self, indexed_variables: u32) -> Self {
		let mut this = self;
		this.indexed_variables = Some(indexed_variables);
		this
	}

	pub fn with_memory_reference<T>(self, memory_reference: String) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.memory_reference = Some(memory_reference.to_string());
		this
	}

	pub fn with_value_location_reference(self, value_location_reference: i32) -> Self {
		let mut this = self;
		this.value_location_reference = Some(value_location_reference);
		this
	}

	/// The new value of the variable.
	pub fn value(&self) -> &String {
		&self.value
	}

	/// The type of the new value. Typically shown in the UI when hovering over the value.
	pub fn typ(&self) -> Option<&String> {
		self.typ.as_ref()
	}

	/// If `variablesReference` is > 0, the new value is structured and its children can be
	/// retrieved by passing `variablesReference` to the `variables` request as long as execution
	/// remains suspended.
	///
	/// If this property is included in the response, any `variablesReference` previously
	/// associated with the updated variable, and those of its children, are no longer valid.
	pub fn variables_reference(&self) -> Option<u32> {
		self.variables_reference
	}

	/// The number of named child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	pub fn named_variables(&self) -> Option<u32> {
		self.named_variables
	}

	/// The number of indexed child variables.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	pub fn indexed_variables(&self) -> Option<u32> {
		self.indexed_variables
	}

	/// A memory reference to a location appropriate for this result.
	///
	/// For pointer type eval results, this is generally a reference to the memory address
	/// contained in the pointer.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	pub fn memory_reference(&self) -> Option<&String> {
		self.memory_reference.as_ref()
	}

	/// A reference that allows the client to request the location where the new value is declared.
	/// For example, if the new value is function pointer, the adapter may be able to look up the
	/// function's location.
	///
	/// This should be present only if the adapter is likely to be able to resolve the
	/// location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	pub fn value_location_reference(&self) -> Option<i32> {
		self.value_location_reference
	}
}

impl SourceParams {
	pub fn new<T>(content: T) -> Self
	where
		T: ToString,
	{
		Self { content: content.to_string(), mime_type: None }
	}

	pub fn with_mime_type<T>(self, mime_type: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.mime_type = Some(mime_type.to_string());
		this
	}

	/// Content of the source reference.
	pub fn content(&self) -> &String {
		&self.content
	}

	/// Content type (MIME type) of the source.
	pub fn mime_type(&self) -> Option<&String> {
		self.mime_type.as_ref()
	}
}

impl StackTraceParams {
	pub fn new(stack_frames: Vec<StackFrame>) -> Self {
		Self { stack_frames, total_frames: None }
	}

	pub fn with_total_frames(self, total_frames: u32) -> Self {
		let mut this = self;
		this.total_frames = Some(total_frames);
		this
	}

	/// The frames of the stack frame. If the array has length zero, there are no stack frames
	/// available.
	///
	/// This means that there is no location information available.
	pub fn stack_frames(&self) -> &Vec<StackFrame> {
		&self.stack_frames
	}

	/// The total number of frames available in the stack. If omitted or if `totalFrames` is larger
	/// than the available frames, a client is expected to request frames until a request returns
	/// less frames than requested (which indicates the end of the stack).
	///
	/// Returning monotonically increasing `totalFrames` values for subsequent requests can be used
	/// to enforce paging in the client.
	pub fn total_frames(&self) -> Option<u32> {
		self.total_frames
	}
}

impl StepInTargetsParams {
	pub fn new(targets: Vec<StepInTarget>) -> Self {
		Self { targets }
	}

	/// The possible step-in targets of the specified source location.
	pub fn targets(&self) -> &Vec<StepInTarget> {
		&self.targets
	}
}

impl ThreadsParams {
	pub fn new(threads: Vec<Thread>) -> Self {
		Self { threads }
	}

	/// All threads.
	pub fn threads(&self) -> &Vec<Thread> {
		&self.threads
	}
}

impl VariablesParams {
	pub fn new(variables: Vec<Variable>) -> Self {
		Self { variables }
	}

	/// All (or a range) of variables for the given variable reference.
	pub fn variables(&self) -> &Vec<Variable> {
		&self.variables
	}
}

impl Default for WriteMemoryParams {
	fn default() -> Self {
		Self::new()
	}
}

impl WriteMemoryParams {
	pub fn new() -> Self {
		Self { offset: None, bytes_written: None }
	}

	pub fn with_offset(self, offset: i64) -> Self {
		let mut this = self;
		this.offset = Some(offset);
		this
	}

	pub fn with_bytes_written(self, bytes_written: u32) -> Self {
		let mut this = self;
		this.bytes_written = Some(bytes_written);
		this
	}

	/// Property that should be returned when `allowPartial` is true to indicate the offset of the
	/// first byte of data successfully written. Can be negative.
	pub fn offset(&self) -> Option<i64> {
		self.offset
	}

	/// Property that should be returned when `allowPartial` is true to indicate the number of
	/// bytes starting from address that were successfully written.
	pub fn bytes_written(&self) -> Option<u32> {
		self.bytes_written
	}
}

impl RunInTerminalParams {
	pub fn new(process_id: i32) -> Self {
		Self { process_id, shell_process_id: None }
	}

	pub fn with_shell_process_id(self, shell_process_id: i32) -> Self {
		let mut this = self;
		this.shell_process_id = Some(shell_process_id);
		this
	}

	/// The process ID.
	pub fn process_id(&self) -> i32 {
		self.process_id
	}

	/// The process ID of the terminal shell.
	pub fn shell_process_id(&self) -> Option<i32> {
		self.shell_process_id
	}
}
