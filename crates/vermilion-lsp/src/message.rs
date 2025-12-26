/* SPDX-License-Identifier: BSD-3-Clause */

use std::fmt::{Debug, Display};

use eyre::Result;

use crate::{notification::Notification, request::Request, response::Response};

/// A JSON-RPC message ID
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum Id {
	Integer(i32),
	String(String),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Message {
	Request(Request),
	Response(Response),
	Notification(Notification),
}

#[derive(Clone, Debug, serde::Serialize)]
struct JsonRpcEnvelope<'a> {
	jsonrpc: &'static str,
	#[serde(flatten)]
	message: &'a Message,
}

impl Message {
	pub fn deserialize(buffer: &[u8]) -> Result<Message> {
		Ok(serde_json::from_slice(buffer)?)
	}

	pub fn serialize(self, buffer: &mut [u8]) -> Result<usize> {
		serde_json::to_writer(buffer, &JsonRpcEnvelope { jsonrpc: "2.0", message: &self })?;
		Ok(0)
	}
}

impl From<i32> for Id {
	fn from(value: i32) -> Self {
		Id::Integer(value)
	}
}

impl From<String> for Id {
	fn from(value: String) -> Self {
		Id::String(value)
	}
}

impl Display for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Id::Integer(id) => Display::fmt(id, f),
			Id::String(id) => Debug::fmt(id, f),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
