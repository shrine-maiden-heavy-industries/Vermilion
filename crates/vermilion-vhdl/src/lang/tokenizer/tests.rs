// SPDX-License-Identifier: BSD-3-Clause
use paste::paste;

use super::*;
use crate::{VhdlAmsStd, VhdlStd, VhdlVariant};

macro_rules! tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $variant:expr) => {
		paste! {
			#[test]
			fn [<test_tokenize_ $test_name>] () {
				let tokenizer = VhdlTokenizer::new($variant, $input.as_bytes().into());

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

macro_rules! vhdl_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		paste! { tokenizer_test!(
				[ <vhdl $test_name> ],
				$input,
				$tokens,
				VhdlVariant::Vhdl($std)
			);
		}
	};
}

macro_rules! vhdl_ams_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		paste! { tokenizer_test!(
				[ <vhdl_ams $test_name> ],
				$input,
				$tokens,
				VhdlVariant::VhdlAms($std)
			);
		}
	};
}

macro_rules! vhdl87_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_tokenizer_test!([<_87_ $test_name>], $input, $tokens, VhdlStd::Vh87); }
	};
}

macro_rules! vhdl93_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_tokenizer_test!([<_93_ $test_name>], $input, $tokens, VhdlStd::Vh93); }
	};
}

macro_rules! vhdl2k_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {  vhdl_tokenizer_test!([<_2k_ $test_name>], $input, $tokens, VhdlStd::Vh2k); }
	};
}

macro_rules! vhdl02_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_tokenizer_test!([<_02_ $test_name>], $input, $tokens, VhdlStd::Vh02); }
	};
}

macro_rules! vhdl08_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_tokenizer_test!([<_08_ $test_name>], $input, $tokens, VhdlStd::Vh08); }
	};
}

macro_rules! vhdl11_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_tokenizer_test!([<_11_ $test_name>], $input, $tokens, VhdlStd::Vh11); }
	};
}

macro_rules! vhdl19_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_tokenizer_test!([<_19_ $test_name>], $input, $tokens, VhdlStd::Vh19); }
	};
}

macro_rules! vhdl23_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_tokenizer_test!([<_23_ $test_name>], $input, $tokens, VhdlStd::Vh23); }
	};
}

macro_rules! vhdl_ams_99_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_ams_tokenizer_test!([<_99_ $test_name>], $input, $tokens, VhdlAms::Vhams99); }
	};
}

macro_rules! vhdl_ams_07_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_ams_tokenizer_test!([<_07_ $test_name>], $input, $tokens, VhdlAms::Vhams07); }
	};
}

macro_rules! vhdl_ams_09_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_ams_tokenizer_test!([<_09_ $test_name>], $input, $tokens, VhdlAms::Vhams09); }
	};
}

macro_rules! vhdl_ams_17_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { vhdl_ams_tokenizer_test!([<_17_ $test_name>], $input, $tokens, VhdlAms::Vhams17); }
	};
}

macro_rules! all_vhdl_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {
			vhdl87_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl93_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl2k_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl02_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl08_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl11_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl19_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl23_tokenizer_test([<$test_name _all>], $input, $tokens);
		}
	};
}

macro_rules! all_vhdl_ams_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! {
			vhdl_ams_99_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl_ams_17_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl_ams_09_tokenizer_test([<$test_name _all>], $input, $tokens);
			vhdl_ams_17_tokenizer_test([<$test_name _all>], $input, $tokens);
		}
	};
}

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
include!("./tests/vhdl07.rs");
include!("./tests/vhdl08.rs");
include!("./tests/vhdl11.rs");
include!("./tests/vhdl19.rs");
include!("./tests/vhdl23.rs");
// VHDL-AMS Tokenizer Tests
include!("./tests/vhdl_ams_common.rs");
include!("./tests/vhdl_ams99.rs");
include!("./tests/vhdl_ams07.rs");
include!("./tests/vhdl_ams17.rs");
