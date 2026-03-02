// SPDX-License-Identifier: BSD-3-Clause

#[allow(unused)]
use paste::paste;

#[allow(unused)]
use crate::{LanguageStd, sdc::lang::parser::*};

#[allow(unused)]
macro_rules! parser_test {
	(sdc, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<sdc_ $test_name>],
			$input,
			LanguageStd::Sdc,
			$($tokens),+
		); }
	};
	(xdc, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<xdc_ $test_name>],
			$input,
			LanguageStd::Xdc,
			$($tokens),+
		); }
	};
	(all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! {
			tokenizer_test!(sdc, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(xdc, [<$test_name _all>], $input, $($tokens),+);
		}
	};
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_parse_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let parser = SdcParser::new($std, $input.as_bytes().into()).unwrap();

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

include!("./tests/sdc.rs");
include!("./tests/xdc.rs");
