// SPDX-License-Identifier: BSD-3-Clause
use paste::paste;

macro_rules! tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $variant:expr) => {
		paste! {
			#[test]
			fn [<test_tokenize_ $test_name>] () {
				let tokenizer = VerilogTokenizer::new($variant, $input.as_bytes().into());

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

macro_rules! verilog_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		paste! { tokenizer_test!(
				[<verilog $test_name>],
				$input,
				$tokens,
				VerilogVariant::Verilog($std)
			);
		}
	};
}

macro_rules! verilog95_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { verilog_tokenizer_test!(
			[<_95_ $test_name>],
			$input,
			$tokens,
			VerilogStd::Vl95
		); }
	};
}

macro_rules! verilog01_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { verilog_tokenizer_test!(
			[<_01_ $test_name>],
			$input,
			$tokens,
			VerilogStd::Vl01
		); }
	};
}

macro_rules! verilog05_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { verilog_tokenizer_test!(
			[<_05_ $test_name>],
			$input,
			$tokens,
			VerilogStd::Vl05
		); }
	};
}

macro_rules! all_verilog_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		verilog95_tokenizer_test!($test_name, $input, $tokens);
		verilog01_tokenizer_test!($test_name, $input, $tokens);
		verilog05_tokenizer_test!($test_name, $input, $tokens);
	};
}

macro_rules! system_verilog_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		paste! { tokenizer_test!(
				[<system_verilog $test_name>],
				$input,
				$tokens,
				VerilogVariant::SystemVerilog($std)
			);
		}
	};
}

macro_rules! system_verilog05_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { system_verilog_tokenizer_test!([<_05_ $test_name>], $input, $tokens, SystemVerilogStd::Sv05); }
	};
}

macro_rules! system_verilog09_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { system_verilog_tokenizer_test!([<_09_ $test_name>], $input, $tokens, SystemVerilogStd::Sv09); }
	};
}

macro_rules! system_verilog12_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { system_verilog_tokenizer_test!([<_12_ $test_name>], $input, $tokens, SystemVerilogStd::Sv12); }
	};
}

macro_rules! system_verilog17_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { system_verilog_tokenizer_test!([<_17_ $test_name>], $input, $tokens, SystemVerilogStd::Sv17); }
	};
}

macro_rules! system_verilog23_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { system_verilog_tokenizer_test!([<_23_ $test_name>], $input, $tokens, SystemVerilogStd::Sv23); }
	};
}

macro_rules! all_system_verilog_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		system_verilog05_tokenizer_test!($test_name, $input, $tokens);
		system_verilog09_tokenizer_test!($test_name, $input, $tokens);
		system_verilog12_tokenizer_test!($test_name, $input, $tokens);
		system_verilog17_tokenizer_test!($test_name, $input, $tokens);
		system_verilog23_tokenizer_test!($test_name, $input, $tokens);
	};
}

macro_rules! verilog_ams_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr, $std:expr) => {
		paste! { tokenizer_test!(
				[<verilog_ams $test_name>],
				$input,
				$tokens,
				VerilogVariant::VerilogAms($std)
			);
		}
	};
}

macro_rules! verilog_ams09_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { verilog_ams_tokenizer_test!([<_09_ $test_name>], $input, $tokens, VerilogAmsStd::Vams09); }
	};
}

macro_rules! verilog_ams14_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { verilog_ams_tokenizer_test!([<_14_ $test_name>], $input, $tokens, VerilogAmsStd::Vams14); }
	};
}

macro_rules! verilog_ams23_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		paste! { verilog_ams_tokenizer_test!([<_23_ $test_name>], $input, $tokens, VerilogAmsStd::Vams23); }
	};
}

macro_rules! all_verilog_ams_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		verilog_ams09_tokenizer_test!($test_name, $input, $tokens);
		verilog_ams14_tokenizer_test!($test_name, $input, $tokens);
		verilog_ams23_tokenizer_test!($test_name, $input, $tokens);
	};
}

macro_rules! all_tokenizer_test {
	($test_name:ident, $input:literal, $tokens:expr) => {
		all_verilog_tokenizer_test!($test_name, $input, $tokens);
		all_system_verilog_tokenizer_test!($test_name, $input, $tokens);
		all_verilog_ams_tokenizer_test!($test_name, $input, $tokens);
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
