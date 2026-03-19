// SPDX-License-Identifier: BSD-3-Clause
vermilion_macros::cfg_fuzzing! {
	use afl::fuzz;
	use vermilion_lang::AtomicByteTendril;
	use vermilion_verilog::{LanguageStd, lang::tokenizer::VerilogTokenizer};
}

fn main() {
	vermilion_macros::cfg_fuzzing! {
		fuzz!(|data: &[u8]| {
			let tokenizer =
				VerilogTokenizer::new(LanguageStd::Vl95, AtomicByteTendril::from_slice(data)).unwrap();
			let _ = tokenizer.collect::<Vec<_>>();
		});
	}
}
