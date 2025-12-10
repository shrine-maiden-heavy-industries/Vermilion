/* SPDX-License-Identifier: BSD-3-Clause */

use std::{fs, path::PathBuf, str::FromStr};

use clap::ArgMatches;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use tracing::{debug, trace, warn};
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
///    1. Look for `vermilion.toml`:
///    2. If found, overlay and return, otherwise...
///    3. If the next upper directory is on the same file-system mount, move up and try again, otherwise return
///
/// This allows you to set system-wide defaults, and then have them be overloaded per user and also
/// per-project.
///
pub fn load_config(args: &ArgMatches) -> eyre::Result<Config> {
	fn _load_cfg_file(file: &PathBuf) -> eyre::Result<Config> {
		trace!("Loading configuration file from {:#?}", file);
		warn!("TODO(aki): Config file merging");
		let cfg = fs::read(file)?;
		Ok(toml::from_slice::<Config>(&cfg)?)
	}

	if let Ok(Some(cfg_file)) = args.try_get_one::<String>("config") {
		return _load_cfg_file(&PathBuf::from_str(cfg_file.as_str())?);
	};

	let user_cfg = paths::config_file();
	if user_cfg.exists() {
		return _load_cfg_file(&user_cfg);
	}

	let current_dir = std::env::current_dir()?;
	let mut search_dir = current_dir.clone();

	let mut cfg_file: Option<PathBuf> = None;
	let mut cfg = Config::default();

	// Search up to either the root or the upper-most mount-point for a valid configuration file.
	while paths::same_fs(&current_dir, &search_dir)? && cfg_file.is_none() {
		cfg_file = current_dir
			.read_dir()?
			.flatten()
			.filter(|f| f.file_name() == "vermilion.toml")
			.map(|f| f.path())
			.next();

		// Check to see if we found a configuration file, if not try to go up a directory if possible
		if let Some(ref path) = cfg_file {
			cfg = _load_cfg_file(path)?;
			break;
		} else if let Some(parent) = search_dir.parent() {
			search_dir = parent.to_path_buf();
		} else {
			break;
		}
	}

	// Emit a debug diagnostic saying we couldn't find a configuration file
	if cfg_file.is_none() {
		debug!("Unable to find valid Vermilion configuration file, using default values");
	}

	Ok(cfg)
}
