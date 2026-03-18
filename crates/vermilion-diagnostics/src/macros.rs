// SPDX-License-Identifier: BSD-3-Clause

#[macro_export]
macro_rules! define_diagnostics_table {
	(info, $code_path:literal, $code_type:ident, $($code:literal => $desc:literal,)*) => {
		paste::paste! {
			$crate::define_diagnostics_table!(_impl, [<$code_type _info>], I, INFO, $code_path, $($code => $desc,)*);

			pub fn [<get_ $code_type _info_diagnostic>](code: $crate::Code) -> Option<&'static $crate::StrDiagnostic<'static>> {
				INFO_DIAGNOSTICS.get(&u32::from(code.value()))
			}
		}
	};
	(lint, $code_path:literal, $code_type:ident, $($code:literal => $desc:literal,)*) => {
		paste::paste! {
			$crate::define_diagnostics_table!(_impl, [<$code_type _lint>], L, LINT, $code_path, $($code => $desc,)*);

			pub fn [<get_ $code_type _lint_diagnostic>](code: $crate::Code) -> Option<&'static $crate::StrDiagnostic<'static>> {
				LINT_DIAGNOSTICS.get(&u32::from(code.value()))
			}
		}
	};
	(warn, $code_path:literal, $code_type:ident, $($code:literal => $desc:literal,)*) => {
		paste::paste! {
			$crate::define_diagnostics_table!(_impl, [<$code_type _warn>], W, WARN, $code_path, $($code => $desc,)*);

			pub fn [<get_ $code_type _warn_diagnostic>](code: $crate::Code) -> Option<&'static $crate::StrDiagnostic<'static>> {
				WARN_DIAGNOSTICS.get(&u32::from(code.value()))
			}
		}
	};
	(error, $code_path:literal, $code_type:ident, $($code:literal => $desc:literal,)*) => {
		paste::paste! {
			$crate::define_diagnostics_table!(_impl, [<$code_type _error>], E, ERROR, $code_path, $($code => $desc,)*);

			pub fn [<get_ $code_type _error_diagnostic>](code: $crate::Code) -> Option<&'static $crate::StrDiagnostic<'static>> {
				ERROR_DIAGNOSTICS.get(&u32::from(code.value()))
			}
		}
	};
	(debug, $code_path:literal, $code_type:ident, $($code:literal => $desc:literal,)*) => {
		paste::paste! {
			$crate::define_diagnostics_table!(_impl, [<$code_type _debug>], D, DEBUG, $code_path, $($code => $desc,)*);

			pub fn [<get_ $code_type _debug_diagnostic>](code: $crate::Code) -> Option<&'static $crate::StrDiagnostic<'static>> {
				DEBUG_DIAGNOSTICS.get(&u32::from(code.value()))
			}
		}
	};
	(_impl, $code_new:ident, $code_prefix:ident, $name:ident, $code_path:literal, $($code:literal => $desc:literal,)*) => {
		$(
			paste::paste! {
				#[allow(clippy::zero_prefixed_literal, reason = "Diagnostic codes are zero-padded to 4 digits")]
				#[doc = $desc]
				pub const [<$code_prefix $code>] : $crate::Code = $crate::Code::[<new_ $code_new>]($code);
			}
		)*

		paste::paste! {
			#[allow(clippy::zero_prefixed_literal, reason = "Diagnostic codes are zero-padded to 4 digits")]
			pub static [<$name _DIAGNOSTICS>] : phf::Map<u32, $crate::StrDiagnostic<'static>> = phf::phf_map! {
				$(
					$code => $crate::StrDiagnostic::init(
						[<$code_prefix $code>],
						include_str!(concat!($code_path, "/", stringify!($code_prefix), stringify!($code), ".md"))
					),
				)*
			};
		}
	};
}
