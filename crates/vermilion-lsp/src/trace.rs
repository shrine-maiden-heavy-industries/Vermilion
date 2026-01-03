// SPDX-License-Identifier: BSD-3-Clause

use crate::message::Message;

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Origin {
	Server,
	Client,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Trace {
	pub(crate) origin:    Origin,
	pub(crate) message:   Message,
	pub(crate) timestamp: chrono::DateTime<chrono::Utc>,
}

impl Trace {
	pub fn new(origin: Origin, message: &Message) -> Self {
		Self {
			origin,
			message: message.clone(),
			timestamp: chrono::Utc::now(),
		}
	}

	pub fn origin(&self) -> &Origin {
		&self.origin
	}

	pub fn message(&self) -> &Message {
		&self.message
	}

	pub fn timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
		&self.timestamp
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
