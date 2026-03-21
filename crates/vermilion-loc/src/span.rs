// SPDX-License-Identifier: BSD-3-Clause

use std::{
	fmt::{Debug, Display},
	hash::Hash,
	ops::{Add, AddAssign, Deref, DerefMut, Range, RangeBounds, RangeInclusive, Sub, SubAssign},
};

use crate::Position;

#[macro_export]
macro_rules! spanned {
	($token:expr) => {
		$crate::Spanned::new($token, $crate::span::Span::empty())
	};
	($token:expr, $span:expr) => {
		$crate::Spanned::new($token, $span)
	};
	($token:expr, $range:expr, $position:expr) => {
		$crate::Spanned::new($token, $crate::span::Span::from_range($range, $position))
	};
}

#[macro_export]
macro_rules! thin_spanned {
	($token:expr) => {
		$crate::ThinSpanned::new($token, $crate::span::ThinSpan::new())
	};
	($token:expr, $span:expr) => {
		$crate::ThinSpanned::new($token, $span)
	};
}

// NOTE(aki):
// We are using `u32` here for 3 reasons:
//  1. On 32-bit machines begin/end will only take up a single register, and on 64-bit machines they
//     can both be packed into a one register.
//  2. It's unlikely that we will ever ingest a file that is over 4GiB
//  3. The `tendril` we use to store source text is limited to 4GiB /anyway/
//
/// A "thin" [`Span`] that only holds that begin and end offsets into a source buffer
///
/// Just like [`Span`] this is `[begin, end)`, e.g. a non-inclusive range
///
/// ## Why not [`Range<u32>`]?
///
/// The [`Range`] type does not require it's index type to implement [`Copy`], as such
/// you can only ever clone them, which is less than optimal.
#[derive(Clone, Copy, Debug)]
pub struct ThinSpan {
	begin: u32,
	end:   u32,
}

// NOTE(aki): See the note above `ThinSpan` for justifications for using `u32` here
/// A non-inclusive span for keeping track of the beginning and end offsets within a
/// buffer along with line and character offset information associated with said offset.
///
/// For only storing the beginning and end offsets, see [`ThinSpan`].
#[derive(Clone, Copy, Debug)]
pub struct Span {
	begin:     u32,
	end:       u32,
	line:      u32,
	character: u32,
}

/// Wraps the given type `T`, attaching an optional [`ThinSpan`] to it for
/// keeping track of location information
#[derive(Clone)]
pub struct ThinSpanned<T>
where
	T: Clone,
{
	inner: T,
	span:  ThinSpan,
}

/// Wraps the given type `T`, attaching an optional [`Span`] to it for
/// keeping track of location information
#[derive(Clone)]
pub struct Spanned<T>
where
	T: Clone,
{
	inner: T,
	span:  Span,
}

impl ThinSpan {
	/// Create a new empty [`ThinSpan`], where [`ThinSpan::begin`] and [`ThinSpan::end`]
	/// are both set to 0.
	#[inline(always)]
	pub const fn empty() -> Self {
		Self { begin: 0, end: 0 }
	}

	/// Create [`ThinSpan`] with the given begin and ending offsets
	///
	/// ## Note
	///
	/// The [`ThinSpan::end`] is non-inclusive, if you want to represent
	/// a span from 0 to 5, then the `end` will be `6`.
	#[inline(always)]
	pub const fn new(begin: u32, end: u32) -> Self {
		Self { begin, end }
	}

	/// Create a [`ThinSpan`] from the given [`Range`]
	#[inline(always)]
	pub fn from_range<T>(range: Range<T>) -> Self
	where
		T: Into<u32> + Copy,
	{
		Self {
			begin: <T as std::convert::Into<u32>>::into(range.start),
			end:   <T as std::convert::Into<u32>>::into(range.end),
		}
	}

