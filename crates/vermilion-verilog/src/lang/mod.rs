// SPDX-License-Identifier: BSD-3-Clause

pub mod ast;
pub mod directives;
pub mod keywords;
pub mod parser;
pub mod sysfuncs;
pub mod tokenizer;
pub mod types;

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
