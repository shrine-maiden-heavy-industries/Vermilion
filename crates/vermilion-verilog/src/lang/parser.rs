/* SPDX-License-Identifier: BSD-3-Clause */

use crate::VerilogVariant;

pub struct VerilogParser {
	std: VerilogVariant,
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_log::test;
}
