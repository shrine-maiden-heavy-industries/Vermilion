// SPDX-License-Identifier: BSD-3-Clause
#[allow(unused)]
use paste::paste;

use crate::{LanguageStd, lang::parser::VerilogParser};

#[allow(unused)]
macro_rules! parser_test {
	(verilog95, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<verilog_95_ $test_name>],
			$input,
			LanguageStd::Vl95,
			$($tokens),+
		); }
	};
	(verilog01, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<verilog_01_ $test_name>],
			$input,
			LanguageStd::Vl01,
			$($tokens),+
		); }
	};
	(verilog01+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(verilog01, $test_name, $input, $($tokens),+);
		parser_test!(verilog05, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog_all, $test_name, $input, $($tokens),+);
	};
	(verilog05, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<verilog_05_ $test_name>],
			$input,
			LanguageStd::Vl05,
			$($tokens),+
		); }
	};
	(verilog05+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(verilog05, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog_all, $test_name, $input, $($tokens),+);
	};
	(verilog_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(verilog95, $test_name, $input, $($tokens),+);
		parser_test!(verilog01, $test_name, $input, $($tokens),+);
		parser_test!(verilog05, $test_name, $input, $($tokens),+);
	};
	(system_verilog05, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<system_verilog_05_ $test_name>],
			$input,
			LanguageStd::Sv05,
			$($tokens),+
		); }
	};
	(system_verilog09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<system_verilog_09_ $test_name>],
			$input,
			LanguageStd::Sv09,
			$($tokens),+
		); }
	};
	(system_verilog09+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(system_verilog09, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog12, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog17, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog12, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<system_verilog_12_ $test_name>],
			$input,
			LanguageStd::Sv12,
			$($tokens),+
		); }
	};
	(system_verilog12+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(system_verilog12, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog17, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog17, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<system_verilog_17_ $test_name>],
			$input,
			LanguageStd::Sv17,
			$($tokens),+
		); }
	};
	(system_verilog17+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(system_verilog17, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog23, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<system_verilog_23_ $test_name>],
			$input,
			LanguageStd::Sv23,
			$($tokens),+
		); }
	};
	(system_verilog23+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(system_verilog_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(system_verilog05, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog09, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog12, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog17, $test_name, $input, $($tokens),+);
		parser_test!(system_verilog23, $test_name, $input, $($tokens),+);
	};
	(verilog_ams09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<verilog_ams_09_ $test_name>],
			$input,
			LanguageStd::Vams09,
			$($tokens),+
		); }
	};
	(verilog_ams14, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<verilog_ams_14_ $test_name>],
			$input,
			LanguageStd::Vams14,
			$($tokens),+
		); }
	};
	(verilog_ams14+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(verilog_ams14, $test_name, $input, $($tokens),+);
		parser_test!(verilog_ams23, $test_name, $input, $($tokens),+);
	};
	(verilog_ams23, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { parser_test!(
			[<verilog_ams_23_ $test_name>],
			$input,
			LanguageStd::Vams23,
			$($tokens),+
		); }
	};
	(verilog_ams23+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(verilog_ams23, $test_name, $input, $($tokens),+);
	};
	(verilog_ams_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		parser_test!(verilog_ams09, $test_name, $input, $($tokens),+);
		parser_test!(verilog_ams14, $test_name, $input, $($tokens),+);
		parser_test!(verilog_ams23, $test_name, $input, $($tokens),+);
	};
	(all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! {
			parser_test!(verilog_all, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(system_verilog_all, [<$test_name _all>], $input, $($tokens),+);
			parser_test!(verilog_ams_all, [<$test_name _all>], $input, $($tokens),+);
		}
	};
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_parse_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let parser = VerilogParser::new($std, $input.as_bytes().into()).unwrap();

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

// Verilog Parser Tests
include!("./tests/verilog95.rs");
include!("./tests/verilog05.rs");
include!("./tests/verilog01.rs");
// SystemVerilog Parser Tests
include!("./tests/system_verilog05.rs");
include!("./tests/system_verilog09.rs");
include!("./tests/system_verilog12.rs");
include!("./tests/system_verilog17.rs");
include!("./tests/system_verilog23.rs");
// Verilog-AMS Parser Tests
include!("./tests/verilog_ams09.rs");
include!("./tests/verilog_ams14.rs");
include!("./tests/verilog_ams23.rs");
