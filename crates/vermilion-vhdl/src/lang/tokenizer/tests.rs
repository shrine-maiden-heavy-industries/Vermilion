// SPDX-License-Identifier: BSD-3-Clause
#[allow(unused)]
use paste::paste;

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::LanguageStd;

#[allow(unused)]
macro_rules! tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		paste! {
			#[test]
			fn [<test_tokenize_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let tokenizer = VhdlTokenizer::new($std, $input.as_bytes().into());

				let parsed = tokenizer.collect::<Vec<_>>();
				let expected = $tokens;

				assert_eq!(
					parsed, expected,
					"Parsed: {parsed:#?}\nExpected: {expected:#?}"
				);
			}
		}
	};
}

#[allow(unused)]
macro_rules! vhdl87_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_87_ $test_name>], $input, $tokens, LanguageStd::Vh87); }
	};
}

#[allow(unused)]
macro_rules! vhdl93_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_93_ $test_name>], $input, $tokens, LanguageStd::Vh93); }
	};
}

#[allow(unused)]
macro_rules! vhdl2k_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {  tokenizer_test!([<vhdl_2k_ $test_name>], $input, $tokens, LanguageStd::Vh2k); }
	};
}

#[allow(unused)]
macro_rules! vhdl02_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_02_ $test_name>], $input, $tokens, LanguageStd::Vh02); }
	};
}

#[allow(unused)]
macro_rules! vhdl04_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_04_ $test_name>], $input, $tokens, LanguageStd::Vh04); }
	};
}

#[allow(unused)]
macro_rules! vhdl08_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_08_ $test_name>], $input, $tokens, LanguageStd::Vh08); }
	};
}

#[allow(unused)]
macro_rules! vhdl11_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_11_ $test_name>], $input, $tokens, LanguageStd::Vh11); }
	};
}

#[allow(unused)]
macro_rules! vhdl19_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_19_ $test_name>], $input, $tokens, LanguageStd::Vh19); }
	};
}

#[allow(unused)]
macro_rules! vhdl23_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_23_ $test_name>], $input, $tokens, LanguageStd::Vh23); }
	};
}

#[allow(unused)]
macro_rules! vhdl_ams_99_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_ams_99_ $test_name>], $input, $tokens, LanguageStd::Vhams99); }
	};
}

#[allow(unused)]
macro_rules! vhdl_ams_07_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_ams_07_ $test_name>], $input, $tokens, LanguageStd::Vhams07); }
	};
}

#[allow(unused)]
macro_rules! vhdl_ams_09_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_ams_09_ $test_name>], $input, $tokens, LanguageStd::Vhams09); }
	};
}

#[allow(unused)]
macro_rules! vhdl_ams_17_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_ams_17_ $test_name>], $input, $tokens, LanguageStd::Vhams17); }
	};
}

#[allow(unused)]
macro_rules! vhdl_ams_21_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<vhdl_ams_21_ $test_name>], $input, $tokens, LanguageStd::Vhams21); }
	};
}

#[allow(unused)]
macro_rules! all_vhdl_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {
			vhdl87_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl93_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl2k_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl02_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl04_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl08_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl11_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl19_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl23_tokenizer_test([<$test_name _all>], $input, $tokens);
		}
	};
}

#[allow(unused)]
macro_rules! all_vhdl_ams_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {
			vhdl_ams_99_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl_ams_17_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl_ams_09_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl_ams_17_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl_ams_21_tokenizer_test([<$test_name _all>], $input, $tokens);
		}
	};
}

#[allow(unused)]
macro_rules! all_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {
			all_vhdl_tokenizer_test!([<$test_name _all>], $input, $tokens);
			all_vhdl_ams_tokenizer_test!([<$test_name _all>], $input, $tokens);
		}
	};
}

// Common Tokenizer Tests
include!("./tests/all_common.rs");
// VHDL Tokenizer Tests
include!("./tests/vhdl_common.rs");
include!("./tests/vhdl87.rs");
include!("./tests/vhdl93.rs");
include!("./tests/vhdl2k.rs");
include!("./tests/vhdl02.rs");
include!("./tests/vhdl04.rs");
include!("./tests/vhdl07.rs");
include!("./tests/vhdl08.rs");
include!("./tests/vhdl11.rs");
include!("./tests/vhdl19.rs");
include!("./tests/vhdl23.rs");
// VHDL-AMS Tokenizer Tests
include!("./tests/vhdl_ams_common.rs");
include!("./tests/vhdl_ams99.rs");
include!("./tests/vhdl_ams07.rs");
include!("./tests/vhdl_ams09.rs");
include!("./tests/vhdl_ams17.rs");
include!("./tests/vhdl_ams21.rs");
