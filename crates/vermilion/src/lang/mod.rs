// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use clap::{ValueEnum, builder::PossibleValue};
use color_print::cformat;
use vermilion_lang::AtomicByteTendril;
pub(crate) use vermilion_verilog::lang::{ast::Ast as VerilogAst, tokenizer::VerilogTokenizer};
use vermilion_verilog::{
	SystemVerilogStd, VerilogAmsStd, VerilogStd, VerilogVariant, lang::parser::VerilogParser,
};
pub(crate) use vermilion_vhdl::lang::{ast::Ast as VhdlAst, tokenizer::VhdlTokenizer};
use vermilion_vhdl::{VhdlAmsStd, VhdlStd, VhdlVariant, lang::parser::VhdlParser};

pub(crate) mod verilog;
pub(crate) mod vhdl;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum Language {
	Verilog(VerilogStd),
	SystemVerilog(SystemVerilogStd),
	VerilogAms(VerilogAmsStd),
	Vhdl(VhdlStd),
	VhdlAms(VhdlAmsStd),
}

pub(crate) enum Tokenizer {
	Verilog(VerilogTokenizer),
	Vhdl(VhdlTokenizer),
}

pub(crate) enum Parser {
	Verilog(VerilogParser),
	Vhdl(VhdlParser),
}

pub(crate) enum Ast {
	Verilog(VerilogAst),
	Vhdl(VhdlAst),
}

impl Display for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			// XXX(aki): There should be a better way to do this, but I have no idea what it would
			// be
			Self::Verilog(std) => std.fmt(f),
			Self::SystemVerilog(std) => std.fmt(f),
			Self::VerilogAms(std) => std.fmt(f),
			Self::Vhdl(std) => std.fmt(f),
			Self::VhdlAms(std) => std.fmt(f),
		}
	}
}

impl Language {
	pub const STD_VALUES: [Language; 25] = [
		Language::Verilog(VerilogStd::Vl95),
		Language::Verilog(VerilogStd::Vl01),
		Language::Verilog(VerilogStd::Vl05),
		Language::SystemVerilog(SystemVerilogStd::Sv05),
		Language::SystemVerilog(SystemVerilogStd::Sv09),
		Language::SystemVerilog(SystemVerilogStd::Sv12),
		Language::SystemVerilog(SystemVerilogStd::Sv17),
		Language::SystemVerilog(SystemVerilogStd::Sv23),
		Language::VerilogAms(VerilogAmsStd::Vams09),
		Language::VerilogAms(VerilogAmsStd::Vams14),
		Language::VerilogAms(VerilogAmsStd::Vams23),
		Language::Vhdl(VhdlStd::Vh87),
		Language::Vhdl(VhdlStd::Vh93),
		Language::Vhdl(VhdlStd::Vh2k),
		Language::Vhdl(VhdlStd::Vh02),
		Language::Vhdl(VhdlStd::Vh07),
		Language::Vhdl(VhdlStd::Vh08),
		Language::Vhdl(VhdlStd::Vh11),
		Language::Vhdl(VhdlStd::Vh19),
		Language::Vhdl(VhdlStd::Vh23),
		Language::VhdlAms(VhdlAmsStd::Vhams99),
		Language::VhdlAms(VhdlAmsStd::Vhams07),
		Language::VhdlAms(VhdlAmsStd::Vhams09),
		Language::VhdlAms(VhdlAmsStd::Vhams17),
		Language::VhdlAms(VhdlAmsStd::Vhams21),
	];

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
			Self::Vhdl(std) => Tokenizer::Vhdl(VhdlTokenizer::new(VhdlVariant::Vhdl(std), content)),
			Self::VhdlAms(std) => {
				Tokenizer::Vhdl(VhdlTokenizer::new(VhdlVariant::VhdlAms(std), content))
			},
		}
	}

	pub fn parser(self, content: AtomicByteTendril) -> Parser {
		match self {
			Self::SystemVerilog(std) => Parser::Verilog(VerilogParser::new(
				VerilogVariant::SystemVerilog(std),
				content,
			)),
			Self::Verilog(std) => {
				Parser::Verilog(VerilogParser::new(VerilogVariant::Verilog(std), content))
			},
			Self::VerilogAms(std) => {
				Parser::Verilog(VerilogParser::new(VerilogVariant::VerilogAms(std), content))
			},
			Self::Vhdl(std) => Parser::Vhdl(VhdlParser::new(VhdlVariant::Vhdl(std), content)),
			Self::VhdlAms(std) => Parser::Vhdl(VhdlParser::new(VhdlVariant::VhdlAms(std), content)),
		}
	}

	pub fn parse_file(self, _content: AtomicByteTendril) -> Ast {
		todo!()
	}
}

