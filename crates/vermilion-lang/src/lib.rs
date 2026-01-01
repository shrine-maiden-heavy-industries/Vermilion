/* SPDX-License-Identifier: BSD-3-Clause */
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::all))]
#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod lexer;
pub mod parser;
pub mod position;
pub mod span;

pub use position::Position;
pub use span::{Span, Spanned};

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
