// SPDX-License-Identifier: BSD-3-Clause

/// Represents an unknown DAP message
///
/// Any messages that are not an [`Event`], [`Request`], or [`Response`]
/// are turned into this type.
///
/// [`Event`]: crate::event::Event
/// [`Request`]: crate::request::Request
/// [`Response`]: crate::response::Response
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Unknown {
	#[serde(rename = "seq")]
	pub(crate) id:   u32,
	#[serde(flatten)]
	pub(crate) body: serde_json::Value,
}

impl Unknown {
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

	/// Get the body of the message
	pub fn body(&self) -> &serde_json::Value {
		&self.body
	}
}

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
