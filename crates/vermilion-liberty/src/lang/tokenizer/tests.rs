// SPDX-License-Identifier: BSD-3-Clause

#[allow(unused)]
use paste::paste;

#[allow(unused)]
use crate::{LanguageStd, lang::tokenizer::*};

#[allow(unused)]
macro_rules! tokenizer_test {
	(liberty2014_09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2014_09_ $test_name>],
			$input,
			LanguageStd::liberty2014_09,
			$($tokens),+
		); }
	};
	(liberty2015_12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2015_12_ $test_name>],
			$input,
			LanguageStd::liberty2015_12,
			$($tokens),+
		); }
	};
	(liberty2015_12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2015_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2016_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2016_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2016_06, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2016_06_ $test_name>],
			$input,
			LanguageStd::liberty2016_06,
			$($tokens),+
		); }
	};
	(liberty2016_06+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2016_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2016_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2016_12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2016_12_ $test_name>],
			$input,
			LanguageStd::liberty2016_12,
			$($tokens),+
		); }
	};
	(liberty2016_12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2016_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2017_06, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2017_06_ $test_name>],
			$input,
			LanguageStd::liberty2017_06,
			$($tokens),+
		); }
	};
	(liberty2017_06+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2017_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2018_06, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2018_06_ $test_name>],
			$input,
			LanguageStd::liberty2018_06,
			$($tokens),+
		); }
	};
	(liberty2018_06+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2018_06, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2019_03, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2019_03_ $test_name>],
			$input,
			LanguageStd::liberty2019_03,
			$($tokens),+
		); }
	};
	(liberty2019_03+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2019_03, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2019_12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2019_12_ $test_name>],
			$input,
			LanguageStd::liberty2019_12,
			$($tokens),+
		); }
	};
	(liberty2019_12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2019_12, $test_name, $input, $($tokens),+);
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(liberty2020_09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<liberty_2020_09_ $test_name>],
			$input,
			LanguageStd::liberty2014_09,
			$($tokens),+
		); }
	};
	(liberty2020_09+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(liberty2020_09, $test_name, $input, $($tokens),+);
	};
	(all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! {
			tokenizer_test!(liberty2014_09, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2015_12, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2016_06, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2016_12, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2017_06, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2018_06, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2019_03, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2019_12, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(liberty2020_09, [<$test_name _all>], $input, $($tokens),+);
		}
	};
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_tokenize_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let tokenizer = LibertyTokenizer::new($std, $input.as_bytes().into()).unwrap();

				let parsed = tokenizer.collect::<Vec<_>>();
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
