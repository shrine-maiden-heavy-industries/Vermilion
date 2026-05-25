// SPDX-License-Identifier: BSD-3-Clause

use std::cmp;

use pulldown_cmark::{BlockQuoteKind, Event, HeadingLevel};

use crate::render::RenderPulldown;

impl RenderPulldown for clap::Arg {
	fn render(&self, level: HeadingLevel, events: &mut Vec<Event>) -> eyre::Result<()> {
		let mut had_short = false;
		let mut header_events = Vec::new();
		let mut value_events = Vec::new();

		if let Some(short) = self.get_short() {
			header_events.push(Event::Code(format!("-{}", short).into()));
			had_short = true;
		}
		if let Some(long) = self.get_long() {
			if had_short {
				header_events.push(Event::Text("/".into()));
			}

			header_events.push(Event::Code(format!("--{}", long).into()));
		}

		if let Some(value_names) = self.get_value_names() {
			for value_name in value_names {
				header_events.push(Event::Text(" ".into()));
				header_events.push(Event::Code(
					if self.is_required_set() {
						format!("<{}>", value_name)
					} else {
						format!("[{}]", value_name)
					}
					.into(),
				));
			}
		}

		self.emit_header_complex(
			level,
			&header_events,
			Some(format!("arg-{}", self.get_id())),
			events,
		);

		if let Some(help_text) = self.get_long_help().or(self.get_help()) {
			for help_paragraph in help_text.to_string().split("\n\n").map(|p| p.to_string()) {
				if help_paragraph.starts_with("NOTE:") {
					self.emit_admonition_simple(
						help_paragraph.clone().split_off(5),
						Some(BlockQuoteKind::Note),
						events,
					);
				} else {
					self.emit_paragraph_simple(help_paragraph, events);
				}
			}
		} else {
			self.emit_paragraph_simple("No Help".into(), events);
		}

		let possible_values = self.get_possible_values();

		if !possible_values.is_empty() {
			value_events.push(Event::Text("Possible values are:".into()));
			value_events.push(Event::HardBreak);

			self.emit_list(
				None,
				&possible_values
					.iter()
					.map(|v| v.get_name().to_string())
					.map(|s| Event::Code(s.into()))
					.collect::<Vec<_>>(),
				&mut value_events,
			);

			self.emit_paragraph_complex(&value_events, events);
		}

		Ok(())
	}
}

impl RenderPulldown for clap::Command {
	fn render(&self, level: HeadingLevel, events: &mut Vec<Event>) -> eyre::Result<()> {
		let name = self.get_name();
		let mut args = self.get_arguments().collect::<Vec<_>>();
		let mut subcommands = self.get_subcommands().collect::<Vec<_>>();

		// Sort args by ID and have all positional args sink to the bottom
		args.sort_by(|a, b| {
			if a.get_short().is_some() || a.get_long().is_some() {
				a.get_id().cmp(b.get_id())
			} else {
				cmp::Ordering::Greater
			}
		});
		subcommands.sort_by(|a, b| a.get_name().cmp(b.get_name()));

		self.emit_header_simple(name.into(), level, Some(name.into()), events);

		if let Some(about) = self.get_about() {
			self.emit_paragraph_simple(about.to_string(), events);
		}

		for arg in args {
			arg.render(self.next_level(level), events)?;
		}

		for subcommand in subcommands {
			if !subcommand.is_hide_set() {
				subcommand.render(self.next_level(level), events)?;
			}
		}

		Ok(())
	}
}
