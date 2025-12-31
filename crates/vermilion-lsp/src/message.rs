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

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Message {
	Request(Box<Request>),
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

	pub fn serialize(self, buffer: &mut Vec<u8>) -> Result<usize> {
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
	use crate::{
		error::{Code, Error},
		request::RequestType,
	};

	#[test]
	fn encode_message_request() -> Result<()> {
		let message_text = r#"{"jsonrpc":"2.0","id":"1","method":"shutdown"}"#;
		let message = Message::Request(Box::new(Request {
			id: "1".to_string().into(),
			req: RequestType::Shutdown,
		}));

		let mut buffer = Vec::new();
		let _ = message.serialize(&mut buffer)?;

		assert_eq!(message_text, str::from_utf8(&buffer)?);

		Ok(())
	}

	#[test]
	fn decode_message_request() -> Result<()> {
		let message_text = r#"{"jsonrpc": "2.0", "id": 1, "method": "shutdown" }"#;
		let message = Message::deserialize(message_text.as_bytes())?;

		assert_eq!(
			message,
			Message::Request(Box::new(Request {
				id: 1.into(),
				req: RequestType::Shutdown
			}))
		);

		Ok(())
	}

	#[test]
	fn encode_message_response_ok() -> Result<()> {
		let message_text = r#"{"jsonrpc":"2.0","id":"2","result":"ニャ〜"}"#;
		let message = Message::Response(Response {
			id: "2".to_string().into(),
			result: Some("ニャ〜".into()),
			error: None,
		});

		let mut buffer = Vec::new();
		let _ = message.serialize(&mut buffer)?;

		assert_eq!(message_text, str::from_utf8(&buffer)?);

		Ok(())
	}

	#[test]
	fn decode_message_response_ok() -> Result<()> {
		let message_text = r#"{"jsonrpc": "2.0", "id": 2, "result": "ニャ〜"}"#;
		let message = Message::deserialize(message_text.as_bytes())?;

		assert_eq!(
			message,
			Message::Response(Response {
				id: 2.into(),
				result: Some("ニャ〜".into()),
				error: None
			})
		);

		Ok(())
	}

	#[test]
	fn encode_message_response_err() -> Result<()> {
		let message_text = r#"{"jsonrpc":"2.0","id":"3","error":{"code":-32803,"message":"nya"}}"#;
		let message = Message::Response(Response {
			id: "3".to_string().into(),
			result: None,
			error: Some(Error {
				code: Code::RequestFailed,
				message: "nya".into(),
				data: None,
			}),
		});

		let mut buffer = Vec::new();
		let _ = message.serialize(&mut buffer)?;

		assert_eq!(message_text, str::from_utf8(&buffer)?);

		Ok(())
	}

	#[test]
	fn decode_message_response_err() -> Result<()> {
		let message_text =
			r#"{"jsonrpc": "2.0", "id": 3, "error": {"code": -32803,"message": "nya"}}"#;
		let message = Message::deserialize(message_text.as_bytes())?;

		assert_eq!(
			message,
			Message::Response(Response {
				id: 3.into(),
				result: None,
				error: Some(Error {
					code: Code::RequestFailed,
					message: "nya".into(),
					data: None
				})
			})
		);

		Ok(())
	}

	#[test]
	fn encode_message_notification_params() -> Result<()> {
		let message_text = r#"{"jsonrpc":"2.0","method":"telemetry/event","params":"meow"}"#;
		let message = Message::Notification(Notification::TelemetryEven("meow".into()));

		let mut buffer = Vec::new();
		let _ = message.serialize(&mut buffer)?;

		assert_eq!(message_text, str::from_utf8(&buffer)?);

		Ok(())
	}

	#[test]
	fn decode_message_notification_params() -> Result<()> {
		let message_text = r#"{"jsonrpc": "2.0", "method": "telemetry/event", "params": "meow"}"#;
		let message = Message::deserialize(message_text.as_bytes())?;

		assert_eq!(
			message,
			Message::Notification(Notification::TelemetryEven("meow".into()))
		);

		Ok(())
	}

	#[test]
	fn encode_message_notification_no_params() -> Result<()> {
		let message_text = r#"{"jsonrpc":"2.0","method":"exit"}"#;
		let message = Message::Notification(Notification::Exit);

		let mut buffer = Vec::new();
		let _ = message.serialize(&mut buffer)?;

		assert_eq!(message_text, str::from_utf8(&buffer)?);

		Ok(())
	}

	#[test]
	fn decode_message_notification_no_params() -> Result<()> {
		let message_text = r#"{"jsonrpc": "2.0", "method": "exit" }"#;
		let message = Message::deserialize(message_text.as_bytes())?;

		assert_eq!(message, Message::Notification(Notification::Exit));

		Ok(())
	}
}
