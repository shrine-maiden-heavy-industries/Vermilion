/* SPDX-License-Identifier: BSD-3-Clause */

use crate::message::Message;

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum MessageKind {
	Request,
	Response,
	Error,
	Notification,
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum MessageOrigin {
	Server,
	Client,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct TraceMessage {
	#[serde(rename = "msgKind")]
	kind: MessageKind,
	#[serde(rename = "from")]
	origin: MessageOrigin,
	#[serde(skip_serializing_if = "Option::is_none")]
	method: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	id: Option<u64>,
	timestamp: chrono::DateTime<chrono::Utc>,
	#[serde(rename = "msg")]
	message: Message,
}

#[cfg(test)]
mod tests {
	use super::*;
}
