/* SPDX-License-Identifier: BSD-3-Clause */

pub(crate) mod token;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	line: usize,
	character: usize,
}

impl Position {
	pub fn next_line(&mut self) {
		self.character = 0;
		self.line += 1;
	}

	pub fn next_char(&mut self) {
		self.character += 1;
	}
}

impl Display for Position {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "line {}, character {}", self.line, self.character)
	}
}
