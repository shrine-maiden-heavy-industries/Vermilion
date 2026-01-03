/* SPDX-License-Identifier: BSD-3-Clause */

use std::vec;

use vermilion_lsp::types::{
	SemanticTokensLegend,
	capabilities::server::SemanticTokensServerCapabilities,
	options::{SemanticTokensOptions, WorkDoneProgressOptions}
};

fn build_legend() -> SemanticTokensLegend {
	let token_types = vec![
		"class",
		"comment",
		"decorator",
		"enum",
		"enumMember",
		"event",
		"function",
		"interface",
		"keyword",
		"macro",
		"method",
		"modifier",
		"namespace",
		"number",
		"operator",
		"parameter",
		"property",
		"regexp",
		"string",
		"struct",
		"type",
		"typeParameter",
		"variable",
	];

	let token_modifiers = vec![
		"abstract",
		"async",
		"declaration",
		"defaultLibrary",
		"definition",
		"deprecated",
		"documentation",
		"modification",
		"readOnly",
		"static",
	];

	SemanticTokensLegend::from_str_vec(token_types, token_modifiers)
}

pub fn capabilities() -> SemanticTokensServerCapabilities {
	let legend = build_legend();

	SemanticTokensServerCapabilities::Options(SemanticTokensOptions {
		legend,
		range: None,
		full: Some(true),
		work_done_progress_options: WorkDoneProgressOptions {
			work_done_progress: None,
		}
	})
}
