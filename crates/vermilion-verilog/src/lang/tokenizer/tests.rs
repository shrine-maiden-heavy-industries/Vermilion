// SPDX-License-Identifier: BSD-3-Clause
use paste::paste;

use super::{
	token::{Directive, Keyword, TextMacro},
	*,
};
use crate::LanguageStd;

macro_rules! tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		paste! {
			#[test]
			fn [<test_tokenize_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let tokenizer = VerilogTokenizer::new($std, $input.as_bytes().into()).unwrap();

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
macro_rules! verilog95_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!(
			[<verilog_95_ $test_name>],
			$input,
			$tokens,
			LanguageStd::Vl95
		); }
	};
}

#[allow(unused)]
macro_rules! verilog01_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!(
			[<verilog_01_ $test_name>],
			$input,
			$tokens,
			LanguageStd::Vl01
		); }
	};
}

#[allow(unused)]
macro_rules! verilog05_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!(
			[<verilog_05_ $test_name>],
			$input,
			$tokens,
			LanguageStd::Vl05
		); }
	};
}

#[allow(unused)]
macro_rules! all_verilog_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		verilog95_tokenizer_test!($test_name, $input, $tokens);
		verilog01_tokenizer_test!($test_name, $input, $tokens);
		verilog05_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! verilog95_and_up_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		all_verilog_tokenizer_test!($test_name, $input, $tokens);
		all_system_verilog_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! verilog01_and_up_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		verilog01_tokenizer_test!($test_name, $input, $tokens);
		verilog05_tokenizer_test!($test_name, $input, $tokens);
		all_system_verilog_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! verilog05_and_up_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		verilog05_tokenizer_test!($test_name, $input, $tokens);
		all_system_verilog_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! system_verilog05_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<system_verilog_05_ $test_name>], $input, $tokens, LanguageStd::Sv05); }
	};
}

#[allow(unused)]
macro_rules! system_verilog09_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<system_verilog_09_ $test_name>], $input, $tokens, LanguageStd::Sv09); }
	};
}

#[allow(unused)]
macro_rules! system_verilog12_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<system_verilog_12_ $test_name>], $input, $tokens, LanguageStd::Sv12); }
	};
}

#[allow(unused)]
macro_rules! system_verilog17_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<system_verilog_17_ $test_name>], $input, $tokens, LanguageStd::Sv17); }
	};
}

#[allow(unused)]
macro_rules! system_verilog23_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<system_verilog_23_ $test_name>], $input, $tokens, LanguageStd::Sv23); }
	};
}

#[allow(unused)]
macro_rules! all_system_verilog_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		system_verilog05_tokenizer_test!($test_name, $input, $tokens);
		system_verilog09_tokenizer_test!($test_name, $input, $tokens);
		system_verilog12_tokenizer_test!($test_name, $input, $tokens);
		system_verilog17_tokenizer_test!($test_name, $input, $tokens);
		system_verilog23_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! system_verilog09_and_up_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		system_verilog09_tokenizer_test!($test_name, $input, $tokens);
		system_verilog12_tokenizer_test!($test_name, $input, $tokens);
		system_verilog17_tokenizer_test!($test_name, $input, $tokens);
		system_verilog23_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! system_verilog12_and_up_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		system_verilog12_tokenizer_test!($test_name, $input, $tokens);
		system_verilog17_tokenizer_test!($test_name, $input, $tokens);
		system_verilog23_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! system_verilog17_and_up_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		system_verilog17_tokenizer_test!($test_name, $input, $tokens);
		system_verilog23_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! verilog_ams09_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<verilog_ams_09_ $test_name>], $input, $tokens, LanguageStd::Vams09); }
	};
}

#[allow(unused)]
macro_rules! verilog_ams14_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<verilog_ams_14_ $test_name>], $input, $tokens, LanguageStd::Vams14); }
	};
}

#[allow(unused)]
macro_rules! verilog_ams23_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { tokenizer_test!([<verilog_ams_23_ $test_name>], $input, $tokens, LanguageStd::Vams23); }
	};
}

#[allow(unused)]
macro_rules! all_verilog_ams_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		verilog_ams09_tokenizer_test!($test_name, $input, $tokens);
		verilog_ams14_tokenizer_test!($test_name, $input, $tokens);
		verilog_ams23_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! verilog_ams14_and_up_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		verilog_ams14_tokenizer_test!($test_name, $input, $tokens);
		verilog_ams23_tokenizer_test!($test_name, $input, $tokens);
	};
}

#[allow(unused)]
macro_rules! all_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {
			all_verilog_tokenizer_test!([<$test_name _all>], $input, $tokens);
			all_system_verilog_tokenizer_test!([<$test_name _all>], $input, $tokens);
			all_verilog_ams_tokenizer_test!([<$test_name _all>], $input, $tokens);
		}
	};
}

// Directive map/set Tests
include!("./tests/directives.rs");
// Keyword map/set Tests
include!("./tests/keywords.rs");
// Common Tokenizer Tests
include!("./tests/all_common.rs");
// Verilog Tokenizer Tests
include!("./tests/verilog_common.rs");
include!("./tests/verilog95.rs");
include!("./tests/verilog05.rs");
include!("./tests/verilog01.rs");
// SystemVerilog Tokenizer Tests
include!("./tests/system_verilog_common.rs");
include!("./tests/system_verilog05.rs");
include!("./tests/system_verilog09.rs");
include!("./tests/system_verilog12.rs");
include!("./tests/system_verilog17.rs");
include!("./tests/system_verilog23.rs");
// Verilog-AMS Tokenizer Tests
include!("./tests/verilog_ams_common.rs");
include!("./tests/verilog_ams09.rs");
include!("./tests/verilog_ams14.rs");
include!("./tests/verilog_ams23.rs");
