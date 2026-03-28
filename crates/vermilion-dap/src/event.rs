// SPDX-License-Identifier: BSD-3-Clause

use crate::types::{
	capabilities::Capabilities,
	params::event::{
		BreakpointParams, ContinuedParams, ExitedParams, InvalidatedParams, LoadedSourceParams,
		MemoryParams, ModuleParams, OutputParams, ProcessParams, ProgressEndParams,
		ProgressStartParams, ProgressUpdateParams, StoppedParams, TerminatedParams, ThreadParams,
	},
};

/// An event from the debug adapter
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Event {
	#[serde(rename = "seq")]
	pub(crate) id:    u32,
	#[serde(flatten)]
	pub(crate) event: EventType,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "event", content = "body", rename_all = "camelCase")]
pub enum EventType {
	/// The event indicates that some information about a breakpoint has changed.
	///
	/// While debug adapters may notify the clients of `changed` breakpoints using this event,
	/// clients should continue to use the breakpoint's original properties when updating a
	/// source's breakpoints in the `breakpoint` request.
	Breakpoint(BreakpointParams),
	/// The event indicates that one or more capabilities have changed.
	///
	/// Since the capabilities are dependent on the client and its UI, it might not be possible to
	/// change that at random times (or too late).
	///
	/// Consequently this event has a hint characteristic: a client can only be expected to make a
	/// 'best effort' in honoring individual capabilities but there are no guarantees.
	///
	/// Only changed capabilities need to be included, all other capabilities keep their values.
	Capabilities(Capabilities),
	/// The event indicates that the execution of the debuggee has continued.
	///
	/// Please note: a debug adapter is not expected to send this event in response to a request
	/// that implies that execution continues, e.g. `launch` or `continue`.
	///
	/// It is only necessary to send a `continued` event if there was no previous request that
	/// implied this.
	Continued(ContinuedParams),
	/// The event indicates that the debuggee has exited and returns its exit code.
	Exited(ExitedParams),
	/// This event indicates that the debug adapter is ready to accept configuration requests (e.g.
	/// `setBreakpoints`, `setExceptionBreakpoints`).
	///
	/// A debug adapter is expected to send this event when it is ready to accept configuration
	/// requests (but not before the `initialize` request has finished).
	///
	/// The sequence of events/requests is as follows:
	/// - adapters sends `initialized` event (after the `initialize` request has returned)
	/// - client sends zero or more `setBreakpoints` requests
	/// - client sends one `setFunctionBreakpoints` request (if corresponding capability
	///   `supportsFunctionBreakpoints` is true)
	/// - client sends a `setExceptionBreakpoints` request if one or more
	///   `exceptionBreakpointFilters` have been defined (or if `supportsConfigurationDoneRequest`
	///   is not true)
	/// - client sends other future configuration requests
	/// - client sends one `configurationDone` request to indicate the end of the configuration.
	Initialized,
	/// This event signals that some state in the debug adapter has changed and requires that the
	/// client needs to re-render the data snapshot previously requested.
	///
	/// Debug adapters do not have to emit this event for runtime changes like stopped or thread
	/// events because in that case the client refetches the new state anyway. But the event can be
	/// used for example to refresh the UI after rendering formatting has changed in the debug
	/// adapter.
	///
	/// This event should only be sent if the corresponding capability `supportsInvalidatedEvent` is
	/// true.
	Invalidated(InvalidatedParams),
	/// The event indicates that some source has been added, changed, or removed from the set of all
	/// loaded sources.
	LoadedSource(LoadedSourceParams),
	/// This event indicates that some memory range has been updated.
	///
	/// It should only be sent if the corresponding capability `supportsMemoryEvent` is true.
	///
	/// Clients typically react to the event by re-issuing a `readMemory` request if they show the
	/// memory identified by the `memoryReference` and if the updated memory range overlaps the
	/// displayed range. Clients should not make assumptions how individual memory references relate
	/// to each other, so they should not assume that they are part of a single continuous address
	/// range and might overlap.
	///
	/// Debug adapters can use this event to indicate that the contents of a memory range has
	/// changed due to some other request like `setVariable` or `setExpression`. Debug adapters are
	/// not expected to emit this event for each and every memory change of a running program,
	/// because that information is typically not available from debuggers and it would flood
	/// clients with too many events.
	Memory(MemoryParams),
	/// The event indicates that some information about a module has changed.
	Module(ModuleParams),
	/// The event indicates that the target has produced some output.
	Output(OutputParams),
	/// The event indicates that the debugger has begun debugging a new process.
	///
	/// Either one that it has launched, or one that it has attached to.
	Process(ProcessParams),
	/// The event signals the end of the progress reporting with a final message.
	///
	/// This event should only be sent if the corresponding capability `supportsProgressReporting`
	/// is true.
	ProgressEnd(ProgressEndParams),
	/// The event signals that a long running operation is about to start and provides additional
	/// information for the client to set up a corresponding progress and cancellation UI.
	///
	/// The client is free to delay the showing of the UI in order to reduce flicker.
	///
	/// This event should only be sent if the corresponding capability `supportsProgressReporting`
	/// is true.
	ProgressStart(ProgressStartParams),
	/// The event signals that the progress reporting needs to be updated with a new message and/or
	/// percentage.
	///
	/// The client does not have to update the UI immediately, but the clients needs to keep track
	/// of the message and/or percentage values.
	///
	/// This event should only be sent if the corresponding capability `supportsProgressReporting`
	/// is true.
	ProgressUpdate(ProgressUpdateParams),
	/// The event indicates that the execution of the debuggee has stopped due to some condition.
	///
	/// This can be caused by a breakpoint previously set, a stepping request has completed, by
	/// executing a debugger statement etc.
	Stopped(StoppedParams),
	/// The event indicates that debugging of the debuggee has terminated. This does **not** mean
	/// that the debuggee itself has exited.
	Terminated(TerminatedParams),
	/// The event indicates that a thread has started or exited.
	Thread(ThreadParams),
	#[serde(untagged)]
	Unknown {
		event: String,
		#[serde(skip_serializing_if = "Option::is_none", default)]
		body:  Option<serde_json::Value>,
	},
}

impl Event {
	/// Create a new event
	pub fn new(id: u32, event: EventType) -> Self {
		Self { id, event }
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

	/// Get the actual event
	pub fn event(&self) -> &EventType {
		&self.event
	}
}

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
