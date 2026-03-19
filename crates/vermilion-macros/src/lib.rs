// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

/// Enables fuzzing specific code when `--cfg fuzzing` is passed.
///
/// Use this rather than `#[cfg(fuzzing)]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_fuzzing {
    ($($item:item)*) => {
        $(
            #[cfg(fuzzing)]
			#[cfg_attr(docsrs, doc(cfg(fuzzing)))]
            $item
        )*
    }
}
