// SPDX-License-Identifier: BSD-3-Clause

use eyre::OptionExt;
use vermilion_lang::{AtomicByteTendril, Position, Span, Spanned};

use crate::{
	VerilogVariant,
	lang::{
		ast::{Ast, Diagnostic, Module, Primitive},
		tokenizer::{
			VerilogTokenizer,
			token::{self, Keyword, Token},
		},
	},
};

pub mod error;

pub struct VerilogParser {
	std:           VerilogVariant,
	tokenizer:     VerilogTokenizer,
	current_token: Option<Spanned<Token, Position>>,
}

impl VerilogParser {
	pub fn new(std: VerilogVariant, content: AtomicByteTendril) -> Self {
		Self {
			std,
			tokenizer: VerilogTokenizer::new(std, content),
			current_token: None,
		}
	}

	fn parse_module_decl(&mut self) -> eyre::Result<Module> {
		// This is called having just matched "module" or "macromodule", so skip that token
		// having grabbed the location information
		let location = self
			.current_token
			.as_ref()
			.ok_or_eyre("current token in impossible state")?
			.span()
			.copied();
		self.current_token = self.tokenizer.next();
		// See if the next token we got was the name of this module
		let name = if let Some(token) = &self.current_token {
			match token.inner() {
				Token::Identifier(ident) => ident.clone(),
				_ => {
					return Ok(Module::new_invalid(
						location,
						Diagnostic::new(
							token.span().copied(),
							format!("Expected module name, got {token}"),
						),
					));
				},
			}
		} else {
			return Ok(Module::new_invalid(
				location,
				Diagnostic::new(location, "Encountered end of file, expected module name"),
			));
		};
		let mut module = Module::new_valid(location, name);
		// Now we've got a valid module decl, let's see what ports it has

		Ok(module)
	}

	fn parse_primitive_decl(&mut self) -> eyre::Result<Primitive> {
		todo!()
	}

	pub fn parse(mut self) -> eyre::Result<Ast> {
		let mut ast = Ast::default();
		self.current_token = self.tokenizer.next();
		while let Some(token) = &self.current_token {
			match token.inner() {
				Token::Keyword(Keyword::Module) | Token::Keyword(Keyword::MacroModule) => {
					ast.append_module(self.parse_module_decl()?)
				},
				Token::Keyword(Keyword::Primitive) => {
					ast.append_primitive(self.parse_primitive_decl()?)
				},
				Token::Comment(comment) => {
					ast.append_comment(Spanned::new(comment.clone(), token.span().copied()))
				},
				_ => ast.append_diagnostic(Diagnostic::new(
					token.span().copied(),
					format!("Unexpected token {} at {:?}", token.inner(), token.span()),
				)),
			}
			self.current_token = self.tokenizer.next();
		}

		Ok(ast)
	}
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
