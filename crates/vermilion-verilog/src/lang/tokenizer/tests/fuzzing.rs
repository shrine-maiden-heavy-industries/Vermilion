// SPDX-License-Identifier: BSD-3-Clause

use proptest::prelude::*;

macro_rules! tokenizer_fuzz {
	(verilog95, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<verilog_95_ $test_name>],
			$spec,
			LanguageStd::Vl95
		); }
	};
	(verilog01, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<verilog_01_ $test_name>],
			$spec,
			LanguageStd::Vl01
		); }
	};
	(verilog01 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(verilog01, $test_name, $spec);
		tokenizer_fuzz!(verilog05, $test_name, $spec);
		tokenizer_fuzz!(system_verilog_all, $test_name, $spec);
	};
	(verilog05, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<verilog_05_ $test_name>],
			$spec,
			LanguageStd::Vl05
		); }
	};
	(verilog05 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(verilog05, $test_name, $spec);
		tokenizer_fuzz!(system_verilog_all, $test_name, $spec);
	};
	(verilog_all, $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(verilog95, $test_name, $spec);
		tokenizer_fuzz!(verilog01, $test_name, $spec);
		tokenizer_fuzz!(verilog05, $test_name, $spec);
	};
	(system_verilog05, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<system_verilog_05_ $test_name>],
			$spec,
			LanguageStd::Sv05
		); }
	};
	(system_verilog09, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<system_verilog_09_ $test_name>],
			$spec,
			LanguageStd::Sv09
		); }
	};
	(system_verilog09 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(system_verilog09, $test_name, $spec);
		tokenizer_fuzz!(system_verilog12, $test_name, $spec);
		tokenizer_fuzz!(system_verilog17, $test_name, $spec);
		tokenizer_fuzz!(system_verilog23, $test_name, $spec);
	};
	(system_verilog12, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<system_verilog_12_ $test_name>],
			$spec,
			LanguageStd::Sv12
		); }
	};
	(system_verilog12 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(system_verilog12, $test_name, $spec);
		tokenizer_fuzz!(system_verilog17, $test_name, $spec);
		tokenizer_fuzz!(system_verilog23, $test_name, $spec);
	};
	(system_verilog17, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<system_verilog_17_ $test_name>],
			$spec,
			LanguageStd::Sv17
		); }
	};
	(system_verilog17 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(system_verilog17, $test_name, $spec);
		tokenizer_fuzz!(system_verilog23, $test_name, $spec);
	};
	(system_verilog23, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<system_verilog_23_ $test_name>],
			$spec,
			LanguageStd::Sv23
		); }
	};
	(system_verilog23 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(system_verilog23, $test_name, $spec);
	};
	(system_verilog_all, $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(system_verilog05, $test_name, $spec);
		tokenizer_fuzz!(system_verilog09, $test_name, $spec);
		tokenizer_fuzz!(system_verilog12, $test_name, $spec);
		tokenizer_fuzz!(system_verilog17, $test_name, $spec);
		tokenizer_fuzz!(system_verilog23, $test_name, $spec);
	};
	(verilog_ams09, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<verilog_ams_09_ $test_name>],
			$spec,
			LanguageStd::Vams09
		); }
	};
	(verilog_ams14, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<verilog_ams_14_ $test_name>],
			$spec,
			LanguageStd::Vams14
		); }
	};
	(verilog_ams14 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(verilog_ams14, $test_name, $spec);
		tokenizer_fuzz!(verilog_ams23, $test_name, $spec);
	};
	(verilog_ams23, $test_name:ident, $spec:literal) => {
		paste! { tokenizer_fuzz!(
			[<verilog_ams_23_ $test_name>],
			$spec,
			LanguageStd::Vams23
		); }
	};
	(verilog_ams23 + , $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(verilog_ams23, $test_name, $spec);
	};
	(verilog_ams_all, $test_name:ident, $spec:literal) => {
		tokenizer_fuzz!(verilog_ams09, $test_name, $spec);
		tokenizer_fuzz!(verilog_ams14, $test_name, $spec);
		tokenizer_fuzz!(verilog_ams23, $test_name, $spec);
	};
	(all, $test_name:ident, $spec:literal) => {
		paste! {
			tokenizer_fuzz!(verilog_all, [<$test_name _all>], $spec);
			tokenizer_fuzz!(system_verilog_all, [<$test_name _all>], $spec);
			tokenizer_fuzz!(verilog_ams_all, [<$test_name _all>], $spec);
		}
	};
	($test_name:ident, $spec:literal, $std:expr) => {
		paste! {
			#[allow(non_upper_case_globals, reason = "I'm stuck in macro hell, help me")]
			static [<fuzz_tokenize_ $test_name _reg_path>]: &'static str = concat!(
				env!("CARGO_MANIFEST_DIR"), "/../../target/proptest-regressions"
			);
			proptest! {
				#![proptest_config(ProptestConfig {
					failure_persistence: Some(Box::new(
						proptest::test_runner::FileFailurePersistence::SourceParallel(
							[<fuzz_tokenize_ $test_name _reg_path>]
						)
					)),
					.. ProptestConfig::default()
				})]
				#[test]
				fn [<fuzz_tokenize_ $test_name>] (input in $spec) {
					#[allow(
						clippy::unwrap_used,
						reason = "We are within tests, and while this shouldn't explode, if it does here that's fine"
					)]
					let tokenizer = VerilogTokenizer::new($std, input.as_bytes().into()).unwrap();
					let _ = tokenizer.collect::<Vec<_>>();
				}
			}
		}
	};
}

tokenizer_fuzz!(all, misc, r"[\PC\pC]*");
