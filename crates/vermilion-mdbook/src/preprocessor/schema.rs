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

	fn render_property(
		&self,
		name: &str,
		parent_name: Option<&str>,
		property: &Map<String, Value>,
		defs: Option<&Map<String, Value>>,
		level: HeadingLevel,
		events: &mut Vec<Event>,
	) -> eyre::Result<()> {
		let mut optional = false;
		match property.get("type").and_then(|v| v.as_str()) {
			Some("string") => {
				self.emit_paragraph_simple(name.into(), events);
			},
			Some("object") => {
				let mut obj_name = String::with_capacity(
					name.len() + parent_name.map_or_else(|| 0, |n| n.len() + 1),
				);
				obj_name += name;
				if let Some(parent_name) = parent_name {
					obj_name += ".";
					obj_name += parent_name;
				}

				self.emit_header_complex(
					level,
					&[Event::Code(format!("[{}]", obj_name).into())],
					Some(name.into()),
					events,
				);

				if let Some(desc) = property.get("description").and_then(|v| v.as_str()) {
					for para in desc.split("\n\n") {
						self.emit_paragraph_simple(para.into(), events);
					}
				}

				if let Some(obj) = property.get("properties").and_then(|v| v.as_object()) {
					for (prop_name, prop_value) in obj {
						self.render_property(
							prop_name,
							Some(obj_name.as_str()),
							prop_value.as_object().ok_or_eyre("!!!")?,
							defs,
							self.next_level(level),
							events,
						)?;
					}
				}
			},
			Some("null") => {
				return Ok(());
			},
			Some(prop_type) => {
				eprintln!("Unhandled property type: '{}'", prop_type)
			},
			None => {},
		}

		if let Some(any_of) = property.get("anyOf").and_then(|v| v.as_array()) {
			for alteration in any_of.iter().flat_map(|v| v.as_object()) {
				if let Some(ref_name) = alteration.get("$ref").and_then(|v| v.as_str()) {
					let def_key = ref_name.split('/').next_back().ok_or_eyre(eyre!(
						"Property '{}' has invalid '$ref' key: '{}'",
						name,
						ref_name
					))?;

					let def_obj = defs
						.ok_or_eyre(eyre!("No '$defs' object passed!"))?
						.get(def_key)
						.and_then(|v| v.as_object())
						.ok_or_eyre(eyre!(
							"Schema '$defs' does not contain an entry for '$ref' key: '{}'",
							def_key
						))?;

					self.render_property(name, None, def_obj, defs, level, events)?;
				} else if let Some(obj_type) = alteration.get("type").and_then(|v| v.as_str()) {
					if obj_type == "null" {
						optional = true;
					} else {
						self.render_property(name, None, alteration, defs, level, events)?
					}
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

		for (name, value) in schema_obj
			.get("properties")
			.and_then(|v| v.as_object())
			.ok_or_eyre(eyre!(
				"Schema '{}' is malformed, no 'properties' object in root",
				schema_title
			))? {
			self.render_property(
				name,
				None,
				value.as_object().ok_or_eyre(eyre!(
					"Property '{}' of schema '{}' is invalid, not an object?",
					name,
					schema_title
				))?,
				schema_defs,
				self.next_level(level),
				events,
			)?;
		}

		Ok(())
	}
}
