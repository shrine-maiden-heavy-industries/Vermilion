// SPDX-License-Identifier: BSD-3-Clause
#[allow(unused)]
use paste::paste;

#[allow(unused)]
use crate::{
	LanguageStd,
	lang::tokenizer::{token::Keyword, *},
};

#[allow(unused)]
macro_rules! tokenizer_test {
	(vhdl87, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_87_ $test_name>],
			$input,
			LanguageStd::Vh87,
			$($tokens),+
		); }
	};
	(vhdl93, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_93_ $test_name>],
			$input,
			LanguageStd::Vh93,
			$($tokens),+
		); }
	};
	(vhdl93+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl93, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl2k, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl02, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl04, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl08, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl2k, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_2k_ $test_name>],
			$input,
			LanguageStd::Vh2k,
			$($tokens),+
		); }
	};
	(vhdl2k+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl2k, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl02, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl04, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl08, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl02, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_02_ $test_name>],
			$input,
			LanguageStd::Vh02,
			$($tokens),+
		); }
	};
	(vhdl02+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl02, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl04, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl08, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl04, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_04_ $test_name>],
			$input,
			LanguageStd::Vh04,
			$($tokens),+
		); }
	};
	(vhdl04+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl04, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl08, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl07, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_07_ $test_name>],
			$input,
			LanguageStd::Vh07,
			$($tokens),+
		); }
	};
	(vhdl07+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl08, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl08, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_08_ $test_name>],
			$input,
			LanguageStd::Vh08,
			$($tokens),+
		); }
	};
	(vhdl08+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl08, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl11, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_11_ $test_name>],
			$input,
			LanguageStd::Vh11,
			$($tokens),+
		); }
	};
	(vhdl11+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl19, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_19_ $test_name>],
			$input,
			LanguageStd::Vh19,
			$($tokens),+
		); }
	};
	(vhdl19+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl23, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_23_ $test_name>],
			$input,
			LanguageStd::Vh23,
			$($tokens),+
		); }
	};
	(vhdl23+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl87, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl93, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl2k, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl02, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl04, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl08, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl11, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl19, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl23, $test_name, $input, $($tokens),+);
	};
	(vhdl_ams99, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_ams_99_ $test_name>],
			$input,
			LanguageStd::Vhams99,
			$($tokens),+
		); }
	};
	(vhdl_ams07, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_ams_07_ $test_name>],
			$input,
			LanguageStd::Vhams07,
			$($tokens),+
		); }
	};
	(vhdl_ams07+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl_ams07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams09, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams17, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams21, $test_name, $input, $($tokens),+);
	};
	(vhdl_ams09, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_ams_09_ $test_name>],
			$input,
			LanguageStd::Vhams09,
			$($tokens),+
		); }
	};
	(vhdl_ams09+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl_ams09, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams17, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams21, $test_name, $input, $($tokens),+);
	};
	(vhdl_ams17, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_ams_17_ $test_name>],
			$input,
			LanguageStd::Vhams17,
			$($tokens),+
		); }
	};
	(vhdl_ams17+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl_ams17, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams21, $test_name, $input, $($tokens),+);
	};
	(vhdl_ams21, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! { tokenizer_test!(
			[<vhdl_ams_21_ $test_name>],
			$input,
			LanguageStd::Vhams21,
			$($tokens),+
		); }
	};
	(vhdl_ams21+, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl_ams21, $test_name, $input, $($tokens),+);
	};
	(vhdl_ams_all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		tokenizer_test!(vhdl_ams99, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams07, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams09, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams17, $test_name, $input, $($tokens),+);
		tokenizer_test!(vhdl_ams21, $test_name, $input, $($tokens),+);
	};
	(all, $test_name:ident, $input:literal, $($tokens:expr),+) => {
		paste! {
			tokenizer_test!(vhdl_all, [<$test_name _all>], $input, $($tokens),+);
			tokenizer_test!(vhdl_ams_all, [<$test_name _all>], $input, $($tokens),+);
		}
	};
	($test_name:ident, $input:literal, $std:expr, $($tokens:expr),+) => {
		paste! {
			#[test]
			fn [<test_tokenize_ $test_name>] () {
				// SAFETY: We're in a testing context, panicking here is fine
				#[allow(clippy::unwrap_used)]
				let tokenizer = VhdlTokenizer::new($std, $input.as_bytes().into());

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
