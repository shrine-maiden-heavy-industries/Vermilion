// SPDX-License-Identifier: BSD-3-Clause

use std::{
	path::{Path, PathBuf},
	sync::OnceLock,
};

macro_rules! cache_path {
	($dir:expr) => {
		static _PATH: OnceLock<PathBuf> = OnceLock::new();

		_PATH.get_or_init(|| {
			#[allow(
				clippy::expect_used,
				reason = "The `xtask` can't actually do anything useful if we don't have the \
				          workspace directory, therefore panicking is entirely fine"
			)]
			$dir
		})
	};
}

pub(crate) fn crates_dir() -> &'static PathBuf {
	cache_path! { workspace_root().join("crates") }
}

pub(crate) fn docs_dir() -> &'static PathBuf {
	cache_path! { workspace_root().join("docs") }
}

pub(crate) fn editors_dir() -> &'static PathBuf {
	cache_path! { workspace_root().join("editors") }
}

pub(crate) fn workspace_root() -> &'static PathBuf {
	cache_path! {
		Path::new(&env!("CARGO_MANIFEST_DIR"))
			.ancestors()
			.nth(1)
			.map(Path::to_path_buf)
			.expect("Unable to get workspace root directory")
	}
}
