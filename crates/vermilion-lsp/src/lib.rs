// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

use vermilion_macros::{cfg_lsp_trace, cfg_lsp_transport};

pub mod error;
pub mod message;
pub mod notification;
pub mod request;
pub mod response;
cfg_lsp_trace! {
	pub mod trace;
}
cfg_lsp_transport! {
	pub mod transports;
}
pub mod types;

pub mod prelude {
	pub use super::{
		message::{Id, Message},
		notification::Notification,
		request::Request,
		response::Response,
	};
}
