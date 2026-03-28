// SPDX-License-Identifier: BSD-3-Clause

use crate::types::{
	ChecksumAlgorithm, ColumnDescriptor, breakpoint::BreakpointMode,
	exception::ExceptionBreakpointsFilter,
};

/// Information about the capabilities of a debug adapter.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
	/// The debug adapter supports the `configurationDone` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_configuration_done_request:       Option<bool>,
	/// The debug adapter supports function breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_function_breakpoints:             Option<bool>,
	/// The debug adapter supports conditional breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_conditional_breakpoints:          Option<bool>,
	/// The debug adapter supports breakpoints that break execution after a specified number of
	/// hits.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_hit_conditional_breakpoints:      Option<bool>,
	/// The debug adapter supports a (side effect free) `evaluate` request for data hovers.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_evaluate_for_hovers:              Option<bool>,
	/// Available exception filter options for the `setExceptionBreakpoints` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) exception_breakpoint_filters:              Option<Vec<ExceptionBreakpointsFilter>>,
	/// The debug adapter supports stepping back via the `stepBack` and `reverseContinue` requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_step_back:                        Option<bool>,
	/// The debug adapter supports setting a variable to a value.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_set_variable:                     Option<bool>,
	/// The debug adapter supports restarting a frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_restart_frame:                    Option<bool>,
	/// The debug adapter supports the `gotoTargets` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_goto_targets_request:             Option<bool>,
	/// The debug adapter supports the `stepInTargets` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_step_in_targets_request:          Option<bool>,
	/// The debug adapter supports the `completions` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_completions_request:              Option<bool>,
	/// The set of characters that should automatically trigger a completion request in a REPL.
	///
	/// If not specified, the client should assume the `.` character.
	///
	/// The client may trigger additional completion requests on characters such as ones that make
	/// up common identifiers, or as otherwise requested by a user.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) completion_trigger_characters:             Option<Vec<String>>,
	/// The debug adapter supports the `modules` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_modules_request:                  Option<bool>,
	/// The set of additional module information exposed by the debug adapter.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) additional_module_columns:                 Option<Vec<ColumnDescriptor>>,
	/// Checksum algorithms supported by the debug adapter.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supported_checksum_algorithms:             Option<Vec<ChecksumAlgorithm>>,
	/// The debug adapter supports the `restart` request.
	///
	/// In this case a client should not implement `restart` by terminating and relaunching the
	/// adapter but by calling the `restart` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_restart_request:                  Option<bool>,
	/// The debug adapter supports `exceptionOptions` on the `setExceptionBreakpoints` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_exception_options:                Option<bool>,
	/// The debug adapter supports a `format` attribute on the `stackTrace`, `variables`, and
	/// `evaluate` requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_value_formatting_options:         Option<bool>,
	/// The debug adapter supports the `exceptionInfo` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_exception_info_request:           Option<bool>,
	/// The debug adapter supports the `terminateDebuggee` attribute on the `disconnect` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) support_terminate_debuggee:                Option<bool>,
	/// The debug adapter supports the `suspendDebuggee` attribute on the `disconnect` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) support_suspend_debugee:                   Option<bool>,
	/// The debug adapter supports the delayed loading of parts of the stack, which requires that
	/// both the `startFrame` and `levels` arguments and the `totalFrames` result of the
	/// `stackTrace` request are supported.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_delayed_stack_trace_loading:      Option<bool>,
	/// The debug adapter supports the `loadedSources` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_loaded_sources_request:           Option<bool>,
	/// The debug adapter supports log points by interpreting the `logMessage` attribute of the
	/// [`SourceBreakpoint`].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_log_points:                       Option<bool>,
	/// The debug adapter supports the `terminateThreads` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_terminate_threads_request:        Option<bool>,
	/// The debug adapter supports the `setExpression` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_set_expression:                   Option<bool>,
	/// The debug adapter supports the `terminate` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_terminate_request:                Option<bool>,
	/// The debug adapter supports data breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_data_breakpoints:                 Option<bool>,
	/// The debug adapter supports the `readMemory` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_read_memory_request:              Option<bool>,
	/// The debug adapter supports the `writeMemory` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_write_memory_request:             Option<bool>,
	/// The debug adapter supports the `disassemble` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_disassemble_request:              Option<bool>,
	/// The debug adapter supports the `cancel` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_cancel_request:                   Option<bool>,
	/// The debug adapter supports the `breakpointLocations` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_breakpoint_locations_request:     Option<bool>,
	/// The debug adapter supports the `clipboard` context value in the `evaluate` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_clipboard_context:                Option<bool>,
	/// The debug adapter supports stepping granularities (argument `granularity`) for the stepping
	/// requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_stepping_granularity:             Option<bool>,
	/// The debug adapter supports adding breakpoints based on instruction references.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_instruction_breakpoints:          Option<bool>,
	/// The debug adapter supports `filterOptions` as an argument on the `setExceptionBreakpoints`
	/// request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_exception_filter_options:         Option<bool>,
	/// The debug adapter supports the `singleThread` property on the execution requests
	/// (`continue`, `next`, `stepIn`, `stepOut`, `reverseContinue`, `stepBack`).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_single_thread_execution_requests: Option<bool>,
	/// The debug adapter supports the `asAddress` and `bytes` fields in the `dataBreakpointInfo`
	/// request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_data_breakpoint_bytes:            Option<bool>,
	/// Modes of breakpoints supported by the debug adapter, such as 'hardware' or 'software'.
	///
	/// If present, the client may allow the user to select a mode and include it in its
	/// `setBreakpoints` request.
	///
	/// Clients may present the first applicable mode in this array as the 'default' mode in
	/// gestures that set breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) breakpoint_modes:                          Option<Vec<BreakpointMode>>,
	/// The debug adapter supports ANSI escape sequences in styling of [`OutputEvent::output`] and
	/// `Variable.value` fields.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "supportsANSIStyling")]
	pub(crate) supports_ansi_styling:                     Option<bool>,
}

