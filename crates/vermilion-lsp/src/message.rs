/* SPDX-License-Identifier: BSD-3-Clause */
//! This module contains the [JSON-RPC] types and the machinery for serializing and
//! deserializing them into LSP types.
//!
//! [JSON-RPC]: https://www.jsonrpc.org/specification

use std::io::{self, BufRead, Write};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub struct Request {}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub struct Response {}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub struct Notification {}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Message {
	Request(Request),
	Response(Response),
	Notification(Notification),
}

// NOTE(aki):
// We can't implement `serde::Deserialize` here due to the reference.
// So when we read a message we need to do some wiggling
#[derive(Clone, Copy, Debug, serde::Serialize)]
struct JsonRpcEnvelope<'a> {
	jsonrpc: &'static str,
	#[serde(flatten)]
	message: &'a Message,
}

impl From<Request> for Message {
	fn from(value: Request) -> Self {
		Message::Request(value)
	}
}

impl From<Response> for Message {
	fn from(value: Response) -> Self {
		Message::Response(value)
	}
}

impl From<Notification> for Message {
	fn from(value: Notification) -> Self {
		Message::Notification(value)
	}
}

impl Message {
	/// Given a [`BufRead`], attempt to read and deserialize a JSON-RPC message
	pub fn deserialize(input: &mut dyn BufRead) -> io::Result<Option<Self>> {
		Ok(None)
	}

	/// Serialize this JSON-RPC message and write it to `out`
	pub fn serialize(&self, out: &mut dyn Write) -> io::Result<()> {
		let msg = serde_json::to_vec(&JsonRpcEnvelope { jsonrpc: "2.0", message: self })?;
		write!(out, "Content-Length: {}\r\n\r\n", msg.len())?;
		out.write_all(msg.as_slice())?;
		out.flush()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
