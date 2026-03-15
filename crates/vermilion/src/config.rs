// SPDX-License-Identifier: BSD-3-Clause

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vermilion_constraints::config::ConstraintConfig;
use vermilion_diagnostics::config::DiagnosticsConfig;
use vermilion_fmt::config::FormattingConfig;
use vermilion_liberty::config::LibertyConfig;
use vermilion_verilog::config::{SystemVerilogConfig, VerilogAmsConfig, VerilogConfig};
use vermilion_vhdl::config::{VhdlAmsConfig, VhdlConfig};
use vermilion_xact::config::XactConfig;

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
pub struct CoreConfig {}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	constraints:    Option<ConstraintConfig>,
	core:           Option<CoreConfig>,
	diagnostics:    Option<DiagnosticsConfig>,
	formatting:     Option<FormattingConfig>,
	liberty:        Option<LibertyConfig>,
	system_verilog: Option<SystemVerilogConfig>,
	verilog_ams:    Option<VerilogAmsConfig>,
	verilog:        Option<VerilogConfig>,
	vhdl_ams:       Option<VhdlAmsConfig>,
	vhdl:           Option<VhdlConfig>,
	xact:           Option<XactConfig>,
}
