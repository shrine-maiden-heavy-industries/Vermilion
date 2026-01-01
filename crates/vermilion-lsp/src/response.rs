/* SPDX-License-Identifier: BSD-3-Clause */

use crate::{error::Error, message::Id};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Response {
	pub(crate) id: Id,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) result: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) error: Option<Error>,
}

impl Response {
	pub fn new(id: Id) -> Self {
		Self { id, result: None, error: None }
	}

	pub fn with_result(mut self, result: serde_json::Value) -> Self {
		self.result = Some(result);
		self
	}

	pub fn with_error(mut self, error: Error) -> Self {
		self.error = Some(error);
		self
	}

	pub fn id(&self) -> &Id {
		&self.id
	}

	pub fn result(&self) -> Option<&serde_json::Value> {
		self.result.as_ref()
	}

	pub fn error(&self) -> Option<&Error> {
		self.error.as_ref()
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
