// SPDX-License-Identifier: BSD-3-Clause

#[allow(unused)]
use paste::paste;

#[allow(unused)]
use crate::{LanguageStd, pdc::lang::parser::*};

#[allow(unused)]
macro_rules! parser_test {
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_parse_lpf_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let parser = PdcParser::new($std, $input.as_bytes().into()).unwrap();

				let parsed = parser.collect::<Vec<_>>();
				let expected = vec![$($tokens),+];

				assert_eq!(
					parsed, expected,
					"Parsed: {parsed:#?}\nExpected: {expected:#?}"
				);
			}
		}
	};
}

include!("./tests/pdc.rs");
