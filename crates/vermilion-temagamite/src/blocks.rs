// SPDX-License-Identifier: BSD-3-Clause

use std::borrow::Cow;

pub struct CommonBlock {}

pub struct ToolBlock<'a> {
	pub(crate) key_owner: Cow<'a, str>,
	pub(crate) key_name:  Cow<'a, str>,
}
