// SPDX-License-Identifier: BSD-3-Clause

use eyre::eyre;
use mdbook_preprocessor::{
	Preprocessor, PreprocessorContext,
	book::{Book, Chapter},
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::render::RenderPulldown;

pub(crate) struct CinnabarPreprocessor;

impl CinnabarPreprocessor {
	pub const fn new() -> Self {
		Self
	}

	fn preprocess_chapter(&self, chapter: &mut Chapter) {
		let mut buffer = String::with_capacity(chapter.content.len());
		let mut events = Vec::new();
		let mut consume_until_closing = false;

		for event in Parser::new(&chapter.content) {
			match event {
				Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
					if lang.starts_with("cinnabar") {
						consume_until_closing = true;
						let opts = lang.split(":").collect::<Vec<_>>();

						if opts.len() != 3 {
							eprintln!("Invalid Cinnabar preprocessor directive: {}", lang);
						} else {
							if let Err(err) = match (opts[1], opts[2]) {
								("schema", obj) => self.generate_schema(obj, &mut events),
								("details", obj) => self.generate_details(obj, &mut events),
								(operation, typ) => {
									Err(eyre!("Unknown operation/type pair: {}:{}", operation, typ))
								},
							} {
								eprintln!("Unable to preprocess: {}", err)
							}
						}
					} else {
						events.push(event);
					}
				},
				Event::End(TagEnd::CodeBlock) => {
					if consume_until_closing {
						consume_until_closing = false
					}
				},
				event => {
					if !consume_until_closing {
						events.push(event)
					}
				},
			}
		}

		if let Ok(res) =
			pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut buffer).map(|_| buffer)
		{
			chapter.content = res;
		} else {
			eprintln!("Failed to preprocess chapter!");
		}
	}

	fn generate_schema(&self, obj: &str, events: &mut Vec<Event>) -> eyre::Result<()> {
		// let mut schema_events = Vec::new();
		match obj {
			// TODO(aki): Cinnabar doesn't have config or workspace schema settings for now
			"configuration" => {
				// super::schema::SchemaTomlizer::new(cinnabar::get_config_schema())
				// 	.render(HeadingLevel::H1, &mut schema_events)?;
				// events.extend(schema_events);
				Ok(())
			},
			"workspace" => {
				// 	super::schema::SchemaTomlizer::new(cinnabar::get_workspace_schema())
				// 		.render(HeadingLevel::H1, &mut schema_events)?;
				// 	events.extend(schema_events);
				Ok(())
			},
			obj => Err(eyre!("Unknown object type {}", obj)),
		}
	}

	fn generate_details(&self, obj: &str, events: &mut Vec<Event>) -> eyre::Result<()> {
		match obj {
			"cli" => {
				let cli = cinnabar::get_cli()?;
				let mut cli_events = Vec::new();

				cli.render(HeadingLevel::H1, &mut cli_events)?;
				events.extend(cli_events);

				Ok(())
			},
			"configuration" => Ok(()),
			"workspace" => Ok(()),
			obj => Err(eyre!("Unknown object type {}", obj)),
		}
	}
}

impl Preprocessor for CinnabarPreprocessor {
	fn name(&self) -> &str {
		"cinnabar-preprocessor"
	}

	fn run(
		&self,
		_ctx: &PreprocessorContext,
		mut book: Book,
	) -> mdbook_preprocessor::errors::Result<Book> {
		book.for_each_chapter_mut(|chapter| self.preprocess_chapter(chapter));
		Ok(book)
	}
}
