// SPDX-License-Identifier: BSD-3-Clause

// TODO(aki): Write tests
#[allow(unused, reason = "Currently unused")]
use paste::paste;

// TODO(aki): Write tests
#[allow(unused, reason = "Currently unused")]
use crate::{LanguageStd, lang::parser::*};

// TODO(aki): Write tests
#[allow(unused, reason = "Currently unused")]
macro_rules! parser_test {
	(liberty2014_09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2014_09_ $test_name>],
			$input,
			LanguageStd::Liberty2014_09,
			$($tokens),+
		); }
	};
	(liberty2015_12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2015_12_ $test_name>],
			$input,
			LanguageStd::Liberty2015_12,
			$($tokens),+
		); }
	};
	(liberty2015_12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2015_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2016_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2016_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2016_06, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2016_06_ $test_name>],
			$input,
			LanguageStd::Liberty2016_06,
			$($tokens),+
		); }
	};
	(liberty2016_06+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2016_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2016_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2016_12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2016_12_ $test_name>],
			$input,
			LanguageStd::Liberty2016_12,
			$($tokens),+
		); }
	};
	(liberty2016_12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2016_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2017_06, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2017_06_ $test_name>],
			$input,
			LanguageStd::Liberty2017_06,
			$($tokens),+
		); }
	};
	(liberty2017_06+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2018_06, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2018_06_ $test_name>],
			$input,
			LanguageStd::Liberty2018_06,
			$($tokens),+
		); }
	};
	(liberty2018_06+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2019_03, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2019_03_ $test_name>],
			$input,
			LanguageStd::Liberty2019_03,
			$($tokens),+
		); }
	};
	(liberty2019_03+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		parser_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2019_12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2019_12_ $test_name>],
			$input,
			LanguageStd::Liberty2019_12,
			$($tokens),+
		); }
	};
	(liberty2019_12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2020_09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<liberty_2020_09_ $test_name>],
			$input,
			LanguageStd::Liberty2014_09,
			$($tokens),+
		); }
	};
	(liberty2020_09+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! {
			parser_test!(liberty2014_09, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2015_12, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2016_06, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2016_12, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2017_06, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2018_06, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2019_03, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2019_12, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(liberty2020_09, [<$test_name _all>], $input, $($tokens),+);
		}
	};
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_parse_ $test_name>] () {
				#[allow(
					clippy::unwrap_used,
					reason = "We are within tests, and while this shouldn't explode, if it does here that's fine"
				)]
				let parser = LibertyParser::new($std, $input.as_bytes().into()).unwrap();

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

include!("./tests/liberty2014_09.rs");
include!("./tests/liberty2015_12.rs");
include!("./tests/liberty2016_06.rs");
include!("./tests/liberty2016_12.rs");
include!("./tests/liberty2017_06.rs");
include!("./tests/liberty2018_06.rs");
include!("./tests/liberty2019_03.rs");
include!("./tests/liberty2019_12.rs");
include!("./tests/liberty2020_09.rs");
