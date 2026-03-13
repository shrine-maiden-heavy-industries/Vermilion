use afl::fuzz;
use vermilion_lang::AtomicByteTendril;
use vermilion_verilog::{LanguageStd, lang::tokenizer::VerilogTokenizer};

fn main() {
	fuzz!(|data: &[u8]| {
		let tokenizer =
			VerilogTokenizer::new(LanguageStd::Vl95, AtomicByteTendril::from_slice(data)).unwrap();
		let _ = tokenizer.collect::<Vec<_>>();
	});
}
