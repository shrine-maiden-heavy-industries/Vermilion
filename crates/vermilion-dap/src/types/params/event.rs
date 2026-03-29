// SPDX-License-Identifier: BSD-3-Clause

use crate::types::{
	EventReason, InvalidatedAreas, Module, OutputCategory, OutputGroup, ProcessStartMethod, Source,
	StoppedReason, ThreadReason, breakpoint::Breakpoint,
};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointParams {
	/// The reason for the event.
	pub(crate) reason:     EventReason,
	/// The `id` attribute is used to find the target breakpoint, the other attributes are used as
	/// the new values.
	pub(crate) breakpoint: Breakpoint,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ContinuedParams {
	/// The thread which was continued
	pub(crate) thread_id:             u32,
	/// If omitted or set to `true`, this event signals to the client that all threads have been
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
pub struct ExitedParams {
	/// The exit code returned from the debuggee.
	pub(crate) exit_code: u32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InvalidatedParams {
	/// Set of logical areas that got invalidated.
	///
	/// This property has a hint characteristic: a client can only be expected to make a 'best
	/// effort' in honoring the areas but there are no guarantees. If this property is missing,
	/// empty, or if values are not understood, the client should assume a single value `all`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) areas:          Option<Vec<InvalidatedAreas>>,
	/// If specified, the client only needs to refetch data related to this thread."
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) thread_id:      Option<i32>,
	/// If specified, the client only needs to refetch data related to this stack frame (and the
	/// `threadId` is ignored).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) stack_frame_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedSourceParams {
	/// The reason for the event.
	pub(crate) reason: EventReason,
	/// The new, changed, or removed source.
	pub(crate) source: Source,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MemoryParams {
	/// Memory reference of a memory range that has been updated.
	pub(crate) memory_reference: String,
	/// Starting offset in bytes where memory has been updated. Can be negative.
	pub(crate) offset:           i64,
	/// Number of bytes updated.
	pub(crate) count:            u64,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ModuleParams {
	/// The reason for the event.
	pub(crate) reason: EventReason,
	/// The new, changed, or removed module. In case of `removed` only the module id is used.
	pub(crate) module: Module,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputParams {
	/// The output category.
	///
	/// If not specified or if the category is not understood by the client, `console` is assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) category:            Option<OutputCategory>,
	/// The output to report.
	///
	/// ANSI escape sequences may be used to influence text color and styling if
	/// `supportsANSIStyling` is present in both the adapter's `Capabilities` and the
	/// client's `InitializeRequestArguments`.
	///
	/// A client may strip any unrecognized ANSI sequences.
	///
	/// If the `supportsANSIStyling` capabilities are not both true, then the client should display
	/// the output literally.
	pub(crate) output:              String,
	/// Support for keeping an output log organized by grouping related messages.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) group:               Option<OutputGroup>,
	/// If an attribute `variablesReference` exists and its value is > 0, the output contains
	/// objects which can be retrieved by passing `variablesReference` to the `variables` request
	/// as long as execution remains suspended.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) variables_reference: Option<u32>,
	/// The source location where the output was produced.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source:              Option<Box<Source>>,
	/// The source location's line where the output was produced.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line:                Option<u64>,
	/// The position in `line` where the output was produced. It is measured in UTF-16 code units
	/// and the client capability `columnsStartAt1` determines whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:              Option<u64>,
	/// Additional data to report. For the `telemetry` category the data is sent to telemetry, for
	/// the other categories the data is shown in JSON format.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data:                Option<serde_json::Value>,
	/// A reference that allows the client to request the location where the new value is declared.
	/// For example, if the logged value is function pointer, the adapter may be able to look up
	/// the function's location.
	///
	/// This should be present only if the adapter is likely to be able to resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) location_reference:  Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ProcessParams {
	/// The logical name of the process. This is usually the full path to process's executable
	/// file. Example: /home/example/myproj/program.rs.
	pub(crate) name:              String,
	/// The process ID of the debugged process, as assigned by the operating system. This property
	/// should be omitted for logical processes that do not map to operating system processes on
	/// the machine.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) system_process_id: Option<i32>,
	/// If true, the process is running on the same computer as the debug adapter.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) is_local_process:  Option<bool>,
	/// Describes how the debug engine started debugging this process.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) start_method:      Option<ProcessStartMethod>,
	/// The size of a pointer or address for this process, in bits.
	///
	/// This value may be used by clients when formatting addresses for display.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) pointer_size:      Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEndParams {
	/// The ID that was introduced in the initial `ProgressStartEvent`.
	pub(crate) progress_id: String,
	/// More detailed progress message. If omitted, the previous message (if any) is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message:     Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ProgressStartParams {
	/// An ID that can be used in subsequent `progressUpdate` and `progressEnd` events to make them
	/// refer to the same progress reporting.
	///
	/// IDs must be unique within a debug session.
	pub(crate) progress_id: String,
	/// Short title of the progress reporting.
	///
	/// Shown in the UI to describe the long running operation.
	pub(crate) title:       String,
	/// The request ID that this progress report is related to.
	///
	/// If specified a debug adapter is expected to emit progress events for the long running
	/// request until the request has been either completed or cancelled.
	///
	/// If the request ID is omitted, the progress report is assumed to be related to some general
	/// activity of the debug adapter.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) request_id:  Option<u32>,
	/// If true, the request that reports progress may be cancelled with a `cancel` request.
	///
	/// So this property basically controls whether the client should use UX that supports
	/// cancellation.
	///
	/// Clients that don't support cancellation are allowed to ignore the setting.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) cancellable: Option<bool>,
	/// More detailed progress message.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message:     Option<String>,
	/// Progress percentage to display. If omitted no percentage is shown.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) percentage:  Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdateParams {
	/// The ID that was introduced in the initial `ProgressStartEvent`.
	pub(crate) progress_id: String,
	/// More detailed progress message. If omitted, the previous message (if any) is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message:     Option<String>,
	/// Progress percentage to display. If omitted no percentage is shown.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) percentage:  Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StoppedParams {
	/// The reason for the event.
	///
	/// For backward compatibility this string is shown in the UI if the `description` attribute is
	/// missing (but it must not be translated).
	pub(crate) reason:              StoppedReason,
	/// The full reason for the event, e.g. 'Paused on exception'. This string is shown in the UI
	/// as is and can be translated.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) description:         Option<String>,
	/// The thread which was stopped
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) thread_id:           Option<u32>,
	/// A value of true hints to the client that this event should not change the focus.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) preserve_focus_hint: Option<bool>,
	/// Additional information. E.g. if reason is `exception`, text contains the exception name.
	/// This string is shown in the UI.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text:                Option<String>,
	/// If `allThreadsStopped` is true, a debug adapter can announce that all threads have
	/// stopped.
	///
	/// - The client should use this information to enable that all threads can be expanded to
	///   access their stacktraces.
	/// - If the attribute is missing or false, only the thread with the given `threadId` can be
	///   expanded.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) all_threads_stopped: Option<bool>,
	/// Ids of the breakpoints that triggered the event.
	///
	/// In most cases there is only a single breakpoint but here are some examples for multiple
	/// breakpoints:
	/// - Different types of breakpoints map to the same location.
	/// - Multiple source breakpoints get collapsed to the same instruction by the
	///   compiler/runtime.
	/// - Multiple function breakpoints with different function names map to the same location.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hit_breakpoint_ids:  Option<Vec<u32>>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminatedParams {
	/// A debug adapter may set `restart` to true (or to an arbitrary object) to request that the
	/// client restarts the session.
	///
	/// The value is not interpreted by the client and passed unmodified as an attribute
	/// `__restart` to the `launch` and `attach` requests
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) restart: Option<serde_json::Value>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ThreadParams {
	/// The reason for the event
	pub(crate) reason:    ThreadReason,
	/// The identifier of the thread
	pub(crate) thread_id: u32,
}

impl BreakpointParams {
	pub fn new(reason: EventReason, breakpoint: Breakpoint) -> Self {
		Self { reason, breakpoint }
	}

	/// The reason for the event.
	pub fn reason(&self) -> &EventReason {
		&self.reason
	}

	/// The `id` attribute is used to find the target breakpoint, the other attributes are used as
	/// the new values.
	pub fn breakpoint(&self) -> &Breakpoint {
		&self.breakpoint
	}
}

impl ContinuedParams {
	pub fn new(thread_id: u32) -> Self {
		Self { thread_id, all_threads_continued: None }
	}

	pub fn with_all_threads_continued(self, all_threads_continued: bool) -> Self {
		let mut this = self;
		this.all_threads_continued = Some(all_threads_continued);
		this
	}

	/// The thread which was continued
	pub fn thread_id(&self) -> u32 {
		self.thread_id
	}

	/// If omitted or set to `true`, this event signals to the client that all threads have been
	/// resumed.
	///
	/// The value `false` indicates that not all threads were resumed.
	pub fn all_threads_continued(&self) -> Option<bool> {
		self.all_threads_continued
	}
}

impl ExitedParams {
	pub fn new(exit_code: u32) -> Self {
		Self { exit_code }
	}

	/// The exit code returned from the debuggee.
	pub fn exit_code(&self) -> u32 {
		self.exit_code
	}
}

impl Default for InvalidatedParams {
	fn default() -> Self {
		Self::new()
	}
}

impl InvalidatedParams {
	pub fn new() -> Self {
		Self {
			areas:          None,
			thread_id:      None,
			stack_frame_id: None,
		}
	}

	pub fn with_areas(self, areas: Vec<InvalidatedAreas>) -> Self {
		let mut this = self;
		this.areas = Some(areas);
		this
	}

	pub fn with_thread_id(self, thread_id: i32) -> Self {
		let mut this = self;
		this.thread_id = Some(thread_id);
		this
	}

	pub fn with_stack_frame_id(self, stack_frame_id: i32) -> Self {
		let mut this = self;
		this.stack_frame_id = Some(stack_frame_id);
		this
	}

	/// Set of logical areas that got invalidated.
	///
	/// This property has a hint characteristic: a client can only be expected to make a 'best
	/// effort' in honoring the areas but there are no guarantees. If this property is missing,
	/// empty, or if values are not understood, the client should assume a single value `all`.
	pub fn areas(&self) -> Option<&Vec<InvalidatedAreas>> {
		self.areas.as_ref()
	}

	/// If specified, the client only needs to refetch data related to this thread."
	pub fn thread_id(&self) -> Option<i32> {
		self.thread_id
	}

	/// If specified, the client only needs to refetch data related to this stack frame (and the
	/// `threadId` is ignored).
	pub fn stack_frame_id(&self) -> Option<i32> {
		self.stack_frame_id
	}
}

impl LoadedSourceParams {
	pub fn new(reason: EventReason, source: Source) -> Self {
		Self { reason, source }
	}

	/// The reason for the event.
	pub fn reason(&self) -> &EventReason {
		&self.reason
	}

	/// The new, changed, or removed source.
	pub fn source(&self) -> &Source {
		&self.source
	}
}

impl MemoryParams {
	pub fn new<T>(memory_reference: T, offset: i64, count: u64) -> Self
	where
		T: ToString,
	{
		Self {
			memory_reference: memory_reference.to_string(),
			offset,
			count,
		}
	}

	/// Memory reference of a memory range that has been updated.
	pub fn memory_reference(&self) -> &String {
		&self.memory_reference
	}

	/// Starting offset in bytes where memory has been updated. Can be negative.
	pub fn offset(&self) -> i64 {
		self.offset
	}

	/// Number of bytes updated.
	pub fn count(&self) -> u64 {
		self.count
	}
}

impl ModuleParams {
	pub fn new(reason: EventReason, module: Module) -> Self {
		Self { reason, module }
	}

	/// The reason for the event.
	pub fn reason(&self) -> &EventReason {
		&self.reason
	}

	/// The new, changed, or removed module. In case of `removed` only the module id is used.
	pub fn module(&self) -> &Module {
		&self.module
	}
}

impl OutputParams {
	pub fn new<T>(output: T) -> Self
	where
		T: ToString,
	{
		Self {
			category:            None,
			output:              output.to_string(),
			group:               None,
			variables_reference: None,
			source:              None,
			line:                None,
			column:              None,
			data:                None,
			location_reference:  None,
		}
	}

	pub fn with_category(self, category: OutputCategory) -> Self {
		let mut this = self;
		this.category = Some(category);
		this
	}

	pub fn with_group(self, group: OutputGroup) -> Self {
		let mut this = self;
		this.group = Some(group);
		this
	}

	pub fn with_variables_reference(self, variables_reference: u32) -> Self {
		let mut this = self;
		this.variables_reference = Some(variables_reference);
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

	pub fn with_data<T>(self, data: T) -> eyre::Result<Self>
	where
		T: serde::Serialize,
	{
		let mut this = self;
		this.data = Some(serde_json::to_value(data)?);
		Ok(this)
	}

	pub fn with_location_reference(self, location_reference: u32) -> Self {
		let mut this = self;
		this.location_reference = Some(location_reference);
		this
	}

	/// The output category.
	///
	/// If not specified or if the category is not understood by the client, `console` is assumed.
	pub fn category(&self) -> Option<&OutputCategory> {
		self.category.as_ref()
	}

	/// The output to report.
	///
	/// ANSI escape sequences may be used to influence text color and styling if
	/// `supportsANSIStyling` is present in both the adapter's `Capabilities` and the
	/// client's `InitializeRequestArguments`.
	///
	/// A client may strip any unrecognized ANSI sequences.
	///
	/// If the `supportsANSIStyling` capabilities are not both true, then the client should display
	/// the output literally.
	pub fn output(&self) -> &String {
		&self.output
	}

	/// Support for keeping an output log organized by grouping related messages.
	pub fn group(&self) -> Option<OutputGroup> {
		self.group
	}

	/// If an attribute `variablesReference` exists and its value is > 0, the output contains
	/// objects which can be retrieved by passing `variablesReference` to the `variables` request
	/// as long as execution remains suspended.
	pub fn variables_reference(&self) -> Option<u32> {
		self.variables_reference
	}

	/// The source location where the output was produced.
	pub fn source(&self) -> Option<&Source> {
		self.source.as_deref()
	}

	/// The source location's line where the output was produced.
	pub fn line(&self) -> Option<u64> {
		self.line
	}

	/// The position in `line` where the output was produced. It is measured in UTF-16 code units
	/// and the client capability `columnsStartAt1` determines whether it is 0- or 1-based.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// Additional data to report. For the `telemetry` category the data is sent to telemetry, for
	/// the other categories the data is shown in JSON format.
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}

	/// A reference that allows the client to request the location where the new value is declared.
	/// For example, if the logged value is function pointer, the adapter may be able to look up
	/// the function's location.
	///
	/// This should be present only if the adapter is likely to be able to resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	pub fn location_reference(&self) -> Option<u32> {
		self.location_reference
	}
}

impl ProcessParams {
	pub fn new<T>(name: T) -> Self
	where
		T: ToString,
	{
		Self {
			name:              name.to_string(),
			system_process_id: None,
			is_local_process:  None,
			start_method:      None,
			pointer_size:      None,
		}
	}

	pub fn with_system_process_id(self, system_process_id: i32) -> Self {
		let mut this = self;
		this.system_process_id = Some(system_process_id);
		this
	}

	pub fn with_is_local_process(self, is_local_process: bool) -> Self {
		let mut this = self;
		this.is_local_process = Some(is_local_process);
		this
	}

	pub fn with_start_method(self, start_method: ProcessStartMethod) -> Self {
		let mut this = self;
		this.start_method = Some(start_method);
		this
	}

	pub fn with_pointer_size(self, pointer_size: u32) -> Self {
		let mut this = self;
		this.pointer_size = Some(pointer_size);
		this
	}

	/// The logical name of the process. This is usually the full path to process's executable
	/// file. Example: /home/example/myproj/program.rs.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The process ID of the debugged process, as assigned by the operating system. This property
	/// should be omitted for logical processes that do not map to operating system processes on
	/// the machine.
	pub fn system_process_id(&self) -> Option<i32> {
		self.system_process_id
	}

	/// If true, the process is running on the same computer as the debug adapter.
	pub fn is_local_process(&self) -> Option<bool> {
		self.is_local_process
	}

	/// Describes how the debug engine started debugging this process.
	pub fn start_method(&self) -> Option<ProcessStartMethod> {
		self.start_method
	}

	/// The size of a pointer or address for this process, in bits.
	///
	/// This value may be used by clients when formatting addresses for display.
	pub fn pointer_size(&self) -> Option<u32> {
		self.pointer_size
	}
}

impl ProgressEndParams {
	pub fn new<T>(progress_id: T) -> Self
	where
		T: ToString,
	{
		Self {
			progress_id: progress_id.to_string(),
			message:     None,
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

	/// The ID that was introduced in the initial `ProgressStartEvent`.
	pub fn progress_id(&self) -> &String {
		&self.progress_id
	}

	/// More detailed progress message. If omitted, the previous message (if any) is used.
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}
}

impl ProgressStartParams {
	pub fn new<T, U>(progress_id: T, title: U) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			progress_id: progress_id.to_string(),
			title:       title.to_string(),
			request_id:  None,
			cancellable: None,
			message:     None,
			percentage:  None,
		}
	}

	pub fn with_request_id(self, request_id: u32) -> Self {
		let mut this = self;
		this.request_id = Some(request_id);
		this
	}

	pub fn with_cancellable(self, cancellable: bool) -> Self {
		let mut this = self;
		this.cancellable = Some(cancellable);
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

	pub fn with_percentage(self, percentage: u32) -> Self {
		let mut this = self;
		this.percentage = Some(percentage);
		this
	}

	/// An ID that can be used in subsequent `progressUpdate` and `progressEnd` events to make them
	/// refer to the same progress reporting.
	///
	/// IDs must be unique within a debug session.
	pub fn progress_id(&self) -> &String {
		&self.progress_id
	}

	/// Short title of the progress reporting.
	///
	/// Shown in the UI to describe the long running operation.
	pub fn title(&self) -> &String {
		&self.title
	}

	/// The request ID that this progress report is related to.
	///
	/// If specified a debug adapter is expected to emit progress events for the long running
	/// request until the request has been either completed or cancelled.
	///
	/// If the request ID is omitted, the progress report is assumed to be related to some general
	/// activity of the debug adapter.
	pub fn request_id(&self) -> Option<u32> {
		self.request_id
	}

	/// If true, the request that reports progress may be cancelled with a `cancel` request.
	///
	/// So this property basically controls whether the client should use UX that supports
	/// cancellation.
	///
	/// Clients that don't support cancellation are allowed to ignore the setting.
	pub fn cancellable(&self) -> Option<bool> {
		self.cancellable
	}

	/// More detailed progress message.
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	/// Progress percentage to display. If omitted no percentage is shown.
	pub fn percentage(&self) -> Option<u32> {
		self.percentage
	}
}

impl ProgressUpdateParams {
	pub fn new<T>(progress_id: T) -> Self
	where
		T: ToString,
	{
		Self {
			progress_id: progress_id.to_string(),
			message:     None,
			percentage:  None,
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

	pub fn with_percentage(self, percentage: u32) -> Self {
		let mut this = self;
		this.percentage = Some(percentage);
		this
	}

	/// The ID that was introduced in the initial `ProgressStartEvent`.
	pub fn progress_id(&self) -> &String {
		&self.progress_id
	}

	/// More detailed progress message. If omitted, the previous message (if any) is used.
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	/// Progress percentage to display. If omitted no percentage is shown.
	pub fn percentage(&self) -> Option<u32> {
		self.percentage
	}
}

impl StoppedParams {
	pub fn new(reason: StoppedReason) -> Self {
		Self {
			reason,
			description: None,
			thread_id: None,
			preserve_focus_hint: None,
			text: None,
			all_threads_stopped: None,
			hit_breakpoint_ids: None,
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

	pub fn with_thread_id(self, thread_id: u32) -> Self {
		let mut this = self;
		this.thread_id = Some(thread_id);
		this
	}

	pub fn with_preserve_focus_hint(self, preserve_focus_hint: bool) -> Self {
		let mut this = self;
		this.preserve_focus_hint = Some(preserve_focus_hint);
		this
	}

	pub fn with_text<T>(self, text: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.text = Some(text.to_string());
		this
	}

	pub fn with_all_threads_stopped(self, all_threads_stopped: bool) -> Self {
		let mut this = self;
		this.all_threads_stopped = Some(all_threads_stopped);
		this
	}

	pub fn with_hit_breakpoint_ids(self, hit_breakpoint_ids: Vec<u32>) -> Self {
		let mut this = self;
		this.hit_breakpoint_ids = Some(hit_breakpoint_ids);
		this
	}

	/// The reason for the event.
	///
	/// For backward compatibility this string is shown in the UI if the `description` attribute is
	/// missing (but it must not be translated).
	pub fn reason(&self) -> &StoppedReason {
		&self.reason
	}

	/// The full reason for the event, e.g. 'Paused on exception'. This string is shown in the UI
	/// as is and can be translated.
	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	/// The thread which was stopped
	pub fn thread_id(&self) -> Option<u32> {
		self.thread_id
	}

	/// A value of true hints to the client that this event should not change the focus.
	pub fn preserve_focus_hint(&self) -> Option<bool> {
		self.preserve_focus_hint
	}

	/// Additional information. E.g. if reason is `exception`, text contains the exception name.
	/// This string is shown in the UI.
	pub fn text(&self) -> Option<&String> {
		self.text.as_ref()
	}

	/// If `allThreadsStopped` is true, a debug adapter can announce that all threads have
	/// stopped.
	///
	/// - The client should use this information to enable that all threads can be expanded to
	///   access their stacktraces.
	/// - If the attribute is missing or false, only the thread with the given `threadId` can be
	///   expanded.
	pub fn all_threads_stopped(&self) -> Option<bool> {
		self.all_threads_stopped
	}

	/// Ids of the breakpoints that triggered the event.
	///
	/// In most cases there is only a single breakpoint but here are some examples for multiple
	/// breakpoints:
	/// - Different types of breakpoints map to the same location.
	/// - Multiple source breakpoints get collapsed to the same instruction by the compiler/runtime.
	/// - Multiple function breakpoints with different function names map to the same location.
	pub fn hit_breakpoint_ids(&self) -> Option<&Vec<u32>> {
		self.hit_breakpoint_ids.as_ref()
	}
}

impl Default for TerminatedParams {
	fn default() -> Self {
		Self::new()
	}
}

impl TerminatedParams {
	pub fn new() -> Self {
		Self { restart: None }
	}

	pub fn with_restart<T>(self, restart: T) -> eyre::Result<Self>
	where
		T: serde::Serialize,
	{
		let mut this = self;
		this.restart = Some(serde_json::to_value(restart)?);
		Ok(this)
	}

	/// A debug adapter may set `restart` to true (or to an arbitrary object) to request that the
	/// client restarts the session.
	///
	/// The value is not interpreted by the client and passed unmodified as an attribute
	/// `__restart` to the `launch` and `attach` requests
	pub fn restart(&self) -> Option<&serde_json::Value> {
		self.restart.as_ref()
	}
}

impl ThreadParams {
	pub fn new(reason: ThreadReason, thread_id: u32) -> Self {
		Self { reason, thread_id }
	}

	/// The reason for the event
	pub fn reason(&self) -> &ThreadReason {
		&self.reason
	}

	/// The identifier of the thread
	pub fn thread_id(&self) -> u32 {
		self.thread_id
	}
}
