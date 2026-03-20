// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

pub mod debug;
pub mod parser;
pub mod tokenizer;

use tendril::{Atomic, Tendril, fmt};

pub type AtomicByteTendril = Tendril<fmt::Bytes, Atomic>;

#[macro_export]
macro_rules! spanned_token {
	($token:expr) => {
		vermilion_loc::spanned!($token)
	};
	($token:expr, $span:expr) => {
		vermilion_loc::spanned!($token, $span)
	};
	($token:expr, $range:expr, $position:expr) => {
		vermilion_loc::spanned!($token, $range, $position)
	};
}

#[macro_export]
macro_rules! thin_spanned_token {
	($token:expr) => {
		vermilion_loc::thin_spanned!($token)
	};
	($token:expr, $span:expr) => {
		vermilion_loc::thin_spanned!($token, $span)
	};
}
