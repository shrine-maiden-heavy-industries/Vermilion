// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod parser;
pub mod position;
pub mod span;
pub mod tokenizer;

pub use position::Position;
pub use span::{Span, Spanned};
use tendril::{Atomic, Tendril, fmt};

pub type AtomicByteTendril = Tendril<fmt::Bytes, Atomic>;

#[macro_export]
macro_rules! spanned_token {
	($token:expr) => {
		vermilion_lang::span::Spanned::new($token, None)
	};
	($token:expr, $span_range:expr, $context:expr) => {
		vermilion_lang::span::Spanned::new(
			$token,
			Some(vermilion_lang::span::Span::new($span_range, $context)),
		)
	};
}

pub trait LanguageMetadata {
	fn file_extensions<'a, 'b: 'a>() -> &'a [&'b str];
}

// TODO(aki):
// Eventual support for dumping AST nodes to Graphivz, maybe something like:
//
// digraph <NAME> {
// 	rankdir = TB;
// 	bgcolor = "#262628";
// 	color = "#DFDFDF";
// 	fontname = "Fira Code";
// 	fontnames = "svg";
// 	center = true;
// 	node [shape=Mrecord];
// 	node [shape=Mrecord, fontname="Fira Code", fontcolor="#DFDFDF", color="#DFDFDF"];
// 	edge [color="#DFDFDF"];
//
// 	blk0 [label=< { { <font color="#946CFA">blk</font> | { <font color="#FF9A56">L:</font> N | <font
// color="#F5AAB9">C:</font> N } } | { { EXT1 | EXT2 | EXT3 } } } >] 	blk1 [label=< { { <font
// color="#946CFA">blk</font> | { <font color="#FF9A56">L:</font> N | <font
// color="#F5AAB9">C:</font> N } } | { { EXT1 | EXT2 | EXT3 } } } >]
// blk0 -> blk1
// }
//
// We will need to generate a unique ID for each node in order to generate the DAG at the end
// but each AST node should be able to turn itself into a DOT record.
//
// We can then walk the AST and render each node and emit the graph

fn _print_errors() {
	// 	let _: Vec<_> = args
	// 		.get_many::<String>("files")
	// 		.expect("files is required")
	// 		.map(move |filename| {
	// 			let src = fs::read_to_string(filename).expect("Unable to read file");
	// 			let parser = crate::lang::get_parser(lang_id);
	//
	// 			match parser.parse(src.as_str()) {
	// 				Ok(ast) => {},
	// 				Err(errs) => errs.into_iter().for_each(|e| {
	// 					Report::build(ReportKind::Error, (filename.clone(), e.span().into_range()))
	// 						.with_config(
	// 							ariadne::Config::new().with_index_type(ariadne::IndexType::Byte),
	// 						)
	// 						.with_message(e.to_string())
	// 						.with_label(
	// 							Label::new((filename.clone(), e.span().into_range()))
	// 								.with_message(e.reason().to_string())
	// 								.with_color(Color::Red),
	// 						)
	// 						.with_labels(e.contexts().map(|(label, span)| {
	// 							Label::new((filename.clone(), span.into_range()))
	// 								.with_message(format!("while parsing this {label}"))
	// 								.with_color(Color::Yellow)
	// 						}))
	// 						.finish()
	// 						.print(sources([(filename.clone(), src.as_str())]))
	// 						.unwrap()
	// 				}),
	// 			}
	// 		})
	// 		.collect();
}
