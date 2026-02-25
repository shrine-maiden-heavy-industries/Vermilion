// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

use rayon::iter::{ParallelBridge, ParallelIterator, walk_tree};

// TODO(aki): Remove once used
#[allow(unused)]
pub trait ParallelWalk {
	fn par_walk(&self) -> impl ParallelIterator<Item = Self>;
}

impl ParallelWalk for PathBuf {
	fn par_walk(&self) -> impl ParallelIterator<Item = Self> {
		walk_tree(self.clone(), |path| {
			if path.is_dir() {
				if let Ok(dir) = path.read_dir() {
					dir.par_bridge().flatten().map(|e| e.path()).collect()
				} else {
					Vec::new()
				}
			} else {
				Vec::new()
			}
		})
	}
}