	/// Attempt to create a [`ThinSpan`] from a [`Range<T>`].
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`Range::start`] or
	/// [`Range::end`]
	#[inline(always)]
	pub fn try_from_range<T>(
		range: Range<T>,
	) -> Result<Self, <T as std::convert::TryInto<u32>>::Error>
	where
		T: TryInto<u32> + Copy,
	{
		Ok(Self {
			begin: <T as std::convert::TryInto<u32>>::try_into(range.start)?,
			end:   <T as std::convert::TryInto<u32>>::try_into(range.end)?,
		})
	}

	/// Create a [`ThinSpan`] from the given [`RangeInclusive<T>`]
	///
	/// ## Note
	///
	/// Because the [`ThinSpan::end`] offset is stored as non-inclusive
	/// this will add `1` to the result of the [`RangeInclusive::end`] calls.
	///
	/// ## Panics
	///
	/// This method will panic if [`RangeInclusive::end`] is already at [`u32::MAX`]
	/// due to addition overflow
	#[inline(always)]
	pub fn from_inclusive_range<T>(range: RangeInclusive<T>) -> Self
	where
		T: Into<u32> + Copy,
	{
		Self {
			begin: <T as std::convert::Into<u32>>::into(*range.start()),
			end:   <T as std::convert::Into<u32>>::into(*range.end()) + 1,
		}
	}

	/// Attempt to create a [`ThinSpan`] from the given [`RangeInclusive<T>`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`RangeInclusive::start`] or
	/// [`RangeInclusive::end`]
	///
	/// ## Note
	///
	/// Because the [`ThinSpan::end`] offset is stored as non-inclusive
	/// this will add `1` to the result of the [`RangeInclusive::end`] calls.
	///
	/// ## Panics
	///
	/// This method will panic if [`RangeInclusive::end`] is already at [`u32::MAX`]
	/// due to addition overflow
	#[inline(always)]
	pub fn try_from_inclusive_range<T>(
		range: RangeInclusive<T>,
	) -> Result<Self, <T as std::convert::TryInto<u32>>::Error>
	where
		T: TryInto<u32> + Copy,
	{
		Ok(Self {
			begin: <T as std::convert::TryInto<u32>>::try_into(*range.start())?,
			end:   <T as std::convert::TryInto<u32>>::try_into(*range.end())? + 1,
		})
	}

	/// Convert the current [`ThinSpan`] into a [`Range<T>`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn as_range<T>(self) -> Range<T>
	where
		T: From<u32>,
	{
		Range::<T> {
			start: <T as std::convert::From<u32>>::from(self.begin),
			end:   <T as std::convert::From<u32>>::from(self.end),
		}
	}

	/// Convert the current [`ThinSpan`] into a [`Range<T>`], unlike [`ThinSpan::as_range`] this
	/// does not consume the [`ThinSpan`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn get_range<T>(&self) -> Range<T>
	where
		T: From<u32>,
	{
		Range::<T> {
			start: <T as std::convert::From<u32>>::from(self.begin),
			end:   <T as std::convert::From<u32>>::from(self.end),
		}
	}

	/// Attempt to the current [`ThinSpan`] into a [`Range<T>`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`Range::start`] or
	/// [`Range::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn try_as_range<T>(self) -> Result<Range<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(Range::<T> {
			start: <T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			end:   <T as std::convert::TryFrom<u32>>::try_from(self.end)?,
		})
	}

	/// Attempt to the current [`ThinSpan`] into a [`Range<T>`], unlike [`ThinSpan::try_as_range`]
	/// this does not consume the [`ThinSpan`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`Range::start`] or
	/// [`Range::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn try_get_range<T>(&self) -> Result<Range<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(Range::<T> {
			start: <T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			end:   <T as std::convert::TryFrom<u32>>::try_from(self.end)?,
		})
	}

	/// Convert the current [`ThinSpan`] into a [`RangeInclusive<T>`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, due to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`ThinSpan::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn as_inclusive_range<T>(self) -> RangeInclusive<T>
	where
		T: From<u32>,
	{
		RangeInclusive::new(
			<T as std::convert::From<u32>>::from(self.begin),
			<T as std::convert::From<u32>>::from(self.end - 1),
		)
	}

	/// Convert the current [`ThinSpan`] into a [`RangeInclusive<T>`], unlike
	/// [`ThinSpan::as_inclusive_range`] this does not consume the [`ThinSpan`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, due to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`ThinSpan::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn get_inclusive_range<T>(&self) -> RangeInclusive<T>
	where
		T: From<u32>,
	{
		RangeInclusive::new(
			<T as std::convert::From<u32>>::from(self.begin),
			<T as std::convert::From<u32>>::from(self.end - 1),
		)
	}

	/// Attempt to convert the current [`ThinSpan`] into a [`RangeInclusive<T>`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`RangeInclusive::start`] or
	/// [`RangeInclusive::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, due to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`ThinSpan::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn try_as_inclusive_range<T>(
		self,
	) -> Result<RangeInclusive<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(RangeInclusive::new(
			<T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			<T as std::convert::TryFrom<u32>>::try_from(self.end - 1)?,
		))
	}

	/// Attempt to convert the current [`ThinSpan`] into a [`RangeInclusive<T>`], unlike
	/// [`ThinSpan::try_as_inclusive_range`] this does not consume the [`ThinSpan`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`RangeInclusive::start`] or
	/// [`RangeInclusive::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, due to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`ThinSpan::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn try_get_inclusive_range<T>(
		&self,
	) -> Result<RangeInclusive<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(RangeInclusive::new(
			<T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			<T as std::convert::TryFrom<u32>>::try_from(self.end - 1)?,
		))
	}

	/// The start of the range represented by [`ThinSpan`]
	#[inline(always)]
	pub const fn begin(&self) -> &u32 {
		&self.begin
	}

	/// The end of the range represented by [`ThinSpan`]
	#[inline(always)]
	pub const fn end(&self) -> &u32 {
		&self.end
	}

	/// Convert this [`ThinSpan`] into a full [`Span`] by attaching a [`Position`]
	///
	/// If no position is specified, `0` will be used for both the [`Span::line`]
	/// and [`Span::character`]
	#[inline(always)]
	pub const fn as_span(self, position: Option<Position>) -> Span {
		let (line, character) = if let Some(position) = position {
			(*position.line(), *position.character())
		} else {
			(0, 0)
		};

		Span::new(self.begin, self.end, line, character)
	}

	/// Convert this [`ThinSpan`] into a full [`Span`] by attaching a [`Position`], unlike
	/// [`ThinSpan::as_span`] this does not consume the [`ThinSpan`]
	///
	/// If no position is specified, `0` will be used for both the [`Span::line`]
	/// and [`Span::character`]
	#[inline(always)]
	pub const fn get_span(&self, position: Option<Position>) -> Span {
		let (line, character) = if let Some(position) = position {
			(*position.line(), *position.character())
		} else {
			(0, 0)
		};

		Span::new(self.begin, self.end, line, character)
	}

	/// Convert this [`ThinSpan`] into a [`ThinSpanned<T>`] using the given
	/// other T, attaching this span to it.
	#[inline(always)]
	pub const fn as_thin_spanned<T>(self, inner: T) -> ThinSpanned<T>
	where
		T: Clone,
	{
		ThinSpanned::new(inner, self)
	}

	/// Convert this [`ThinSpan`] into a [`ThinSpanned<T>`] using the given
	/// other T, attaching this span to it.
	///
	/// Unlike [`ThinSpan::as_thin_spanned`], this does not consume the [`ThinSpan`]
	#[inline(always)]
	pub const fn get_thin_spanned<T>(&self, inner: T) -> ThinSpanned<T>
	where
		T: Clone,
	{
		ThinSpanned::new(inner, *self)
	}

	/// Convert this [`ThinSpan`] into a [`Spanned<T>`] using the given
	/// other T, first converting this [`ThinSpan`] into a [`Span`] with
	/// [`ThinSpan::as_span`] then attaching to it.
	#[inline(always)]
	pub const fn as_spanned<T>(self, inner: T) -> Spanned<T>
	where
		T: Clone,
	{
		Spanned::new(inner, self.as_span(None))
	}

	/// Convert this [`ThinSpan`] into a [`Spanned<T>`] using the given
	/// other T, first converting this [`ThinSpan`] into a [`Span`] with
	/// [`ThinSpan::as_span`] then attaching to it.
	///
	/// Unlike [`ThinSpan::as_spanned`], this does not consume the [`ThinSpan`]
	#[inline(always)]
	pub const fn get_spanned<T>(&self, inner: T) -> Spanned<T>
	where
		T: Clone,
	{
		Spanned::new(inner, self.get_span(None))
	}
}

