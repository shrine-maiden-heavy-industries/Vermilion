// SPDX-License-Identifier: BSD-3-Clause

use std::{
	fmt::{Debug, Display},
	hash::Hash,
	ops::{Deref, DerefMut},
};

use crate::{Span, span::ThinSpan};

/// Represents a unique ID for mapping locations and other constructs
/// to a specific file that has been loaded File
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileId(usize);

/// Much like how a [`Spanned<T>`] attaches a [`Span`] to an object, the
/// [`Location<T>`] also provides that association along with a [`FileId`] in which
/// the given `T` was from.
///
/// This lets us provide a mapping from AST objects and other constructs to the files
/// they originate from, allowing things like "Goto Definition" support and other diagnostic
/// facilities.
///
/// [`Spanned<T>`]: crate::span::Spanned<T>
pub struct Location<T> {
	id:    FileId,
	inner: T,
	span:  Span,
}

impl FileId {
	/// Create a new [`FileId`]
	#[inline(always)]
	pub fn new<T>(id: T) -> Self
	where
		T: Into<usize> + Copy,
	{
		Self(<T as std::convert::Into<usize>>::into(id))
	}

	/// Get the raw value of this [`FileId`]
	#[inline(always)]
	pub const fn raw(&self) -> &usize {
		&self.0
	}

	/// Get the next valid [`FileId`] after this one
	#[inline(always)]
	pub const fn next(&self) -> Self {
		Self(self.0 + 1)
	}
}

// SAFETY:
// Due to `FileId` being just a newtype of `usize` we /should/ automatically get
// `Sync` and `Send`, but just in case be explicit about it.
unsafe impl Sync for FileId {}
unsafe impl Send for FileId {}

impl<T> From<T> for FileId
where
	T: Into<usize> + Copy,
{
	fn from(value: T) -> Self {
		Self::new(<T as std::convert::Into<usize>>::into(value))
	}
}

impl Display for FileId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl<T> Location<T> {
	#[inline(always)]
	pub const fn new(id: FileId, inner: T, span: Span) -> Self {
		Self { id, inner, span }
	}

	#[inline(always)]
	pub const fn file_id(&self) -> &FileId {
		&self.id
	}

	#[inline(always)]
	pub const fn inner(&self) -> &T {
		&self.inner
	}

	#[inline(always)]
	pub const fn inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	#[inline(always)]
	pub const fn span(&self) -> &Span {
		&self.span
	}

	#[inline(always)]
	pub const fn thin_span(&self) -> ThinSpan {
		self.span.get_thin()
	}

	#[inline(always)]
	pub const fn begin(&self) -> &u32 {
		self.span.begin()
	}

	#[inline(always)]
	pub const fn end(&self) -> &u32 {
		self.span.end()
	}

	#[inline(always)]
	pub const fn line(&self) -> &u32 {
		self.span.line()
	}

	#[inline(always)]
	pub const fn character(&self) -> &u32 {
		self.span.character()
	}
}

impl<T> Clone for Location<T>
where
	T: Clone,
{
	fn clone(&self) -> Self {
		Self {
			id:    self.id,
			inner: self.inner.clone(),
			span:  self.span,
		}
	}
}

impl<T> Copy for Location<T> where T: Copy {}

// SAFETY:
// The `Span` and `FileId` inside `Location<T>` are `Sync + Send` so if `T` is
// also `Sync`/`Send` then `Location<T>` should also be safe to be so.
unsafe impl<T> Sync for Location<T> where T: Sync {}
unsafe impl<T> Send for Location<T> where T: Send {}

impl<T> Deref for Location<T> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> DerefMut for Location<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T> Hash for Location<T>
where
	T: Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
		self.inner.hash(state);
		self.span.hash(state);
	}
}

impl<T> PartialEq for Location<T>
where
	T: PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id && self.inner == other.inner && self.span == other.span
	}
}

impl<T> Eq for Location<T> where T: Eq {}

impl<T> PartialOrd for Location<T>
where
	T: PartialOrd,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match self.id.partial_cmp(&other.id) {
			Some(std::cmp::Ordering::Equal) => match self.span.partial_cmp(&other.span) {
				Some(std::cmp::Ordering::Equal) => self.inner.partial_cmp(&other.inner),
				ord => ord,
			},
			ord => ord,
		}
	}
}

impl<T> Ord for Location<T>
where
	T: Ord,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self.id.cmp(&other.id) {
			std::cmp::Ordering::Equal => match self.span.cmp(&other.span) {
				std::cmp::Ordering::Equal => self.inner.cmp(&other.inner),
				ord => ord,
			},
			ord => ord,
		}
	}
}

impl<T> Display for Location<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.inner, self.span, self.id)
	}
}

impl<T> Debug for Location<T>
where
	T: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Location")
			.field("id", &self.id)
			.field("inner", &self.inner)
			.field("span", &self.span)
			.finish()
	}
}
