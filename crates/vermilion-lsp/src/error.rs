/* SPDX-License-Identifier: BSD-3-Clause */

use std::fmt::Display;

#[derive(Clone, Copy, Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(i32)]
pub enum JsonRpcError {
	ParseError = -32700,
	InvalidRequest = -32600,
	MethodNotFound = -32601,
	InvalidParams = -32602,
	InternalError = -32603,
	ServerNotInitialized = -32002,
	UnknownErrorCode = -32001,
}

#[derive(Clone, Copy, Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(i32)]
pub enum LspError {
	RequestFailed = -32803,
	SeverCancelled = -32802,
	ContentModified = -32801,
	RequestCancelled = -32800,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Error {
	JsonRpc(JsonRpcError),
	Lsp(LspError),
}

impl Display for JsonRpcError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::ParseError => "Parse Error",
				Self::InvalidRequest => "Invalid Request",
				Self::MethodNotFound => "Method Not Found",
				Self::InvalidParams => "Invalid Params",
				Self::InternalError => "Internal Errors",
				Self::ServerNotInitialized => "Server Not Initialized",
				Self::UnknownErrorCode => "Unknown",
			}
		)
	}
}

impl Display for LspError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::RequestFailed => "Request Failed",
				Self::SeverCancelled => "Served Cancelled",
				Self::ContentModified => "Content Modified",
				Self::RequestCancelled => "Request Cancelled",
			}
		)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::JsonRpc(e) => write!(f, "JSON-RPC Error: {}", e),
			Self::Lsp(e) => write!(f, "LSP Error: {}", e),
		}
	}
}
