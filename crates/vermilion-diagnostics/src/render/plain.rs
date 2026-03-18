// SPDX-License-Identifier: BSD-3-Clause

use pulldown_cmark::{CowStr, Event, Parser, Tag, TagEnd};

use crate::{Diagnostic, StrDiagnostic, StringDiagnostic};

pub trait DiagnosticRenderPlain {
	fn render_plain(&self) -> String;
}

struct PlainRenderer<'a> {
	parser: Parser<'a>,
	text:   String,
}

impl<'a> DiagnosticRenderPlain for StrDiagnostic<'a> {
	fn render_plain(&self) -> String {
		let renderer = PlainRenderer::new(self.message());
		renderer.render()
	}
}

impl DiagnosticRenderPlain for StringDiagnostic {
	fn render_plain(&self) -> String {
		let renderer = PlainRenderer::new(self.message());
		renderer.render()
	}
}

impl<'a> PlainRenderer<'a> {
	pub fn new(text: &'a str) -> Self {
		Self { parser: Parser::new(text), text: String::new() }
	}

	pub fn render(mut self) -> String {
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
