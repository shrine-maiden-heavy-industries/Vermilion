// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use vermilion_lang::AtomicByteTendril;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Token {
	Invalid(Option<AtomicByteTendril>),
	Newline(AtomicByteTendril),
	Whitespace(AtomicByteTendril),
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Invalid(_tendril) => todo!(),
			Self::Newline(tendril) => write!(f, "Newline({})", tendril.len()),
			Self::Whitespace(tendril) => write!(f, "Whitespace({})", unsafe {
				str::from_utf8_unchecked(tendril)
			}),
		}
	}
}

impl Default for Token {
	fn default() -> Self {
		Self::Invalid(None)
	}
}

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