impl<T, U> From<(T, U)> for ThinSpan
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: (T, U)) -> Self {
		Self { begin: value.0.into(), end: value.1.into() }
	}
}

impl<T> From<Range<T>> for ThinSpan
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: Range<T>) -> Self {
		Self::from_range(value)
	}
}

impl<T> From<RangeInclusive<T>> for ThinSpan
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: RangeInclusive<T>) -> Self {
		Self::from_inclusive_range(value)
	}
}

impl From<ThinSpan> for Span {
	#[inline(always)]
	fn from(value: ThinSpan) -> Self {
		value.as_span(None)
	}
}

impl Add for ThinSpan {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			begin: if self.begin < rhs.begin {
				self.begin
			} else {
				rhs.begin
			},
			end:   if self.end > rhs.end {
				self.end
			} else {
				rhs.end
			},
		}
	}
}

impl<T> Add<T> for ThinSpan
where
	T: Into<u32> + Copy,
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: T) -> Self::Output {
		Self {
			begin: self.begin,
			end:   self.end + <T as std::convert::Into<u32>>::into(rhs),
		}
	}
}

impl AddAssign for ThinSpan {
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) {
		if self.begin > rhs.begin {
			self.begin = rhs.begin;
		}

		if self.end < rhs.end {
			self.end = rhs.end;
		}
	}
}

impl<T> AddAssign<T> for ThinSpan
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: T) {
		self.end += <T as std::convert::Into<u32>>::into(rhs);
	}
}

impl Sub for ThinSpan {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			begin: if rhs.begin < self.end {
				rhs.begin
			} else {
				0
			},
			end:   if self.end < rhs.end {
				self.end
			} else {
				0
			},
		}
	}
}

impl<T> Sub<T> for ThinSpan
where
	T: Into<u32> + Copy,
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: T) -> Self::Output {
		Self {
			begin: self.begin - <T as std::convert::Into<u32>>::into(rhs),
			end:   self.end,
		}
	}
}

impl SubAssign for ThinSpan {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) {
		self.begin = if rhs.begin < self.end {
			rhs.begin
		} else {
			0
		};

		self.end = if self.end < rhs.end {
			self.end
		} else {
			0
		};
	}
}

impl<T> SubAssign<T> for ThinSpan
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: T) {
		self.begin -= <T as std::convert::Into<u32>>::into(rhs);
	}
}

impl RangeBounds<u32> for ThinSpan {
	#[inline(always)]
	fn start_bound(&self) -> std::ops::Bound<&u32> {
		std::ops::Bound::Included(&self.begin)
	}

	#[inline(always)]
	fn end_bound(&self) -> std::ops::Bound<&u32> {
		std::ops::Bound::Excluded(&self.end)
	}
}

impl Display for ThinSpan {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}..{}", self.begin, self.end)
	}
}

impl Default for ThinSpan {
	#[inline(always)]
	fn default() -> Self {
		Self::empty()
	}
}

impl PartialEq for ThinSpan {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.begin == other.begin && self.end == other.end
	}
}

impl<T, U> PartialEq<(T, U)> for ThinSpan
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	fn eq(&self, other: &(T, U)) -> bool {
		self.begin == <T as std::convert::Into<u32>>::into(other.0) &&
			self.end == <U as std::convert::Into<u32>>::into(other.1)
	}
}

impl<T> PartialEq<Range<T>> for ThinSpan
where
	T: Into<u32> + Copy + PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &Range<T>) -> bool {
		self.begin == <T as std::convert::Into<u32>>::into(other.start) &&
			self.end == <T as std::convert::Into<u32>>::into(other.end)
	}
}

impl<T> PartialEq<RangeInclusive<T>> for ThinSpan
where
	T: Into<u32> + Copy + PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &RangeInclusive<T>) -> bool {
		self.begin == <T as std::convert::Into<u32>>::into(*other.start()) &&
			self.end == (<T as std::convert::Into<u32>>::into(*other.end()) + 1)
	}
}

impl Eq for ThinSpan {}

impl PartialOrd for ThinSpan {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<T, U> PartialOrd<(T, U)> for ThinSpan
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	fn partial_cmp(&self, other: &(T, U)) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&<Self as std::convert::From<(T, U)>>::from(*other)))
	}
}

