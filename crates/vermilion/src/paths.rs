/* SPDX-License-Identifier: BSD-3-Clause */

use std::{
	path::{Path, PathBuf},
	sync::OnceLock,
};

use directories::ProjectDirs;

static DIRS: OnceLock<ProjectDirs> = OnceLock::new();

pub(crate) fn proj_dirs() -> &'static ProjectDirs {
	// SAFETY:
	// At this point exploding here is fine, if we can't get the users home directory
	// we're out of luck anyway.
	#[allow(clippy::expect_used)]
	DIRS.get_or_init(|| ProjectDirs::from("", "", "Vermilion").expect(""))
}

/// Get the Vermilion user-specific configuration directory
pub(crate) fn config_dir() -> &'static Path {
	proj_dirs().config_dir()
}

/// Get the Vermilion user-specific cache directory
pub(crate) fn cache_dir() -> &'static Path {
	proj_dirs().cache_dir()
}

/// Get the path to the user-specific Vermilion configuration file
pub(crate) fn config_file() -> PathBuf {
	config_dir().join("config.toml")
}