impl Capabilities {
	#[allow(clippy::too_many_arguments, reason = "Big structure, can't do much about it")]
	pub fn new(
		supports_configuration_done_request: Option<bool>,
		supports_function_breakpoints: Option<bool>,
		supports_conditional_breakpoints: Option<bool>,
		supports_hit_conditional_breakpoints: Option<bool>,
		supports_evaluate_for_hovers: Option<bool>,
		exception_breakpoint_filters: Option<Vec<ExceptionBreakpointsFilter>>,
		supports_step_back: Option<bool>,
		supports_set_variable: Option<bool>,
		supports_restart_frame: Option<bool>,
		supports_goto_targets_request: Option<bool>,
		supports_step_in_targets_request: Option<bool>,
		supports_completions_request: Option<bool>,
		completion_trigger_characters: Option<Vec<String>>,
		supports_modules_request: Option<bool>,
		additional_module_columns: Option<Vec<ColumnDescriptor>>,
		supported_checksum_algorithms: Option<Vec<ChecksumAlgorithm>>,
		supports_restart_request: Option<bool>,
		supports_exception_options: Option<bool>,
		supports_value_formatting_options: Option<bool>,
		supports_exception_info_request: Option<bool>,
		support_terminate_debuggee: Option<bool>,
		support_suspend_debugee: Option<bool>,
		supports_delayed_stack_trace_loading: Option<bool>,
		supports_loaded_sources_request: Option<bool>,
		supports_log_points: Option<bool>,
		supports_terminate_threads_request: Option<bool>,
		supports_set_expression: Option<bool>,
		supports_terminate_request: Option<bool>,
		supports_data_breakpoints: Option<bool>,
		supports_read_memory_request: Option<bool>,
		supports_write_memory_request: Option<bool>,
		supports_disassemble_request: Option<bool>,
		supports_cancel_request: Option<bool>,
		supports_breakpoint_locations_request: Option<bool>,
		supports_clipboard_context: Option<bool>,
		supports_stepping_granularity: Option<bool>,
		supports_instruction_breakpoints: Option<bool>,
		supports_exception_filter_options: Option<bool>,
		supports_single_thread_execution_requests: Option<bool>,
		supports_data_breakpoint_bytes: Option<bool>,
		breakpoint_modes: Option<Vec<BreakpointMode>>,
		supports_ansi_styling: Option<bool>,
	) -> Self {
		Self {
			supports_configuration_done_request,
			supports_function_breakpoints,
			supports_conditional_breakpoints,
			supports_hit_conditional_breakpoints,
			supports_evaluate_for_hovers,
			exception_breakpoint_filters,
			supports_step_back,
			supports_set_variable,
			supports_restart_frame,
			supports_goto_targets_request,
			supports_step_in_targets_request,
			supports_completions_request,
			completion_trigger_characters,
			supports_modules_request,
			additional_module_columns,
			supported_checksum_algorithms,
			supports_restart_request,
			supports_exception_options,
			supports_value_formatting_options,
			supports_exception_info_request,
			support_terminate_debuggee,
			support_suspend_debugee,
			supports_delayed_stack_trace_loading,
			supports_loaded_sources_request,
			supports_log_points,
			supports_terminate_threads_request,
			supports_set_expression,
			supports_terminate_request,
			supports_data_breakpoints,
			supports_read_memory_request,
			supports_write_memory_request,
			supports_disassemble_request,
			supports_cancel_request,
			supports_breakpoint_locations_request,
			supports_clipboard_context,
			supports_stepping_granularity,
			supports_instruction_breakpoints,
			supports_exception_filter_options,
			supports_single_thread_execution_requests,
			supports_data_breakpoint_bytes,
			breakpoint_modes,
			supports_ansi_styling,
		}
	}

