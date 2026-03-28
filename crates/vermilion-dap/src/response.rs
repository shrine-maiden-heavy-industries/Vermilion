// SPDX-License-Identifier: BSD-3-Clause

use crate::types::params::response::{
	BreakpointLocationsParams, CompletionsParams, ContinueParams, DataBreakpointInfoParams,
	DisassembleParams, EvaluateParams, ExceptionInfoParams, GotoTargetsParams, InitializeParams,
	LoadedSourcesParams, LocationsParams, ModuleParams, ReadMemoryParams, RunInTerminalParams,
	ScopesParams, SetBreakpointsParams, SetDataBreakpointsParams, SetExceptionBreakpointsParams,
	SetExpressionParams, SetFunctionBreakpointsParams, SetInstructionBreakpointsParams,
	SetVariableParams, SourceParams, StackTraceParams, StepInTargetsParams, ThreadsParams,
	VariablesParams, WriteMemoryParams,
};

/// A response from a DAP [`Request`]
///
/// [`Request`]: crate::request::Request
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Response {
	#[serde(rename = "seq")]
	pub(crate) id:       u32,
	#[serde(rename = "request_seq")]
	pub(crate) req_id:   u32,
	pub(crate) success:  bool,
	#[serde(flatten)]
	pub(crate) response: ResponseType,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message:  Option<ResponseMessage>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "command", content = "body", rename_all = "camelCase")]
