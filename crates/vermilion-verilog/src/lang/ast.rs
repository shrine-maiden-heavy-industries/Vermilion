/* SPDX-License-Identifier: BSD-3-Clause */

use std::collections::HashMap;

use vermilion_lang::{Span, Spanned};

#[derive(Debug, PartialEq)]
pub enum NetType {
	Wire,
	Tri,
	Tri0,
	Tri1,
	Supply0,
	Supply1,
	Wand,
	Wor,
	TriAnd,
	TriOr,
}

#[derive(Debug, PartialEq)]
pub enum StrengthLow {
	Supply,
	Strong,
	Pull,
	Weak,
}

#[derive(Debug, PartialEq)]
pub enum StrengthHigh {
	Supply,
	Strong,
	Pull,
	Weak,
}

#[derive(Debug, PartialEq)]
pub enum ChargeStrength {
	Small,
	Medium,
	Large,
}

#[derive(Debug, PartialEq)]
pub enum DriveStrength {
	HighLow(StrengthHigh, StrengthLow),
	LowHigh(StrengthLow, StrengthHigh),
	LowHighZ(StrengthLow),
	HighLowZ(StrengthHigh),
	HighZLow(StrengthLow),
	LowZHigh(StrengthHigh),
}

#[derive(Debug, PartialEq)]
pub enum PullupStrength {
	HighLow(StrengthHigh, StrengthLow),
	LowHigh(StrengthLow, StrengthHigh),
	Value(StrengthHigh),
}

#[derive(Debug, PartialEq)]
pub enum PulldownStrength {
	HighLow(StrengthHigh, StrengthLow),
	LowHigh(StrengthLow, StrengthHigh),
	Value(StrengthLow),
}

#[derive(Debug, PartialEq)]
pub enum Edge {
	Positive,
	Negative,
	Any,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
	Pos,
	Neg,
	LogicalNot,
	BitwiseNot,
	ReductionAnd,
	ReductionOr,
	ReductionNand,
	RedcutionNor,
	ReductionXor,
	ReductionXnor,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
	Plus,
	Minus,
	Mul,
	Div,
	Mod,
	Eq,
	Neq,
	Eeq,
	Neeq,
	LogicalAnd,
	LogicalOr,
	Gt,
	Lt,
	GtEq,
	LtEq,
	BitwiseAnd,
	BitwiseOr,
	BitwiseXor,
	BitEq,
	Shl,
	Shr,
}

#[derive(Debug)]
pub enum Delay {
	Value,
	MinMax,
	MinTypMax,
}

#[derive(Debug)]
pub struct CompilerDirective<'src> {
	typ: &'src str,
	value: Option<&'src str>,
}

#[derive(Debug)]
pub struct Comment<'src>(&'src str);