impl<T> PartialOrd<Range<T>> for ThinSpan
where
	T: Into<u32> + Copy + PartialEq,
{
	fn partial_cmp(&self, other: &Range<T>) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&Self::from_range(other.clone())))
	}
}

impl<T> PartialOrd<RangeInclusive<T>> for ThinSpan
where
	T: Into<u32> + Copy + PartialEq,
{
	fn partial_cmp(&self, other: &RangeInclusive<T>) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&Self::from_inclusive_range(other.clone())))
	}
}

impl Ord for ThinSpan {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.begin > other.end {
			std::cmp::Ordering::Greater
		} else if self.end < other.begin {
			std::cmp::Ordering::Less
		} else if self.begin < other.begin {
			if self.end >= other.end {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Less
			}
		} else if self.begin == other.begin {
			if self.end < other.end {
				std::cmp::Ordering::Less
			} else if self.end > other.end {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		} else if self.end == other.end {
			if self.begin < other.begin {
				std::cmp::Ordering::Less
			} else if self.begin > other.begin {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		} else {
			std::cmp::Ordering::Equal
		}
	}
}

impl Hash for ThinSpan {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.begin.hash(state);
		self.end.hash(state);
	}
}

impl Span {
	/// Create a new empty [`Span`], where [`Span::begin`], [`Span::end`],
	/// [`Span::line`], and [`Span::character`] are all set to 0.
	#[inline(always)]
	pub const fn empty() -> Self {
		Self {
			begin:     0,
			end:       0,
			line:      0,
			character: 0,
		}
	}

	#[inline(always)]
	pub const fn from_position(begin: u32, end: u32, position: Position) -> Self {
		Self {
			begin,
			end,
			line: *position.line(),
			character: *position.character(),
		}
	}

	#[inline(always)]
	pub const fn new(begin: u32, end: u32, line: u32, character: u32) -> Self {
		Self { begin, end, line, character }
	}

	/// Create a [`Span`] from the given [`Range`] and [`Position`].
	#[inline(always)]
	pub fn from_range<T>(range: Range<T>, position: Position) -> Self
	where
		T: Into<u32> + Copy,
	{
		Self {
			begin:     <T as std::convert::Into<u32>>::into(range.start),
			end:       <T as std::convert::Into<u32>>::into(range.end),
			line:      *position.line(),
			character: *position.character(),
		}
	}

