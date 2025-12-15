/* SPDX-License-Identifier: BSD-3-Clause */

pub(crate) mod token;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	line: usize,
	character: usize,
}
