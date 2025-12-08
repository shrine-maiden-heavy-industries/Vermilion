/* SPDX-License-Identifier: BSD-3-Clause */

use std::{fs, path::PathBuf};

use clap::ArgMatches;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use tracing::warn;
use vermilion_core::settings::VermilionConfig;
use vermilion_verilog::settings::{SystemVerilogConfig, VerilogConfig};
use vermilion_vhdl::settings::VhdlConfig;

use crate::paths;

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
/// 1. A default, empty configuration is created
/// 2. If the `-c`/`--config` option is passed, use that
/// 3. If `${USER_CFG_DIR}/vermilion/config.toml` exists, overlay that
/// 4. Starting from the current working directory:
///    1. Look for one of the following:
///       1. `.vermilion.toml`
///       2. `vermilion.toml` (preferred if both exist)
///    2. If found, overlay and return, otherwise...
///    3. If the next upper directory is on the same file-system mount, move up and try again, otherwise return
///
/// This allows you to set system-wide defaults, and then have them be overloaded per user and also
/// per-project.
///
pub fn load_config(args: &ArgMatches) -> eyre::Result<Config> {
	const VERMILION_CONFIG_NAMES: [&str; 2] = ["vermilion.toml", ".vermilion.toml"];

	fn _load_cfg_file(file: PathBuf) -> eyre::Result<Config> {
		let cfg = fs::read(file)?;
		Ok(toml::from_slice::<Config>(&cfg)?)
	}

	if let Ok(Some(cfg_file)) = args.try_get_one::<String>("config") {
		return _load_cfg_file(cfg_file.into());
	};

	let user_cfg = paths::config_file();
	if user_cfg.exists() {
		return _load_cfg_file(user_cfg);
	}

	let current_dir = std::env::current_dir()?;

	fn _check_cfg_files(dir: &PathBuf) -> Option<PathBuf> {
		None
	}

	// let f = current_dir
	// 	.read_dir()?
	// 	.flatten()
	// 	.map(|f| f.file_name().to_string_lossy())
	// 	.filter(|f| VERMILION_CONFIG_NAMES.contains(&f.as_ref()));

	let mut cfg = Config::default();

	warn!("TODO(aki): Config file loading/merging");

	Ok(cfg)
}
