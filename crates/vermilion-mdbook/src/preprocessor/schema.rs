// SPDX-License-Identifier: BSD-3-Clause

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Tag, TagEnd};

use crate::render::RenderPulldown;

impl RenderPulldown for schemars::Schema {
	fn render(&self, level: HeadingLevel, events: &mut Vec<Event>) -> eyre::Result<()> {
		events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
			"json".into(),
		))));

		events.push(Event::Text(serde_json::to_string_pretty(self)?.into()));

		events.push(Event::End(TagEnd::CodeBlock));

		Ok(())
	}
}