	/// The debug adapter supports the `configurationDone` request.
	pub fn supports_configuration_done_request(&self) -> Option<bool> {
		self.supports_configuration_done_request
	}

	/// The debug adapter supports function breakpoints.
	pub fn supports_function_breakpoints(&self) -> Option<bool> {
		self.supports_function_breakpoints
	}

	/// The debug adapter supports conditional breakpoints.
	pub fn supports_conditional_breakpoints(&self) -> Option<bool> {
		self.supports_conditional_breakpoints
	}

	/// The debug adapter supports breakpoints that break execution after a specified number of
	/// hits.
	pub fn supports_hit_conditional_breakpoints(&self) -> Option<bool> {
		self.supports_hit_conditional_breakpoints
	}

	/// The debug adapter supports a (side effect free) `evaluate` request for data hovers.
	pub fn supports_evaluate_for_hovers(&self) -> Option<bool> {
		self.supports_evaluate_for_hovers
	}

	/// Available exception filter options for the `setExceptionBreakpoints` request.
	pub fn exception_breakpoint_filters(&self) -> Option<&Vec<ExceptionBreakpointsFilter>> {
		self.exception_breakpoint_filters.as_ref()
	}

	/// The debug adapter supports stepping back via the `stepBack` and `reverseContinue` requests.
	pub fn supports_step_back(&self) -> Option<bool> {
		self.supports_step_back
	}

	/// The debug adapter supports setting a variable to a value.
	pub fn supports_set_variable(&self) -> Option<bool> {
		self.supports_set_variable
	}

	/// The debug adapter supports restarting a frame.
	pub fn supports_restart_frame(&self) -> Option<bool> {
		self.supports_restart_frame
	}

	/// The debug adapter supports the `gotoTargets` request.
	pub fn supports_goto_targets_request(&self) -> Option<bool> {
		self.supports_goto_targets_request
	}

	/// The debug adapter supports the `stepInTargets` request.
	pub fn supports_step_in_targets_request(&self) -> Option<bool> {
		self.supports_step_in_targets_request
	}

	/// The debug adapter supports the `completions` request.
	pub fn supports_completions_request(&self) -> Option<bool> {
		self.supports_completions_request
	}

	/// The set of characters that should automatically trigger a completion request in a REPL.
	///
	/// If not specified, the client should assume the `.` character.
	///
	/// The client may trigger additional completion requests on characters such as ones that make
	/// up common identifiers, or as otherwise requested by a user.
	pub fn completion_trigger_characters(&self) -> Option<&Vec<String>> {
		self.completion_trigger_characters.as_ref()
	}

	/// The debug adapter supports the `modules` request.
	pub fn supports_modules_request(&self) -> Option<bool> {
		self.supports_modules_request
	}

	/// The set of additional module information exposed by the debug adapter.
	pub fn additional_module_columns(&self) -> Option<&Vec<ColumnDescriptor>> {
		self.additional_module_columns.as_ref()
	}

	/// Checksum algorithms supported by the debug adapter.
	pub fn supported_checksum_algorithms(&self) -> Option<&Vec<ChecksumAlgorithm>> {
		self.supported_checksum_algorithms.as_ref()
	}

	/// The debug adapter supports the `restart` request.
	///
	/// In this case a client should not implement `restart` by terminating and relaunching the
	/// adapter but by calling the `restart` request.
	pub fn supports_restart_request(&self) -> Option<bool> {
		self.supports_restart_request
	}

	/// The debug adapter supports `exceptionOptions` on the `setExceptionBreakpoints` request.
	pub fn supports_exception_options(&self) -> Option<bool> {
		self.supports_exception_options
	}

	/// The debug adapter supports a `format` attribute on the `stackTrace`, `variables`, and
	/// `evaluate` requests.
	pub fn supports_value_formatting_options(&self) -> Option<bool> {
		self.supports_value_formatting_options
	}

	/// The debug adapter supports the `exceptionInfo` request.
	pub fn supports_exception_info_request(&self) -> Option<bool> {
		self.supports_exception_info_request
	}

