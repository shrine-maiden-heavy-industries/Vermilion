// SPDX-License-Identifier: BSD-3-Clause

use super::*;
use crate::VhdlStd;

macro_rules! vhdl_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		#[test]
		fn $test_name() {
			let tokenizer = VhdlTokenizer::new($std, $input.as_bytes().into());

			let parsed = tokenizer.collect::<Vec<_>>();
			let expected = $tokens;

			assert_eq!(
				parsed, expected,
				"Parsed: {parsed:#?}\nExpected: {expected:#?}"
			);
		}
	};
}

macro_rules! vhdl87_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh87);
	};
}

macro_rules! vhdl93_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh93);
	};
}

macro_rules! vhdl2k_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh2k);
	};
}

macro_rules! vhdl02_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh02);
	};
}

macro_rules! vhdl08_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh08);
	};
}

macro_rules! vhdl11_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh11);
	};
}

macro_rules! vhdl19_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh19);
	};
}

macro_rules! vhdl23_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		vhdl_tokenizer_test!($test_name, $input, $tokens, VhdlStd::Vh23);
	};
}

include!("./tests/vhdl87.rs");
include!("./tests/vhdl93.rs");
include!("./tests/vhdl2k.rs");
include!("./tests/vhdl02.rs");
include!("./tests/vhdl07.rs");
include!("./tests/vhdl08.rs");
include!("./tests/vhdl11.rs");
include!("./tests/vhdl19.rs");
include!("./tests/vhdl23.rs");
