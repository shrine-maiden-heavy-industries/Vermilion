// SPDX-License-Identifier: BSD-3-Clause

use crate::types::params::request::{
	AttachParams, BreakpointLocationsParams, CancelParams, CompletionsParams,
	ConfigurationDoneParams, ContinueParams, DataBreakpointInfoParams, DisassembleParams,
	DisconnectParams, EvaluateParams, ExceptionInfoParams, GotoParams, GotoTargetsParams,
	InitializeParams, LaunchParams, LocationsParams, ModuleParams, NextParams, PauseParams,
	ReadMemoryParams, RestartFrameParams, RestartParams, ReverseContinueParams,
	RunInTerminalParams, ScopesParams, SetBreakpointsParams, SetDataBreakpointsParams,
	SetExceptionBreakpointsParams, SetExpressionParams, SetFunctionBreakpointsParams,
	SetInstructionBreakpointsParams, SetVariableParams, SourceParams, StackTraceParams,
	StartDebuggingParams, StepBackParams, StepInParams, StepInTargetsParams, StepOutParams,
	TerminateParams, TerminateThreadsParams, VariablesParams, WriteMemoryParams,
};

/// A request from/to a DAP client/server
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Request {
	#[serde(rename = "seq")]
	pub(crate) id:      u32,
	#[serde(flatten)]
	pub(crate) request: RequestType,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "command", content = "arguments", rename_all = "camelCase")]