	/// The debug adapter supports the `terminateDebuggee` attribute on the `disconnect` request.
	pub fn support_terminate_debuggee(&self) -> Option<bool> {
		self.support_terminate_debuggee
	}

	/// The debug adapter supports the `suspendDebuggee` attribute on the `disconnect` request.
	pub fn support_suspend_debugee(&self) -> Option<bool> {
		self.support_suspend_debugee
	}

	/// The debug adapter supports the delayed loading of parts of the stack, which requires that
	/// both the `startFrame` and `levels` arguments and the `totalFrames` result of the
	/// `stackTrace` request are supported.
	pub fn supports_delayed_stack_trace_loading(&self) -> Option<bool> {
		self.supports_delayed_stack_trace_loading
	}

	/// The debug adapter supports the `loadedSources` request.
	pub fn supports_loaded_sources_request(&self) -> Option<bool> {
		self.supports_loaded_sources_request
	}

	/// The debug adapter supports log points by interpreting the `logMessage` attribute of the
	/// [`SourceBreakpoint`].
	pub fn supports_log_points(&self) -> Option<bool> {
		self.supports_log_points
	}

	/// The debug adapter supports the `terminateThreads` request.
	pub fn supports_terminate_threads_request(&self) -> Option<bool> {
		self.supports_terminate_threads_request
	}

	/// The debug adapter supports the `setExpression` request.
	pub fn supports_set_expression(&self) -> Option<bool> {
		self.supports_set_expression
	}

	/// The debug adapter supports the `terminate` request.
	pub fn supports_terminate_request(&self) -> Option<bool> {
		self.supports_terminate_request
	}

	/// The debug adapter supports data breakpoints.
	pub fn supports_data_breakpoints(&self) -> Option<bool> {
		self.supports_data_breakpoints
	}

	/// The debug adapter supports the `readMemory` request.
	pub fn supports_read_memory_request(&self) -> Option<bool> {
		self.supports_read_memory_request
	}

	/// The debug adapter supports the `writeMemory` request.
	pub fn supports_write_memory_request(&self) -> Option<bool> {
		self.supports_write_memory_request
	}

	/// The debug adapter supports the `disassemble` request.
	pub fn supports_disassemble_request(&self) -> Option<bool> {
		self.supports_disassemble_request
	}

	/// The debug adapter supports the `cancel` request.
	pub fn supports_cancel_request(&self) -> Option<bool> {
		self.supports_cancel_request
	}

	/// The debug adapter supports the `breakpointLocations` request.
	pub fn supports_breakpoint_locations_request(&self) -> Option<bool> {
		self.supports_breakpoint_locations_request
	}

	/// The debug adapter supports the `clipboard` context value in the `evaluate` request.
	pub fn supports_clipboard_context(&self) -> Option<bool> {
		self.supports_clipboard_context
	}

	/// The debug adapter supports stepping granularities (argument `granularity`) for the stepping
	/// requests.
	pub fn supports_stepping_granularity(&self) -> Option<bool> {
		self.supports_stepping_granularity
	}

	/// The debug adapter supports adding breakpoints based on instruction references.
	pub fn supports_instruction_breakpoints(&self) -> Option<bool> {
		self.supports_instruction_breakpoints
	}

	/// The debug adapter supports `filterOptions` as an argument on the `setExceptionBreakpoints`
	/// request.
	pub fn supports_exception_filter_options(&self) -> Option<bool> {
		self.supports_exception_filter_options
	}

	/// The debug adapter supports the `singleThread` property on the execution requests
	/// (`continue`, `next`, `stepIn`, `stepOut`, `reverseContinue`, `stepBack`).
	pub fn supports_single_thread_execution_requests(&self) -> Option<bool> {
		self.supports_single_thread_execution_requests
	}

	/// The debug adapter supports the `asAddress` and `bytes` fields in the `dataBreakpointInfo`
	/// request.
	pub fn supports_data_breakpoint_bytes(&self) -> Option<bool> {
		self.supports_data_breakpoint_bytes
	}

	/// Modes of breakpoints supported by the debug adapter, such as 'hardware' or 'software'.
	///
	/// If present, the client may allow the user to select a mode and include it in its
	/// `setBreakpoints` request.
	///
	/// Clients may present the first applicable mode in this array as the 'default' mode in
	/// gestures that set breakpoints.
	pub fn breakpoint_modes(&self) -> Option<&Vec<BreakpointMode>> {
		self.breakpoint_modes.as_ref()
	}

	/// The debug adapter supports ANSI escape sequences in styling of [`OutputEvent::output`] and
	/// `Variable.value` fields.
	pub fn supports_ansi_styling(&self) -> Option<bool> {
		self.supports_ansi_styling
	}
}
