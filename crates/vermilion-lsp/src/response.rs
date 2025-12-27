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

impl Response {}

#[cfg(test)]
mod tests {
	use super::*;
}
