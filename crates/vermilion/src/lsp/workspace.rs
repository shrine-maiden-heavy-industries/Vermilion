/* SPDX-License-Identifier: BSD-3-Clause */

use std::collections::HashMap;

use vermilion_lang::AtomicByteTendril;
use vermilion_lsp::types::{Uri, semantic_tokens::SemanticTokens};

use crate::lang::{Ast, Language};

pub struct Workspace {
	documents: HashMap<Uri, Document>,
}

pub struct Document {
	contents: AtomicByteTendril,
	lang: Language,
	ast: Ast,
}

impl Workspace {
	pub fn new() -> Self {
		Self {
			documents: HashMap::new(),
		}
	}

	pub fn find_document(&self, uri: &Uri) -> Option<&Document> {
		self.documents.get(uri)
	}
}

impl Document {
	pub fn semantic_tokens(&self) -> SemanticTokens {
		let mut tokens = Vec::new();

		SemanticTokens::new(tokens)
	}
}
