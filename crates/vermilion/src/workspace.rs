// SPDX-License-Identifier: BSD-3-Clause

use std::fs;

use clap::ArgMatches;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::debug;
use vermilion_constraints::workspace::ConstraintWorkspace;
use vermilion_liberty::workspace::LibertyWorkspace;
use vermilion_verilog::workspace::{SystemVerilogWorkspace, VerilogAmsWorkspace, VerilogWorkspace};
use vermilion_vhdl::workspace::{VhdlAmsWorkspace, VhdlWorkspace};
use vermilion_xact::workspace::XactWorkspace;

use crate::paths;

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Workspace {
	verilog:        Option<VerilogWorkspace>,
	system_verilog: Option<SystemVerilogWorkspace>,
	verilog_ams:    Option<VerilogAmsWorkspace>,
	vhdl:           Option<VhdlWorkspace>,
	vhdl_ams:       Option<VhdlAmsWorkspace>,
	constraints:    Option<ConstraintWorkspace>,
	liberty:        Option<LibertyWorkspace>,
	xact:           Option<XactWorkspace>,
}

/// This method loads the Vermilion workspace configuration file if found.
///
/// The Vermilion workspace configuration file is called `vermilion.toml` and ideally
/// should be located at the root of the project which is using Vermilion, but can also
/// be passed in with the `--workspace` command line flag.
///
/// The file is searched for starting from the current working directory,
/// and going up a director until the file is found, we hit the FS root, or we
/// encounter a FS boundary (e.g. we're invoked in a mounted volume).
pub fn load_workspace(args: &ArgMatches) -> eyre::Result<Option<Workspace>> {
	// If we were passed a workspace file on the command line, try that
	if let Ok(Some(workspace_file)) = args.try_get_one::<String>("workspace") {
		return Ok(toml::from_slice(&fs::read(workspace_file)?)?);
	}

	let current_dir = std::env::current_dir()?;
	let mut search_dir = current_dir.clone();

	let mut workspace_file = None;
	let mut workspace = None;

	// Search up to either the root or the upper-most mount-point for a valid configuration file.
	while paths::same_fs(&current_dir, &search_dir)? && workspace_file.is_none() {
		workspace_file = current_dir
			.read_dir()?
			.flatten()
			.filter(|f| f.file_name() == "vermilion.toml")
			.map(|f| f.path())
			.next();

		// Check to see if we found a configuration file, if not try to go up a directory if
		// possible
		if let Some(ref path) = workspace_file {
			workspace = Some(toml::from_slice(&fs::read(path)?)?);
			break;
		} else if let Some(parent) = search_dir.parent() {
			search_dir = parent.to_path_buf();
		} else {
			break;
		}
	}

	// Emit a debug diagnostic saying we couldn't find a workspace file
	if workspace_file.is_none() {
		debug!("Unable to find valid Vermilion workspace file, using default values");
	}

	Ok(workspace)
}