pub enum RequestType {
	/// The `cancel` request is used by the client in two situations:
	/// - to indicate that it is no longer interested in the result produced by a specific request
	///   issued earlier
	/// - to cancel a progress sequence.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsCancelRequest` is true.
	///
	/// This request has a hint characteristic: a debug adapter can only be expected to make a 'best
	/// effort' in honoring this request but there are no guarantees.
	///
	/// The `cancel` request may return an error if it could not cancel an operation but a client
	/// should refrain from presenting this error to end users.
	///
	/// The request that got cancelled still needs to send a response back. This can either be a
	/// normal result (`success` attribute true) or an error response (`success` attribute false
	/// and the `message` set to `cancelled`).
	///
	/// Returning partial results from a cancelled request is possible but please note that a client
	/// has no generic way for detecting that a response is partial or not.
	///
	/// The progress that got cancelled still needs to send a `progressEnd` event back.
	///
	/// A client should not assume that progress just got cancelled after sending the `cancel`
	/// request.
	Cancel(CancelParams),
	/// The `attach` request is sent from the client to the debug adapter to attach to a debuggee
	/// that is already running.
	///
	/// Since attaching is debugger/runtime specific, the arguments for this request are not part of
	/// this specification.
	Attach(AttachParams),
	/// The `breakpointLocations` request returns all possible locations for source breakpoints in a
	/// given range.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsBreakpointLocationsRequest` is true.
	BreakpointLocations(BreakpointLocationsParams),
	/// Returns a list of possible completions for a given caret position and text.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsCompletionsRequest` is true.
	Completions(CompletionsParams),
	/// This request indicates that the client has finished initialization of the debug adapter.
	///
	/// So it is the last request in the sequence of configuration requests (which was started by
	/// the `initialized` event).
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsConfigurationDoneRequest` is true.
	ConfigurationDone(ConfigurationDoneParams),
	/// The request resumes execution of all threads.
	///
	/// If the debug adapter supports single thread execution (see capability
	/// `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true
	/// resumes only the specified thread.
	///
	/// If not all threads were resumed, the `allThreadsContinued` attribute of the response should
	/// be set to false.
	Continue(ContinueParams),
	/// Obtains information on a possible data breakpoint that could be set on an expression or
	/// variable.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsDataBreakpoints` is true.
	DataBreakpointInfo(DataBreakpointInfoParams),
	/// Disassembles code stored at the provided location.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsDisassembleRequest` is true.
	Disassemble(DisassembleParams),
	/// The `disconnect` request asks the debug adapter to disconnect from the debuggee (thus ending
	/// the debug session) and then to shut down itself (the debug adapter).
	///
	/// In addition, the debug adapter must terminate the debuggee if it was started with the
	/// `launch` request. If an `attach` request was used to connect to the debuggee, then the
	/// debug adapter must not terminate the debuggee.
	///
	/// This implicit behavior of when to terminate the debuggee can be overridden with the
	/// `terminateDebuggee` argument (which is only supported by a debug adapter if the
	/// corresponding capability `supportTerminateDebuggee` is true).
	Disconnect(DisconnectParams),
	/// Evaluates the given expression in the context of a stack frame.
	///
	/// The expression has access to any variables and arguments that are in scope.
	Evaluate(EvaluateParams),
	/// Retrieves the details of the exception that caused this event to be raised.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsExceptionInfoRequest` is true.
	ExceptionInfo(ExceptionInfoParams),
	/// The request sets the location where the debuggee will continue to run.
	///
	/// This makes it possible to skip the execution of code or to execute code again.
	///
	/// The code between the current location and the goto target is not executed but skipped.
	///
	/// The debug adapter first sends the response and then a `stopped` event with reason
	/// `goto`.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsGotoTargetsRequest` is true (because only then goto targets exist that can be
	/// passed as arguments).
	Goto(GotoParams),
	/// This request retrieves the possible goto targets for the specified source location.
	///
	/// These targets can be used in the `goto` request.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsGotoTargetsRequest` is true.
	GotoTargets(GotoTargetsParams),
	/// The `initialize` request is sent as the first request from the client to the debug adapter
	/// in order to configure it with client capabilities and to retrieve capabilities from the
	/// debug adapter.
	///
	/// Until the debug adapter has responded with an `initialize` response, the client must not
	/// send any additional requests or events to the debug adapter.
	///
	/// In addition the debug adapter is not allowed to send any requests or events to the client
	/// until it has responded with an `initialize` response.
	///
	/// The `initialize` request may only be sent once.
	Initialize(InitializeParams),
	/// This launch request is sent from the client to the debug adapter to start the debuggee with
	/// or without debugging (if `noDebug` is true).
	///
	/// Since launching is debugger/runtime specific, the arguments for this request are not part of
	/// this specification.
	Launch(LaunchParams),
	/// Retrieves the set of all sources currently loaded by the debugged process.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsLoadedSourcesRequest` is true.
	LoadedSources,
	/// Looks up information about a location reference previously returned by the debug adapter.
	Locations(LocationsParams),
	/// Modules can be retrieved from the debug adapter with this request which can either return
	/// all modules or a range of modules to support paging.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsModulesRequest` is true.
	Module(ModuleParams),
	/// The request executes one step (in the given granularity) for the specified thread and allows
	/// all other threads to run freely by resuming them.
	///
	/// If the debug adapter supports single thread execution (see capability
	/// `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true
	/// prevents other suspended threads from resuming.
	///
	/// The debug adapter first sends the response and then a `stopped` event (with reason `step`)
	/// after the step has completed.
	Next(NextParams),
	/// The request suspends the debuggee.
	///
	/// The debug adapter first sends the response and then a `stopped` event (with reason `pause`)
	/// after the thread has been paused successfully.
	Pause(PauseParams),
	/// Reads bytes from memory at the provided location.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsReadMemoryRequest` is true.
	ReadMemory(ReadMemoryParams),
	/// Restarts a debug session. Clients should only call this request if the corresponding
	/// capability `supportsRestartRequest` is true.
	///
	/// If the capability is missing or has the value false, a typical client emulates `restart` by
	/// terminating the debug adapter first and then launching it anew.
	Restart(RestartParams),
	/// The request restarts execution of the specified stack frame.
	///
	/// The debug adapter first sends the response and then a `stopped` event (with reason
	/// `restart`) after the restart has completed.
	///
	/// Clients should only call this request if the corresponding capability `supportsRestartFrame`
	/// is true.
	RestartFrame(RestartFrameParams),
	/// The request resumes backward execution of all threads. If the debug adapter supports single
	/// thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the
	/// `singleThread` argument to true resumes only the specified thread. If not all threads were
	/// resumed, the `allThreadsContinued` attribute of the response should be set to false.
	///
	/// Clients should only call this request if the corresponding capability `supportsStepBack` is
	/// true.
	ReverseContinue(ReverseContinueParams),
	/// The request returns the variable scopes for a given stack frame ID
	Scopes(ScopesParams),
	/// Sets multiple breakpoints for a single source and clears all previous breakpoints in that
	/// source.
	///
	/// To clear all breakpoint for a source, specify an empty array.
	///
	/// When a breakpoint is hit, a `stopped` event (with reason `breakpoint`) is generated.
	SetBreakpoints(SetBreakpointsParams),
	/// Replaces all existing data breakpoints with new data breakpoints.
	///
	/// To clear all data breakpoints, specify an empty array.
	///
	/// When a data breakpoint is hit, a `stopped` event (with reason `data breakpoint`) is
	/// generated.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsDataBreakpoints` is true.
	SetDataBreakpoints(SetDataBreakpointsParams),
	/// The request configures the debugger's response to thrown exceptions.
	///
	/// Each of the `filters`, `filterOptions`, and `exceptionOptions` in the request are
	/// independent configurations to a debug adapter indicating a kind of exception to catch.
	///
	/// An exception thrown in a program should result in a `stopped` event from the debug adapter
	/// (with reason `exception`) if any of the configured filters match.
	///
	/// Clients should only call this request if the corresponding capability
	/// `exceptionBreakpointFilters` returns one or more filters.
	SetExceptionBreakpoints(SetExceptionBreakpointsParams),
	/// Evaluates the given `value` expression and assigns it to the `expression` which must be a
	/// modifiable l-value.
	///
	/// The expressions have access to any variables and arguments that are in scope of the
	/// specified frame.
	///
	/// Clients should only call this request if the corresponding capability supportsSetExpression`
	/// is true.
	///
	/// If a debug adapter implements both `setExpression` and `setVariable`, a client uses
	/// `setExpression` if the variable has an `evaluateName` property.
	SetExpression(SetExpressionParams),
	/// Replaces all existing function breakpoints with new function breakpoints.
	///
	/// To clear all function breakpoints, specify an empty array.
	///
	/// When a function breakpoint is hit, a `stopped` event (with reason `function breakpoint`) is
	/// generated.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsFunctionBreakpoints` is true.
	SetFunctionBreakpoints(SetFunctionBreakpointsParams),
	/// Replaces all existing instruction breakpoints.
	///
	/// Typically, instruction breakpoints would be set from a disassembly window.
	///
	/// To clear all instruction breakpoints, specify an empty array.
	///
	/// When an instruction breakpoint is hit, a `stopped` event (with reason `instruction
	/// breakpoint`) is generated.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsInstructionBreakpoints` is true.
	SetInstructionBreakpoints(SetInstructionBreakpointsParams),
	/// Set the variable with the given name in the variable container to a new value. Clients
	/// should only call this request if the corresponding capability `supportsSetVariable` is
	/// true.
	///
	/// If a debug adapter implements both `setVariable` and `setExpression`, a client will
	/// only use `setExpression` if the variable has an `evaluateName` property.
	SetVariable(SetVariableParams),
	/// The request retrieves the source code for a given source reference.
	Source(SourceParams),
	/// The request returns a stacktrace from the current execution state of a given thread.
	///
	/// A client can request all stack frames by omitting the startFrame and levels arguments.
	///
	/// For performance-conscious clients and if the corresponding capability
	/// `supportsDelayedStackTraceLoading` is true, stack frames can be retrieved in a piecemeal way
	/// with the `startFrame` and `levels` arguments.
	///
	/// The response of the `stackTrace` request may contain a `totalFrames` property that hints at
	/// the total number of frames in the stack.
	///
	/// If a client needs this total number upfront, it can issue a request for a single (first)
	/// frame and depending on the value of `totalFrames` decide how to proceed.
	///
	/// In any case a client should be prepared to receive fewer frames than requested, which is an
	/// indication that the end of the stack has been reached.
	StackTrace(StackTraceParams),
	/// The request executes one backward step (in the given granularity) for the specified thread
	/// and allows all other threads to run backward freely by resuming them.
	///
	/// If the debug adapter supports single thread execution (see capability
	/// `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true
	/// prevents other suspended threads from resuming.
	///
	/// The debug adapter first sends the response and then a `stopped` event (with reason `step`)
	/// after the step has completed.
	///
	/// Clients should only call this request if the corresponding capability `supportsStepBack` is
	/// true.
	StepBack(StepBackParams),
	/// The request resumes the given thread to step into a function/method and allows all other
	/// threads to run freely by resuming them.
	///
	/// If the debug adapter supports single thread execution (see capability
	/// `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true
	/// prevents other suspended threads from resuming.
	///
	/// If the request cannot step into a target, `stepIn` behaves like the `next` request.
	///
	/// The debug adapter first sends the response and then a `stopped` event (with reason `step`)
	/// after the step has completed.
	///
	/// If there are multiple function/method calls (or other targets) on the source line, the
	/// argument `targetId` can be used to control into which target the `stepIn` should occur.
	///
	/// The list of possible targets for a given source line can be retrieved via the
	/// `stepInTargets` request.
	StepIn(StepInParams),
	/// This request retrieves the possible step-in targets for the specified stack frame.
	///
	/// These targets can be used in the `stepIn` request.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsStepInTargetsRequest` is true.
	StepInTargets(StepInTargetsParams),
	/// The request resumes the given thread to step out (return) from a function/method and allows
	/// all other threads to run freely by resuming them.
	///
	/// If the debug adapter supports single thread execution (see capability
	/// `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true
	/// prevents other suspended threads from resuming.
	///
	/// The debug adapter first sends the response and then a `stopped` event (with reason `step`)
	/// after the step has completed.
	StepOut(StepOutParams),
	/// The `terminate` request is sent from the client to the debug adapter in order to shut down
	/// the debuggee gracefully. Clients should only call this request if the capability
	/// `supportsTerminateRequest` is true.
	///
	/// Typically a debug adapter implements `terminate` by sending a software signal which the
	/// debuggee intercepts in order to clean things up properly before terminating itself.
	///
	/// Please note that this request does not directly affect the state of the debug session: if
	/// the debuggee decides to veto the graceful shutdown for any reason by not terminating
	/// itself, then the debug session just continues.
	///
	/// Clients can surface the `terminate` request as an explicit command or they can integrate it
	/// into a two stage Stop command that first sends `terminate` to request a graceful shutdown,
	/// and if that fails uses `disconnect` for a forceful shutdown.
	Terminate(TerminateParams),
	/// The request terminates the threads with the given ids.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsTerminateThreadsRequest` is true.
	TerminateThreads(TerminateThreadsParams),
	/// The request retrieves a list of all threads.
	Threads,
	/// Retrieves all child variables for the given variable reference.
	///
	/// A filter can be used to limit the fetched children to either named or indexed children.
	Variables(VariablesParams),
	/// Writes bytes to memory at the provided location.
	///
	/// Clients should only call this request if the corresponding capability
	/// `supportsWriteMemoryRequest` is true.
	WriteMemory(WriteMemoryParams),
	/// This request is sent from the debug adapter to the client to run a command in a
	/// terminal.
	///
	/// This is typically used to launch the debuggee in a terminal provided by the client.
	///
	/// This request should only be called if the corresponding client capability
	/// `supportsRunInTerminalRequest` is true.
	///
	/// Client implementations of `runInTerminal` are free to run the command however they choose
	/// including issuing the command to a command line interpreter (aka 'shell').
	///
	/// Argument strings passed to the `runInTerminal` request must arrive verbatim in the command
	/// to be run. As a consequence, clients which use a shell are responsible for escaping any
	/// special shell characters in the argument strings to prevent them from being interpreted
	/// (and modified) by the shell.
	///
	/// Some users may wish to take advantage of shell processing in the argument strings. For
	/// clients which implement `runInTerminal` using an intermediary shell, the
	/// `argsCanBeInterpretedByShell` property can be set to true. In this case the client is
	/// requested not to escape any special shell characters in the argument strings.
	RunInTerminal(RunInTerminalParams),
	/// This request is sent from the debug adapter to the client to start a new debug session of
	/// the same type as the caller.
	///
	/// This request should only be sent if the corresponding client capability
	/// `supportsStartDebuggingRequest` is true.
	///
	/// A client implementation of `startDebugging` should start a new debug session (of the same
	/// type as the caller) in the same way that the caller's session was started. If the client
	/// supports hierarchical debug sessions, the newly created session can be treated as a child
	/// of the caller session.
	StartDebugging(StartDebuggingParams),
	#[serde(untagged)]
	Unknown {
		command:   String,
		#[serde(skip_serializing_if = "Option::is_none", default)]
		arguments: Option<serde_json::Value>,
	},
}

impl Request {
	/// Create a new request with the given id and type
	pub fn new(id: u32, request: RequestType) -> Self {
		Self { id, request }
	}

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

	/// Get the actual request
	pub fn request(&self) -> &RequestType {
		&self.request
	}
}

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
