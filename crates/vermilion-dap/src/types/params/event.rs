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
	pub fn new(thread_id: u32, all_threads_continued: Option<bool>) -> Self {
		Self { thread_id, all_threads_continued }
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

impl InvalidatedParams {
	pub fn new(
		areas: Option<Vec<InvalidatedAreas>>,
		thread_id: Option<i32>,
		stack_frame_id: Option<i32>,
	) -> Self {
		Self { areas, thread_id, stack_frame_id }
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
	pub fn new(memory_reference: String, offset: i64, count: u64) -> Self {
		Self { memory_reference, offset, count }
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
	#[allow(clippy::too_many_arguments, reason = "Big structure, can't do much about it")]
	pub fn new(
		category: Option<OutputCategory>,
		output: String,
		group: Option<OutputGroup>,
		variables_reference: Option<u32>,
		source: Option<Box<Source>>,
		line: Option<u64>,
		column: Option<u64>,
		data: Option<serde_json::Value>,
		location_reference: Option<u32>,
	) -> Self {
		Self {
			category,
			output,
			group,
			variables_reference,
			source,
			line,
			column,
			data,
			location_reference,
		}
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
	pub fn new(
		name: String,
		system_process_id: Option<i32>,
		is_local_process: Option<bool>,
		start_method: Option<ProcessStartMethod>,
		pointer_size: Option<u32>,
	) -> Self {
		Self {
			name,
			system_process_id,
			is_local_process,
			start_method,
			pointer_size,
		}
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
	pub fn new(progress_id: String, message: Option<String>) -> Self {
		Self { progress_id, message }
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
	pub fn new(
		progress_id: String,
		title: String,
		request_id: Option<u32>,
		cancellable: Option<bool>,
		message: Option<String>,
		percentage: Option<u32>,
	) -> Self {
		Self {
			progress_id,
			title,
			request_id,
			cancellable,
			message,
			percentage,
		}
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
	pub fn new(progress_id: String, message: Option<String>, percentage: Option<u32>) -> Self {
		Self { progress_id, message, percentage }
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
	pub fn new(
		reason: StoppedReason,
		description: Option<String>,
		thread_id: Option<u32>,
		preserve_focus_hint: Option<bool>,
		text: Option<String>,
		all_threads_stopped: Option<bool>,
		hit_breakpoint_ids: Option<Vec<u32>>,
	) -> Self {
		Self {
			reason,
			description,
			thread_id,
			preserve_focus_hint,
			text,
			all_threads_stopped,
			hit_breakpoint_ids,
		}
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

impl TerminatedParams {
	pub fn new(restart: Option<serde_json::Value>) -> Self {
		Self { restart }
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
