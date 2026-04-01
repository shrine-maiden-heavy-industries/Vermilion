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

/// General global Vermilion configuration settings
#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
pub struct ConfigGeneral {}

/// Global Vermilion configuration
///
/// Unlike the Vermilion [`Workspace`], which is used to identify and configure
/// projects that wish to use Vermilion. The global configuration is used to specify user-specific
/// options that are not project specific and are inappropriate to include in the workspace.
///
/// One example would be with the [`XactConfig`], where you specify an IP-XACT repository that is
/// system or organization specific, as not to accidentally commit and publish internal paths.
///
/// [`Workspace`]: crate::workspace::Workspace
#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	/// General configuration options
	pub vermilion:      Option<ConfigGeneral>,
	/// Design constraints configuration options
	pub constraints:    Option<ConstraintConfig>,
	/// General diagnostics configuration options
	pub diagnostics:    Option<DiagnosticsConfig>,
	/// General formatting configuration options
	pub formatting:     Option<FormattingConfig>,
	/// Liberty specific configuration options
	pub liberty:        Option<LibertyConfig>,
	/// SystemVerilog specific configuration options
	pub system_verilog: Option<SystemVerilogConfig>,
	/// Verilog-AMS specific configuration options
	pub verilog_ams:    Option<VerilogAmsConfig>,
	/// Verilog specific configuration options
	pub verilog:        Option<VerilogConfig>,
	/// VHDL-AMS specific configuration options
	pub vhdl_ams:       Option<VhdlAmsConfig>,
	/// VHDL specific configuration options
	pub vhdl:           Option<VhdlConfig>,
	/// IP-XACT specific configuration options
	pub xact:           Option<XactConfig>,
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
