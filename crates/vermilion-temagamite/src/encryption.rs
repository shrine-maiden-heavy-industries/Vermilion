// SPDX-License-Identifier: BSD-3-Clause

use std::borrow::Cow;

pub struct Envelope<'a> {
	pub(crate) author:      Cow<'a, str>,
	pub(crate) author_info: Cow<'a, str>,
}
