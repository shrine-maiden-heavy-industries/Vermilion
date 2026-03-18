// SPDX-License-Identifier: BSD-3-Clause

use pulldown_cmark::{CowStr, Event, Parser, Tag, TagEnd};

use crate::{Diagnostic, StrDiagnostic, StringDiagnostic};

pub trait DiagnosticFancyRender<'a> {
	fn render_fancy(&'a self) -> ratatui::text::Text<'a>;
}

struct FancyRenderer<'a> {
	parser: Parser<'a>,
	text:   ratatui::text::Text<'a>,
}

impl<'a> DiagnosticFancyRender<'a> for StrDiagnostic<'a> {
	fn render_fancy(&'a self) -> ratatui::text::Text<'a> {
		let renderer = FancyRenderer::new(self.message());
		renderer.render()
	}
}

impl<'a> DiagnosticFancyRender<'a> for StringDiagnostic {
	fn render_fancy(&'a self) -> ratatui::text::Text<'a> {
		let renderer = FancyRenderer::new(self.message());
		renderer.render()
	}
}

impl<'a> FancyRenderer<'a> {
	fn new(text: &'a str) -> Self {
		Self {
			parser: Parser::new(text),
			text:   ratatui::text::Text::default(),
		}
	}

	pub fn render(mut self) -> ratatui::text::Text<'a> {
		while let Some(event) = self.parser.next() {
			match event {
				Event::Start(tag) => self.on_start_tag(tag),
				Event::End(tag) => self.on_end_tag(tag),
				Event::Text(text) => self.on_text(text),
				Event::Code(code) => self.on_code(code),
				Event::InlineMath(math) => self.on_inline_math(math),
				Event::DisplayMath(math) => self.on_display_math(math),
				Event::Html(html) => self.on_html(html),
				Event::InlineHtml(html) => self.on_inline_html(html),
				Event::FootnoteReference(footnote) => self.on_footnote_ref(footnote),
				Event::SoftBreak => self.on_soft_break(),
				Event::HardBreak => self.on_hard_break(),
				Event::Rule => self.on_rule(),
				Event::TaskListMarker(checked) => self.on_task_list_marker(checked),
			}
		}

		self.text
	}

	fn on_start_tag(&mut self, _tag: Tag) {}

	fn on_end_tag(&mut self, _tag: TagEnd) {}

	fn on_text(&mut self, _text: CowStr) {}

	fn on_code(&mut self, _code: CowStr) {}

	fn on_inline_math(&mut self, _math: CowStr) {}

	fn on_display_math(&mut self, _math: CowStr) {}

	fn on_html(&mut self, _math: CowStr) {}

	fn on_inline_html(&mut self, _math: CowStr) {}

	fn on_footnote_ref(&mut self, _math: CowStr) {}

	fn on_soft_break(&mut self) {}

	fn on_hard_break(&mut self) {}

	fn on_rule(&mut self) {}

	fn on_task_list_marker(&mut self, _checked: bool) {}
}
