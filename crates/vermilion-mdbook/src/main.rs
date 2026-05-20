// SPDX-License-Identifier: BSD-3-Clause

use clap::{Arg, ArgMatches, Command};
use mdbook_preprocessor::{Preprocessor, PreprocessorContext, book::Book};

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	Ok(())
}
