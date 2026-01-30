// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::AtomicByteTendril;

use crate::VhdlVariant;

pub mod error;

pub struct VhdlParser {
	_std:     VhdlVariant,
	_content: AtomicByteTendril,
}

impl VhdlParser {
	pub fn new(_std: VhdlVariant, _content: AtomicByteTendril) -> Self {
		Self { _std, _content }
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
