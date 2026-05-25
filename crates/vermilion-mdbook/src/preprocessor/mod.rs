// SPDX-License-Identifier: BSD-3-Clause
// SPDX-License-Identifier: BSD-3-Clause

use eyre::eyre;
use mdbook_preprocessor::{
	Preprocessor, PreprocessorContext,
	book::{Book, Chapter},
};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

pub(crate) struct VermilionPreprocessor;

impl VermilionPreprocessor {
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
					if lang.starts_with("vermilion") {
						consume_until_closing = true;
						let opts = lang.split(":").collect::<Vec<_>>();

						if opts.len() != 3 {
							eprintln!("Invalid Vermilion preprocessor directive: {}", lang);
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
		match obj {
			"configuration" => {
				events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
					"json".into(),
				))));

				events.push(Event::Text(vermilion::dump_config_schema()?.into()));

				events.push(Event::End(TagEnd::CodeBlock));

				Ok(())
			},
			"workspace" => {
				events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
					"json".into(),
				))));

				events.push(Event::Text(vermilion::dump_workspace_schema()?.into()));

				events.push(Event::End(TagEnd::CodeBlock));

				Ok(())
			},
			obj => Err(eyre!("Unknown object type {}", obj)),
		}
	}

	fn generate_details(&self, obj: &str, events: &mut Vec<Event>) -> eyre::Result<()> {
		match obj {
			"cli" => self.generate_cli_details(events),
			"configuration" => Ok(()),
			"workspace" => Ok(()),
			obj => Err(eyre!("Unknown object type {}", obj)),
		}
	}

	fn generate_cli_details(&self, events: &mut Vec<Event>) -> eyre::Result<()> {
		let cli = vermilion::get_cli()?;

		// TODO(aki): Actually render this out

		Ok(())
	}
}

impl Preprocessor for VermilionPreprocessor {
	fn name(&self) -> &str {
		"vermilion-preprocessor"
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