impl ValueEnum for Language {
	fn value_variants<'a>() -> &'a [Self] {
		&Language::STD_VALUES
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		Some(match self {
			Language::Verilog(std) => match std {
				VerilogStd::Vl95 => PossibleValue::new("vl95").help(cformat!(
					"<magenta>Verilog</> 1995 (<blue>IEEE</> 1364-1995)"
				)),
				VerilogStd::Vl01 => PossibleValue::new("vl01").help(cformat!(
					"<magenta>Verilog</> 2001 (<blue>IEEE</> 1364-2001)"
				)),
				VerilogStd::Vl05 => PossibleValue::new("vl05").help(cformat!(
					"<magenta>Verilog</> 2005 (<blue>IEEE</> 1364-2005)"
				)),
			},
			Language::SystemVerilog(std) => match std {
				SystemVerilogStd::Sv05 => PossibleValue::new("sv05").help(cformat!(
					"<cyan>SystemVerilog</> 2005 (<blue>IEEE</> 1800-2005)"
				)),
				SystemVerilogStd::Sv09 => PossibleValue::new("sv09").help(cformat!(
					"<cyan>SystemVerilog</> 2009 (<blue>IEEE</> 1800-2009)"
				)),
				SystemVerilogStd::Sv12 => PossibleValue::new("sv12").help(cformat!(
					"<cyan>SystemVerilog</> 2012 (<blue>IEEE</> 1800-2012)"
				)),
				SystemVerilogStd::Sv17 => PossibleValue::new("sv17").help(cformat!(
					"<cyan>SystemVerilog</> 2017 (<blue>IEEE</> 1800-2017)"
				)),
				SystemVerilogStd::Sv23 => PossibleValue::new("sv23").help(cformat!(
					"<cyan>SystemVerilog</> 2023 (<blue>IEEE</> 1800-2023)"
				)),
			},
			Language::VerilogAms(std) => match std {
				VerilogAmsStd::Vams09 => PossibleValue::new("vams09").help(cformat!(
					"<yellow>Verilog-AMS</> 2009 (<yellow>Accellera</> Verilog-AMS 2.3.1)"
				)),
				VerilogAmsStd::Vams14 => PossibleValue::new("vams14").help(cformat!(
					"<yellow>Verilog-AMS</> 2014 (<yellow>Accellera</> Verilog-AMS 2.4)"
				)),
				VerilogAmsStd::Vams23 => PossibleValue::new("vams23").help(cformat!(
					"<yellow>Verilog-AMS</> 2023 (<yellow>Accellera</> Verilog-AMS 2023)"
				)),
			},
			Language::Vhdl(std) => match std {
				VhdlStd::Vh87 => PossibleValue::new("vhd87")
					.help(cformat!("<green>VHDL</> 1987 (<blue>IEEE</> 1076-1987)")),
				VhdlStd::Vh93 => PossibleValue::new("vhd93")
					.help(cformat!("<green>VHDL</> 1993 (<blue>IEEE</> 1076-1993)")),
				VhdlStd::Vh2k => PossibleValue::new("vhd2k")
					.help(cformat!("<green>VHDL</> 2000 (<blue>IEEE</> 1076-2000)")),
				VhdlStd::Vh02 => PossibleValue::new("vhd02")
					.help(cformat!("<green>VHDL</> 2002 (<blue>IEEE</> 1076-2002)")),
				VhdlStd::Vh04 => PossibleValue::new("vhd04").help(cformat!(
					"<green>VHDL</> 2004 (<cyan>IEC</> 61691-1-1:2004)"
				)),
				VhdlStd::Vh07 => PossibleValue::new("vhd07")
					.help(cformat!("<green>VHDL</> 2007 (<blue>IEEE</> 1076-2007)")),
				VhdlStd::Vh08 => PossibleValue::new("vhd08")
					.help(cformat!("<green>VHDL</> 2008 (<blue>IEEE</> 1076-2008)")),
				VhdlStd::Vh11 => PossibleValue::new("vhd11").help(cformat!(
					"<green>VHDL</> 2011 (<cyan>IEC</> 61691-1-1:2011)"
				)),
				VhdlStd::Vh19 => PossibleValue::new("vhd19")
					.help(cformat!("<green>VHDL</> 2019 (<blue>IEEE</> 1076-2019)")),
				VhdlStd::Vh23 => PossibleValue::new("vhd23").help(cformat!(
					"<green>VHDL</> 2023 (<cyan>IEC</> 61691-1-1:2023)"
				)),
			},
			Language::VhdlAms(std) => match std {
				VhdlAmsStd::Vhams99 => PossibleValue::new("vhdams99").help(cformat!(
					"<red>VHDL-AMS</> 1999 (<blue>IEEE</> 1076.1-1999)"
				)),
				VhdlAmsStd::Vhams07 => PossibleValue::new("vhdams07").help(cformat!(
					"<red>VHDL-AMS</> 2007 (<blue>IEEE</> 1076.1-2007)"
				)),
				VhdlAmsStd::Vhams09 => PossibleValue::new("vhdams09").help(cformat!(
					"<red>VHDL-AMS</> 2009 (<blue>IEC</> 61691-6:2021)"
				)),
				VhdlAmsStd::Vhams17 => PossibleValue::new("vhdams17").help(cformat!(
					"<red>VHDL-AMS</> 2017 (<blue>IEEE</> 1076.1-2017)"
				)),
				VhdlAmsStd::Vhams21 => PossibleValue::new("vhdams21").help(cformat!(
					"<red>VHDL-AMS</> 2021 (<cyan>IEC</> 61691-6:2021)"
				)),
			},
		})
	}
}