#[derive(Debug, PartialEq)]
pub enum Number<'src> {
	Binary(Option<usize>, &'src str),
	Octal(Option<usize>, &'src str),
	Decimal(Option<usize>, usize),
	Hex(Option<usize>, &'src str),
}

#[derive(Debug)]
pub struct Range<'src>(
	Box<Spanned<ConstExpr<'src>, ()>>,
	Box<Spanned<ConstExpr<'src>, ()>>,
);

#[derive(Debug)]
pub enum Value<'src> {
	Number(Spanned<Number<'src>, ()>),
	Ident(&'src str),
	Str(&'src str),
	Index(&'src str, Box<Spanned<Expr<'src>, ()>>),
	Range(Spanned<Range<'src>, ()>),
}

#[derive(Debug)]
pub enum Expr<'src> {
	Value(Spanned<Value<'src>, ()>),
	Unary(UnaryOp, Spanned<Value<'src>, ()>),
	Binary(Box<Spanned<Self, ()>>, BinaryOp, Box<Spanned<Self, ()>>),
	Ternary(
		Box<Spanned<Self, ()>>,
		Box<Spanned<Self, ()>>,
		Box<Spanned<Self, ()>>,
	),
	Cat(Vec<Spanned<Self, ()>>),
	MultiCat(Box<Self>, Vec<Spanned<Self, ()>>),
	MinTypMax(
		Box<Spanned<Expr<'src>, ()>>,
		Box<Spanned<Expr<'src>, ()>>,
		Box<Spanned<Expr<'src>, ()>>,
	),
}

#[derive(Debug)]
pub enum ConstValue<'src> {
	Number(Spanned<Number<'src>, ()>),
	Ident(&'src str),
	Str(&'src str),
}

#[derive(Debug)]
pub enum ConstExpr<'src> {
	Value(Spanned<ConstValue<'src>, ()>),
	Unary(UnaryOp, Spanned<ConstValue<'src>, ()>),
	Binary(Box<Spanned<Self, ()>>, BinaryOp, Box<Spanned<Self, ()>>),
	Ternary(
		Box<Spanned<Self, ()>>,
		Box<Spanned<Self, ()>>,
		Box<Spanned<Self, ()>>,
	),
	Cat(Vec<Spanned<Self, ()>>),
	MultiCat(Box<Self>, Vec<Spanned<Self, ()>>),
	MinTypMax(
		Box<Spanned<ConstExpr<'src>, ()>>,
		Box<Spanned<ConstExpr<'src>, ()>>,
		Box<Spanned<ConstExpr<'src>, ()>>,
	),
}

#[derive(Debug)]
pub struct Primitive<'src> {
	name: &'src str,
	directives: Vec<Spanned<CompilerDirective<'src>, ()>>,
	comments: Vec<Spanned<Comment<'src>, ()>>,
}

#[derive(Debug)]
pub enum Port<'src> {
	Ident(&'src str),
}

#[derive(Debug, PartialEq)]
pub enum ModuleType {
	Module,
	MacroModule,
}

#[derive(Debug)]
pub struct Module<'src> {
	name: &'src str,
	typ: ModuleType,
	ports: Vec<Spanned<Port<'src>, ()>>,
	directives: Vec<Spanned<CompilerDirective<'src>, ()>>,
	comments: Vec<Spanned<Comment<'src>, ()>>,
}

#[derive(Debug)]
pub enum RootExpr<'src> {
	Module(Spanned<Module<'src>, ()>),
	Primitive(Spanned<Primitive<'src>, ()>),
	Directive(Spanned<CompilerDirective<'src>, ()>),
	Comment(Spanned<Comment<'src>, ()>),
}

#[derive(Debug, Default)]
pub struct Ast<'src> {
	modules: HashMap<&'src str, Spanned<Module<'src>, ()>>,
	primitives: HashMap<&'src str, Spanned<Primitive<'src>, ()>>,
	directives: Vec<Spanned<CompilerDirective<'src>, ()>>,
	comments: Vec<Spanned<Comment<'src>, ()>>,
}

impl<'src> Primitive<'src> {
	pub fn new(name: &'src str) -> Self {
		Self { name, directives: Vec::new(), comments: Vec::new() }
	}

	pub fn name(&self) -> &'src str {
		self.name
	}

	pub fn directives(&mut self) -> &mut Vec<Spanned<CompilerDirective<'src>, ()>> {
		&mut self.directives
	}

	pub const fn has_directives(&self) -> bool {
		!self.directives.is_empty()
	}

	pub fn comments(&mut self) -> &mut Vec<Spanned<Comment<'src>, ()>> {
		&mut self.comments
	}

	pub const fn has_comments(&self) -> bool {
		!self.comments.is_empty()
	}
}

impl<'src> Module<'src> {
	pub fn new(name: &'src str, typ: ModuleType) -> Self {
		Self {
			name,
			typ,
			ports: Vec::new(),
			directives: Vec::new(),
			comments: Vec::new(),
		}
	}

	pub fn with_span(self, span: Span<()>) -> (Self, Span<()>) {
		(self, span)
	}

	pub fn name(&self) -> &'src str {
		self.name
	}

	pub fn typ(&self) -> &ModuleType {
		&self.typ
	}

	pub fn ports(&mut self) -> &Vec<Spanned<Port<'src>, ()>> {
		&self.ports
	}

	pub const fn has_ports(&self) -> bool {
		!self.ports.is_empty()
	}

	pub fn directives(&mut self) -> &mut Vec<Spanned<CompilerDirective<'src>, ()>> {
		&mut self.directives
	}

	pub const fn has_directives(&self) -> bool {
		!self.directives.is_empty()
	}

	pub fn comments(&mut self) -> &mut Vec<Spanned<Comment<'src>, ()>> {
		&mut self.comments
	}

	pub const fn has_comments(&self) -> bool {
		!self.comments.is_empty()
	}
}

impl<'src> CompilerDirective<'src> {
	pub fn new(typ: &'src str, value: Option<&'src str>) -> Self {
		Self { typ, value }
	}

	pub fn with_span<C>(self, span: Span<C>) -> (Self, Span<C>) {
		(self, span)
	}

	pub fn typ(&self) -> &str {
		self.typ
	}

	pub fn value(&self) -> Option<&str> {
		self.value
	}

	pub fn has_value(&self) -> bool {
		self.value.is_some()
	}
}

impl<'src> Comment<'src> {
	pub fn new(text: &'src str) -> Self {
		Self(text)
	}

	pub fn with_span<C>(self, span: Span<C>) -> (Self, Span<C>) {
		(self, span)
	}

	pub fn value(&self) -> &str {
		self.0
	}
}

impl<'src> Ast<'src> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn modules(&mut self) -> &mut HashMap<&'src str, Spanned<Module<'src>, ()>> {
		&mut self.modules
	}

	pub fn has_modules(&self) -> bool {
		!self.modules.is_empty()
	}

	pub fn primitives(&mut self) -> &mut HashMap<&'src str, Spanned<Primitive<'src>, ()>> {
		&mut self.primitives
	}

	pub fn has_primitives(&self) -> bool {
		!self.primitives.is_empty()
	}

	pub fn directives(&mut self) -> &mut Vec<Spanned<CompilerDirective<'src>, ()>> {
		&mut self.directives
	}

	pub const fn has_directives(&self) -> bool {
		!self.directives.is_empty()
	}

	pub fn comments(&mut self) -> &mut Vec<Spanned<Comment<'src>, ()>> {
		&mut self.comments
	}

	pub const fn has_comments(&self) -> bool {
		!self.comments.is_empty()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_log::test;
}
