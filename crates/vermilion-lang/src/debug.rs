// SPDX-License-Identifier: BSD-3-Clause

use crate::position::Position;

// TODO(aki):
// Eventual support for dumping AST nodes to Graphivz, maybe something like:
//
// digraph <NAME> {
//     rankdir = TB;
//     bgcolor = "#262628";
//     color = "#DFDFDF";
//     fontname = "Fira Code";
//     fontnames = "svg";
//     center = true;
//     node [shape=Mrecord];
//     node [shape=Mrecord, fontname="Fira Code", fontcolor="#DFDFDF", color="#DFDFDF"];
//     edge [color="#DFDFDF"];
//
// }
//
// We will need to generate a unique ID for each node in order to generate the DAG at the end
// but each AST node should be able to turn itself into a DOT record.
//
// We can then walk the AST and render each node and emit the graph
pub trait GraphvizNode {
	fn node_id(&self) -> &str;
	fn node_type(&self) -> &str;
	fn node_position(&self) -> &Position;
	fn node_extra(&self) -> Option<String>;

	fn render_node(&self) -> String {
		let pos = self.node_position();

		format!(
			"{} [label=<{{{{<font color=\"#946CFA\">{}</font>|{{<font color=\"#FF9A56\">L:</font> \
			 {}|<font color=\"#F5AAB9\">C:</font> {}}}|{}}}>]",
			self.node_id(),
			self.node_type(),
			pos.line(),
			pos.character(),
			self.node_extra().unwrap_or("None".to_string())
		)
	}
}

#[cfg(test)]
mod tests {
	#[allow(unused)]
	use super::*;
}
