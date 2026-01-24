// SPDX-License-Identifier: BSD-3-Clause

use crate::{
	VerilogStd, VerilogVariant,
	lang::parser::{Diagnostic, Position, Span, VerilogParser},
};

#[test]
fn test_truncated_module() {
	let result = VerilogParser::new(
		VerilogVariant::Verilog(VerilogStd::Vl95),
		"module".as_bytes().into(),
	)
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
}
