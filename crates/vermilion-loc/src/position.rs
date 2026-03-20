// SPDX-License-Identifier: BSD-3-Clause

use std::{
	fmt::{Debug, Display},
	hash::Hash,
};

/// The creature-readable contextual position
/// within a document
#[derive(Clone, Copy)]
pub struct Position {
	line:      u32,
	character: u32,
}

impl Position {
	/// Create a new [`Position`] that represents the Start of File (sof)
	#[inline(always)]
	pub const fn sof() -> Self {
		Self { line: 0, character: 0 }
	}

	/// Create a new [`Position`] that represents an arbitrary End of File (eof)
	#[inline(always)]
	pub const fn eof() -> Self {
		Self { line: u32::MAX, character: u32::MAX }
	}

	/// Create a new [`Position`] with a specific line and character number
	#[inline(always)]
	pub const fn new(line: u32, character: u32) -> Self {
		Self { line, character }
	}

	/// Get the line this [`Position`] currently sits on
	#[inline(always)]
	pub const fn line(&self) -> &u32 {
		&self.line
	}

	/// Get a mutable reference to this [`Position`]'s line
	#[inline(always)]
	pub const fn line_mut(&mut self) -> &mut u32 {
		&mut self.line
	}

	/// Get the character this [`Position`] is currently on
	#[inline(always)]
	pub const fn character(&self) -> &u32 {
		&self.character
	}

	/// Get a mutable reference to this [`Position`]'s character
	#[inline(always)]
	pub const fn character_mut(&mut self) -> &mut u32 {
		&mut self.character
	}

	/// Advance this [`Position`]'s line, resetting [`Position::character`] to `0`
	#[inline(always)]
	pub const fn next_line(&mut self) {
		self.character = 0;
		self.line += 1;
	}

	/// Advance this [`Position`]'s character
	#[inline(always)]
	pub const fn next_char(&mut self) {
		self.character += 1;
	}
}

impl<T, U> From<(T, U)> for Position
where
	T: Into<u32>,
	U: Into<u32>,
{
	fn from(value: (T, U)) -> Self {
		Self {
			line:      <T as std::convert::Into<u32>>::into(value.0),
			character: <U as std::convert::Into<u32>>::into(value.1),
		}
	}
}

impl Hash for Position {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.line.hash(state);
		self.character.hash(state);
	}
}

impl PartialEq for Position {
	fn eq(&self, other: &Self) -> bool {
		self.line == other.line && self.character == other.character
	}
}

impl<T, U> PartialEq<(T, U)> for Position
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	fn eq(&self, other: &(T, U)) -> bool {
		self.line == <T as std::convert::Into<u32>>::into(other.0) &&
			self.character == <U as std::convert::Into<u32>>::into(other.1)
	}
}

impl Eq for Position {}

impl PartialOrd for Position {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<T, U> PartialOrd<(T, U)> for Position
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	fn partial_cmp(&self, other: &(T, U)) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&Self::new(
			<T as std::convert::Into<u32>>::into(other.0),
			<U as std::convert::Into<u32>>::into(other.1),
		)))
	}
}

impl Ord for Position {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.line < other.line {
			std::cmp::Ordering::Less
		} else if self.line > other.line {
			std::cmp::Ordering::Greater
		} else {
			if self.character < other.character {
				std::cmp::Ordering::Less
			} else if self.character > other.character {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_position_equality() {
		assert_eq!(Position::new(0, 0), Position::new(0, 0));
		assert_ne!(Position::new(0, 1), Position::new(0, 2));
		assert_ne!(Position::new(0, 1), Position::new(1, 0));
		assert_ne!(Position::new(0, 1), Position::new(2, 0));
		assert_ne!(Position::new(1, 0), Position::new(2, 0));
		assert_ne!(Position::new(1, 0), Position::new(0, 1));
		assert_ne!(Position::new(1, 0), Position::new(0, 2));
	}

	#[test]
	fn test_position_ordering() {
		assert!(Position::new(0, 0) < Position::new(0, 1));
		assert!(Position::new(1, 0) > Position::new(0, 1));
		assert!(Position::new(1, 1) > Position::new(0, 1));
		assert!(Position::new(1, 1) > Position::new(1, 0));
	}

	#[test]
	fn test_position_from_tuple() {
		assert_eq!(Position::new(0, 0), (0u32, 0u32));
	}
}
