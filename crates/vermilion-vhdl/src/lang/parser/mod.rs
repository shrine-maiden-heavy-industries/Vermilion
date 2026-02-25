// SPDX-License-Identifier: BSD-3-Clause

use vermilion_lang::AtomicByteTendril;

use crate::LanguageStd;

pub mod error;

pub struct VhdlParser {
	_std:     LanguageStd,
	_content: AtomicByteTendril,
}

impl VhdlParser {
	pub fn new(_std: LanguageStd, _content: AtomicByteTendril) -> eyre::Result<Self> {
		Ok(Self { _std, _content })
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
