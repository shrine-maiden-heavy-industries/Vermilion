// SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;

use crate::types::{
	EvalContext, OutputPresentation, PathFormat, Source, StackFrameFormat, StartDebuggingRequest,
	SteppingGranularity, TerminalKind, ValueFormat, VariablesFilter,
	breakpoint::{DataBreakpoint, FunctionBreakpoint, InstructionBreakpoint, SourceBreakpoint},
	options::{ExceptionFilterOptions, ExceptionOptions},
};

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CancelParams {
	/// The ID (attribute `seq`) of the request to cancel. If missing no request is
	/// cancelled.
	/// Both a `requestId` and a `progressId` can be specified in one request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) request_id:  Option<u32>,
	/// The ID (attribute `progressId`) of the progress to cancel. If missing no
	/// progress is cancelled.
	/// Both a `requestId` and a `progressId` can be specified in one request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) progress_id: Option<String>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachParams {
	/// Arbitrary data from the previous, restarted session.
	///
	/// The data is sent as the `restart` attribute of the `terminated` event.
	///
	/// The client should leave the data intact.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "__restart")]
	pub(crate) restart: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocationsParams {
	/// The source location of the breakpoints; either `source.path` or `source.sourceReference`
	/// must be specified.
	pub(crate) source:     Source,
	/// Start line of range to search possible breakpoint locations in.
	///
	/// If only the line is specified, the request returns all possible locations in that line.
	pub(crate) line:       u64,
	/// Start position within `line` to search possible breakpoint locations in.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based. If no column is given, the first position in the start line
	/// is assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:     Option<u64>,
	/// End line of range to search possible breakpoint locations in.
	///
	/// If no end line is given, then the end line is assumed to be the start line.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line:   Option<u64>,
	/// End position within `endLine` to search possible breakpoint locations in.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based. If no end column is given, the last position in the end line
	/// is assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column: Option<u64>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsParams {
	/// Returns completions in the scope of this stack frame.
	///
	/// If not specified, the completions are returned for the global scope.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) frame_id: Option<i32>,
	/// One or more source lines.
	///
	/// Typically this is the text users have typed into the debug console before they asked for
	/// completion.
	pub(crate) text:     String,
	/// The position within `text` for which to determine the completion proposals.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub(crate) column:   u64,
	/// A line for which to determine the completion proposals.
	///
	/// If missing the first line of the text is assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line:     Option<u64>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationDoneParams {}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ContinueParams {
	/// Specifies the active thread.
	///
	/// If the debug adapter supports single thread execution (see
	/// `supportsSingleThreadExecutionRequests`) and the argument `singleThread` is true, only the
	/// thread with this ID is resumed.
	pub(crate) thread_id:     i32,
	/// If this flag is true, execution is resumed only for the thread with given `threadId`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) single_thread: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpointInfoParams {
	/// Reference to the variable container if the data breakpoint is requested for a child of the
	/// container. The `variablesReference` must have been obtained in the current suspended state.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) variables_reference: Option<u32>,
	/// The name of the variable's child to obtain data breakpoint information for.
	///
	/// If `variablesReference` isn't specified, this can be an expression, or an address if
	/// `asAddress` is also true.
	pub(crate) name:                String,
	/// When `name` is an expression, evaluate it in the scope of this stack frame.
	///
	/// If not specified, the expression is evaluated in the global scope. When
	/// `variablesReference` is specified, this property has no effect.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) frame_id:            Option<i32>,
	/// If specified, a debug adapter should return information for the range of memory extending
	/// `bytes` number of bytes from the address or variable specified by `name`. Breakpoints set
	/// using the resulting data ID should pause on data access anywhere within that
	/// range.\n\nClients may set this property only if the `supportsDataBreakpointBytes`
	/// capability is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) bytes:               Option<u32>,
	/// If `true`, the `name` is a memory address and the debugger should interpret it as a decimal
	/// value, or hex value if it is prefixed with `0x`.
	///
	/// Clients may set this property only if the `supportsDataBreakpointBytes` capability is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) as_address:          Option<bool>,
	/// The mode of the desired breakpoint.
	///
	/// If defined, this must be one of the `breakpointModes` the debug adapter advertised in its
	/// `Capabilities`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) mode:                Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleParams {
	/// Memory reference to the base location containing the instructions to disassemble.
	pub(crate) memory_reference:   String,
	/// Offset (in bytes) to be applied to the reference location before disassembling. Can be
	/// negative.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) offset:             Option<i64>,
	/// Offset (in instructions) to be applied after the byte offset (if any) before disassembling.
	/// Can be negative.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) instruction_offset: Option<i64>,
	/// Number of instructions to disassemble starting at the specified location and offset.
	///
	/// An adapter must return exactly this number of instructions - any unavailable instructions
	/// should be replaced with an implementation-defined 'invalid instruction' value.
	pub(crate) instruction_count:  u32,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	/// If true, the adapter should attempt to resolve memory addresses and other values to
	/// symbolic names.
	pub(crate) resolve_symbols:    Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectParams {
	/// A value of true indicates that this `disconnect` request is part of a restart sequence.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) restart:            Option<bool>,
	/// Indicates whether the debuggee should be terminated when the debugger is disconnected.
	///
	/// If unspecified, the debug adapter is free to do whatever it thinks is best.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportTerminateDebuggee` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) terminate_debuggee: Option<bool>,
	/// Indicates whether the debuggee should stay suspended when the debugger is disconnected.
	///
	/// If unspecified, the debuggee should resume execution.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportSuspendDebuggee` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) suspend_debuggee:   Option<bool>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateParams {
	/// The expression to evaluate.
	pub(crate) expression: String,
	/// Evaluate the expression in the scope of this stack frame. If not specified, the expression
	/// is evaluated in the global scope.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) frame_id:   Option<i32>,
	/// The contextual line where the expression should be evaluated. In the 'hover' context, this
	/// should be set to the start of the expression being hovered.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line:       Option<u64>,
	/// The contextual column where the expression should be evaluated. This may be provided if
	/// `line` is also provided.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:     Option<u64>,
	/// The contextual source in which the `line` is found. This must be provided if `line` is
	/// provided.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source:     Option<Box<Source>>,
	/// The context in which the evaluate request is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) context:    Option<EvalContext>,
	/// Specifies details on how to format the result.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsValueFormattingOptions` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) format:     Option<ValueFormat>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfoParams {
	/// Thread for which exception information should be retrieved.
	pub(crate) thread_id: i32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GotoParams {
	/// Set the goto target for this thread.
	pub(crate) thread_id: i32,
	/// The location where the debuggee will continue to run.
	pub(crate) target_id: i32,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoTargetsParams {
	/// The source location for which the goto targets are determined.
	pub(crate) source: Source,
	/// The line location for which the goto targets are determined.
	pub(crate) line:   u64,
	/// The position within `line` for which the goto targets are determined.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column: Option<u64>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
	/// The ID of the client using this adapter.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) client_id: Option<String>,
	/// The human-readable name of the client using this adapter.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) client_name: Option<String>,
	/// The ID of the debug adapter.
	pub(crate) adapter_id: String,
	/// The ISO-639 locale of the client using this adapter, e.g. en-US or de-CH.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) locale: Option<String>,
	/// If true all line numbers are 1-based (default).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) lines_start_at_1: Option<bool>,
	/// If true all column numbers are 1-based (default).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) columns_start_at_1: Option<bool>,
	/// Determines in what format paths are specified. The default is `path`, which is the native
	/// format.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) path_format: Option<PathFormat>,
	/// Client supports the `type` attribute for variables.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_variable_type: Option<bool>,
	/// Client supports the paging of variables.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_variable_paging: Option<bool>,
	/// Client supports the `runInTerminal` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_run_in_terminal_request: Option<bool>,
	/// Client supports memory references.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_memory_references: Option<bool>,
	/// Client supports progress reporting.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_progress_reporting: Option<bool>,
	/// Client supports the `invalidated` event.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_invalidated_event: Option<bool>,
	/// Client supports the `memory` event.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_memory_event: Option<bool>,
	/// Client supports the `argsCanBeInterpretedByShell` attribute on the `runInTerminal` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_args_can_be_interpreted_by_shell: Option<bool>,
	/// Client supports the `startDebugging` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supports_start_debugging_request: Option<bool>,
	/// The client will interpret ANSI escape sequences in the display of `OutputEvent.output` and
	/// `Variable.value` fields when `Capabilities.supportsANSIStyling` is also enabled.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "supportsANSIStyling")]
	pub(crate) supports_ansi_styling: Option<bool>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchParams {
	/// If true, the launch request should launch the program without enabling debugging.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) no_debug: Option<bool>,
	/// Arbitrary data from the previous, restarted session.
	///
	/// The data is sent as the `restart` attribute of the `terminated` event.
	///
	/// The client should leave the data intact.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "__restart")]
	pub(crate) restart:  Option<serde_json::Value>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LocationsParams {
	/// Location reference to resolve.
	pub(crate) location_reference: i32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ModuleParams {
	/// The index of the first module to return; if omitted modules start at 0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) start_module: Option<i32>,
	/// The number of modules to return. If `moduleCount` is not specified or 0, all modules are
	/// returned.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) module_count: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NextParams {
	/// Specifies the thread for which to resume execution for one step (of the given granularity).
	pub(crate) thread_id:     i32,
	/// If this flag is true, all other suspended threads are not resumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) single_thread: Option<bool>,
	/// Stepping granularity. If no granularity is specified, a granularity of `statement` is
	/// assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) granularity:   Option<SteppingGranularity>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PauseParams {
	/// Pause execution for this thread.
	pub(crate) thread_id: i32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReadMemoryParams {
	/// Memory reference to the base location from which data should be read.
	pub(crate) memory_reference: String,
	/// Offset (in bytes) to be applied to the reference location before reading data. Can be
	/// negative.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) offset:           Option<i64>,
	/// Number of bytes to read at the specified location and offset.
	pub(crate) count:            u64,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum RestartParams {
	Launch(LaunchParams),
	Attach(AttachParams),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RestartFrameParams {
	/// Restart the stack frame identified by `frameId`.
	///
	/// The `frameId` must have been obtained in the current suspended state.
	pub(crate) frame_id: i32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReverseContinueParams {
	/// Specifies the active thread. If the debug adapter supports single thread execution (see
	/// `supportsSingleThreadExecutionRequests`) and the `singleThread` argument is true, only the
	/// thread with this ID is resumed.
	pub(crate) thread_id:     i32,
	/// If this flag is true, backward execution is resumed only for the thread with given
	/// `threadId`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) single_thread: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ScopesParams {
	/// Retrieve the scopes for the stack frame identified by `frameId`.
	///
	/// The `frameId` must have been obtained in the current suspended state.
	pub(crate) frame_id: i32,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsParams {
	/// The source location of the breakpoints; either `source.path` or `source.sourceReference`
	/// must be specified.
	pub(crate) source:          Source,
	/// The code locations of the breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) breakpoints:     Option<Vec<SourceBreakpoint>>,
	/// The code locations of the breakpoints.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) lines:           Option<Vec<u64>>,
	/// A value of true indicates that the underlying source has been modified which results in new
	/// breakpoint locations.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source_modified: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetDataBreakpointsParams {
	/// The contents of this array replaces all existing data breakpoints. An empty array clears
	/// all data breakpoints.
	pub(crate) breakpoints: Vec<DataBreakpoint>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetExceptionBreakpointsParams {
	/// Set of exception filters specified by their ID.
	///
	/// The set of all possible exception filters is defined by the `exceptionBreakpointFilters`
	/// capability. The `filter` and `filterOptions` sets are additive.
	pub(crate) filters:           Vec<String>,
	/// Set of exception filters and their options.
	///
	/// The set of all possible exception filters is defined by the `exceptionBreakpointFilters`
	/// capability.
	///
	/// This attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsExceptionFilterOptions` is true. The `filter` and `filterOptions` sets
	/// are additive.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) filter_options:    Option<Vec<ExceptionFilterOptions>>,
	/// Configuration options for selected exceptions.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsExceptionOptions` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) exception_options: Option<Vec<ExceptionOptions>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetExpressionParams {
	/// The l-value expression to assign to.
	pub(crate) expression: String,
	/// The value expression to assign to the l-value expression.
	pub(crate) value:      String,
	/// Evaluate the expressions in the scope of this stack frame.
	///
	/// If not specified, the expressions are evaluated in the global scope.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) frame_id:   Option<i32>,
	/// Specifies how the resulting value should be formatted.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) format:     Option<ValueFormat>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetFunctionBreakpointsParams {
	/// The function names of the breakpoints.
	pub(crate) breakpoints: Vec<FunctionBreakpoint>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetInstructionBreakpointsParams {
	/// The instruction references of the breakpoints
	pub(crate) breakpoints: Vec<InstructionBreakpoint>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableParams {
	/// The reference of the variable container.
	///
	/// The `variablesReference` must have been obtained in the current suspended state.
	pub(crate) variables_reference: i32,
	/// The name of the variable in the container.
	pub(crate) name:                String,
	/// The value of the variable.
	pub(crate) value:               String,
	/// Specifies details on how to format the response value.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) format:              Option<ValueFormat>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceParams {
	/// Specifies the source content to load. Either `source.path` or `source.sourceReference` must
	/// be specified.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source:           Option<Source>,
	/// The reference to the source. This is the same as `source.sourceReference`.
	///
	/// This is provided for backward compatibility since old clients do not understand the
	/// `source` attribute.
	pub(crate) source_reference: u32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceParams {
	/// Retrieve the stacktrace for this thread.
	pub(crate) thread_id:   i32,
	/// The index of the first frame to return; if omitted frames start at 0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) start_frame: Option<u32>,
	/// The maximum number of frames to return. If levels is not specified or 0, all frames are
	/// returned.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) levels:      Option<u32>,
	/// Specifies details on how to format the returned `StackFrame.name`.
	///
	/// The debug adapter may format requested details in any way that would make sense to a
	/// developer.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsValueFormattingOptions` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) format:      Option<StackFrameFormat>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StepBackParams {
	/// Specifies the thread for which to resume execution for one step backwards (of the given
	/// granularity).
	pub(crate) thread_id:     i32,
	/// If this flag is true, all other suspended threads are not resumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) single_thread: Option<bool>,
	/// Stepping granularity to step. If no granularity is specified, a granularity of `statement`
	/// is assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) granularity:   Option<SteppingGranularity>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StepInParams {
	/// Specifies the thread for which to resume execution for one step-into (of the given
	/// granularity).
	pub(crate) thread_id:     i32,
	/// If this flag is true, all other suspended threads are not resumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) single_thread: Option<bool>,
	/// Id of the target to step into.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) target_id:     Option<i32>,
	/// Stepping granularity. If no granularity is specified, a granularity of `statement` is
	/// assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) granularity:   Option<SteppingGranularity>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StepInTargetsParams {
	/// The stack frame for which to retrieve the possible step-in targets.
	pub(crate) frame_id: i32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StepOutParams {
	/// Specifies the thread for which to resume execution for one step-out (of the given
	/// granularity).
	pub(crate) thread_id:     i32,
	/// If this flag is true, all other suspended threads are not resumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) single_thread: Option<bool>,
	/// Stepping granularity. If no granularity is specified, a granularity of `statement` is
	/// assumed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) granularity:   Option<SteppingGranularity>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TerminateParams {
	/// A value of true indicates that this `terminate` request is part of a restart sequence.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) restart: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TerminateThreadsParams {
	/// Ids of threads to be terminated.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) thread_ids: Option<Vec<i32>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct VariablesParams {
	/// The variable for which to retrieve its children. The `variablesReference` must have been
	/// obtained in the current suspended state.
	pub(crate) variables_reference: u32,
	/// Filter to limit the child variables to either named or indexed. If omitted, both types are
	/// fetched.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) filter:              Option<VariablesFilter>,
	/// The index of the first variable to return; if omitted children start at 0.
	///
	/// If the value of `start` exceeds the number of available variables, the debug adapter should
	/// return an empty array.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsVariablePaging` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) start:               Option<u32>,
	/// The number of variables to return. If count is missing or 0, all variables are returned.
	///
	/// If fewer than `count` variables are returned, the client should assume no further variables
	/// are available.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsVariablePaging` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) count:               Option<u32>,
	/// Specifies details on how to format the Variable values.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsValueFormattingOptions` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) format:              Option<ValueFormat>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WriteMemoryParams {
	/// Memory reference to the base location to which data should be written.
	pub(crate) memory_reference: String,
	/// Offset (in bytes) to be applied to the reference location before writing data. Can be
	/// negative.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) offset:           Option<i64>,
	/// Property to control partial writes.
	///
	/// If true, the debug adapter should attempt to write memory even if the entire memory region
	/// is not writable.
	///
	/// In such a case the debug adapter should stop after hitting the first byte of memory that
	/// cannot be written and return the number of bytes written in the response via the `offset`
	/// and `bytesWritten` properties.
	///
	/// If false or missing, a debug adapter should attempt to verify the region is writable before
	/// writing, and fail the response if it is not.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) allow_partial:    Option<bool>,
	/// Bytes to write, encoded using base64.
	pub(crate) data:             String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalParams {
	/// What kind of terminal to launch. Defaults to `integrated` if not specified.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<TerminalKind>,
	/// Title of the terminal.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) title: Option<String>,
	/// Working directory for the command.
	///
	/// For non-empty, valid paths this typically results in execution of a change directory
	/// command.
	///
	/// If `pathFormat` is set to `uri` in the `InitializeRequestArguments`, this must be a file
	/// URI.
	pub(crate) cwd: String,
	/// List of arguments. The first argument is the command to run.
	pub(crate) args: Vec<String>,
	/// Environment key-value pairs that are added to or removed from the default environment.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) env: Option<HashMap<String, Option<String>>>,
	/// This property should only be set if the corresponding capability
	/// `supportsArgsCanBeInterpretedByShell` is true.
	///
	/// If the client uses an intermediary shell to launch the application, then the client must
	/// not attempt to escape characters with special meanings for the shell.
	///
	/// The user is fully responsible for escaping as needed and that arguments using special
	/// characters may not be portable across shells.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) args_can_be_interpreted_by_shell: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartDebuggingParams {
	/// Arguments passed to the new debug session.
	///
	/// The arguments must only contain properties understood by the `launch` or `attach` requests
	/// of the debug adapter and they must not contain any client-specific properties (e.g.
	/// `type`) or client-specific features (e.g. substitutable 'variables')
	pub(crate) configuration:       HashMap<String, serde_json::Value>,
	/// Indicates whether the new debug session should be started with a `launch` or `attach`
	/// request.
	pub(crate) request:             StartDebuggingRequest,
	/// Hints whether output of the child sessions should be presented separately or merged with
	/// that of the parent session's.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) output_presentation: Option<OutputPresentation>,
}

impl Default for CancelParams {
	fn default() -> Self {
		Self::new()
	}
}

impl CancelParams {
	pub fn new() -> Self {
		Self { request_id: None, progress_id: None }
	}

	pub fn with_request_id(self, request_id: u32) -> Self {
		let mut this = self;
		this.request_id = Some(request_id);
		this
	}

	pub fn with_progress_id<T>(self, progress_id: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.progress_id = Some(progress_id.to_string());
		this
	}

	/// The ID (attribute `seq`) of the request to cancel. If missing no request is
	/// cancelled.
	/// Both a `requestId` and a `progressId` can be specified in one request.
	pub fn request_id(&self) -> Option<u32> {
		self.request_id
	}

	/// The ID (attribute `progressId`) of the progress to cancel. If missing no
	/// progress is cancelled.
	/// Both a `requestId` and a `progressId` can be specified in one request.
	pub fn progress_id(&self) -> Option<&String> {
		self.progress_id.as_ref()
	}
}

impl Default for AttachParams {
	fn default() -> Self {
		Self::new()
	}
}

impl AttachParams {
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

	/// Arbitrary data from the previous, restarted session.
	///
	/// The data is sent as the `restart` attribute of the `terminated` event.
	///
	/// The client should leave the data intact.
	pub fn restart(&self) -> Option<&serde_json::Value> {
		self.restart.as_ref()
	}
}

impl BreakpointLocationsParams {
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

	/// The source location of the breakpoints; either `source.path` or `source.sourceReference`
	/// must be specified.
	pub fn source(&self) -> &Source {
		&self.source
	}

	/// Start line of range to search possible breakpoint locations in.
	///
	/// If only the line is specified, the request returns all possible locations in that line.
	pub fn line(&self) -> u64 {
		self.line
	}

	/// Start position within `line` to search possible breakpoint locations in.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based. If no column is given, the first position in the start line
	/// is assumed.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// End line of range to search possible breakpoint locations in.
	///
	/// If no end line is given, then the end line is assumed to be the start line.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// End position within `endLine` to search possible breakpoint locations in.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based. If no end column is given, the last position in the end line
	/// is assumed.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}
}

impl CompletionsParams {
	pub fn new<T>(text: T, column: u64) -> Self
	where
		T: ToString,
	{
		Self {
			frame_id: None,
			text: text.to_string(),
			column,
			line: None,
		}
	}

	pub fn with_frame_id(self, frame_id: i32) -> Self {
		let mut this = self;
		this.frame_id = Some(frame_id);
		this
	}

	pub fn with_line(self, line: u64) -> Self {
		let mut this = self;
		this.line = Some(line);
		this
	}

	/// Returns completions in the scope of this stack frame.
	///
	/// If not specified, the completions are returned for the global scope.
	pub fn frame_id(&self) -> Option<i32> {
		self.frame_id
	}

	/// One or more source lines.
	///
	/// Typically this is the text users have typed into the debug console before they asked for
	/// completion.
	pub fn text(&self) -> &String {
		&self.text
	}

	/// The position within `text` for which to determine the completion proposals.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn column(&self) -> u64 {
		self.column
	}

	/// A line for which to determine the completion proposals.
	///
	/// If missing the first line of the text is assumed.
	pub fn line(&self) -> Option<u64> {
		self.line
	}
}

impl ContinueParams {
	pub fn new(thread_id: i32) -> Self {
		Self { thread_id, single_thread: None }
	}

	pub fn with_single_thread(self, single_thread: bool) -> Self {
		let mut this = self;
		this.single_thread = Some(single_thread);
		this
	}

	/// Specifies the active thread.
	///
	/// If the debug adapter supports single thread execution (see
	/// `supportsSingleThreadExecutionRequests`) and the argument `singleThread` is true, only the
	/// thread with this ID is resumed.
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// If this flag is true, execution is resumed only for the thread with given `threadId`.
	pub fn single_thread(&self) -> Option<bool> {
		self.single_thread
	}
}

impl DataBreakpointInfoParams {
	pub fn new<T>(name: T) -> Self
	where
		T: ToString,
	{
		Self {
			variables_reference: None,
			name:                name.to_string(),
			frame_id:            None,
			bytes:               None,
			as_address:          None,
			mode:                None,
		}
	}

	pub fn with_variables_reference(self, variables_reference: u32) -> Self {
		let mut this = self;
		this.variables_reference = Some(variables_reference);
		this
	}

	pub fn with_frame_id(self, frame_id: i32) -> Self {
		let mut this = self;
		this.frame_id = Some(frame_id);
		this
	}

	pub fn with_bytes(self, bytes: u32) -> Self {
		let mut this = self;
		this.bytes = Some(bytes);
		this
	}

	pub fn with_as_address(self, as_address: bool) -> Self {
		let mut this = self;
		this.as_address = Some(as_address);
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

	/// Reference to the variable container if the data breakpoint is requested for a child of the
	/// container. The `variablesReference` must have been obtained in the current suspended state.
	pub fn variables_reference(&self) -> Option<u32> {
		self.variables_reference
	}

	/// The name of the variable's child to obtain data breakpoint information for.
	///
	/// If `variablesReference` isn't specified, this can be an expression, or an address if
	/// `asAddress` is also true.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// When `name` is an expression, evaluate it in the scope of this stack frame.
	///
	/// If not specified, the expression is evaluated in the global scope. When
	/// `variablesReference` is specified, this property has no effect.
	pub fn frame_id(&self) -> Option<i32> {
		self.frame_id
	}

	/// If specified, a debug adapter should return information for the range of memory extending
	/// `bytes` number of bytes from the address or variable specified by `name`. Breakpoints set
	/// using the resulting data ID should pause on data access anywhere within that
	/// range.\n\nClients may set this property only if the `supportsDataBreakpointBytes`
	/// capability is true.
	pub fn bytes(&self) -> Option<u32> {
		self.bytes
	}

	/// If `true`, the `name` is a memory address and the debugger should interpret it as a decimal
	/// value, or hex value if it is prefixed with `0x`.
	///
	/// Clients may set this property only if the `supportsDataBreakpointBytes` capability is true.
	pub fn as_address(&self) -> Option<bool> {
		self.as_address
	}

	/// The mode of the desired breakpoint.
	///
	/// If defined, this must be one of the `breakpointModes` the debug adapter advertised in its
	/// `Capabilities`.
	pub fn mode(&self) -> Option<&String> {
		self.mode.as_ref()
	}
}

impl DisassembleParams {
	pub fn new<T>(memory_reference: T, instruction_count: u32) -> Self
	where
		T: ToString,
	{
		Self {
			memory_reference: memory_reference.to_string(),
			offset: None,
			instruction_offset: None,
			instruction_count,
			resolve_symbols: None,
		}
	}

	pub fn with_offset(self, offset: i64) -> Self {
		let mut this = self;
		this.offset = Some(offset);
		this
	}

	pub fn with_instruction_offset(self, instruction_offset: i64) -> Self {
		let mut this = self;
		this.instruction_offset = Some(instruction_offset);
		this
	}

	pub fn with_resolve_symbols(self, resolve_symbols: bool) -> Self {
		let mut this = self;
		this.resolve_symbols = Some(resolve_symbols);
		this
	}

	/// Memory reference to the base location containing the instructions to disassemble.
	pub fn memory_reference(&self) -> &String {
		&self.memory_reference
	}

	/// Offset (in bytes) to be applied to the reference location before disassembling. Can be
	/// negative.
	pub fn offset(&self) -> Option<i64> {
		self.offset
	}

	/// Offset (in instructions) to be applied after the byte offset (if any) before disassembling.
	/// Can be negative.
	pub fn instruction_offset(&self) -> Option<i64> {
		self.instruction_offset
	}

	/// Number of instructions to disassemble starting at the specified location and offset.
	///
	/// An adapter must return exactly this number of instructions - any unavailable instructions
	/// should be replaced with an implementation-defined 'invalid instruction' value.
	pub fn instruction_count(&self) -> u32 {
		self.instruction_count
	}

	/// If true, the adapter should attempt to resolve memory addresses and other values to
	/// symbolic names.
	pub fn resolve_symbols(&self) -> Option<bool> {
		self.resolve_symbols
	}
}

impl Default for DisconnectParams {
	fn default() -> Self {
		Self::new()
	}
}

impl DisconnectParams {
	pub fn new() -> Self {
		Self {
			restart:            None,
			terminate_debuggee: None,
			suspend_debuggee:   None,
		}
	}

	pub fn with_restart(self, restart: bool) -> Self {
		let mut this = self;
		this.restart = Some(restart);
		this
	}

	pub fn with_terminate_debuggee(self, terminate_debuggee: bool) -> Self {
		let mut this = self;
		this.terminate_debuggee = Some(terminate_debuggee);
		this
	}

	pub fn with_suspend_debuggee(self, suspend_debuggee: bool) -> Self {
		let mut this = self;
		this.suspend_debuggee = Some(suspend_debuggee);
		this
	}

	/// A value of true indicates that this `disconnect` request is part of a restart sequence.
	pub fn restart(&self) -> Option<bool> {
		self.restart
	}

	/// Indicates whether the debuggee should be terminated when the debugger is disconnected.
	///
	/// If unspecified, the debug adapter is free to do whatever it thinks is best.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportTerminateDebuggee` is true.
	pub fn terminate_debuggee(&self) -> Option<bool> {
		self.terminate_debuggee
	}

	/// Indicates whether the debuggee should stay suspended when the debugger is disconnected.
	///
	/// If unspecified, the debuggee should resume execution.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportSuspendDebuggee` is true.
	pub fn suspend_debuggee(&self) -> Option<bool> {
		self.suspend_debuggee
	}
}

impl EvaluateParams {
	pub fn new<T>(expression: T) -> Self
	where
		T: ToString,
	{
		Self {
			expression: expression.to_string(),
			frame_id:   None,
			line:       None,
			column:     None,
			source:     None,
			context:    None,
			format:     None,
		}
	}

	pub fn with_frame_id(self, frame_id: i32) -> Self {
		let mut this = self;
		this.frame_id = Some(frame_id);
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

	pub fn with_source(self, source: Box<Source>) -> Self {
		let mut this = self;
		this.source = Some(source);
		this
	}

	pub fn with_context(self, context: EvalContext) -> Self {
		let mut this = self;
		this.context = Some(context);
		this
	}

	pub fn with_format(self, format: ValueFormat) -> Self {
		let mut this = self;
		this.format = Some(format);
		this
	}

	/// The expression to evaluate.
	pub fn expression(&self) -> &String {
		&self.expression
	}

	/// Evaluate the expression in the scope of this stack frame. If not specified, the expression
	/// is evaluated in the global scope
	pub fn frame_id(&self) -> Option<i32> {
		self.frame_id
	}

	/// The contextual line where the expression should be evaluated. In the 'hover' context, this
	/// should be set to the start of the expression being hovered
	pub fn line(&self) -> Option<u64> {
		self.line
	}

	/// The contextual column where the expression should be evaluated. This may be provided if
	/// `line` is also provided.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The contextual source in which the `line` is found. This must be provided if `line` is
	/// provided
	pub fn source(&self) -> Option<&Source> {
		self.source.as_deref()
	}

	/// The context in which the evaluate request is used
	pub fn context(&self) -> Option<&EvalContext> {
		self.context.as_ref()
	}

	/// Specifies details on how to format the result.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsValueFormattingOptions` is true.
	pub fn format(&self) -> Option<&ValueFormat> {
		self.format.as_ref()
	}
}

impl ExceptionInfoParams {
	pub fn new(thread_id: i32) -> Self {
		Self { thread_id }
	}

	/// Thread for which exception information should be retrieved.
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}
}

impl GotoParams {
	pub fn new(thread_id: i32, target_id: i32) -> Self {
		Self { thread_id, target_id }
	}

	/// Set the goto target for this thread.
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// The location where the debuggee will continue to run.
	pub fn target_id(&self) -> i32 {
		self.target_id
	}
}

impl GotoTargetsParams {
	pub fn new(source: Source, line: u64) -> Self {
		Self { source, line, column: None }
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	/// The source location for which the goto targets are determined.
	pub fn source(&self) -> &Source {
		&self.source
	}

	/// The line location for which the goto targets are determined.
	pub fn line(&self) -> u64 {
		self.line
	}

	/// The position within `line` for which the goto targets are determined.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn column(&self) -> Option<u64> {
		self.column
	}
}

impl InitializeParams {
	pub fn new<T>(adapter_id: T) -> Self
	where
		T: ToString,
	{
		Self {
			client_id: None,
			client_name: None,
			adapter_id: adapter_id.to_string(),
			locale: None,
			lines_start_at_1: None,
			columns_start_at_1: None,
			path_format: None,
			supports_variable_type: None,
			supports_variable_paging: None,
			supports_run_in_terminal_request: None,
			supports_memory_references: None,
			supports_progress_reporting: None,
			supports_invalidated_event: None,
			supports_memory_event: None,
			supports_args_can_be_interpreted_by_shell: None,
			supports_start_debugging_request: None,
			supports_ansi_styling: None,
		}
	}

	pub fn with_client_id<T>(self, client_id: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.client_id = Some(client_id.to_string());
		this
	}

	pub fn with_client_name<T>(self, client_name: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.client_name = Some(client_name.to_string());
		this
	}

	pub fn with_locale<T>(self, locale: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.locale = Some(locale.to_string());
		this
	}

	pub fn with_lines_start_at_1(self, lines_start_at_1: bool) -> Self {
		let mut this = self;
		this.lines_start_at_1 = Some(lines_start_at_1);
		this
	}

	pub fn with_columns_start_at_1(self, columns_start_at_1: bool) -> Self {
		let mut this = self;
		this.columns_start_at_1 = Some(columns_start_at_1);
		this
	}

	pub fn with_path_format(self, path_format: PathFormat) -> Self {
		let mut this = self;
		this.path_format = Some(path_format);
		this
	}

	pub fn with_supports_variable_type(self, supports_variable_type: bool) -> Self {
		let mut this = self;
		this.supports_variable_type = Some(supports_variable_type);
		this
	}

	pub fn with_supports_variable_paging(self, supports_variable_paging: bool) -> Self {
		let mut this = self;
		this.supports_variable_paging = Some(supports_variable_paging);
		this
	}

	pub fn with_supports_run_in_terminal_request(
		self,
		supports_run_in_terminal_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_run_in_terminal_request = Some(supports_run_in_terminal_request);
		this
	}

	pub fn with_supports_memory_references(self, supports_memory_references: bool) -> Self {
		let mut this = self;
		this.supports_memory_references = Some(supports_memory_references);
		this
	}

	pub fn with_supports_progress_reporting(self, supports_progress_reporting: bool) -> Self {
		let mut this = self;
		this.supports_progress_reporting = Some(supports_progress_reporting);
		this
	}

	pub fn with_supports_invalidated_event(self, supports_invalidated_event: bool) -> Self {
		let mut this = self;
		this.supports_invalidated_event = Some(supports_invalidated_event);
		this
	}

	pub fn with_supports_memory_event(self, supports_memory_event: bool) -> Self {
		let mut this = self;
		this.supports_memory_event = Some(supports_memory_event);
		this
	}

	pub fn with_supports_args_can_be_interpreted_by_shell(
		self,
		supports_args_can_be_interpreted_by_shell: bool,
	) -> Self {
		let mut this = self;
		this.supports_args_can_be_interpreted_by_shell =
			Some(supports_args_can_be_interpreted_by_shell);
		this
	}

	pub fn with_supports_start_debugging_request(
		self,
		supports_start_debugging_request: bool,
	) -> Self {
		let mut this = self;
		this.supports_start_debugging_request = Some(supports_start_debugging_request);
		this
	}

	pub fn with_supports_ansi_styling(self, supports_ansi_styling: bool) -> Self {
		let mut this = self;
		this.supports_ansi_styling = Some(supports_ansi_styling);
		this
	}

	/// The ID of the client using this adapter.
	pub fn client_id(&self) -> Option<&String> {
		self.client_id.as_ref()
	}

	/// The human-readable name of the client using this adapter.
	pub fn client_name(&self) -> Option<&String> {
		self.client_name.as_ref()
	}

	/// The ID of the debug adapter.
	pub fn adapter_id(&self) -> &String {
		&self.adapter_id
	}

	/// The ISO-639 locale of the client using this adapter, e.g. en-US or de-CH.
	pub fn locale(&self) -> Option<&String> {
		self.locale.as_ref()
	}

	/// If true all line numbers are 1-based (default).
	pub fn lines_start_at_1(&self) -> Option<bool> {
		self.lines_start_at_1
	}

	/// If true all column numbers are 1-based (default).
	pub fn columns_start_at_1(&self) -> Option<bool> {
		self.columns_start_at_1
	}

	/// Determines in what format paths are specified. The default is `path`, which is the native
	/// format.
	pub fn path_format(&self) -> Option<&PathFormat> {
		self.path_format.as_ref()
	}

	/// Client supports the `type` attribute for variables.
	pub fn supports_variable_type(&self) -> Option<bool> {
		self.supports_variable_type
	}

	/// Client supports the paging of variables.
	pub fn supports_variable_paging(&self) -> Option<bool> {
		self.supports_variable_paging
	}

	/// Client supports the `runInTerminal` request.
	pub fn supports_run_in_terminal_request(&self) -> Option<bool> {
		self.supports_run_in_terminal_request
	}

	/// Client supports memory references.
	pub fn supports_memory_references(&self) -> Option<bool> {
		self.supports_memory_references
	}

	/// Client supports progress reporting.
	pub fn supports_progress_reporting(&self) -> Option<bool> {
		self.supports_progress_reporting
	}

	/// Client supports the `invalidated` event.
	pub fn supports_invalidated_event(&self) -> Option<bool> {
		self.supports_invalidated_event
	}

	/// Client supports the `memory` event.
	pub fn supports_memory_event(&self) -> Option<bool> {
		self.supports_memory_event
	}

	/// Client supports the `argsCanBeInterpretedByShell` attribute on the `runInTerminal` request.
	pub fn supports_args_can_be_interpreted_by_shell(&self) -> Option<bool> {
		self.supports_args_can_be_interpreted_by_shell
	}

	/// Client supports the `startDebugging` request.
	pub fn supports_start_debugging_request(&self) -> Option<bool> {
		self.supports_start_debugging_request
	}

	/// The client will interpret ANSI escape sequences in the display of `OutputEvent.output` and
	/// `Variable.value` fields when `Capabilities.supportsANSIStyling` is also enabled.
	pub fn supports_ansi_styling(&self) -> Option<bool> {
		self.supports_ansi_styling
	}
}

impl Default for LaunchParams {
	fn default() -> Self {
		Self::new()
	}
}

impl LaunchParams {
	pub fn new() -> Self {
		Self { no_debug: None, restart: None }
	}

	pub fn with_no_debug(self, no_debug: bool) -> Self {
		let mut this = self;
		this.no_debug = Some(no_debug);
		this
	}

	pub fn with_restart<T>(self, restart: T) -> eyre::Result<Self>
	where
		T: serde::Serialize,
	{
		let mut this = self;
		this.restart = Some(serde_json::to_value(restart)?);
		Ok(this)
	}

	/// If true, the launch request should launch the program without enabling debugging.
	pub fn no_debug(&self) -> Option<bool> {
		self.no_debug
	}

	/// Arbitrary data from the previous, restarted session.
	///
	/// The data is sent as the `restart` attribute of the `terminated` event.
	///
	/// The client should leave the data intact.
	pub fn restart(&self) -> Option<&serde_json::Value> {
		self.restart.as_ref()
	}
}

impl LocationsParams {
	pub fn new(location_reference: i32) -> Self {
		Self { location_reference }
	}

	/// Location reference to resolve.
	pub fn location_reference(&self) -> i32 {
		self.location_reference
	}
}

impl Default for ModuleParams {
	fn default() -> Self {
		Self::new()
	}
}

impl ModuleParams {
	pub fn new() -> Self {
		Self { start_module: None, module_count: None }
	}

	pub fn with_start_module(self, start_module: i32) -> Self {
		let mut this = self;
		this.start_module = Some(start_module);
		this
	}

	pub fn with_module_count(self, module_count: u32) -> Self {
		let mut this = self;
		this.module_count = Some(module_count);
		this
	}

	/// The index of the first module to return; if omitted modules start at 0.
	pub fn start_module(&self) -> Option<i32> {
		self.start_module
	}

	/// The number of modules to return. If `moduleCount` is not specified or 0, all modules are
	/// returned.
	pub fn module_count(&self) -> Option<u32> {
		self.module_count
	}
}

impl NextParams {
	pub fn new(thread_id: i32) -> Self {
		Self { thread_id, single_thread: None, granularity: None }
	}

	pub fn with_single_thread(self, single_thread: bool) -> Self {
		let mut this = self;
		this.single_thread = Some(single_thread);
		this
	}

	pub fn with_granularity(self, granularity: SteppingGranularity) -> Self {
		let mut this = self;
		this.granularity = Some(granularity);
		this
	}

	/// Specifies the thread for which to resume execution for one step (of the given granularity).
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// If this flag is true, all other suspended threads are not resumed.
	pub fn single_thread(&self) -> Option<bool> {
		self.single_thread
	}

	/// Stepping granularity. If no granularity is specified, a granularity of `statement` is
	/// assumed.
	pub fn granularity(&self) -> Option<SteppingGranularity> {
		self.granularity
	}
}

impl PauseParams {
	pub fn new(thread_id: i32) -> Self {
		Self { thread_id }
	}

	/// Pause execution for this thread.
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}
}

impl ReadMemoryParams {
	pub fn new<T>(memory_reference: T, count: u64) -> Self
	where
		T: ToString,
	{
		Self {
			memory_reference: memory_reference.to_string(),
			offset: None,
			count,
		}
	}

	pub fn with_offset(self, offset: i64) -> Self {
		let mut this = self;
		this.offset = Some(offset);
		this
	}

	/// Memory reference to the base location from which data should be read.
	pub fn memory_reference(&self) -> &String {
		&self.memory_reference
	}

	/// Offset (in bytes) to be applied to the reference location before reading data. Can be
	/// negative.
	pub fn offset(&self) -> Option<i64> {
		self.offset
	}

	/// Number of bytes to read at the specified location and offset.
	pub fn count(&self) -> u64 {
		self.count
	}
}

impl RestartFrameParams {
	pub fn new(frame_id: i32) -> Self {
		Self { frame_id }
	}

	/// Restart the stack frame identified by `frameId`.
	///
	/// The `frameId` must have been obtained in the current suspended state.
	pub fn frame_id(&self) -> i32 {
		self.frame_id
	}
}

impl ReverseContinueParams {
	pub fn new(thread_id: i32) -> Self {
		Self { thread_id, single_thread: None }
	}

	pub fn with_single_thread(self, single_thread: bool) -> Self {
		let mut this = self;
		this.single_thread = Some(single_thread);
		this
	}

	/// Specifies the active thread. If the debug adapter supports single thread execution (see
	/// `supportsSingleThreadExecutionRequests`) and the `singleThread` argument is true, only the
	/// thread with this ID is resumed.
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// If this flag is true, backward execution is resumed only for the thread with given
	/// `threadId`.
	pub fn single_thread(&self) -> Option<bool> {
		self.single_thread
	}
}

impl ScopesParams {
	pub fn new(frame_id: i32) -> Self {
		Self { frame_id }
	}

	/// Retrieve the scopes for the stack frame identified by `frameId`.
	///
	/// The `frameId` must have been obtained in the current suspended state.
	pub fn frame_id(&self) -> i32 {
		self.frame_id
	}
}

impl SetBreakpointsParams {
	pub fn new(source: Source) -> Self {
		Self {
			source,
			breakpoints: None,
			lines: None,
			source_modified: None,
		}
	}

	pub fn with_breakpoints(self, breakpoints: Vec<SourceBreakpoint>) -> Self {
		let mut this = self;
		this.breakpoints = Some(breakpoints);
		this
	}

	pub fn with_source_modified(self, source_modified: bool) -> Self {
		let mut this = self;
		this.source_modified = Some(source_modified);
		this
	}

	/// The source location of the breakpoints; either `source.path` or `source.sourceReference`
	/// must be specified.
	pub fn source(&self) -> &Source {
		&self.source
	}

	/// The code locations of the breakpoints.
	pub fn breakpoints(&self) -> Option<&Vec<SourceBreakpoint>> {
		self.breakpoints.as_ref()
	}

	/// The code locations of the breakpoints.
	#[deprecated]
	pub fn lines(&self) -> Option<&Vec<u64>> {
		self.lines.as_ref()
	}

	/// A value of true indicates that the underlying source has been modified which results in new
	/// breakpoint locations.
	pub fn source_modified(&self) -> Option<bool> {
		self.source_modified
	}
}

impl SetDataBreakpointsParams {
	pub fn new(breakpoints: Vec<DataBreakpoint>) -> Self {
		Self { breakpoints }
	}

	/// The contents of this array replaces all existing data breakpoints. An empty array clears
	/// all data breakpoints.
	pub fn breakpoints(&self) -> &Vec<DataBreakpoint> {
		&self.breakpoints
	}
}

impl SetExceptionBreakpointsParams {
	pub fn new(filters: Vec<String>) -> Self {
		Self {
			filters,
			filter_options: None,
			exception_options: None,
		}
	}

	pub fn with_filter_options(self, filter_options: Vec<ExceptionFilterOptions>) -> Self {
		let mut this = self;
		this.filter_options = Some(filter_options);
		this
	}

	pub fn with_exception_options(self, exception_options: Vec<ExceptionOptions>) -> Self {
		let mut this = self;
		this.exception_options = Some(exception_options);
		this
	}

	/// Set of exception filters specified by their ID.
	///
	/// The set of all possible exception filters is defined by the `exceptionBreakpointFilters`
	/// capability. The `filter` and `filterOptions` sets are additive.
	pub fn filters(&self) -> &Vec<String> {
		&self.filters
	}

	/// Set of exception filters and their options.
	///
	/// The set of all possible exception filters is defined by the `exceptionBreakpointFilters`
	/// capability.
	///
	/// This attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsExceptionFilterOptions` is true. The `filter` and `filterOptions` sets
	/// are additive.
	pub fn filter_options(&self) -> Option<&Vec<ExceptionFilterOptions>> {
		self.filter_options.as_ref()
	}

	/// Configuration options for selected exceptions.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsExceptionOptions` is true.
	pub fn exception_options(&self) -> Option<&Vec<ExceptionOptions>> {
		self.exception_options.as_ref()
	}
}

impl SetExpressionParams {
	pub fn new<T, U>(expression: T, value: U) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			expression: expression.to_string(),
			value:      value.to_string(),
			frame_id:   None,
			format:     None,
		}
	}

	pub fn with_frame_id(self, frame_id: i32) -> Self {
		let mut this = self;
		this.frame_id = Some(frame_id);
		this
	}

	pub fn with_format(self, format: ValueFormat) -> Self {
		let mut this = self;
		this.format = Some(format);
		this
	}

	/// The l-value expression to assign to.
	pub fn expression(&self) -> &String {
		&self.expression
	}

	/// The value expression to assign to the l-value expression.
	pub fn value(&self) -> &String {
		&self.value
	}

	/// Evaluate the expressions in the scope of this stack frame.
	///
	/// If not specified, the expressions are evaluated in the global scope.
	pub fn frame_id(&self) -> Option<i32> {
		self.frame_id
	}

	/// Specifies how the resulting value should be formatted.
	pub fn format(&self) -> Option<&ValueFormat> {
		self.format.as_ref()
	}
}

impl SetFunctionBreakpointsParams {
	pub fn new(breakpoints: Vec<FunctionBreakpoint>) -> Self {
		Self { breakpoints }
	}

	/// The function names of the breakpoints.
	pub fn breakpoints(&self) -> &Vec<FunctionBreakpoint> {
		&self.breakpoints
	}
}

impl SetInstructionBreakpointsParams {
	pub fn new(breakpoints: Vec<InstructionBreakpoint>) -> Self {
		Self { breakpoints }
	}

	/// The instruction references of the breakpoints
	pub fn breakpoints(&self) -> &Vec<InstructionBreakpoint> {
		&self.breakpoints
	}
}

impl SetVariableParams {
	pub fn new<T, U>(variables_reference: i32, name: T, value: U) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			variables_reference,
			name: name.to_string(),
			value: value.to_string(),
			format: None,
		}
	}

	pub fn with_format(self, format: ValueFormat) -> Self {
		let mut this = self;
		this.format = Some(format);
		this
	}

	/// The reference of the variable container.
	///
	/// The `variablesReference` must have been obtained in the current suspended state.
	pub fn variables_reference(&self) -> i32 {
		self.variables_reference
	}

	/// The name of the variable in the container.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The value of the variable.
	pub fn value(&self) -> &String {
		&self.value
	}

	/// Specifies details on how to format the response value.
	pub fn format(&self) -> Option<&ValueFormat> {
		self.format.as_ref()
	}
}

impl SourceParams {
	pub fn new(source_reference: u32) -> Self {
		Self { source: None, source_reference }
	}

	pub fn with_source(self, source: Source) -> Self {
		let mut this = self;
		this.source = Some(source);
		this
	}

	/// Specifies the source content to load. Either `source.path` or `source.sourceReference` must
	/// be specified.
	pub fn source(&self) -> Option<&Source> {
		self.source.as_ref()
	}

	/// The reference to the source. This is the same as `source.sourceReference`.
	///
	/// This is provided for backward compatibility since old clients do not understand the
	/// `source` attribute.
	pub fn source_reference(&self) -> u32 {
		self.source_reference
	}
}

impl StackTraceParams {
	pub fn new(thread_id: i32) -> Self {
		Self {
			thread_id,
			start_frame: None,
			levels: None,
			format: None,
		}
	}

	pub fn with_start_frame(self, start_frame: u32) -> Self {
		let mut this = self;
		this.start_frame = Some(start_frame);
		this
	}

	pub fn with_levels(self, levels: u32) -> Self {
		let mut this = self;
		this.levels = Some(levels);
		this
	}

	pub fn with_format(self, format: StackFrameFormat) -> Self {
		let mut this = self;
		this.format = Some(format);
		this
	}

	/// Retrieve the stacktrace for this thread.
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// The index of the first frame to return; if omitted frames start at 0.
	pub fn start_frame(&self) -> Option<u32> {
		self.start_frame
	}

	/// The maximum number of frames to return. If levels is not specified or 0, all frames are
	/// returned.
	pub fn levels(&self) -> Option<u32> {
		self.levels
	}

	/// Specifies details on how to format the returned `StackFrame.name`.
	///
	/// The debug adapter may format requested details in any way that would make sense to a
	/// developer.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsValueFormattingOptions` is true.
	pub fn format(&self) -> Option<&StackFrameFormat> {
		self.format.as_ref()
	}
}

impl StepBackParams {
	pub fn new(thread_id: i32) -> Self {
		Self { thread_id, single_thread: None, granularity: None }
	}

	pub fn with_single_thread(self, single_thread: bool) -> Self {
		let mut this = self;
		this.single_thread = Some(single_thread);
		this
	}

	pub fn with_granularity(self, granularity: SteppingGranularity) -> Self {
		let mut this = self;
		this.granularity = Some(granularity);
		this
	}

	/// Specifies the thread for which to resume execution for one step backwards (of the given
	/// granularity).
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// If this flag is true, all other suspended threads are not resumed.
	pub fn single_thread(&self) -> Option<bool> {
		self.single_thread
	}

	/// Stepping granularity to step. If no granularity is specified, a granularity of `statement`
	/// is assumed.
	pub fn granularity(&self) -> Option<SteppingGranularity> {
		self.granularity
	}
}

impl StepInParams {
	pub fn new(thread_id: i32) -> Self {
		Self {
			thread_id,
			single_thread: None,
			target_id: None,
			granularity: None,
		}
	}

	pub fn with_single_thread(self, single_thread: bool) -> Self {
		let mut this = self;
		this.single_thread = Some(single_thread);
		this
	}

	pub fn with_target_id(self, target_id: i32) -> Self {
		let mut this = self;
		this.target_id = Some(target_id);
		this
	}

	pub fn with_granularity(self, granularity: SteppingGranularity) -> Self {
		let mut this = self;
		this.granularity = Some(granularity);
		this
	}

	/// Specifies the thread for which to resume execution for one step-into (of the given
	/// granularity).
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// If this flag is true, all other suspended threads are not resumed.
	pub fn single_thread(&self) -> Option<bool> {
		self.single_thread
	}

	/// Id of the target to step into.
	pub fn target_id(&self) -> Option<i32> {
		self.target_id
	}

	/// Stepping granularity. If no granularity is specified, a granularity of `statement` is
	/// assumed.
	pub fn granularity(&self) -> Option<SteppingGranularity> {
		self.granularity
	}
}

impl StepInTargetsParams {
	pub fn new(frame_id: i32) -> Self {
		Self { frame_id }
	}

	/// The stack frame for which to retrieve the possible step-in targets.
	pub fn frame_id(&self) -> i32 {
		self.frame_id
	}
}

impl StepOutParams {
	pub fn new(thread_id: i32) -> Self {
		Self { thread_id, single_thread: None, granularity: None }
	}

	pub fn with_single_thread(self, single_thread: bool) -> Self {
		let mut this = self;
		this.single_thread = Some(single_thread);
		this
	}

	pub fn with_granularity(self, granularity: SteppingGranularity) -> Self {
		let mut this = self;
		this.granularity = Some(granularity);
		this
	}

	/// Specifies the thread for which to resume execution for one step-out (of the given
	/// granularity).
	pub fn thread_id(&self) -> i32 {
		self.thread_id
	}

	/// If this flag is true, all other suspended threads are not resumed.
	pub fn single_thread(&self) -> Option<bool> {
		self.single_thread
	}

	/// Stepping granularity. If no granularity is specified, a granularity of `statement` is
	/// assumed.
	pub fn granularity(&self) -> Option<SteppingGranularity> {
		self.granularity
	}
}

impl Default for TerminateParams {
	fn default() -> Self {
		Self::new()
	}
}

impl TerminateParams {
	pub fn new() -> Self {
		Self { restart: None }
	}

	pub fn with_restart(self, restart: bool) -> Self {
		let mut this = self;
		this.restart = Some(restart);
		this
	}

	/// A value of true indicates that this `terminate` request is part of a restart sequence.
	pub fn restart(&self) -> Option<bool> {
		self.restart
	}
}

impl Default for TerminateThreadsParams {
	fn default() -> Self {
		Self::new()
	}
}

impl TerminateThreadsParams {
	pub fn new() -> Self {
		Self { thread_ids: None }
	}

	pub fn wiht_thread_ids(self, thread_ids: Vec<i32>) -> Self {
		let mut this = self;
		this.thread_ids = Some(thread_ids);
		this
	}

	/// Ids of threads to be terminated.
	pub fn thread_ids(&self) -> Option<&Vec<i32>> {
		self.thread_ids.as_ref()
	}
}

impl VariablesParams {
	pub fn new(variables_reference: u32) -> Self {
		Self {
			variables_reference,
			filter: None,
			start: None,
			count: None,
			format: None,
		}
	}

	pub fn with_filter(self, filter: VariablesFilter) -> Self {
		let mut this = self;
		this.filter = Some(filter);
		this
	}

	pub fn with_start(self, start: u32) -> Self {
		let mut this = self;
		this.start = Some(start);
		this
	}

	pub fn with_count(self, count: u32) -> Self {
		let mut this = self;
		this.count = Some(count);
		this
	}

	pub fn with_format(self, format: ValueFormat) -> Self {
		let mut this = self;
		this.format = Some(format);
		this
	}

	/// The variable for which to retrieve its children. The `variablesReference` must have been
	/// obtained in the current suspended state.
	pub fn variables_reference(&self) -> u32 {
		self.variables_reference
	}

	/// Filter to limit the child variables to either named or indexed. If omitted, both types are
	/// fetched.
	pub fn filter(&self) -> Option<VariablesFilter> {
		self.filter
	}

	/// The index of the first variable to return; if omitted children start at 0.
	///
	/// If the value of `start` exceeds the number of available variables, the debug adapter should
	/// return an empty array.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsVariablePaging` is true.
	pub fn start(&self) -> Option<u32> {
		self.start
	}

	/// The number of variables to return. If count is missing or 0, all variables are returned.
	///
	/// If fewer than `count` variables are returned, the client should assume no further variables
	/// are available.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsVariablePaging` is true.
	pub fn count(&self) -> Option<u32> {
		self.count
	}

	/// Specifies details on how to format the Variable values.
	///
	/// The attribute is only honored by a debug adapter if the corresponding capability
	/// `supportsValueFormattingOptions` is true.
	pub fn format(&self) -> Option<&ValueFormat> {
		self.format.as_ref()
	}
}

impl WriteMemoryParams {
	pub fn new<T, U>(memory_reference: T, data: U) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			memory_reference: memory_reference.to_string(),
			offset:           None,
			allow_partial:    None,
			data:             data.to_string(),
		}
	}

	pub fn with_offset(self, offset: i64) -> Self {
		let mut this = self;
		this.offset = Some(offset);
		this
	}

	pub fn with_allow_partial(self, allow_partial: bool) -> Self {
		let mut this = self;
		this.allow_partial = Some(allow_partial);
		this
	}

	/// Memory reference to the base location to which data should be written.
	pub fn memory_reference(&self) -> &String {
		&self.memory_reference
	}

	/// Offset (in bytes) to be applied to the reference location before writing data. Can be
	/// negative.
	pub fn offset(&self) -> Option<i64> {
		self.offset
	}

	/// Property to control partial writes.
	///
	/// If true, the debug adapter should attempt to write memory even if the entire memory region
	/// is not writable.
	///
	/// In such a case the debug adapter should stop after hitting the first byte of memory that
	/// cannot be written and return the number of bytes written in the response via the `offset`
	/// and `bytesWritten` properties.
	///
	/// If false or missing, a debug adapter should attempt to verify the region is writable before
	/// writing, and fail the response if it is not.
	pub fn allow_partial(&self) -> Option<bool> {
		self.allow_partial
	}

	/// Bytes to write, encoded using base64.
	pub fn data(&self) -> &String {
		&self.data
	}
}

impl RunInTerminalParams {
	pub fn new<T>(cwd: T, args: Vec<String>) -> Self
	where
		T: ToString,
	{
		Self {
			kind: None,
			title: None,
			cwd: cwd.to_string(),
			args,
			env: None,
			args_can_be_interpreted_by_shell: None,
		}
	}

	pub fn with_kind(self, kind: TerminalKind) -> Self {
		let mut this = self;
		this.kind = Some(kind);
		this
	}

	pub fn with_title(self, title: String) -> Self {
		let mut this = self;
		this.title = Some(title);
		this
	}

	pub fn with_env(self, env: HashMap<String, Option<String>>) -> Self {
		let mut this = self;
		this.env = Some(env);
		this
	}

	pub fn with_args_can_be_interpreted_by_shell(
		self,
		args_can_be_interpreted_by_shell: bool,
	) -> Self {
		let mut this = self;
		this.args_can_be_interpreted_by_shell = Some(args_can_be_interpreted_by_shell);
		this
	}

	/// What kind of terminal to launch. Defaults to `integrated` if not specified.
	pub fn kind(&self) -> Option<TerminalKind> {
		self.kind
	}

	/// Title of the terminal.
	pub fn title(&self) -> Option<&String> {
		self.title.as_ref()
	}

	/// Working directory for the command.
	///
	/// For non-empty, valid paths this typically results in execution of a change directory
	/// command.
	///
	/// If `pathFormat` is set to `uri` in the `InitializeRequestArguments`, this must be a file
	/// URI.
	pub fn cwd(&self) -> &String {
		&self.cwd
	}

	/// List of arguments. The first argument is the command to run.
	pub fn args(&self) -> &Vec<String> {
		&self.args
	}

	/// Environment key-value pairs that are added to or removed from the default environment.
	pub fn env(&self) -> Option<&HashMap<String, Option<String>>> {
		self.env.as_ref()
	}

	/// This property should only be set if the corresponding capability
	/// `supportsArgsCanBeInterpretedByShell` is true.
	///
	/// If the client uses an intermediary shell to launch the application, then the client must
	/// not attempt to escape characters with special meanings for the shell.
	///
	/// The user is fully responsible for escaping as needed and that arguments using special
	/// characters may not be portable across shells.
	pub fn args_can_be_interpreted_by_shell(&self) -> Option<bool> {
		self.args_can_be_interpreted_by_shell
	}
}

impl StartDebuggingParams {
	pub fn new(
		configuration: HashMap<String, serde_json::Value>,
		request: StartDebuggingRequest,
	) -> Self {
		Self { configuration, request, output_presentation: None }
	}

	pub fn with_output_presentation(self, output_presentation: OutputPresentation) -> Self {
		let mut this = self;
		this.output_presentation = Some(output_presentation);
		this
	}

	/// Arguments passed to the new debug session.
	///
	/// The arguments must only contain properties understood by the `launch` or `attach` requests
	/// of the debug adapter and they must not contain any client-specific properties (e.g.
	/// `type`) or client-specific features (e.g. substitutable 'variables')
	pub fn configuration(&self) -> &HashMap<String, serde_json::Value> {
		&self.configuration
	}

	/// Indicates whether the new debug session should be started with a `launch` or `attach`
	/// request.
	pub fn request(&self) -> StartDebuggingRequest {
		self.request
	}

	/// Hints whether output of the child sessions should be presented separately or merged with
	/// that of the parent session's.
	pub fn output_presentation(&self) -> Option<OutputPresentation> {
		self.output_presentation
	}
}
