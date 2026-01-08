// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod error;
pub mod message;
pub mod notification;
pub mod request;
pub mod response;
pub mod trace;
#[cfg(feature = "transport")]
pub mod transports;
pub mod types;

pub mod prelude {
	pub use super::{
		message::{Id, Message},
		notification::Notification,
		request::Request,
		response::Response,
	};
}
