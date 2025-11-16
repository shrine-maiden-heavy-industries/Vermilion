/* SPDX-License-Identifier: BSD-3-Clause */

use clap::ArgMatches;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use tracing::warn;
use vermilion_core::settings::VermilionConfig;
use vermilion_verilog::settings::{SystemVerilogConfig, VerilogConfig};
use vermilion_vhdl::settings::VhdlConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Config {
	#[serde(rename = "vermilion")]
	core: VermilionConfig,

	verilog: Option<VerilogConfig>,
	system_verilog: Option<SystemVerilogConfig>,
	vhdl: Option<VhdlConfig>,
}

/// ## Load the Vermillion Configuration
///
/// This method loads the Vermillion configuration file applying overloads
/// at each appropriate level where appropriate.
///
/// The order of the lookup and loading is as follows:
///
/// 1. Check if `-c`/`--config` option is passed, if so, use that, otherwise...
/// 2. Default empty config
/// 3. Overlay from `${SYS_CFG_DIR}/vermillion/config.toml`
/// 4. Overlay from `${USER_CFG_DIR}/vermillion/config.toml`
/// 5. Overlay from `${CWD}/vermillion.toml` -or- `${CWD}/.vermillion.toml` (prefer, non `.` prefixed)
///
/// This allows you to set system-wide defaults, and then have them be overloaded per user and also
/// per-project.
///
pub fn load_config(args: &ArgMatches) -> Config {
	let _cfg_file = args.get_one::<String>("config");

	warn!("TODO(aki): Config file loading/merging");

	Config::default()
}
