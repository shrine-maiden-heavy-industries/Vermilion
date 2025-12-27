/* SPDX-License-Identifier: BSD-3-Clause */

use crate::message::Id;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Request {
	pub(crate) id: Id,
	pub(crate) method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) params: Option<serde_json::Value>,
}

impl Request {}

#[cfg(test)]
mod tests {
	use super::*;
}
