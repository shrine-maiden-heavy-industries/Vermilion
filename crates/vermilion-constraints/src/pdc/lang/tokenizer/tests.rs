// SPDX-License-Identifier: BSD-3-Clause

// TODO(aki): Write tests
#[allow(unused, reason = "Currently unused")]
use paste::paste;

// TODO(aki): Write tests
#[allow(unused, reason = "Currently unused")]
use crate::{LanguageStd, pdc::lang::tokenizer::*};

// TODO(aki): Write tests
#[allow(unused, reason = "Currently unused")]
macro_rules! tokenizer_test {
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_tokenize_pdc_ $test_name>] () {
				#[allow(
					clippy::unwrap_used,
					reason = "We are within tests, and while this shouldn't explode, if it does here that's fine"
				)]
				let tokenizer = PdcTokenizer::new($std, $input.as_bytes().into()).unwrap();

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

include!("./tests/pdc.rs");
