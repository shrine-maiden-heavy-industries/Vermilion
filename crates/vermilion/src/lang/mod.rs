// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use clap::ArgMatches;
use vermilion_lang::AtomicByteTendril;
pub(crate) use vermilion_verilog::lang::{
	ast::Ast as VerilogAst, tokenizer::Tokenizer as VerilogTokenizer,
};
use vermilion_verilog::{SystemVerilogStd, VerilogAmsStd, VerilogStd, VerilogVariant};
use vermilion_vhdl::VhdlStd;
pub(crate) use vermilion_vhdl::lang::{ast::Ast as VhdlAst, tokenizer::Tokenizer as VhdlTokenizer};

pub(crate) mod verilog;
pub(crate) mod vhdl;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum Language {
	Verilog(VerilogStd),
	SystemVerilog(SystemVerilogStd),
	VerilogAms(VerilogAmsStd),
	Vhdl(VhdlStd),
}

pub(crate) enum Tokenizer {
	Verilog(VerilogTokenizer),
	Vhdl(VhdlTokenizer),
}

pub(crate) enum Ast {
	Verilog(VerilogAst),
	Vhdl(VhdlAst),
}

pub(crate) fn get_langid(a: &ArgMatches) -> Option<Language> {
	match a.get_one::<String>("std")?.as_str() {
		"vl95" => Some(Language::Verilog(VerilogStd::Vl95)),
		"vl01" => Some(Language::Verilog(VerilogStd::Vl01)),
		"vl05" => Some(Language::Verilog(VerilogStd::Vl05)),
		"sv05" => Some(Language::SystemVerilog(SystemVerilogStd::Sv05)),
		"sv09" => Some(Language::SystemVerilog(SystemVerilogStd::Sv09)),
		"sv12" => Some(Language::SystemVerilog(SystemVerilogStd::Sv12)),
		"sv17" => Some(Language::SystemVerilog(SystemVerilogStd::Sv17)),
		"sv23" => Some(Language::SystemVerilog(SystemVerilogStd::Sv23)),
		"vams09" => Some(Language::VerilogAms(VerilogAmsStd::Vams09)),
		"vams14" => Some(Language::VerilogAms(VerilogAmsStd::Vams14)),
		"vams23" => Some(Language::VerilogAms(VerilogAmsStd::Vams23)),
		"vhd87" => Some(Language::Vhdl(VhdlStd::Vh87)),
		"vhd93" => Some(Language::Vhdl(VhdlStd::Vh93)),
		"vhd2k" => Some(Language::Vhdl(VhdlStd::Vh2k)),
		"vhd02" => Some(Language::Vhdl(VhdlStd::Vh02)),
		"vhd07" => Some(Language::Vhdl(VhdlStd::Vh07)),
		"vhd08" => Some(Language::Vhdl(VhdlStd::Vh08)),
		"vhd11" => Some(Language::Vhdl(VhdlStd::Vh11)),
		"vhd19" => Some(Language::Vhdl(VhdlStd::Vh19)),
		"vhd23" => Some(Language::Vhdl(VhdlStd::Vh23)),
		_ => unreachable!(),
	}
}

impl Display for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			// XXX(aki): There should be a better way to do this, but I have no idea what it would
			// be
			Self::Verilog(std) => write!(f, "{std}"),
			Self::SystemVerilog(std) => write!(f, "{std}"),
			Self::VerilogAms(std) => write!(f, "{std}"),
			Self::Vhdl(std) => write!(f, "{std}"),
		}
	}
}

impl Language {
	pub fn tokenizer(self, content: AtomicByteTendril) -> Tokenizer {
		match self {
			Self::SystemVerilog(std) => Tokenizer::Verilog(VerilogTokenizer::new(
				VerilogVariant::SystemVerilog(std),
				content,
			)),
			Self::Verilog(std) => {
				Tokenizer::Verilog(VerilogTokenizer::new(VerilogVariant::Verilog(std), content))
			},
			Self::VerilogAms(std) => Tokenizer::Verilog(VerilogTokenizer::new(
				VerilogVariant::VerilogAms(std),
				content,
			)),
			Self::Vhdl(std) => Tokenizer::Vhdl(VhdlTokenizer::new(std, content)),
		}
	}

	pub fn parse_file(self, _content: AtomicByteTendril) -> Ast {
		todo!()
	}
}
