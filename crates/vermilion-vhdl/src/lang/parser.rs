/* SPDX-License-Identifier: BSD-3-Clause */

use tendril::ByteTendril;

use crate::VhdlStd;

pub struct VhdlParser {
	_std: VhdlStd,
}

impl VhdlParser {
	pub fn new(std: VhdlStd, content: ByteTendril) -> Self {
		Self { _std: std }
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
	// use test_log::test;
}
