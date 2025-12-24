/* SPDX-License-Identifier: BSD-3-Clause */

use std::fmt::{Debug, Display};

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

impl Message {}

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
