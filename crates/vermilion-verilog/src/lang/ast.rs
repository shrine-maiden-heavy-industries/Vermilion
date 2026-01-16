// SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;

use vermilion_lang::{AtomicByteTendril, Position, Spanned};

use crate::lang::tokenizer::token::{Comment, CompilerDirective};

#[derive(Debug)]
pub struct Module {
	pub name: AtomicByteTendril,
}

#[derive(Debug)]
pub struct Primitive {
	pub name: AtomicByteTendril,
}

#[derive(Debug, Default)]
pub struct Ast {
	pub modules:    HashMap<AtomicByteTendril, Spanned<Module, Position>>,
	pub primitives: HashMap<AtomicByteTendril, Spanned<Primitive, Position>>,
	pub directives: Vec<Spanned<CompilerDirective, Position>>,
	pub comments:   Vec<Spanned<Comment, Position>>,
}

#[cfg(test)]
mod tests {
	// use test_log::test;
	// use super::*;
}
