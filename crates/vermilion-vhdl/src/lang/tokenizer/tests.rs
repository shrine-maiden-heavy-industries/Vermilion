// SPDX-License-Identifier: BSD-3-Clause

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

include!("./tests/vhdl87.rs");
include!("./tests/vhdl93.rs");
include!("./tests/vhdl2k.rs");
include!("./tests/vhdl02.rs");
include!("./tests/vhdl07.rs");
include!("./tests/vhdl08.rs");
include!("./tests/vhdl11.rs");
include!("./tests/vhdl19.rs");
include!("./tests/vhdl23.rs");
