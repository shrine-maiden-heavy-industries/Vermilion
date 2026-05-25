// SPDX-License-Identifier: BSD-3-Clause

use pulldown_cmark::{BlockQuoteKind, Event, HeadingLevel, Tag, TagEnd};

pub(crate) trait RenderPulldown {
	fn render(&self, level: HeadingLevel, events: &mut Vec<Event>) -> eyre::Result<()>;

	fn next_level(&self, level: HeadingLevel) -> HeadingLevel {
		match level {
			HeadingLevel::H1 => HeadingLevel::H2,
			HeadingLevel::H2 => HeadingLevel::H3,
			HeadingLevel::H3 => HeadingLevel::H4,
			HeadingLevel::H4 => HeadingLevel::H5,
			HeadingLevel::H5 => HeadingLevel::H6,
			HeadingLevel::H6 => unreachable!(),
		}
	}

	fn escape_id(&self, id: String) -> String {
		id.replace("!@`#$%^&*()_".chars().collect::<Vec<_>>().as_slice(), "-")
	}

	fn emit_header_simple(
		&self,
		text: String,
		level: HeadingLevel,
		id: Option<String>,
		events: &mut Vec<Event>,
	) {
		events.push(Event::Start(Tag::Heading {
			level,
			id: id.map(|s| self.escape_id(s).into()),
			classes: Vec::new(),
			attrs: Vec::new(),
		}));
		events.push(Event::Text(text.into()));
		events.push(Event::End(TagEnd::Heading(level)));
	}

	fn emit_header_complex<'a>(
		&self,
		level: HeadingLevel,
		header_events: &[Event<'a>],
		id: Option<String>,
		events: &mut Vec<Event<'a>>,
	) {
		events.push(Event::Start(Tag::Heading {
			level,
			id: id.map(|s| self.escape_id(s).into()),
			classes: Vec::new(),
			attrs: Vec::new(),
		}));

		events.extend_from_slice(header_events);

		events.push(Event::End(TagEnd::Heading(level)));
	}

	fn emit_paragraph_simple(&self, text: String, events: &mut Vec<Event>) {
		events.push(Event::Start(Tag::Paragraph));
		events.push(Event::Text(text.into()));
		events.push(Event::End(TagEnd::Paragraph));
	}

	fn emit_paragraph_complex<'a>(
		&self,
		paragraph_events: &[Event<'a>],
		events: &mut Vec<Event<'a>>,
	) {
		events.push(Event::Start(Tag::Paragraph));
		events.extend_from_slice(paragraph_events);
		events.push(Event::End(TagEnd::Paragraph));
	}

	fn emit_list<'a>(
		&self,
		first_num: Option<u64>,
		list_events: &[Event<'a>],
		events: &mut Vec<Event<'a>>,
	) {
		events.push(Event::Start(Tag::List(first_num)));
		for evt in list_events.iter().cloned() {
			events.push(Event::Start(Tag::Item));
			events.push(evt);
			events.push(Event::End(TagEnd::Item));
		}
		events.push(Event::End(TagEnd::List(first_num.is_some())));
	}

	fn emit_admonition_simple(
		&self,
		text: String,
		kind: Option<BlockQuoteKind>,
		events: &mut Vec<Event>,
	) {
		events.push(Event::Start(Tag::BlockQuote(kind)));
		events.push(Event::Text(text.into()));
		events.push(Event::End(TagEnd::BlockQuote(kind)));
	}
}