	/// Attempt to create a [`ThinSpan`] from a [`Range<T>`] and [`Position`].
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`Range::start`] or
	/// [`Range::end`]
	#[inline(always)]
	pub fn try_from_range<T>(
		range: Range<T>,
		position: Position,
	) -> Result<Self, <T as std::convert::TryInto<u32>>::Error>
	where
		T: TryInto<u32> + Copy,
	{
		Ok(Self {
			begin:     <T as std::convert::TryInto<u32>>::try_into(range.start)?,
			end:       <T as std::convert::TryInto<u32>>::try_into(range.end)?,
			line:      *position.line(),
			character: *position.character(),
		})
	}

	/// Create a [`Span`] from the given [`RangeInclusive<T>`] and [`Position`]
	///
	/// Because the [`Span::end`] offset is stored as non-inclusive
	/// this will add `1` to the result of the [`RangeInclusive::end`] calls.
	///
	/// ## Panics
	///
	/// This method will panic if [`RangeInclusive::end`] is already at [`u32::MAX`]
	/// due to an overflow.
	#[inline(always)]
	pub fn from_inclusive_range<T>(range: RangeInclusive<T>, position: Position) -> Self
	where
		T: Into<u32> + Copy,
	{
		Self {
			begin:     <T as std::convert::Into<u32>>::into(*range.start()),
			end:       <T as std::convert::Into<u32>>::into(*range.end()) + 1,
			line:      *position.line(),
			character: *position.character(),
		}
	}

	/// Attempt to create a [`Span`] from the given [`RangeInclusive<T>`] and [`Position`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`RangeInclusive::start`] or
	/// [`RangeInclusive::end`]
	///
	/// ## Note
	///
	/// Because the [`Span::end`] offset is stored as non-inclusive
	/// this will add `1` to the result of the [`RangeInclusive::end`] calls.
	///
	/// ## Panics
	///
	/// This method will panic if [`RangeInclusive::end`] is already at [`u32::MAX`]
	/// due to addition overflow
	#[inline(always)]
	pub fn try_from_inclusive_range<T>(
		range: RangeInclusive<T>,
		position: Position,
	) -> Result<Self, <T as std::convert::TryInto<u32>>::Error>
	where
		T: TryInto<u32> + Copy,
	{
		Ok(Self {
			begin:     <T as std::convert::TryInto<u32>>::try_into(*range.start())?,
			end:       <T as std::convert::TryInto<u32>>::try_into(*range.end())? + 1,
			line:      *position.line(),
			character: *position.character(),
		})
	}

	/// Create a full-fat [`Span`] given a [`ThinSpan`] and [`Position`]
	#[inline(always)]
	pub const fn from_thin(span: ThinSpan, position: Position) -> Self {
		Self {
			begin:     *span.begin(),
			end:       *span.end(),
			line:      *position.line(),
			character: *position.character(),
		}
	}

	/// Convert the current [`Span`] into a [`Range<T>`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn as_range<T>(self) -> Range<T>
	where
		T: From<u32>,
	{
		Range::<T> {
			start: <T as std::convert::From<u32>>::from(self.begin),
			end:   <T as std::convert::From<u32>>::from(self.end),
		}
	}

	/// Convert the current [`Span`] into a [`Range<T>`], unlike [`Span::as_range`] this
	/// does not consume the [`Span`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn get_range<T>(&self) -> Range<T>
	where
		T: From<u32>,
	{
		Range::<T> {
			start: <T as std::convert::From<u32>>::from(self.begin),
			end:   <T as std::convert::From<u32>>::from(self.end),
		}
	}

	/// Attempt to the current [`Span`] into a [`Range<T>`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`Range::start`] or
	/// [`Range::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn try_as_range<T>(self) -> Result<Range<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(Range::<T> {
			start: <T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			end:   <T as std::convert::TryFrom<u32>>::try_from(self.end)?,
		})
	}

	/// Attempt to the current [`Span`] into a [`Range<T>`], unlike [`Span::try_get_range`] this
	/// does not consume the [`Span`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`Range::start`] or
	/// [`Range::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`Range<T>`] where absolutely required as [`Range`] does
	/// not implement [`Copy`]
	#[inline(always)]
	pub fn try_get_range<T>(&self) -> Result<Range<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(Range::<T> {
			start: <T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			end:   <T as std::convert::TryFrom<u32>>::try_from(self.end)?,
		})
	}

	/// Convert the current [`Span`] into a [`RangeInclusive<T>`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, due to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`Span::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn as_inclusive_range<T>(self) -> RangeInclusive<T>
	where
		T: From<u32>,
	{
		RangeInclusive::new(
			<T as std::convert::From<u32>>::from(self.begin),
			<T as std::convert::From<u32>>::from(self.end - 1),
		)
	}

	/// Convert the current [`Span`] into a [`RangeInclusive<T>`], unlike
	/// [`Span::as_inclusive_range`] this does not consume the [`Span`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, due to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`Span::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn get_inclusive_range<T>(&self) -> RangeInclusive<T>
	where
		T: From<u32>,
	{
		RangeInclusive::new(
			<T as std::convert::From<u32>>::from(self.begin),
			<T as std::convert::From<u32>>::from(self.end - 1),
		)
	}

	/// Attempt to convert the current [`Span`] into a [`RangeInclusive<T>`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`RangeInclusive::start`] or
	/// [`RangeInclusive::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`Span::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn try_as_inclusive_range<T>(
		self,
	) -> Result<RangeInclusive<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(RangeInclusive::new(
			<T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			<T as std::convert::TryFrom<u32>>::try_from(self.end - 1)?,
		))
	}

	/// Attempt to convert the current [`Span`] into a [`RangeInclusive<T>`], unlike
	/// [`Span::try_as_inclusive_range`] this does not consume the [`Span`]
	///
	/// Will fail if [`TryInto<u32>`] fails for either [`RangeInclusive::start`] or
	/// [`RangeInclusive::end`]
	///
	/// ## Note
	///
	/// You should only convert to a [`RangeInclusive<T>`] where absolutely required as
	/// [`RangeInclusive`] does not implement [`Copy`]
	///
	/// Additionally, to how the the offsets are stored, this returns the inclusive
	/// range of begin..=end - 1
	///
	/// ## Panics
	///
	/// This method will panic if [`Span::end`] is already at [`u32::MIN`]
	/// due to subtraction underflow.
	#[inline(always)]
	pub fn try_get_inclusive_range<T>(
		&self,
	) -> Result<RangeInclusive<T>, <T as std::convert::TryFrom<u32>>::Error>
	where
		T: TryFrom<u32>,
	{
		Ok(RangeInclusive::new(
			<T as std::convert::TryFrom<u32>>::try_from(self.begin)?,
			<T as std::convert::TryFrom<u32>>::try_from(self.end - 1)?,
		))
	}

	/// Convert this [`Span`] into a [`ThinSpan`], throwing away the textual position information
	///
	/// If you wish to keep this information, see [`Span::split`].
	#[inline(always)]
	pub const fn as_thin(self) -> ThinSpan {
		ThinSpan::new(self.begin, self.end)
	}

	/// Create a new [`ThinSpan`] from the [`Span::begin`] and [`Span::end`] stored in this
	/// span.
	///
	/// Unlike [`Span::as_thin`] this does no consume the [`Span`]
	#[inline(always)]
	pub const fn get_thin(&self) -> ThinSpan {
		ThinSpan::new(self.begin, self.end)
	}

	/// Convert this [`Span`] into a [`Position`], throwing away the span information
	///
	/// If you wish to keep this information, see [`Span::split`].
	#[inline(always)]
	pub const fn as_position(self) -> Position {
		Position::new(self.line, self.character)
	}

	/// Create a new [`Position`] from the [`Span::line`] and [`Span::character`] stored in this
	/// span.
	///
	/// Unlike [`Span::as_position`] this does no consume the [`Span`]
	#[inline(always)]
	pub const fn get_position(&self) -> Position {
		Position::new(self.line, self.character)
	}

	/// Split this [`Span`] into a [`ThinSpan`] and a [`Position`]
	///
	/// If you only care about the textual position information see [`Span::as_position`], likewise
	/// see [`Span::as_thin`] if you only want the span information.
	#[inline(always)]
	pub const fn split(self) -> (ThinSpan, Position) {
		(
			ThinSpan::new(self.begin, self.end),
			Position::new(self.line, self.character),
		)
	}

	/// The start of the range represented by [`Span`]
	#[inline(always)]
	pub const fn begin(&self) -> &u32 {
		&self.begin
	}

	/// The end of the range represented by [`Span`]
	#[inline(always)]
	pub const fn end(&self) -> &u32 {
		&self.end
	}

	/// The logical line number (number of previous new lines) that [`Span::begin`]
	/// falls on.
	#[inline(always)]
	pub const fn line(&self) -> &u32 {
		&self.line
	}

	/// The logical character number (number of characters since last newline) that
	/// [`Span::begin`] falls on.
	#[inline(always)]
	pub const fn character(&self) -> &u32 {
		&self.character
	}

	/// Convert this [`Span`] into a [`ThinSpanned<T>`] using the given
	/// other T, by first converting this [`Span`] to a [`ThinSpanned`] with
	/// [`Span::as_thin`] then attaching to it.
	#[inline(always)]
	pub const fn as_thin_spanned<T>(self, inner: T) -> ThinSpanned<T>
	where
		T: Clone,
	{
		ThinSpanned::new(inner, self.as_thin())
	}

	/// Convert this [`Span`] into a [`ThinSpanned<T>`] using the given
	/// other T, by first converting this [`Span`] to a [`ThinSpanned`] with
	/// [`Span::as_thin`] then attaching to it.
	///
	/// Unlike [`Span::as_thin_spanned`] this does not consume the [`Span`]
	#[inline(always)]
	pub const fn get_thin_spanned<T>(&self, inner: T) -> ThinSpanned<T>
	where
		T: Clone,
	{
		ThinSpanned::new(inner, self.get_thin())
	}

	/// Convert this [`Span`] into a [`Spanned<T>`] using the given
	/// other T.
	#[inline(always)]
	pub const fn as_spanned<T>(self, inner: T) -> Spanned<T>
	where
		T: Clone,
	{
		Spanned::new(inner, self)
	}

	/// Convert this [`Span`] into a [`Spanned<T>`] using the given
	/// other T.
	///
	/// Unlike [`Span::as_spanned`] this does not consume the [`Span`]
	#[inline(always)]
	pub const fn get_spanned<T>(&self, inner: T) -> Spanned<T>
	where
		T: Clone,
	{
		Spanned::new(inner, *self)
	}
}

