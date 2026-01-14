// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::AtomicByteTendril;

use crate::VhdlStd;

pub struct VhdlParser {
	_std:     VhdlStd,
	_content: AtomicByteTendril,
}

impl VhdlParser {
	pub fn new(_std: VhdlStd, _content: AtomicByteTendril) -> Self {
		Self { _std, _content }
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
