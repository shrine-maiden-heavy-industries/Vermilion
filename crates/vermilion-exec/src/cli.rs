// SPDX-License-Identifier: BSD-3-Clause

use clap::{Arg, ArgAction, ColorChoice, Command, builder::styling, value_parser};
use color_print::cformat;

pub(crate) fn init(
	name: &'static str,
	version: &'static str,
	description: &'static str,
	log_env_var: &'static str,
) -> eyre::Result<Command> {
	Ok(Command::new(name)
		.version(version)
		.about(description)
		.before_help(cformat!("<blue>{}</> v{}", name, version))
		.after_help(cformat!(
			"\
			<blue,bold>{0}</> is licensed under the <cyan,bold>{2}</> and is part of the \
			 <magenta>Vermilion</> project,\nwhich can be found at {1}\n\n<red,bold>For bug \
			 reporting instructions, please see</>: {1}/issues\n\n \
			 <bg:#D52D00>                </>  <bg:#5BCEFA>                </>\n \
			 <bg:#FF9A56>                </>  <bg:#FEA9B8>                </>\n \
			 <bg:#FFFFFF>                </>  <bg:#FFFFFF>                </>\n \
			 <bg:#D362A4>                </>  <bg:#FEA9B8>                </>\n \
			 <bg:#A30262>                </>  <bg:#5BCEFA>                </>\n\n
			",
			name,
			env!("CARGO_PKG_REPOSITORY"),
			env!("CARGO_PKG_LICENSE")
		))
		.color(ColorChoice::Auto)
		.styles(
			styling::Styles::styled()
				.header(styling::AnsiColor::Magenta.on_default().bold())
				.usage(styling::AnsiColor::Cyan.on_default().bold())
				.literal(styling::AnsiColor::Blue.on_default().bold())
				.placeholder(styling::AnsiColor::Green.on_default().bold())
				.error(styling::AnsiColor::Red.on_default().bold())
				.invalid(styling::AnsiColor::Red.on_default().bold()),
		)
		.arg(
			Arg::new("color")
				.long("color")
				.action(ArgAction::Set)
				.help("Control if color is enabled or disabled")
				.value_parser(value_parser!(ColorChoice))
				.value_name("WHEN")
				.default_value("auto"),
		)
		.arg(
			Arg::new("verbose")
				.short('v')
				.action(ArgAction::Count)
				.help("Enable verbose logging")
				.long_help(cformat!(
					"Enable verbose logging\n\nRepeat this option multiple times for increasingly \
					 verbose output.\n\nThis setting is overridden by the <magenta>{}</> \
					 environment variable and also yields to the <blue>-q</> flag for quiet \
					 operation.\n\nWhen this flag is set to the highest tracing level (e.g. any \
					 more than 2) the \nTokio console subscriber is also enabled for debugging \
					 the async runtime with the <green>tokio-console</> command.",
					log_env_var
				)),
		)
		.arg(
			Arg::new("quiet")
				.short('q')
				.action(ArgAction::SetTrue)
				.help("Disable all non-critical logging")
				.long_help(cformat!(
					"Disable all non-critical logging\n\nDisable all output unless it is a \
					 warning or error.\n\nThis setting is overridden by the <magenta>{}</> \
					 environment variable and overrides the <blue>-v</> flag if set.
					",
					log_env_var,
				)),
		))
}