impl<T, U> From<(T, U)> for Span
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: (T, U)) -> Self {
		Self {
			begin:     <T as std::convert::Into<u32>>::into(value.0),
			end:       <U as std::convert::Into<u32>>::into(value.1),
			line:      0,
			character: 0,
		}
	}
}

impl<T, U, V, W> From<(T, U, V, W)> for Span
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
	V: Into<u32> + Copy,
	W: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: (T, U, V, W)) -> Self {
		Self {
			begin:     <T as std::convert::Into<u32>>::into(value.0),
			end:       <U as std::convert::Into<u32>>::into(value.1),
			line:      <V as std::convert::Into<u32>>::into(value.2),
			character: <W as std::convert::Into<u32>>::into(value.3),
		}
	}
}

impl<T, U> From<(T, U, Position)> for Span
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: (T, U, Position)) -> Self {
		let pos = value.2;
		Self {
			begin:     <T as std::convert::Into<u32>>::into(value.0),
			end:       <U as std::convert::Into<u32>>::into(value.1),
			line:      *pos.line(),
			character: *pos.character(),
		}
	}
}

impl<T> From<(Range<T>, Position)> for Span
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: (Range<T>, Position)) -> Self {
		Self::from_range(value.0, value.1)
	}
}

impl<T> From<Range<T>> for Span
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: Range<T>) -> Self {
		Self::from_range(value, Position::sof())
	}
}

impl<T> From<RangeInclusive<T>> for Span
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn from(value: RangeInclusive<T>) -> Self {
		Self::from_inclusive_range(value, Position::sof())
	}
}

impl From<Span> for ThinSpan {
	#[inline(always)]
	fn from(value: Span) -> Self {
		value.as_thin()
	}
}

impl Add for Span {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			begin:     if self.begin < rhs.begin {
				self.begin
			} else {
				rhs.begin
			},
			end:       if self.end > rhs.end {
				self.end
			} else {
				rhs.end
			},
			line:      if self.line < rhs.line {
				self.line
			} else {
				rhs.line
			},
			character: if self.character > rhs.character {
				self.character
			} else {
				rhs.character
			},
		}
	}
}

impl<T> Add<T> for Span
where
	T: Into<u32> + Copy,
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: T) -> Self::Output {
		Self {
			begin:     self.begin,
			end:       self.end + <T as std::convert::Into<u32>>::into(rhs),
			line:      self.line,
			character: self.character,
		}
	}
}

impl AddAssign for Span {
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) {
		if self.begin > rhs.begin {
			self.begin = rhs.begin;
		}

		if self.end < rhs.end {
			self.end = rhs.end;
		}

		if self.line > rhs.line {
			self.line = rhs.line;
		}

		if self.character < rhs.character {
			self.character = rhs.character;
		}
	}
}

impl<T> AddAssign<T> for Span
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: T) {
		self.end += <T as std::convert::Into<u32>>::into(rhs);
	}
}

impl Sub for Span {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			begin: if rhs.begin < self.end {
				rhs.begin
			} else {
				0
			},
			end:   if self.end < rhs.end {
				self.end
			} else {
				0
			},

			line:      if rhs.begin < self.end {
				rhs.line
			} else {
				0
			},
			character: if self.end < rhs.end {
				self.character
			} else {
				0
			},
		}
	}
}

impl<T> Sub<T> for Span
where
	T: Into<u32> + Copy,
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: T) -> Self::Output {
		Self {
			begin:     self.begin - <T as std::convert::Into<u32>>::into(rhs),
			end:       self.end,
			line:      self.line,
			character: self.character,
		}
	}
}

