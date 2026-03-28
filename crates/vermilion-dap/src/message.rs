// SPDX-License-Identifier: BSD-3-Clause

use crate::{event::Event, request::Request, response::Response, unknown::Unknown};

/// Represents a DAP message that either came over a transport
/// or to be sent over a transport
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Message {
	Request(Request),
	Response(Response),
	Event(Event),
	#[serde(untagged)]
	Unknown(Unknown),
}

impl Message {
	/// Sequence number of the message (also known as message ID).
	///
	/// The `seq` for the first message sent by a client or debug adapter is 1, and for each
	/// subsequent message is 1 greater than the previous message sent by that actor. `seq` can be
	/// used to order requests, responses, and events, and to associate requests with their
	/// corresponding responses. For protocol messages of type `request` the sequence number can
	/// be used to cancel the request.
	pub fn id(&self) -> u32 {
		match self {
			Self::Request(request) => request.id,
			Self::Response(response) => response.id,
			Self::Event(event) => event.id,
			Self::Unknown(unknown) => unknown.id,
		}
	}
}

impl From<Request> for Message {
	fn from(value: Request) -> Self {
		Self::Request(value)
	}
}

impl From<Response> for Message {
	fn from(value: Response) -> Self {
		Self::Response(value)
	}
}

impl From<Event> for Message {
	fn from(value: Event) -> Self {
		Self::Event(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_serialize_request_msg() {}

	#[test]
	fn test_serialize_response_msg() {}

	#[test]
	fn test_serialize_event_msg() {}

	#[test]
	fn test_serialize_other_msg() {}

	#[test]
	fn test_deserialize_request_msg() {}

	#[test]
	fn test_deserialize_response_msg() {}

	#[test]
	fn test_deserialize_event_msg() {}

	#[test]
	fn test_deserialize_other_msg() {}
}
