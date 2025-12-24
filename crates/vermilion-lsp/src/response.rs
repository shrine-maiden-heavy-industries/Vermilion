/* SPDX-License-Identifier: BSD-3-Clause */

use crate::{error::Error, message::Id};

#[derive(Clone, Debug, Hash, serde::Deserialize, serde::Serialize)]
pub struct Response {
	id: Id,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	result: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	error: Option<Error>,
}

#[cfg(test)]
mod tests {
	use super::*;
}
