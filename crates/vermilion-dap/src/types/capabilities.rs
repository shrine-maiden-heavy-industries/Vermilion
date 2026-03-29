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

impl Default for Capabilities {
	fn default() -> Self {
		Self::new()
	}
}

impl Capabilities {
	pub fn new() -> Self {
		Self {
			supports_configuration_done_request:       None,
			supports_function_breakpoints:             None,
			supports_conditional_breakpoints:          None,
			supports_hit_conditional_breakpoints:      None,
			supports_evaluate_for_hovers:              None,
			exception_breakpoint_filters:              None,
			supports_step_back:                        None,
			supports_set_variable:                     None,
			supports_restart_frame:                    None,
			supports_goto_targets_request:             None,
			supports_step_in_targets_request:          None,
			supports_completions_request:              None,
			completion_trigger_characters:             None,
			supports_modules_request:                  None,
			additional_module_columns:                 None,
			supported_checksum_algorithms:             None,
			supports_restart_request:                  None,
			supports_exception_options:                None,
			supports_value_formatting_options:         None,
			supports_exception_info_request:           None,
			support_terminate_debuggee:                None,
			support_suspend_debugee:                   None,
			supports_delayed_stack_trace_loading:      None,
			supports_loaded_sources_request:           None,
			supports_log_points:                       None,
			supports_terminate_threads_request:        None,
			supports_set_expression:                   None,
			supports_terminate_request:                None,
			supports_data_breakpoints:                 None,
			supports_read_memory_request:              None,
			supports_write_memory_request:             None,
			supports_disassemble_request:              None,
			supports_cancel_request:                   None,
			supports_breakpoint_locations_request:     None,
			supports_clipboard_context:                None,
			supports_stepping_granularity:             None,
			supports_instruction_breakpoints:          None,
			supports_exception_filter_options:         None,
			supports_single_thread_execution_requests: None,
			supports_data_breakpoint_bytes:            None,
			breakpoint_modes:                          None,
			supports_ansi_styling:                     None,
		}
	}

	pub fn with_supports_configuration_done_request(
		self,
		supports_configuration_done_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_configuration_done_request = Some(supports_configuration_done_request);
		this
	}

	pub fn with_supports_function_breakpoints(self, supports_function_breakpoints: bool) -> Self {
		let mut this = self;
		this.supports_function_breakpoints = Some(supports_function_breakpoints);
		this
	}

	pub fn with_supports_conditional_breakpoints(
		self,
		supports_conditional_breakpoints: bool,
	) -> Self {
		let mut this = self;
		this.supports_conditional_breakpoints = Some(supports_conditional_breakpoints);
		this
	}

	pub fn with_supports_hit_conditional_breakpoints(
		self,
		supports_hit_conditional_breakpoints: bool,
	) -> Self {
		let mut this = self;
		this.supports_hit_conditional_breakpoints = Some(supports_hit_conditional_breakpoints);
		this
	}

	pub fn with_supports_evaluate_for_hovers(self, supports_evaluate_for_hovers: bool) -> Self {
		let mut this = self;
		this.supports_evaluate_for_hovers = Some(supports_evaluate_for_hovers);
		this
	}

	pub fn with_exception_breakpoint_filters(
		self,
		exception_breakpoint_filters: Vec<ExceptionBreakpointsFilter>,
	) -> Self {
		let mut this = self;
		this.exception_breakpoint_filters = Some(exception_breakpoint_filters);
		this
	}

	pub fn with_supports_step_back(self, supports_step_back: bool) -> Self {
		let mut this = self;
		this.supports_step_back = Some(supports_step_back);
		this
	}

	pub fn with_supports_set_variable(self, supports_set_variable: bool) -> Self {
		let mut this = self;
		this.supports_set_variable = Some(supports_set_variable);
		this
	}

	pub fn with_supports_restart_frame(self, supports_restart_frame: bool) -> Self {
		let mut this = self;
		this.supports_restart_frame = Some(supports_restart_frame);
		this
	}

	pub fn with_supports_goto_targets_request(self, supports_goto_targets_request: bool) -> Self {
		let mut this = self;
		this.supports_goto_targets_request = Some(supports_goto_targets_request);
		this
	}

	pub fn with_supports_step_in_targets_request(
		self,
		supports_step_in_targets_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_step_in_targets_request = Some(supports_step_in_targets_request);
		this
	}

	pub fn with_supports_completions_request(self, supports_completions_request: bool) -> Self {
		let mut this = self;
		this.supports_completions_request = Some(supports_completions_request);
		this
	}

	pub fn with_completion_trigger_characters(
		self,
		completion_trigger_characters: Vec<String>,
	) -> Self {
		let mut this = self;
		this.completion_trigger_characters = Some(completion_trigger_characters);
		this
	}

	pub fn with_supports_modules_request(self, supports_modules_request: bool) -> Self {
		let mut this = self;
		this.supports_modules_request = Some(supports_modules_request);
		this
	}

	pub fn with_additional_module_columns(
		self,
		additional_module_columns: Vec<ColumnDescriptor>,
	) -> Self {
		let mut this = self;
		this.additional_module_columns = Some(additional_module_columns);
		this
	}

	pub fn with_supported_checksum_algorithms(
		self,
		supported_checksum_algorithms: Vec<ChecksumAlgorithm>,
	) -> Self {
		let mut this = self;
		this.supported_checksum_algorithms = Some(supported_checksum_algorithms);
		this
	}

