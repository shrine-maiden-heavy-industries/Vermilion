// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod debug;
pub mod parser;
pub mod position;
pub mod span;
pub mod tokenizer;

pub use position::Position;
pub use span::{Span, Spanned};
use tendril::{Atomic, Tendril, fmt};

pub type AtomicByteTendril = Tendril<fmt::Bytes, Atomic>;

pub trait LanguageMetadata {
	fn file_extensions<'a, 'b: 'a>() -> &'a [&'b str];
}
