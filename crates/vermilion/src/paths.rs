// SPDX-License-Identifier: BSD-3-Clause

use std::{
	path::{Path, PathBuf},
	sync::OnceLock,
};

use directories::ProjectDirs;

static DIRS: OnceLock<ProjectDirs> = OnceLock::new();

pub(crate) fn proj_dirs() -> &'static ProjectDirs {
	#[allow(
		clippy::expect_used,
		reason = "At this point exploding here is fine, if we can't get the users home directory \
		          we're out of luck anyway."
	)]
	DIRS.get_or_init(|| {
		ProjectDirs::from("", "", "Vermilion").expect("Unable to initialize project directories")
	})
}

// TODO(aki): Remove once used
#[allow(unused, reason = "Currently unused")]
/// Get the Vermilion user-specific configuration directory
pub(crate) fn local_config_dir() -> &'static Path {
	proj_dirs().config_dir()
}

/// Get the Vermilion system-wide configuration directory
///
/// ## NOTE
/// This is hard-coded to `/etc` on Linux and is equivalent to [`local_config_dir`]
/// on other platforms
pub(crate) fn system_config_dir() -> &'static Path {
	#[cfg(target_os = "linux")]
	#[inline(always)]
	fn get_path() -> &'static Path {
		Path::new("/etc")
	}

	#[cfg(not(target_os = "linux"))]
	#[inline(always)]
	fn get_path() -> &'static Path {
		proj_dirs().config_dir()
	}

	get_path()
}

// TODO(aki): Remove once used
#[allow(unused, reason = "Currently unused")]
/// Get the Vermilion user-specific cache directory
pub(crate) fn cache_dir() -> &'static Path {
	proj_dirs().cache_dir()
}

/// Check if two paths are located on the same filesystem
#[cfg(not(target_os = "windows"))]
#[inline(always)]
pub(crate) fn same_fs(path1: &PathBuf, path2: &PathBuf) -> eyre::Result<bool> {
	#[cfg(not(target_os = "windows"))]
	#[inline(always)]
	fn is_same(path1: &PathBuf, path2: &PathBuf) -> eyre::Result<bool> {
		use std::{fs, os::unix::fs::MetadataExt};

		Ok(fs::metadata(path1)?.dev() == fs::metadata(path2)?.dev())
	}

	#[cfg(target_os = "windows")]
	#[inline(always)]
	fn is_same(path1: &PathBuf, path2: &PathBuf) -> eyre::Result<bool> {
		// TODO(aki): Figure this out, eventually, maybe, who even uses windows anyway?
		todo!()
	}

	is_same(path1, path2)
}
