// SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;

use vermilion_lang::{AtomicByteTendril, Position, Span, Spanned};

use crate::lang::tokenizer::token::{Comment, CompilerDirective};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
	position: Position,
	message:  String,
}

impl Diagnostic {
	pub fn new<Str>(location: Option<Span<usize, Position>>, message: Str) -> Self
	where
		Str: Into<String>,
	{
		let position = if let Some(span) = location {
			*span.context()
		} else {
			Position::sof()
		};

		Self { position, message: message.into() }
	}
}

#[derive(Debug)]
pub enum AstContents<T> {
	Valid(T),
	Invalid(Diagnostic),
}

#[derive(Debug)]
pub struct AstNode<T> {
	source_position: Option<Span<usize, Position>>,
	contents:        AstContents<T>,
}

#[derive(Debug)]
struct ModuleInner {
	name:        AtomicByteTendril,
	diagnostics: Vec<Diagnostic>,
	ports:       Option<PortList>,
}

#[derive(Debug)]
pub struct Module {
	inner: AstNode<ModuleInner>,
}

impl Module {
	pub fn new_valid(
		source_position: Option<Span<usize, Position>>,
		name: AtomicByteTendril,
	) -> Self {
		Self {
			inner: AstNode {
				source_position,
				contents: AstContents::Valid(ModuleInner {
					name,
					diagnostics: Vec::new(),
					ports: None,
				}),
			},
		}
	}

	pub fn new_invalid(
		source_position: Option<Span<usize, Position>>,
		diagnostic: Diagnostic,
	) -> Self {
		Self {
			inner: AstNode {
				source_position,
				contents: AstContents::Invalid(diagnostic),
			},
		}
	}

	pub fn append_diagnostic(&mut self, diagnostic: Diagnostic) {
		match &mut self.inner.contents {
			AstContents::Valid(module) => module.diagnostics.push(diagnostic),
			AstContents::Invalid(_) => {
				panic!("Attempted to append a diagnostic to an invalid module")
			},
		}
	}

	pub fn ports(&mut self, port_list: PortList) {
		match &mut self.inner.contents {
			AstContents::Valid(module) => module.ports = Some(port_list),
			AstContents::Invalid(_) => {
				panic!("Attempted to set the ports for an invalid module")
			},
		}
	}
}

#[derive(Debug)]
pub struct Port {}

#[derive(Debug, Default)]
pub struct PortList {
	ports:       Vec<Port>,
	diagnostics: Vec<Diagnostic>,
}

impl PortList {
	pub fn new() -> Self {
		Self { ports: Vec::new(), diagnostics: Vec::new() }
	}

	pub fn append_diagnostic(&mut self, diagnostic: Diagnostic) {
		self.diagnostics.push(diagnostic);
	}
}

#[derive(Debug)]
struct PrimitiveInner {
	name: AtomicByteTendril,
}

#[derive(Debug)]
pub struct Primitive {
	inner: AstNode<PrimitiveInner>,
}

#[derive(Debug, Default)]
pub struct Ast {
	modules:     HashMap<AtomicByteTendril, Module>,
	primitives:  HashMap<AtomicByteTendril, Primitive>,
	directives:  Vec<Spanned<CompilerDirective, Position>>,
	comments:    Vec<Spanned<Comment, Position>>,
	diagnostics: Vec<Diagnostic>,
}

impl Ast {
	pub fn append_module(&mut self, module: Module) {
		match &module.inner.contents {
			AstContents::Valid(inner) => {
				self.modules.insert(inner.name.clone(), module);
			},
			AstContents::Invalid(diagnostic) => self.append_diagnostic(diagnostic.clone()),
		}
	}

	pub fn append_primitive(&mut self, primitive: Primitive) {
		match &primitive.inner.contents {
			AstContents::Valid(inner) => {
				self.primitives.insert(inner.name.clone(), primitive);
			},
			AstContents::Invalid(diagnostic) => self.append_diagnostic(diagnostic.clone()),
		}
	}

	pub fn append_comment(&mut self, comment: Spanned<Comment, Position>) {
		self.comments.push(comment);
	}

	pub fn append_diagnostic(&mut self, diagnostic: Diagnostic) {
		self.diagnostics.push(diagnostic);
	}

	pub fn has_diagnostics(&self) -> bool {
		!self.diagnostics.is_empty()
	}

	pub fn diagnostics(&self) -> &Vec<Diagnostic> {
		&self.diagnostics
	}
}

#[cfg(test)]
mod tests {
	// use test_log::test;
	// use super::*;
}
