/* SPDX-License-Identifier: BSD-3-Clause */

#[derive(Clone, Debug, Hash, serde::Deserialize, serde::Serialize)]
pub struct Notification {
	method: String,
	#[serde(default = "serde_json::Value::default")]
	#[serde(skip_serializing_if = "serde_json::Value::is_null")]
	params: serde_json::Value,
}

#[cfg(test)]
mod tests {
	use super::*;
}
