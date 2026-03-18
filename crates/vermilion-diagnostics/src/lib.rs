// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

pub mod code;
pub mod config;
pub mod diagnostic;
pub mod macros;
#[cfg(feature = "render-diagnostics")]
pub mod render;
pub mod workspace;

pub use crate::{
	code::Code,
	diagnostic::{Diagnostic, StrDiagnostic, StringDiagnostic},
};