	pub fn with_supports_restart_request(self, supports_restart_request: bool) -> Self {
		let mut this = self;
		this.supports_restart_request = Some(supports_restart_request);
		this
	}

	pub fn with_supports_exception_options(self, supports_exception_options: bool) -> Self {
		let mut this = self;
		this.supports_exception_options = Some(supports_exception_options);
		this
	}

	pub fn with_supports_value_formatting_options(
		self,
		supports_value_formatting_options: bool,
	) -> Self {
		let mut this = self;
		this.supports_value_formatting_options = Some(supports_value_formatting_options);
		this
	}

	pub fn with_supports_exception_info_request(
		self,
		supports_exception_info_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_exception_info_request = Some(supports_exception_info_request);
		this
	}

	pub fn with_support_terminate_debuggee(self, support_terminate_debuggee: bool) -> Self {
		let mut this = self;
		this.support_terminate_debuggee = Some(support_terminate_debuggee);
		this
	}

	pub fn with_support_suspend_debugee(self, support_suspend_debugee: bool) -> Self {
		let mut this = self;
		this.support_suspend_debugee = Some(support_suspend_debugee);
		this
	}

	pub fn with_supports_delayed_stack_trace_loading(
		self,
		supports_delayed_stack_trace_loading: bool,
	) -> Self {
		let mut this = self;
		this.supports_delayed_stack_trace_loading = Some(supports_delayed_stack_trace_loading);
		this
	}

	pub fn with_supports_loaded_sources_request(
		self,
		supports_loaded_sources_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_loaded_sources_request = Some(supports_loaded_sources_request);
		this
	}

	pub fn with_supports_log_points(self, supports_log_points: bool) -> Self {
		let mut this = self;
		this.supports_log_points = Some(supports_log_points);
		this
	}

	pub fn with_supports_terminate_threads_request(
		self,
		supports_terminate_threads_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_terminate_threads_request = Some(supports_terminate_threads_request);
		this
	}

	pub fn with_supports_set_expression(self, supports_set_expression: bool) -> Self {
		let mut this = self;
		this.supports_set_expression = Some(supports_set_expression);
		this
	}

	pub fn with_supports_terminate_request(self, supports_terminate_request: bool) -> Self {
		let mut this = self;
		this.supports_terminate_request = Some(supports_terminate_request);
		this
	}

	pub fn with_supports_data_breakpoints(self, supports_data_breakpoints: bool) -> Self {
		let mut this = self;
		this.supports_data_breakpoints = Some(supports_data_breakpoints);
		this
	}

	pub fn with_supports_read_memory_request(self, supports_read_memory_request: bool) -> Self {
		let mut this = self;
		this.supports_read_memory_request = Some(supports_read_memory_request);
		this
	}

	pub fn with_supports_write_memory_request(self, supports_write_memory_request: bool) -> Self {
		let mut this = self;
		this.supports_write_memory_request = Some(supports_write_memory_request);
		this
	}

	pub fn with_supports_disassemble_request(self, supports_disassemble_request: bool) -> Self {
		let mut this = self;
		this.supports_disassemble_request = Some(supports_disassemble_request);
		this
	}

	pub fn with_supports_cancel_request(self, supports_cancel_request: bool) -> Self {
		let mut this = self;
		this.supports_cancel_request = Some(supports_cancel_request);
		this
	}

	pub fn with_supports_breakpoint_locations_request(
		self,
		supports_breakpoint_locations_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_breakpoint_locations_request = Some(supports_breakpoint_locations_request);
		this
	}

	pub fn with_supports_clipboard_context(self, supports_clipboard_context: bool) -> Self {
		let mut this = self;
		this.supports_clipboard_context = Some(supports_clipboard_context);
		this
	}

	pub fn with_supports_stepping_granularity(self, supports_stepping_granularity: bool) -> Self {
		let mut this = self;
		this.supports_stepping_granularity = Some(supports_stepping_granularity);
		this
	}

	pub fn with_supports_instruction_breakpoints(
		self,
		supports_instruction_breakpoints: bool,
	) -> Self {
		let mut this = self;
		this.supports_instruction_breakpoints = Some(supports_instruction_breakpoints);
		this
	}

	pub fn with_supports_exception_filter_options(
		self,
		supports_exception_filter_options: bool,
	) -> Self {
		let mut this = self;
		this.supports_exception_filter_options = Some(supports_exception_filter_options);
		this
	}

	pub fn with_supports_single_thread_execution_requests(
		self,
		supports_single_thread_execution_requests: bool,
	) -> Self {
		let mut this = self;
		this.supports_single_thread_execution_requests =
			Some(supports_single_thread_execution_requests);
		this
	}

	pub fn with_supports_data_breakpoint_bytes(self, supports_data_breakpoint_bytes: bool) -> Self {
		let mut this = self;
		this.supports_data_breakpoint_bytes = Some(supports_data_breakpoint_bytes);
		this
	}

	pub fn with_breakpoint_modes(self, breakpoint_modes: Vec<BreakpointMode>) -> Self {
		let mut this = self;
		this.breakpoint_modes = Some(breakpoint_modes);
		this
	}

	pub fn with_supports_ansi_styling(self, supports_ansi_styling: bool) -> Self {
		let mut this = self;
		this.supports_ansi_styling = Some(supports_ansi_styling);
		this
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
