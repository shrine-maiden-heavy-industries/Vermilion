// SPDX-License-Identifier: BSD-3-Clause

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vermilion_verilog::workspace::{SystemVerilogConfig, VerilogConfig};
use vermilion_vhdl::workspace::VhdlConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceConfig {
	verilog:        Option<VerilogConfig>,
	system_verilog: Option<SystemVerilogConfig>,
	vhdl:           Option<VhdlConfig>,
}
