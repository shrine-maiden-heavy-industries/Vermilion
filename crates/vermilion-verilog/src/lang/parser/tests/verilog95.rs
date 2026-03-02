// SPDX-License-Identifier: BSD-3-Clause

use crate::lang::parser::{Diagnostic, Position, Span};

// #[test]
#[allow(unused)]
fn test_truncated_module() {
	// SAFETY: We're in a testing context, panicking here is fine
	#[allow(clippy::unwrap_used)]
	let result = VerilogParser::new(LanguageStd::Vl95, "module".as_bytes().into())
		.unwrap()
		.parse();

	assert!(result.is_ok());
	#[allow(clippy::unwrap_used)]
	let ast = result.unwrap();

	assert!(ast.has_diagnostics());
	let diagnostics = ast.diagnostics();
	assert_eq!(
		diagnostics,
		&vec![Diagnostic::new(
			Some(Span::new(0..0, Position::new(0, 0))),
			"Encountered end of file, expected module name"
		)]
	);

	// SAFETY: We're in a testing context, panicking here is fine
	#[allow(clippy::unwrap_used)]
	let result = VerilogParser::new(LanguageStd::Vl95, "module;".as_bytes().into())
		.unwrap()
		.parse();

	assert!(result.is_ok());
	#[allow(clippy::unwrap_used)]
	let ast = result.unwrap();

	assert!(ast.has_diagnostics());
	let diagnostics = ast.diagnostics();
	assert_eq!(
		diagnostics,
		&vec![Diagnostic::new(
			Some(Span::new(6..7, Position::new(0, 6))),
			"Expected module name, got ';' @ [6,7) => line: 0 char: 6"
		)]
	);

	// SAFETY: We're in a testing context, panicking here is fine
	#[allow(clippy::unwrap_used)]
	let result = VerilogParser::new(LanguageStd::Vl95, "module a".as_bytes().into())
		.unwrap()
		.parse();

	assert!(result.is_ok());
	#[allow(clippy::unwrap_used)]
	let ast = result.unwrap();
	assert!(!ast.has_diagnostics());
	let modules: Vec<_> = ast.modules().collect();
	assert_eq!(modules.len(), 1);
	let module = modules[0];

	let diagnostics = module.diagnostics();
	assert!(diagnostics.is_some());
	#[allow(clippy::unwrap_used)]
	let diagnostics = diagnostics.unwrap();

	assert_eq!(
		diagnostics,
		&vec![Diagnostic::new(
			Some(Span::new(0..0, Position::eof())),
			"Unexpected end of file, expected ';' following module identifier and port list"
		)]
	);
}
