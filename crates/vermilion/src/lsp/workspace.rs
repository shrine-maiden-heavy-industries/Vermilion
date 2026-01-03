// SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;

use tracing::debug;
use vermilion_lang::AtomicByteTendril;
use vermilion_lsp::types::{
	Diagnostic, LanguageId, TextDocumentItem, Uri,
	semantic_tokens::{SemanticToken, SemanticTokens},
};
use vermilion_verilog::{SystemVerilogStd, VerilogAmsStd};

use crate::lang::{Ast, Language, VerilogAst, VhdlAst};

pub struct Workspace {
	documents: HashMap<Uri, Document>,
}

pub struct Document {
	content:  AtomicByteTendril,
	language: Language,
	ast:      Ast,
	diagnostics: Vec<Diagnostic>,
}

fn language_for(id: &LanguageId) -> Option<Language> {
	match id {
		LanguageId::Other(language) => match language.as_str() {
			"verilog" => Some(Language::Verilog(vermilion_verilog::VerilogStd::Vl95)),
			"vhdl" => Some(Language::Vhdl(vermilion_vhdl::VhdlStd::Vh87)),
			"system-verilog" => Some(Language::SystemVerilog(SystemVerilogStd::Sv05)),
			"verilog-ams" => Some(Language::VerilogAms(VerilogAmsStd::Vams09)),
			&_ => None,
		},
		_ => None,
	}
}

impl Workspace {
	pub fn new() -> Self {
		Self { documents: HashMap::new() }
	}

	pub fn find_document(&self, uri: &Uri) -> Option<&Document> {
		self.documents.get(uri)
	}

	pub fn open_document(&mut self, document: TextDocumentItem) {
		let language = if let Some(language) = language_for(document.language_id()) {
			language
		} else {
			debug!(
				"Ignoring document {} with unsupported language {:?}",
				document.uri(),
				document.language_id()
			);
			return;
		};

		let content = AtomicByteTendril::from_slice(document.text().as_bytes());
		let ast = language.parse_file(content.clone());

		self.documents.insert(
			document.uri().clone(),
			Document { content, language, ast, diagnostics: Vec::new() },
		);
	}
}

impl Document {
	fn vhdl_semantic_tokens(&self, _ast: &VhdlAst) -> Vec<SemanticToken> {
		todo!();
	}

	fn verilog_semantic_tokens(&self, ast: &VerilogAst) -> Vec<SemanticToken> {
		let mut tokens = Vec::new();

		debug!("meow - have to convert ast into tokens here");

		tokens
	}

	pub fn semantic_tokens(&self) -> SemanticTokens {
		SemanticTokens::new(match &self.ast {
			Ast::Vhdl(ast) => self.vhdl_semantic_tokens(ast),
			Ast::Verilog(ast) => self.verilog_semantic_tokens(ast),
		})
	}
}
