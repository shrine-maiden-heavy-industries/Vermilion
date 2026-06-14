// SPDX-License-Identifier: BSD-3-Clause

use clap::{Arg, ArgMatches, Command};
use eyre::eyre;
use mdbook_preprocessor::Preprocessor;

use crate::preprocessor::{CinnabarPreprocessor, VermilionPreprocessor};

mod preprocessor;
pub(crate) mod render;

fn init_cli() -> Command {
	Command::new(env!("CARGO_PKG_NAME"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.version(env!("CARGO_PKG_VERSION"))
		.subcommand(Command::new("supports").arg(Arg::new("renderer").value_name("RENDERER")))
}

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let args = init_cli().clone().get_matches();

	match args.subcommand() {
		Some((sub_cmd, args)) => match sub_cmd {
			"supports" => supports(args),
			_ => {
				eprintln!("Unknown subcommand {}", sub_cmd);
				std::process::exit(1);
			},
		},
		_ => process(&args),
	}?;

	std::process::exit(0);
}

// We support every backend, as we're just emitting extra markdown
fn supports(_args: &ArgMatches) -> eyre::Result<()> {
	Ok(())
}

fn process(_args: &ArgMatches) -> eyre::Result<()> {
	let (ctx, book) =
		mdbook_preprocessor::parse_input(std::io::stdin()).map_err(|err| eyre!(Box::new(err)))?;

	let cinnabar_processor = CinnabarPreprocessor::new();
	let vermilion_processor = VermilionPreprocessor::new();

	let processed_book = cinnabar_processor
		.run(&ctx, book)
		.map_err(|err| eyre!(Box::new(err)))?;

	let processed_book = vermilion_processor
		.run(&ctx, processed_book)
		.map_err(|err| eyre!(Box::new(err)))?;

	serde_json::to_writer(std::io::stdout(), &processed_book).map_err(|err| eyre!(Box::new(err)))
}
