// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use clap::{ValueEnum, builder::PossibleValue};
use color_print::cformat;
use vermilion_lang::AtomicByteTendril;
pub(crate) use vermilion_verilog::lang::{ast::Ast as VerilogAst, tokenizer::VerilogTokenizer};
use vermilion_verilog::{LanguageStd as VerilogStd, lang::parser::VerilogParser};
pub(crate) use vermilion_vhdl::lang::{ast::Ast as VhdlAst, tokenizer::VhdlTokenizer};
use vermilion_vhdl::{LanguageStd as VhdlStd, lang::parser::VhdlParser};

pub(crate) mod verilog;
pub(crate) mod vhdl;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum Language {
	Verilog(VerilogStd),
	Vhdl(VhdlStd),
}

pub(crate) enum Tokenizer {
	Verilog(Box<VerilogTokenizer>),
	Vhdl(Box<VhdlTokenizer>),
}

pub(crate) enum Parser {
	Verilog(Box<VerilogParser>),
	Vhdl(Box<VhdlParser>),
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
			Self::Vhdl(std) => std.fmt(f),
		}
	}
}

impl Language {
	pub const STD_VALUES: [Language; 25] = [
		Language::Verilog(VerilogStd::Vl95),
		Language::Verilog(VerilogStd::Vl01),
		Language::Verilog(VerilogStd::Vl05),
		Language::Verilog(VerilogStd::Sv05),
		Language::Verilog(VerilogStd::Sv09),
		Language::Verilog(VerilogStd::Sv12),
		Language::Verilog(VerilogStd::Sv17),
		Language::Verilog(VerilogStd::Sv23),
		Language::Verilog(VerilogStd::Vams09),
		Language::Verilog(VerilogStd::Vams14),
		Language::Verilog(VerilogStd::Vams23),
		Language::Vhdl(VhdlStd::Vh87),
		Language::Vhdl(VhdlStd::Vh93),
		Language::Vhdl(VhdlStd::Vh2k),
		Language::Vhdl(VhdlStd::Vh02),
		Language::Vhdl(VhdlStd::Vh07),
		Language::Vhdl(VhdlStd::Vh08),
		Language::Vhdl(VhdlStd::Vh11),
		Language::Vhdl(VhdlStd::Vh19),
		Language::Vhdl(VhdlStd::Vh23),
		Language::Vhdl(VhdlStd::Vhams99),
		Language::Vhdl(VhdlStd::Vhams07),
		Language::Vhdl(VhdlStd::Vhams09),
		Language::Vhdl(VhdlStd::Vhams17),
		Language::Vhdl(VhdlStd::Vhams21),
	];

	pub fn tokenizer(self, content: AtomicByteTendril) -> eyre::Result<Tokenizer> {
		Ok(match self {
			Self::Verilog(std) => {
				Tokenizer::Verilog(Box::new(VerilogTokenizer::new(std, content)?))
			},
			Self::Vhdl(std) => Tokenizer::Vhdl(Box::new(VhdlTokenizer::new(std, content)?)),
		})
	}

