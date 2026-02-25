// SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;

use tracing::{debug, warn};
use vermilion_lang::AtomicByteTendril;
use vermilion_lsp::types::{
	Diagnostic, LanguageId, TextDocumentContentChangeEvent, TextDocumentItem, Uri,
	semantic_tokens::{SemanticToken, SemanticTokens},
};
use vermilion_verilog::LanguageStd as VerilogStd;
use vermilion_vhdl::LanguageStd as VhdlStd;

use crate::lang::{Ast, Language, VerilogAst, VhdlAst};

pub struct Workspace {
	documents: HashMap<Uri, Document>,
}

pub struct Document {
	content:      AtomicByteTendril,
	_language:    Language,
	ast:          Ast,
	_diagnostics: Vec<Diagnostic>,
}

fn language_for(id: &LanguageId) -> Option<Language> {
	match id {
		LanguageId::Other(language) => match language.as_str() {
			"verilog" => Some(Language::Verilog(VerilogStd::Vl95)),
			"vhdl" => Some(Language::Vhdl(VhdlStd::Vh87)),
			"system-verilog" => Some(Language::Verilog(VerilogStd::Sv05)),
			"verilog-ams" => Some(Language::Verilog(VerilogStd::Vams09)),
			"vhdl-ams" => Some(Language::Vhdl(VhdlStd::Vhams99)),
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
			Document {
				content,
				_language: language,
				ast,
				_diagnostics: Vec::new(),
			},
		);
	}

	pub fn close_document(&mut self, uri: &Uri) {
		self.documents.remove(uri);
	}

	pub fn change_document(&mut self, uri: &Uri, changes: Vec<TextDocumentContentChangeEvent>) {
		match self.documents.get_mut(uri) {
			Some(document) => document.apply_changes(changes),
			None => warn!("Got document changes for unknown/unopened document {uri}"),
		}
	}
}

impl Document {
	fn vhdl_semantic_tokens(&self, _ast: &VhdlAst) -> Vec<SemanticToken> {
		todo!();
	}

	fn verilog_semantic_tokens(&self, _ast: &VerilogAst) -> Vec<SemanticToken> {
		let tokens = Vec::new();

		debug!("meow - have to convert ast into tokens here");

		tokens
	}

	pub fn semantic_tokens(&self) -> SemanticTokens {
		SemanticTokens::new(match &self.ast {
			Ast::Vhdl(ast) => self.vhdl_semantic_tokens(ast),
			Ast::Verilog(ast) => self.verilog_semantic_tokens(ast),
		})
	}

	pub fn apply_changes(&mut self, changes: Vec<TextDocumentContentChangeEvent>) {
		for change in changes {
			match change.range() {
				Some(range) => unimplemented!("Partial update attempted for range {range:?}"),
				None => self.content = AtomicByteTendril::from_slice(change.text().as_bytes()),
			}
		}
	}
}
