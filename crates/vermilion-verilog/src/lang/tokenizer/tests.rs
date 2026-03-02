// SPDX-License-Identifier: BSD-3-Clause
use paste::paste;

use crate::{
	LanguageStd,
	lang::tokenizer::{
		token::{Directive, Keyword, TextMacro},
		*,
	},
};

macro_rules! tokenizer_test {
	(verilog95, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<verilog_95_ $test_name>],
			$input,
			LanguageStd::Vl95,
			$($tokens),+
		); }
	};
	(verilog01, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<verilog_01_ $test_name>],
			$input,
			LanguageStd::Vl01,
			$($tokens),+
		); }
	};
	(verilog01+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(verilog01, $test_name, $input, $($tokens),+);
		tokenizer_test!(verilog05, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog_all, $test_name, $input, $($tokens),+);
	};
	(verilog05, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<verilog_05_ $test_name>],
			$input,
			LanguageStd::Vl05,
			$($tokens),+
		); }
	};
	(verilog05+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(verilog05, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog_all, $test_name, $input, $($tokens),+);
	};
	(verilog_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(verilog95, $test_name, $input, $($tokens),+);
		tokenizer_test!(verilog01, $test_name, $input, $($tokens),+);
		tokenizer_test!(verilog05, $test_name, $input, $($tokens),+);
	};
	(system_verilog05, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<system_verilog_05_ $test_name>],
			$input,
			LanguageStd::Sv05,
			$($tokens),+
		); }
	};
	(system_verilog09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<system_verilog_09_ $test_name>],
			$input,
			LanguageStd::Sv09,
			$($tokens),+
		); }
	};
	(system_verilog09+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(system_verilog09, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog12, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog17, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<system_verilog_12_ $test_name>],
			$input,
			LanguageStd::Sv12,
			$($tokens),+
		); }
	};
	(system_verilog12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(system_verilog12, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog17, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog17, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<system_verilog_17_ $test_name>],
			$input,
			LanguageStd::Sv17,
			$($tokens),+
		); }
	};
	(system_verilog17+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(system_verilog17, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog23, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<system_verilog_23_ $test_name>],
			$input,
			LanguageStd::Sv23,
			$($tokens),+
		); }
	};
	(system_verilog23+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(system_verilog05, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog09, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog12, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog17, $test_name, $input, $($tokens),+);
		tokenizer_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(verilog_ams09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<verilog_ams_09_ $test_name>],
			$input,
			LanguageStd::Vams09,
			$($tokens),+
		); }
	};
	(verilog_ams14, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<verilog_ams_14_ $test_name>],
			$input,
			LanguageStd::Vams14,
			$($tokens),+
		); }
	};
	(verilog_ams14+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(verilog_ams14, $test_name, $input, $($tokens),+);
		tokenizer_test!(verilog_ams23, $test_name, $input, $($tokens),+);
	};
	(verilog_ams23, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<verilog_ams_23_ $test_name>],
			$input,
			LanguageStd::Vams23,
			$($tokens),+
		); }
	};
	(verilog_ams23+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(verilog_ams23, $test_name, $input, $($tokens),+);
	};
	(verilog_ams_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(verilog_ams09, $test_name, $input, $($tokens),+);
		tokenizer_test!(verilog_ams14, $test_name, $input, $($tokens),+);
		tokenizer_test!(verilog_ams23, $test_name, $input, $($tokens),+);
	};
	(all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! {
			tokenizer_test!(verilog_all, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(system_verilog_all, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(verilog_ams_all, [<$test_name _all>], $input, $($tokens),+);
		}
	};
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_tokenize_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let tokenizer = VerilogTokenizer::new($std, $input.as_bytes().into()).unwrap();

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
