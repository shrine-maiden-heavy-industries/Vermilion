// SPDX-License-Identifier: BSD-3-Clause

use std::fs;

use clap::ArgMatches;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vermilion_constraints::config::ConstraintConfig;
use vermilion_diagnostics::config::DiagnosticsConfig;
use vermilion_fmt::config::FormattingConfig;
use vermilion_liberty::config::LibertyConfig;
use vermilion_verilog::config::{SystemVerilogConfig, VerilogAmsConfig, VerilogConfig};
use vermilion_vhdl::config::{VhdlAmsConfig, VhdlConfig};
use vermilion_xact::config::XactConfig;

use crate::paths;

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

// TODO(aki): Remove once used
#[allow(unused, reason = "Currently unused")]
/// This method loads the Vermilion configuration file if found.
///
/// The Vermilion configuration file is is located in the user-specific
/// configuration file directory, and/or the system-wide configuration file
/// directory if the system supports it.
///
/// We first check the user-specific path and then only if that's not found do
/// we check the system-wide path.
///
/// Due to how the [`paths::system_config_dir`] call is implemented, that being on
/// systems that don't support system-wide configuration files in a known location.
/// This search results in us looking for both `config.toml` *and* `vermilion.toml`
/// in the user-specific configuration path.
pub fn load_config(args: &ArgMatches) -> eyre::Result<Option<Config>> {
	// If we were passed a workspace file on the command line, try that
	if let Ok(Some(config_file)) = args.try_get_one::<String>("config") {
		return Ok(toml::from_slice(&fs::read(config_file)?)?);
	}

	let mut cfg_file = paths::local_config_dir().join("config.toml");
	let mut configuration = None;

	if cfg_file.exists() {
		configuration = Some(toml::from_slice(&fs::read(cfg_file)?)?);
	} else {
		cfg_file = paths::system_config_dir().join("vermilion.toml");
		if cfg_file.exists() {
			configuration = Some(toml::from_slice(&fs::read(cfg_file)?)?);
		}
	}

	Ok(configuration)
}
