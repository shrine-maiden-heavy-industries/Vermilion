// SPDX-License-Identifier: BSD-3-Clause

use crate::paths;

pub struct Repo {
	inner: git2::Repository,
}

impl Repo {
	pub fn get() -> eyre::Result<Option<Self>> {
		match git2::Repository::open(paths::workspace_root()) {
			Ok(repo) => Ok(Some(Self { inner: repo })),
			Err(err) => match err.code() {
				git2::ErrorCode::NotFound => Ok(None),
				_ => Err(err.into()),
			},
		}
	}

	pub fn tags(&self) -> eyre::Result<Option<Vec<git2::Tag<'_>>>> {
		let tags = self
			.inner
			.tag_names(None)?
			.iter()
			.flat_map(|tag_name| tag_name.ok())
			.flatten()
			.flat_map(|name| self.inner.resolve_reference_from_short_name(name))
			.flat_map(|refname| refname.target())
			.flat_map(|id| self.inner.find_tag(id))
			.collect::<Vec<_>>();

		if !tags.is_empty() {
			Ok(Some(tags))
		} else {
			Ok(None)
		}
	}

	pub fn latest_tag(&self) -> eyre::Result<Option<git2::Tag<'_>>> {
		if let Some(mut tags) = self.tags()? {
			tags.sort_by(|tag_a, tag_b| {
				if let Ok(commit_a) = tag_a.as_object().peel_to_commit() &&
					let Ok(commit_b) = tag_b.as_object().peel_to_commit()
				{
					commit_b.time().cmp(&commit_a.time())
				} else {
					std::cmp::Ordering::Equal
				}
			});

			Ok(tags.pop())
		} else {
			Ok(None)
		}
	}
}
