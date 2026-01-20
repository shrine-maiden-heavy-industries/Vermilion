// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::{Debug, Display};

use vermilion_diagnostics::Position as DiagnosticPosition;

/// The creature-readable contextual position
/// within a document
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
	line:      usize,
	character: usize,
}

impl Position {
	pub const fn sof() -> Self {
		Self { line: 0, character: 0 }
	}

	pub const fn new(line: usize, character: usize) -> Self {
		Self { line, character }
	}

	pub fn line(&self) -> &usize {
		&self.line
	}

	pub fn character(&self) -> &usize {
		&self.character
	}

	pub fn next_line(&mut self) {
		self.character = 0;
		self.line += 1;
	}

	pub fn next_char(&mut self) {
		self.character += 1;
	}
}

// TODO(aki): Do we want `std::ops` (Add, Sub) for this?

impl<T, U> From<(T, U)> for Position
where
	T: Into<usize>,
	U: Into<usize>,
{
	fn from(value: (T, U)) -> Self {
		Self {
			line:      value.0.into(),
			character: value.1.into(),
		}
	}
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "line: {} char: {}", self.line, self.character)
	}
}

impl Debug for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Position")
			.field("line", &self.line)
			.field("character", &self.character)
			.finish()
	}
}

impl From<Position> for DiagnosticPosition {
	fn from(position: Position) -> Self {
		Self::new(position.line, position.character)
	}
}