impl SubAssign for Span {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) {
		if rhs.begin < self.end {
			self.begin = rhs.begin;
			self.line = rhs.line;
		} else {
			self.begin = 0;
			self.line = 0;
		}

		if self.end < rhs.end {
			self.character = 0
		} else {
			self.end = 0;
			self.character = rhs.character;
		}
	}
}

impl<T> SubAssign<T> for Span
where
	T: Into<u32> + Copy,
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: T) {
		self.begin -= <T as std::convert::Into<u32>>::into(rhs);
	}
}

impl RangeBounds<u32> for Span {
	#[inline(always)]
	fn start_bound(&self) -> std::ops::Bound<&u32> {
		std::ops::Bound::Included(&self.begin)
	}

	#[inline(always)]
	fn end_bound(&self) -> std::ops::Bound<&u32> {
		std::ops::Bound::Excluded(&self.end)
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}..{}", self.begin, self.end)
	}
}

impl Default for Span {
	#[inline(always)]
	fn default() -> Self {
		Self::empty()
	}
}

impl PartialEq for Span {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.begin == other.begin &&
			self.end == other.end &&
			self.line == other.line &&
			self.character == other.character
	}
}

impl<T, U> PartialEq<(T, U)> for Span
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	fn eq(&self, other: &(T, U)) -> bool {
		self.begin == <T as std::convert::Into<u32>>::into(other.0) &&
			self.end == <U as std::convert::Into<u32>>::into(other.1)
	}
}

impl<T> PartialEq<Range<T>> for Span
where
	T: Into<u32> + Copy + PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &Range<T>) -> bool {
		self.begin == <T as std::convert::Into<u32>>::into(other.start) &&
			self.end == <T as std::convert::Into<u32>>::into(other.end)
	}
}

impl<T> PartialEq<RangeInclusive<T>> for Span
where
	T: Into<u32> + Copy + PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &RangeInclusive<T>) -> bool {
		self.begin == <T as std::convert::Into<u32>>::into(*other.start()) &&
			self.end == (<T as std::convert::Into<u32>>::into(*other.end()) + 1)
	}
}

impl Eq for Span {}

impl PartialOrd for Span {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<T, U> PartialOrd<(T, U)> for Span
where
	T: Into<u32> + Copy,
	U: Into<u32> + Copy,
{
	fn partial_cmp(&self, other: &(T, U)) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&<Self as std::convert::From<(T, U)>>::from(*other)))
	}
}

impl<T> PartialOrd<Range<T>> for Span
where
	T: Into<u32> + Copy + PartialEq,
{
	fn partial_cmp(&self, other: &Range<T>) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&Self::from_range(other.clone(), Position::eof())))
	}
}

impl<T> PartialOrd<RangeInclusive<T>> for Span
where
	T: Into<u32> + Copy + PartialEq,
{
	fn partial_cmp(&self, other: &RangeInclusive<T>) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&Self::from_inclusive_range(other.clone(), Position::sof())))
	}
}

impl Ord for Span {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.begin > other.end {
			std::cmp::Ordering::Greater
		} else if self.end < other.begin {
			std::cmp::Ordering::Less
		} else if self.begin < other.begin {
			if self.end >= other.end {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Less
			}
		} else if self.begin == other.begin && self.end != other.end {
			if self.end < other.end {
				std::cmp::Ordering::Less
			} else if self.end > other.end {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		} else if self.end == other.end && self.begin != other.begin {
			if self.begin < other.begin {
				std::cmp::Ordering::Less
			} else if self.begin > other.begin {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		} else {
			self.get_position().cmp(&other.get_position())
		}
	}
}

impl Hash for Span {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.begin.hash(state);
		self.end.hash(state);
		self.line.hash(state);
		self.character.hash(state);
	}
}

impl<T> ThinSpanned<T>
where
	T: Clone,
{
	#[inline(always)]
	pub const fn new(inner: T, span: ThinSpan) -> Self {
		Self { inner, span }
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
	pub fn as_inner(self) -> T {
		self.inner
	}

	#[inline(always)]
	pub const fn span(&self) -> &ThinSpan {
		&self.span
	}

	#[inline(always)]
	pub const fn span_mut(&mut self) -> &mut ThinSpan {
		&mut self.span
	}

	#[inline(always)]
	pub fn as_span(self) -> ThinSpan {
		self.span
	}

	#[inline(always)]
	pub fn split(self) -> (T, ThinSpan) {
		(self.inner, self.span)
	}

	#[inline(always)]
	pub fn into_spanned(self, position: Option<Position>) -> Spanned<T> {
		Spanned::new(self.inner, self.span.as_span(position))
	}
}

impl<T> From<(T, ThinSpan)> for ThinSpanned<T>
where
	T: Clone,
{
	#[inline(always)]
	fn from(value: (T, ThinSpan)) -> Self {
		Self::new(value.0, value.1)
	}
}

impl<T> Deref for ThinSpanned<T>
where
	T: Clone,
{
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> DerefMut for ThinSpanned<T>
where
	T: Clone,
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T> PartialEq for ThinSpanned<T>
where
	T: Clone + PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner && self.span == other.span
	}
}

impl<T> Eq for ThinSpanned<T> where T: Clone + Eq {}

impl<T> PartialOrd for ThinSpanned<T>
where
	T: Clone + PartialOrd,
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.span.partial_cmp(&other.span)
	}
}

impl<T> Ord for ThinSpanned<T>
where
	T: Clone + Ord,
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.span.cmp(&other.span)
	}
}

impl<T> Hash for ThinSpanned<T>
where
	T: Clone + Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
		self.span.hash(state);
	}
}

