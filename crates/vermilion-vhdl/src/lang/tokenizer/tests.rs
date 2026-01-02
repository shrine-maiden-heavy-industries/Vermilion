/* SPDX-License-Identifier: BSD-3-Clause */

use super::*;
use crate::VhdlStd;

macro_rules! tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		#[test]
		fn $test_name() {
			let tokenizer = Tokenizer::new(VhdlStd::Vh87, $input.as_bytes().into());

			let parsed = tokenizer.collect::<Vec<_>>();
			let expected = $tokens;

			assert_eq!(
				parsed, expected,
				"Parsed: {parsed:#?}\nExpected: {expected:#?}"
			);
		}
	};
}
