// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

pub mod debug;
pub mod parser;
pub mod tokenizer;

use tendril::{Atomic, Tendril, fmt};
pub use vermilion_loc::{
	position::Position,
	span::{Span, Spanned},
};

pub type AtomicByteTendril = Tendril<fmt::Bytes, Atomic>;

#[macro_export]
macro_rules! spanned_token {
	($token:expr) => {
		vermilion_loc::spanned!($token)
	};
	($token:expr, $span_range:expr, $context:expr) => {
		vermilion_loc::spanned!($token, $span_range, $context)
	};
}

pub trait LanguageMetadata {
	fn file_extensions<'a, 'b: 'a>() -> &'a [&'b str];
}
