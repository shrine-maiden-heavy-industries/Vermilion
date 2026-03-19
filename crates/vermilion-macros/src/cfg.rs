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

/// Enables serde specific code.
///
/// Use this rather than `#[cfg(feature = "serde")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_serde {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "serde")]
			#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
            $item
        )*
    }
}

/// Enables schema specific code.
///
/// Use this rather than `#[cfg(feature = "schema")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_schema {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "schema")]
			#[cfg_attr(docsrs, doc(cfg(feature = "schema")))]
            $item
        )*
    }
}

/// Enables vermilion cli specific code.
///
/// Use this rather than `#[cfg(feature = "cli")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_cli {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "cli")]
			#[cfg_attr(docsrs, doc(cfg(feature = "cli")))]
            $item
        )*
    }
}

/// Enables graphviz output specific code.
///
/// Use this rather than `#[cfg(feature = "graphviz")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_graphviz {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "graphviz")]
			#[cfg_attr(docsrs, doc(cfg(feature = "graphviz")))]
            $item
        )*
    }
}

/// Enables lsp-trace specific code.
///
/// Use this rather than `#[cfg(feature = "trace")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_lsp_trace {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "trace")]
			#[cfg_attr(docsrs, doc(cfg(feature = "trace")))]
            $item
        )*
    }
}

/// Enables lsp-trace server specific code.
///
/// Use this rather than `#[cfg(feature = "trace-server")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_lsp_trace_server {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "trace-server")]
			#[cfg_attr(docsrs, doc(cfg(feature = "trace-server")))]
            $item
        )*
    }
}

/// Enables lsp transport specific code.
///
/// Use this rather than `#[cfg(feature = "transport")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_lsp_transport {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "transport")]
			#[cfg_attr(docsrs, doc(cfg(feature = "transport")))]
            $item
        )*
    }
}

/// Enables diagnostic rendering specific code.
///
/// Use this rather than `#[cfg(feature = "render")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_diagnostics_render {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "render")]
			#[cfg_attr(docsrs, doc(cfg(feature = "render")))]
            $item
        )*
    }
}

/// Enables pretty diagnostic rendering specific code.
///
/// Use this rather than `#[cfg(feature = "pretty")]` to ensure docs are properly generated.
#[macro_export]
macro_rules! cfg_diagnostics_pretty {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "pretty")]
			#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
            $item
        )*
    }
}