	pub fn parser(self, content: AtomicByteTendril) -> eyre::Result<Parser> {
		Ok(match self {
			Self::Verilog(std) => Parser::Verilog(Box::new(VerilogParser::new(std, content)?)),
			Self::Vhdl(std) => Parser::Vhdl(Box::new(VhdlParser::new(std, content)?)),
		})
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
		match self {
			Language::Verilog(std) => match *std {
				VerilogStd::Vl95 => Some(PossibleValue::new("vl95").help(cformat!(
					"<magenta>Verilog</> 1995 (<blue>IEEE</> 1364-1995)"
				))),
				VerilogStd::Vl01 => Some(PossibleValue::new("vl01").help(cformat!(
					"<magenta>Verilog</> 2001 (<blue>IEEE</> 1364-2001)"
				))),
				VerilogStd::Vl05 => Some(PossibleValue::new("vl05").help(cformat!(
					"<magenta>Verilog</> 2005 (<blue>IEEE</> 1364-2005)"
				))),
				VerilogStd::Sv05 => Some(PossibleValue::new("sv05").help(cformat!(
					"<cyan>SystemVerilog</> 2005 (<blue>IEEE</> 1800-2005)"
				))),
				VerilogStd::Sv09 => Some(PossibleValue::new("sv09").help(cformat!(
					"<cyan>SystemVerilog</> 2009 (<blue>IEEE</> 1800-2009)"
				))),
				VerilogStd::Sv12 => Some(PossibleValue::new("sv12").help(cformat!(
					"<cyan>SystemVerilog</> 2012 (<blue>IEEE</> 1800-2012)"
				))),
				VerilogStd::Sv17 => Some(PossibleValue::new("sv17").help(cformat!(
					"<cyan>SystemVerilog</> 2017 (<blue>IEEE</> 1800-2017)"
				))),
				VerilogStd::Sv23 => Some(PossibleValue::new("sv23").help(cformat!(
					"<cyan>SystemVerilog</> 2023 (<blue>IEEE</> 1800-2023)"
				))),
				VerilogStd::Vams09 => Some(PossibleValue::new("vams09").help(cformat!(
					"<yellow>Verilog-AMS</> 2009 (<yellow>Accellera</> Verilog-AMS 2.3.1)"
				))),
				VerilogStd::Vams14 => Some(PossibleValue::new("vams14").help(cformat!(
					"<yellow>Verilog-AMS</> 2014 (<yellow>Accellera</> Verilog-AMS 2.4)"
				))),
				VerilogStd::Vams23 => Some(PossibleValue::new("vams23").help(cformat!(
					"<yellow>Verilog-AMS</> 2023 (<yellow>Accellera</> Verilog-AMS 2023)"
				))),
				_ => None,
			},
			Language::Vhdl(std) => match *std {
				VhdlStd::Vh87 => Some(
					PossibleValue::new("vhd87")
						.help(cformat!("<green>VHDL</> 1987 (<blue>IEEE</> 1076-1987)")),
				),
				VhdlStd::Vh93 => Some(
					PossibleValue::new("vhd93")
						.help(cformat!("<green>VHDL</> 1993 (<blue>IEEE</> 1076-1993)")),
				),
				VhdlStd::Vh2k => Some(
					PossibleValue::new("vhd2k")
						.help(cformat!("<green>VHDL</> 2000 (<blue>IEEE</> 1076-2000)")),
				),
				VhdlStd::Vh02 => Some(
					PossibleValue::new("vhd02")
						.help(cformat!("<green>VHDL</> 2002 (<blue>IEEE</> 1076-2002)")),
				),
				VhdlStd::Vh04 => Some(PossibleValue::new("vhd04").help(cformat!(
					"<green>VHDL</> 2004 (<cyan>IEC</> 61691-1-1:2004)"
				))),
				VhdlStd::Vh07 => Some(
					PossibleValue::new("vhd07")
						.help(cformat!("<green>VHDL</> 2007 (<blue>IEEE</> 1076-2007)")),
				),
				VhdlStd::Vh08 => Some(
					PossibleValue::new("vhd08")
						.help(cformat!("<green>VHDL</> 2008 (<blue>IEEE</> 1076-2008)")),
				),
				VhdlStd::Vh11 => Some(PossibleValue::new("vhd11").help(cformat!(
					"<green>VHDL</> 2011 (<cyan>IEC</> 61691-1-1:2011)"
				))),
				VhdlStd::Vh19 => Some(
					PossibleValue::new("vhd19")
						.help(cformat!("<green>VHDL</> 2019 (<blue>IEEE</> 1076-2019)")),
				),
				VhdlStd::Vh23 => Some(PossibleValue::new("vhd23").help(cformat!(
					"<green>VHDL</> 2023 (<cyan>IEC</> 61691-1-1:2023)"
				))),
				VhdlStd::Vhams99 => Some(PossibleValue::new("vhdams99").help(cformat!(
					"<red>VHDL-AMS</> 1999 (<blue>IEEE</> 1076.1-1999)"
				))),
				VhdlStd::Vhams07 => Some(PossibleValue::new("vhdams07").help(cformat!(
					"<red>VHDL-AMS</> 2007 (<blue>IEEE</> 1076.1-2007)"
				))),
				VhdlStd::Vhams09 => Some(PossibleValue::new("vhdams09").help(cformat!(
					"<red>VHDL-AMS</> 2009 (<cyan>IEC</> 61691-6:2021)"
				))),
				VhdlStd::Vhams17 => Some(PossibleValue::new("vhdams17").help(cformat!(
					"<red>VHDL-AMS</> 2017 (<blue>IEEE</> 1076.1-2017)"
				))),
				VhdlStd::Vhams21 => Some(PossibleValue::new("vhdams21").help(cformat!(
					"<red>VHDL-AMS</> 2021 (<cyan>IEC</> 61691-6:2021)"
				))),
				_ => None,
			},
		}
	}
}
