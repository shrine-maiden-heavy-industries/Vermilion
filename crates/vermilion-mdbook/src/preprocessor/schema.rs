// SPDX-License-Identifier: BSD-3-Clause

use eyre::{OptionExt, eyre};
use pulldown_cmark::{Event, HeadingLevel};
use serde_json::{Map, Value};

use crate::render::RenderPulldown;

pub(crate) struct SchemaTomlizer {
	schema: schemars::Schema,
}

impl SchemaTomlizer {
	pub fn new(schema: schemars::Schema) -> Self {
		Self { schema }
	}

	fn normalize_name(
		&self,
		name: &str,
		parent_name: Option<&str>,
		grandparent_name: Option<&str>,
	) -> String {
		let mut norm_name = String::with_capacity(
			name.len() +
				parent_name.map_or(0, |n| n.len() + 1) +
				grandparent_name.map_or(0, |n| n.len() + 1),
		);

		if let Some(gname) = grandparent_name {
			norm_name += gname;
			norm_name += ".";
		}

		if let Some(pname) = parent_name {
			norm_name += pname;
			norm_name += ".";
		}

		norm_name += name;

		norm_name
	}

	fn render_properties(
		&self,
		name: Option<&str>,
		parent_name: Option<&str>,
		properties: &Map<String, Value>,
		defs: Option<&Map<String, Value>>,
		level: HeadingLevel,
		events: &mut Vec<Event>,
	) -> eyre::Result<()> {
		for (prop_name, prop_value) in properties {
			let prop_obj = prop_value
				.as_object()
				.ok_or_eyre(eyre!("Property '{}' is not an object!", prop_name))?;
			let norm_name = self.normalize_name(prop_name, name, parent_name);
			let prop_desc = prop_obj.get("description").and_then(|v| v.as_str());
			let prop_type = prop_obj.get("type").and_then(|v| v.as_str());
			let prop_alters = prop_obj
				.get("anyOf")
				.and_then(|v| v.as_array())
				.map(|v| v.iter().flat_map(|p| p.as_object()).collect::<Vec<_>>());

			eprintln!("Property: '{}'", norm_name);

			if !matches!(prop_type, Some("string")) {
				self.emit_header_complex(
					level,
					&[Event::Code(format!("[{}]", norm_name).into())],
					Some(norm_name.clone()),
					events,
				);
			}

			if let Some(desc) = prop_desc {
				for para in desc.split("\n\n") {
					self.emit_paragraph_simple(para.into(), events);
				}
			}

			if let Some(ref_obj) = if let Some(alters) = prop_alters {
				if let Some(ref_name) = alters
					.iter()
					.find(|p| p.contains_key("$ref"))
					.and_then(|k| k.get("$ref"))
					.and_then(|v| v.as_str())
					.and_then(|v| v.split('/').next_back())
				{
					defs.and_then(|d| d.get(ref_name))
						.and_then(|v| v.as_object())
				} else {
					None
				}
			} else {
				None
			} {
				if prop_desc.is_none() &&
					let Some(desc) = ref_obj.get("description").and_then(|v| v.as_str())
				{
					for para in desc.split("\n\n") {
						self.emit_paragraph_simple(para.into(), events);
					}
				}

				if let Some(obj_props) = ref_obj.get("properties").and_then(|v| v.as_object()) {
					self.render_properties(
						Some(norm_name.as_str()),
						name,
						obj_props,
						defs,
						self.next_level(level),
						events,
					)?;
				}
			} else {
				match prop_type {
					Some(typ) => {
						eprintln!("Unknown property type '{}' in '{}'", typ, prop_name);
					},
					None => {
						eprintln!("????");
					},
				}
			}
		}

		Ok(())
	}
}

impl RenderPulldown for SchemaTomlizer {
	fn render(&self, level: HeadingLevel, events: &mut Vec<Event>) -> eyre::Result<()> {
		let schema_obj = self
			.schema
			.as_object()
			.ok_or_eyre("Schema malformed, root element is not an object")?;
		let schema_defs = schema_obj.get("$defs").and_then(|v| v.as_object());

		let schema_title = schema_obj
			.get("title")
			.and_then(|v| v.as_str())
			.unwrap_or("<UNNAMED>");

		if let Some(desc) = schema_obj.get("description").and_then(|v| v.as_str()) {
			for para in desc.split("\n\n") {
				self.emit_paragraph_simple(para.into(), events);
			}
		}

		self.render_properties(
			None,
			None,
			schema_obj
				.get("properties")
				.and_then(|v| v.as_object())
				.ok_or_eyre(eyre!(
					"Schema '{}' is malformed, no 'properties' object in root",
					schema_title
				))?,
			schema_defs,
			level,
			events,
		)?;

		Ok(())
	}
}
