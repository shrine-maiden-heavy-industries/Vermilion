/* SPDX-License-Identifier: BSD-3-Clause */

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Notification {
	pub(crate) method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) params: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
	use super::*;
}
