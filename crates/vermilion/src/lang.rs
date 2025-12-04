/* SPDX-License-Identifier: BSD-3-Clause */

use clap::ArgMatches;
use vermilion_verilog::{SystemVerilogStd, VerilogAmsStd, VerilogStd};
use vermilion_vhdl::VhdlStd;

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum Language {
	Verilog(VerilogStd),
	SystemVerilog(SystemVerilogStd),
	VerilogAms(VerilogAmsStd),
	Vhdl(VhdlStd),
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
		"vh87" => Some(Language::Vhdl(VhdlStd::Vh87)),
		"vh93" => Some(Language::Vhdl(VhdlStd::Vh93)),
		"vh2k" => Some(Language::Vhdl(VhdlStd::Vh2k)),
		"vh02" => Some(Language::Vhdl(VhdlStd::Vh02)),
		"vh07" => Some(Language::Vhdl(VhdlStd::Vh07)),
		"vh08" => Some(Language::Vhdl(VhdlStd::Vh08)),
		"vh11" => Some(Language::Vhdl(VhdlStd::Vh11)),
		"vh19" => Some(Language::Vhdl(VhdlStd::Vh19)),
		"vh23" => Some(Language::Vhdl(VhdlStd::Vh23)),
		_ => unreachable!(),
	}
}

impl Display for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			// XXX(aki): There should be a better way to do this, but I have no idea what it would be
			Self::Verilog(std) => write!(f, "{std}"),
			Self::SystemVerilog(std) => write!(f, "{std}"),
			Self::VerilogAms(std) => write!(f, "{std}"),
			Self::Vhdl(std) => write!(f, "{std}"),
		}
	}
}
