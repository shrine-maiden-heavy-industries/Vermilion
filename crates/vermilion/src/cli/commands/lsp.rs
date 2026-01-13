// SPDX-License-Identifier: BSD-3-Clause

use std::{net::SocketAddr, path::PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command, ValueHint};
use tracing::error;
use vermilion_lsp::transports::{self, trace::TraceTransport};

use crate::{lsp, workspace::load_workspace_config};

pub(crate) const COMMAND_NAME: &str = "server";

pub(crate) fn init() -> eyre::Result<Command> {
	Ok(Command::new(COMMAND_NAME)
		.about("Start the Vermillion LSP server")
		.arg(
			Arg::new("pipe")
				.long("pipe")
				.help("Path to the Unix socket or named pipe for LSP communication")
				.value_hint(ValueHint::FilePath)
				.value_name("PIPE")
				.value_parser(clap::value_parser!(PathBuf))
				.action(ArgAction::Set)
				.conflicts_with_all(["socket", "stdio"]),
		)
		.arg(
			Arg::new("socket")
				.long("port")
				.help("The port to connect to for the LSP server to talk to")
				.value_name("PORT")
				.value_parser(clap::value_parser!(u16))
				.action(ArgAction::Set)
				.conflicts_with_all(["pipe", "stdio"]),
		)
		.arg(
			Arg::new("stdio")
				.long("stdio")
				.help("Use STDIO for LSP communication")
				.action(ArgAction::SetTrue)
				.conflicts_with_all(["pipe", "socket"]),
		)
		.arg(
			Arg::new("client-pid")
				.long("clientProcessId")
				.help("The LSP client's PID")
				.value_name("PID")
				.value_parser(clap::value_parser!(usize))
				.action(ArgAction::Set),
		)
		.arg(
			Arg::new("trace-file")
				.long("trace-file")
				.help("Enable LSP server tracing to specified file")
				.action(ArgAction::Set)
				.value_hint(ValueHint::FilePath)
				.value_parser(clap::value_parser!(PathBuf))
				.conflicts_with("trace-socket"),
		)
		.arg(
			Arg::new("trace-socket")
				.long("trace-socket")
				.help("Enable LSP server tracing to specified socket")
				.action(ArgAction::Set)
				.value_parser(clap::value_parser!(SocketAddr))
				.conflicts_with("trace-file"),
		))
}

pub(crate) fn exec(args: &ArgMatches) -> eyre::Result<()> {
	// If we get passed `--clientProcessId` we want to watch for that PID to die
	let client_pid = args.try_get_one::<usize>("client-pid")?.copied();

	// The possible transport types
	let pipe = args.try_get_one::<PathBuf>("pipe")?.cloned();
	let port = args.try_get_one::<u16>("socket")?.copied();
	let stdio = args.try_get_one::<bool>("stdio")?;

	// Possible trace transports
	let trace_file = args.try_get_one::<PathBuf>("trace-file")?.cloned();
	let trace_socket = args.try_get_one::<SocketAddr>("trace-socket")?.cloned();

	// Figure out which transport we want to use
	let transport = if let Some(pipe) = pipe {
		Some(transports::TransportType::Pipe(pipe))
	} else if let Some(port) = port {
		Some(transports::TransportType::Socket(port))
	} else {
		stdio.map(|_| transports::TransportType::Stdio)
	};

	// Select the trace transport backend if given
	let trace_transport = if let Some(path) = trace_file {
		Some(TraceTransport::File(path))
	} else {
		trace_socket.map(TraceTransport::Socket)
	};

	if let Some(transport) = transport {
		lsp::start(
			transport,
			client_pid,
			load_workspace_config(args)?,
			trace_transport,
		)
	} else {
		error!("You must specify an LSP transport type!");
		Ok(())
	}
}
