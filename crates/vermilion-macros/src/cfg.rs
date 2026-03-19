// SPDX-License-Identifier: BSD-3-Clause

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
