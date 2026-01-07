// SPDX-License-Identifier: BSD-3-Clause

macro_rules! tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		#[test]
		fn $test_name() {
			let tokenizer = VerilogTokenizer::new(
				VerilogVariant::Verilog(VerilogStd::Vl95),
				$input.as_bytes().into(),
			);

			let parsed = tokenizer.collect::<Vec<_>>();
			let expected = $tokens;

			assert_eq!(
				parsed, expected,
				"Parsed: {parsed:#?}\nExpected: {expected:#?}"
			);
		}
	};
}

// Verilog Tokenizer Tests
include!("./tests/verilog95.rs");
include!("./tests/verilog05.rs");
include!("./tests/verilog01.rs");
// SystemVerilog Tokenizer Tests
include!("./tests/system_verilog05.rs");
include!("./tests/system_verilog09.rs");
include!("./tests/system_verilog12.rs");
include!("./tests/system_verilog17.rs");
include!("./tests/system_verilog23.rs");
// Verilog-AMS Tokenizer Tests
include!("./tests/verilog_ams09.rs");
include!("./tests/verilog_ams14.rs");
include!("./tests/verilog_ams23.rs");