pub enum ResponseType {
	/// Response to `cancel` request. This is just an acknowledgement, so no body field is required.
	Cancel,
	/// Response to `attach` request. This is just an acknowledgement, so no body field is required.
	Attach,
	/// Response to `breakpointLocations` request.
	///
	/// Contains possible locations for source breakpoints.
	BreakpointLocations(BreakpointLocationsParams),
	/// Response to `completions` request.
	Completions(CompletionsParams),
	/// Response to `configurationDone` request. This is just an acknowledgement, so no body field
	/// is required.
	ConfigurationDone,
	/// Response to `continue` request.
	Continue(ContinueParams),
	/// Response to `dataBreakpointInfo` request.
	DataBreakpointInfo(DataBreakpointInfoParams),
	/// Response to `disassemble` request.
	Disassemble(DisassembleParams),
	/// Response to `disconnect` request. This is just an acknowledgement, so no body field is
	/// required.
	Disconnect,
	/// Response to `evaluate` request.
	Evaluate(EvaluateParams),
	/// Response to `exceptionInfo` request.
	ExceptionInfo(ExceptionInfoParams),
	/// Response to `goto` request. This is just an acknowledgement, so no body field is required.
	Goto,
	/// Response to `gotoTargets` request.
	GotoTargets(GotoTargetsParams),
	/// Response to `initialize` request.
	Initialize(InitializeParams),
	/// Response to `launch` request. This is just an acknowledgement, so no body field is required.
	Launch,
	/// Response to `loadedSources` request.
	LoadedSources(LoadedSourcesParams),
	/// Response to `locations` request.
	Locations(LocationsParams),
	/// Response to `modules` request.
	Module(ModuleParams),
	/// Response to `next` request. This is just an acknowledgement, so no body field is required.
	Next,
	/// Response to `pause` request. This is just an acknowledgement, so no body field is required.
	Pause,
	/// Response to `readMemory` request.
	ReadMemory(ReadMemoryParams),
	/// Response to `restart` request. This is just an acknowledgement, so no body field is
	/// required.
	Restart,
	/// Response to `restartFrame` request. This is just an acknowledgement, so no body field is
	/// required.
	RestartFrame,
	/// Response to `reverseContinue` request. This is just an acknowledgement, so no body field is
	/// required.
	ReverseContinue,
	/// Response to `scopes` request.
	Scopes(ScopesParams),
	/// Response to `setBreakpoints` request.
	///
	/// Returned is information about each breakpoint created by this request.
	///
	/// This includes the actual code location and whether the breakpoint could be verified.
	///
	/// The breakpoints returned are in the same order as the elements of the `breakpoints` (or the
	/// deprecated `lines`) array in the arguments.
	SetBreakpoints(SetBreakpointsParams),
	/// Response to `setDataBreakpoints` request.
	///
	/// Returned is information about each breakpoint created by this request.
	SetDataBreakpoints(SetDataBreakpointsParams),
	/// Response to `setExceptionBreakpoints` request.
	///
	/// The response contains an array of `Breakpoint` objects with information about each exception
	/// breakpoint or filter.
	///
	/// The `Breakpoint` objects are in the same order as the elements of the `filters`,
	/// `filterOptions`, `exceptionOptions` arrays given as arguments.
	///
	/// If both `filters` and `filterOptions` are given, the returned array must start with
	/// `filters` information first, followed by `filterOptions` information.
	///
	/// The `verified` property of a `Breakpoint` object signals whether the exception breakpoint or
	/// filter could be successfully created and whether the condition is valid. In case of an
	/// error the `message` property explains the problem. The `id` property can be used to
	/// introduce a unique ID for the exception breakpoint or filter so that it can be updated
	/// subsequently by sending breakpoint events.
	///
	/// For backward compatibility both the `breakpoints` array and the enclosing `body` are
	/// optional. If these elements are missing a client is not able to show problems for
	/// individual exception breakpoints or filters.
	SetExceptionBreakpoints(SetExceptionBreakpointsParams),
	/// Response to `setExpression` request.
	SetExpression(SetExpressionParams),
	/// Response to `setFunctionBreakpoints` request.
	///
	/// Returned is information about each breakpoint created by this request.
	SetFunctionBreakpoints(SetFunctionBreakpointsParams),
	/// Response to `setInstructionBreakpoints` request
	SetInstructionBreakpoints(SetInstructionBreakpointsParams),
	/// Response to `setVariable` request.
	SetVariable(SetVariableParams),
	/// Response to `source` request.
	Source(SourceParams),
	/// Response to `stackTrace` request.
	StackTrace(StackTraceParams),
	/// Response to `stepBack` request. This is just an acknowledgement, so no body field is
	/// required.
	StepBack,
	/// Response to `stepIn` request. This is just an acknowledgement, so no body field is required.
	StepIn,
	/// Response to `stepInTargets` request.
	StepInTargets(StepInTargetsParams),
	/// Response to `stepOut` request. This is just an acknowledgement, so no body field is
	/// required.
	StepOut,
	/// Response to `terminate` request. This is just an acknowledgement, so no body field is
	/// required.
	Terminate,
	/// Response to `terminateThreads` request. This is just an acknowledgement, no body field is
	/// required.
	TerminateThreads,
	/// Response to `threads` request.
	Threads(ThreadsParams),
	/// Response to `variables` request.
	Variables(VariablesParams),
	/// Response to `writeMemory` request.
	WriteMemory(WriteMemoryParams),
	/// Response to `runInTerminal` request.
	RunInTerminal(RunInTerminalParams),
	/// Response to `startDebugging` request. This is just an acknowledgement, so no body field is
	/// required.
	StartDebugging,
	#[serde(untagged)]
	Unknown {
		command: String,
		#[serde(skip_serializing_if = "Option::is_none", default)]
		body:    Option<serde_json::Value>,
	},
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum ResponseMessage {
	/// The request was cancelled
	Cancelled,
	/// The request may be retried once the adapter is in a 'stopped' state
	NotStopped,
	#[serde(untagged)]
	Other(String),
}

impl Response {
	/// Sequence number of the message (also known as message ID).
	///
	/// The `seq` for the first message sent by a client or debug adapter is 1, and for each
	/// subsequent message is 1 greater than the previous message sent by that actor. `seq` can be
	/// used to order requests, responses, and events, and to associate requests with their
	/// corresponding responses. For protocol messages of type `request` the sequence number can
	/// be used to cancel the request.
	pub fn id(&self) -> u32 {
		self.id
	}

	/// The sequence number of the corresponding request
	pub fn request_id(&self) -> u32 {
		self.req_id
	}

	/// Contains request result if success is true and error details if [`Response::success`] is
	/// false
	pub fn response(&self) -> &ResponseType {
		&self.response
	}

	/// Contains the raw error in short form if `success` is false.
	///
	/// This raw error might be interpreted by the client and is not shown in the UI.
	///
	/// Some predefined values exist.
	pub fn message(&self) -> Option<&ResponseMessage> {
		self.message.as_ref()
	}

	/// Outcome of the request.\
	///
	/// If true, the request was successful and the `body` attribute may
	/// contain the result of the request.
	///
	/// If the value is false, the attribute `message` contains the error in short form and the
	/// `body` may contain additional information (see `ErrorResponse.body.error`).
	pub fn success(&self) -> bool {
		self.success
	}
}

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
