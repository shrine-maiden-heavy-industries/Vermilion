// SPDX-License-Identifier: BSD-3-Clause

use eyre::eyre;
use mdbook_preprocessor::{
	Preprocessor as MdBookPreprocessor, PreprocessorContext,
	book::{Book, Chapter},
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::render::RenderPulldown;

mod cli;
mod schema;

pub(crate) trait PreprocessorImpl {
	fn name(&self) -> &str;

	fn cli(&self) -> eyre::Result<clap::Command>;

	#[inline]
	fn cfg_schema(&self) -> Option<schemars::Schema> {
		None
	}

	#[inline]
	fn workspace_schema(&self) -> Option<schemars::Schema> {
		None
	}
}

pub(crate) struct AlembicPreprocessor;

impl AlembicPreprocessor {
	#[allow(clippy::new_ret_no_self, reason = "The `Preprocessor` is the proper interface")]
	pub fn new() -> Preprocessor {
		Preprocessor::new(Box::new(Self))
	}
}

impl PreprocessorImpl for AlembicPreprocessor {
	#[inline]
	fn name(&self) -> &str {
		"alembic"
	}

	#[inline]
	fn cli(&self) -> eyre::Result<clap::Command> {
		alembic::get_cli()
	}
}

pub(crate) struct CinnabarPreprocessor;

impl CinnabarPreprocessor {
	#[allow(clippy::new_ret_no_self, reason = "The `Preprocessor` is the proper interface")]
	pub fn new() -> Preprocessor {
		Preprocessor::new(Box::new(Self))
	}
}

impl PreprocessorImpl for CinnabarPreprocessor {
	#[inline]
	fn name(&self) -> &str {
		"cinnabar"
	}

	#[inline]
	fn cli(&self) -> eyre::Result<clap::Command> {
		cinnabar::get_cli()
	}
}

pub(crate) struct VermilionPreprocessor;

impl VermilionPreprocessor {
	#[allow(clippy::new_ret_no_self, reason = "The `Preprocessor` is the proper interface")]
	pub fn new() -> Preprocessor {
		Preprocessor::new(Box::new(Self))
	}
}

impl PreprocessorImpl for VermilionPreprocessor {
	#[inline]
	fn name(&self) -> &str {
		"vermilion"
	}

	#[inline]
	fn cli(&self) -> eyre::Result<clap::Command> {
		vermilion::get_cli()
	}

	#[inline]
	fn cfg_schema(&self) -> Option<schemars::Schema> {
		Some(vermilion::get_config_schema())
	}

	#[inline]
	fn workspace_schema(&self) -> Option<schemars::Schema> {
		Some(vermilion::get_workspace_schema())
	}
}

pub(crate) struct Preprocessor {
	name:      String,
	processor: Box<dyn PreprocessorImpl>,
}

impl Preprocessor {
	pub fn new(processor: Box<dyn PreprocessorImpl>) -> Self {
		Self {
			name: format!("{}-preprocessor", processor.name()),
			processor,
		}
	}

	fn preprocess_chapter(&self, chapter: &mut Chapter) {
		let mut buffer = String::with_capacity(chapter.content.len());
		let mut events = Vec::new();
		let mut consume_until_closing = false;

		for event in Parser::new(&chapter.content) {
			match event {
				Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
					if lang.starts_with(self.processor.name()) {
						consume_until_closing = true;
						let opts = lang.split(":").collect::<Vec<_>>();

						if opts.len() != 3 {
							eprintln!("Invalid preprocessor directive: {}", lang);
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
		let mut schema_events = Vec::new();
		match obj {
			"configuration" => {
				if let Some(schema) = self.processor.cfg_schema() {
					schema::SchemaTomlizer::new(schema)
						.render(HeadingLevel::H1, &mut schema_events)?;
					events.extend(schema_events);
				}

				Ok(())
			},
			"workspace" => {
				if let Some(schema) = self.processor.workspace_schema() {
					schema::SchemaTomlizer::new(schema)
						.render(HeadingLevel::H1, &mut schema_events)?;
					events.extend(schema_events);
				}
				Ok(())
			},
			obj => Err(eyre!("Unknown object type {}", obj)),
		}
	}

	fn generate_details(&self, obj: &str, events: &mut Vec<Event>) -> eyre::Result<()> {
		match obj {
			"cli" => {
				let cli = self.processor.cli()?;
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

impl MdBookPreprocessor for Preprocessor {
	fn name(&self) -> &str {
		self.name.as_str()
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
