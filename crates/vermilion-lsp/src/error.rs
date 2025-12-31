/* SPDX-License-Identifier: BSD-3-Clause */

use std::fmt::Display;

#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Serialize_repr,
	serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum Code {
	// JSON-RPC Errors
	/// Invalid JSON was received by the server.
	ParseError = -32700,
	/// The JSON sent is not a valid Request object.
	InvalidRequest = -32600,
	/// The method does not exist / is not available.
	MethodNotFound = -32601,
	/// Invalid method parameter(s).
	InvalidParams = -32602,
	/// Internal JSON-RPC error.
	InternalError = -32603,
	/// Error code indicating that a server received a notification or
	/// request before the server received the `initialize` request.
	ServerNotInitialized = -32002,
	UnknownErrorCode = -32001,
	/// A request failed but it was syntactically correct, e.g the
	/// method name was known and the parameters were valid. The error
	/// message should contain human readable information about why
	/// the request failed.
	RequestFailed = -32803,
	/// The server cancelled the request. This error code should
	/// only be used for requests that explicitly support being
	/// server cancellable.
	SeverCancelled = -32802,
	/// The server detected that the content of a document got
	/// modified outside normal conditions. A server should
	/// NOT send this error code if it detects a content change
	/// in its unprocessed messages. The result even computed
	/// on an older state might still be useful for the client.
	///
	/// If a client decides that a result is not of any use anymore
	/// the client should cancel the request.
	ContentModified = -32801,
	/// The client has canceled a request and a server has detected
	/// the cancel.
	RequestCancelled = -32800,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Error {
	pub(crate) code: Code,
	pub(crate) message: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
}

impl Error {
	pub const fn new(message: String, code: Code, data: Option<serde_json::Value>) -> Self {
		Self { code, message, data }
	}

	pub const fn code(&self) -> &Code {
		&self.code
	}

	pub const fn message(&self) -> &String {
		&self.message
	}

	pub const fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"JSON-RPC Error, Type: {} Message: {}",
			self.code, self.message
		)?;
		if let Some(data) = self.data.as_ref() {
			write!(f, "Associated Data: {:#?}", data)?;
		}

		Ok(())
	}
}

impl Display for Code {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Code::ParseError => "Parse Error",
				Code::InvalidRequest => "Invalid Request",
				Code::MethodNotFound => "Method Not Found",
				Code::InvalidParams => "Invalid Params",
				Code::InternalError => "Internal Error",
				Code::ServerNotInitialized => "Server Not Initialized",
				Code::UnknownErrorCode => "Unknown",
				Code::RequestFailed => "Request Failed",
				Code::SeverCancelled => "Server Cancelled",
				Code::ContentModified => "Content Modified",
				Code::RequestCancelled => "Request Cancelled",
			}
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
