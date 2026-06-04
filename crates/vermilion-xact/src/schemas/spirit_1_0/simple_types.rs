// SPDX-License-Identifier: BSD-3-Clause

use crate::schemas::xs;

/// Number format used by SPIRIT.  Supports any string handled by java.lang.Long.decode().  Also
/// supports magnitude suffix of upper or lower case K, M, G or T.
///
/// validation-regex: -?((0x)|(0X)|#)?[0-9a-fA-F]*[kmgtKMGT]?
pub type SpiritNumber = xs::String; // TODO(aki): Properly new-type and wrap/validate/serde

/// List of doubles.  Used to build configurableDouble type.
pub type DoubleList = Vec<xs::Double>; // TODO(aki): Properly new-type and wrap/validate/serde

/// Can be a double or an empty string.
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(::schemars::JsonSchema))]
pub enum ConfigurableDouble {
	// XXX(aki): This is specced as a spirit:doubleList with a min len of 0 and max of 1...
	Double(xs::Double),
	#[serde(rename = "$text")]
	Text(String),
}
