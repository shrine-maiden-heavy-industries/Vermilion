// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

pub mod config;
pub mod workspace;

pub enum Version {
	/// Conforms to the SystemVerilog and VHDL LRMs with tool-specific interpretation of the use
	/// model and pragma syntax.
	///
	/// There is no expectation of interoperability.
	///
	/// This version only exists to support backward compatibility to older implementations.
	Zero,
	/// Conforms to IEEE 1735 basic interoperability specifications, which are important both
	/// because they allow basic interoperability of protected IP across a set of tools and because
	/// there is already defacto conformance to them at this time.
	One,
	/// Conforms to all IEEE 1735 recommendations for use model and pragmas, including all Version 1
	/// requirements.
	Two,
}
