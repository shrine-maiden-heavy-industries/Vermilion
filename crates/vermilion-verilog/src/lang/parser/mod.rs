// SPDX-License-Identifier: BSD-3-Clause

use eyre::OptionExt;
use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::{
	VerilogVariant,
	lang::{
		ast::{Ast, Diagnostic, Module, Port, Primitive},
		tokenizer::{
			VerilogTokenizer,
			token::{Control, Keyword, Token},
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

	fn parse_port_expression(&mut self) {
		todo!()
	}

	fn parse_port(&mut self) {
		todo!()
	}

	fn parse_ports(&mut self) -> eyre::Result<Vec<Port>> {
		// This is called having just matched "(", so skip that token and start
		// consuming port definitions until we find a closing ")"
		let mut ports = Vec::new();
		let mut diagnostics = Vec::new();
		self.current_token = self.tokenizer.next();

		while let Some(token) = &self.current_token {
			// See what kind of port expression or identifier comes next
			match token.inner() {
				Token::Identifier(_) => self.parse_port_expression(),
				Token::Control(Control::Dot) => self.parse_port(),
				_ => {
					diagnostics.push(Diagnostic::new(
						token.span().copied(),
						format!("Expected port identifier or '.', got {token}"),
					));
				},
			}
			// Now we're done parsing  that, see if the next token is a comma (keep going),
			// or a closing paren (stop and return) - if it's neither that's a diagnostic
			if let Some(token) = &self.current_token {
				match token.inner() {
					Token::Control(Control::ParenClose) => break,
					Token::Control(Control::Comma) => self.current_token = self.tokenizer.next(),
					_ => {
						diagnostics.push(Diagnostic::new(
							token.span().copied(),
							format!("Expected ',' or ')', got {token}"),
						));
					},
				}
			}
		}
		// Consume the ")" token so we return ready for whatever comes next
		self.current_token = self.tokenizer.next();

		Ok(ports)
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

		if let Some(token) = &self.current_token {
			match token.inner() {
				Token::Control(Control::ParenOpen) => module.ports(self.parse_ports()?),
				Token::Control(Control::Semicolon) => {},
				_ => module.append_diagnostic(Diagnostic::new(
					token.span().copied(),
					format!("Expected ports defintion or ';', got {token}"),
				)),
			};
		}

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