impl<T> Display for ThinSpanned<T>
where
	T: Clone + Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} @ {}", self.inner, self.span)
	}
}

impl<T> RangeBounds<u32> for ThinSpanned<T>
where
	T: Clone,
{
	#[inline(always)]
	fn start_bound(&self) -> std::ops::Bound<&u32> {
		self.span.start_bound()
	}

	#[inline(always)]
	fn end_bound(&self) -> std::ops::Bound<&u32> {
		self.span.end_bound()
	}
}

impl<T> Debug for ThinSpanned<T>
where
	T: Clone + Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ThinSpanned")
			.field("inner", &self.inner)
			.field("span", &self.span)
			.finish()
	}
}

impl<T> Spanned<T>
where
	T: Clone,
{
	#[inline(always)]
	pub const fn new(inner: T, span: Span) -> Self {
		Self { inner, span }
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
	pub fn as_inner(self) -> T {
		self.inner
	}

	#[inline(always)]
	pub const fn span(&self) -> &Span {
		&self.span
	}

	#[inline(always)]
	pub const fn span_mut(&mut self) -> &mut Span {
		&mut self.span
	}

	#[inline(always)]
	pub fn as_span(self) -> Span {
		self.span
	}

	#[inline(always)]
	pub fn split(self) -> (T, Span) {
		(self.inner, self.span)
	}

	#[inline(always)]
	pub fn into_thin_spanned(self) -> ThinSpanned<T> {
		ThinSpanned::new(self.inner, self.span.as_thin())
	}
}

impl<T> From<(T, Span)> for Spanned<T>
where
	T: Clone,
{
	#[inline(always)]
	fn from(value: (T, Span)) -> Self {
		Self::new(value.0, value.1)
	}
}

impl<T> Deref for Spanned<T>
where
	T: Clone,
{
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> DerefMut for Spanned<T>
where
	T: Clone,
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T> PartialEq for Spanned<T>
where
	T: Clone + PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner && self.span == other.span
	}
}

impl<T> Eq for Spanned<T> where T: Clone + Eq {}

impl<T> PartialOrd for Spanned<T>
where
	T: Clone + PartialOrd,
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.span.partial_cmp(&other.span)
	}
}

impl<T> Ord for Spanned<T>
where
	T: Clone + Ord,
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.span.cmp(&other.span)
	}
}

impl<T> Display for Spanned<T>
where
	T: Clone + Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} @ {}", self.inner, self.span)
	}
}

impl<T> Hash for Spanned<T>
where
	T: Clone + Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
		self.span.hash(state);
	}
}

impl<T> RangeBounds<u32> for Spanned<T>
where
	T: Clone,
{
	#[inline(always)]
	fn start_bound(&self) -> std::ops::Bound<&u32> {
		self.span.start_bound()
	}

	#[inline(always)]
	fn end_bound(&self) -> std::ops::Bound<&u32> {
		self.span.end_bound()
	}
}

impl<T> Debug for Spanned<T>
where
	T: Clone + Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Spanned")
			.field("inner", &self.inner)
			.field("span", &self.span)
			.finish()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_thin_span_equality() {
		assert_eq!(ThinSpan::new(0, 0), ThinSpan::new(0, 0));
		assert_ne!(ThinSpan::new(0, 0), ThinSpan::new(0, 1));
		assert_ne!(ThinSpan::new(0, 0), ThinSpan::new(1, 0));
		assert_ne!(ThinSpan::new(0, 0), ThinSpan::new(1, 1));
	}

	#[test]
	fn test_span_equality() {
		assert_eq!(Span::new(0, 0, 0, 0), Span::new(0, 0, 0, 0));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(0, 1, 0, 0));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(1, 0, 0, 0));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(1, 1, 0, 0));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(0, 0, 0, 1));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(0, 0, 1, 0));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(0, 0, 1, 1));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(0, 1, 0, 1));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(1, 0, 1, 0));
		assert_ne!(Span::new(0, 0, 0, 0), Span::new(1, 1, 1, 1));
	}

	#[test]
	fn test_thin_span_ordering() {
		assert!(ThinSpan::new(0, 0) < ThinSpan::new(0, 1));
		assert!(ThinSpan::new(0, 0) < ThinSpan::new(1, 0));
		assert!(ThinSpan::new(0, 0) < ThinSpan::new(1, 1));
		assert!(ThinSpan::new(1, 5) > ThinSpan::new(2, 5));
		assert!(ThinSpan::new(1, 5) > ThinSpan::new(2, 3));
		assert!(ThinSpan::new(1, 5) > ThinSpan::new(1, 3));
		assert!(ThinSpan::new(1, 5) < ThinSpan::new(2, 7));
	}

	#[test]
	fn test_span_ordering() {
		assert!(Span::new(0, 0, 0, 0) < Span::new(0, 1, 0, 0));
		assert!(Span::new(0, 0, 0, 0) < Span::new(1, 0, 0, 0));
		assert!(Span::new(0, 0, 0, 0) < Span::new(1, 1, 0, 0));
		assert!(Span::new(1, 5, 0, 0) > Span::new(2, 5, 0, 0));
		assert!(Span::new(1, 5, 0, 0) > Span::new(2, 3, 0, 0));
		assert!(Span::new(1, 5, 0, 0) > Span::new(1, 3, 0, 0));
		assert!(Span::new(1, 5, 0, 0) < Span::new(2, 7, 0, 0));
		assert!(Span::new(0, 0, 0, 0) < Span::new(0, 0, 1, 0));
		assert!(Span::new(0, 0, 0, 0) < Span::new(0, 0, 0, 1));
	}
}
