// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

use eyre::{Result, eyre};
use tokio::{
	sync::mpsc::{UnboundedReceiver, UnboundedSender},
	task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, trace};

use super::message::Message;
#[cfg(feature = "trace-server")]
use crate::{trace::Trace, transports::trace::TraceTransport};

#[cfg_attr(unix, path = "pipe_unix.rs")]
#[cfg_attr(windows, path = "pipe_win.rs")]
pub mod pipe;
pub mod socket;
pub mod stdio;
#[cfg(feature = "trace-server")]
pub mod trace;

#[derive(Clone, Copy)]
enum ReadPhase {
	Header,
	Content(usize),
}

fn get_split_index(buffer: &[u8]) -> Option<usize> {
	buffer
		.windows(4)
		.enumerate()
		.find(|&(_, w)| matches!(w, b"\r\n\r\n"))
		.map(|(idx, _)| idx + 4)
}

fn parse_header(header: &[u8], shutdown_channel: &UnboundedSender<()>) -> Result<usize> {
	Ok(match str::from_utf8(header) {
		Ok(size) => match size.trim().parse::<usize>() {
			Ok(size) => size,
			Err(err) => {
				error!("{}", err);
				shutdown_channel.send(())?;
				return Err(err.into());
			},
		},
		Err(err) => {
			error!("{}", err);
			shutdown_channel.send(())?;
			return Err(err.into());
		},
	})
}

fn deserialise_message(
	content: &mut Vec<u8>,
	sender: &UnboundedSender<Message>,
	#[cfg(feature = "trace-server")] trace_sender: &Option<UnboundedSender<Trace>>,
) -> eyre::Result<()> {
	// Otherwise deserialize message and clear the buffer
	match Message::deserialize(content) {
		Ok(msg) => {
			#[cfg(feature = "trace-server")]
			if let Some(trace_sender) = trace_sender {
				// We don't want to abort the task if the send to the
				// trace writer failed
				let _ = trace_sender.send(Trace::new(crate::trace::Origin::Client, &msg));
			}
			sender.send(msg)?
		},
		Err(e) => {
			error!("Unable to deserialize LSP message:");
			error!("{}", e);
			error!("Message contents:");
			error!("{}", str::from_utf8(content)?);
		},
	}
	content.clear();
	Ok(())
}

fn parse_chunk(
	bytes_available: usize,
	buf: &[u8],
	content: &mut Vec<u8>,
	current_phase: ReadPhase,
	sender: &UnboundedSender<Message>,
	shutdown_channel: &UnboundedSender<()>,
	#[cfg(feature = "trace-server")] trace_sender: &Option<UnboundedSender<Trace>>,
) -> eyre::Result<(ReadPhase, usize)> {
	match current_phase {
		ReadPhase::Header => {
			// Check to see if what we read in starts with the content length header
			if buf.starts_with(b"Content-Length: ") {
				// Next, see if we can find the end of this header
				let split_index = get_split_index(buf);
				if let Some(end) = split_index {
					// Split out the header
					let header = &buf[16..end];

					// Extract the content length from the header
					let content_length = parse_header(header, shutdown_channel)?;
					trace!("Got content header of len: {}", content_length);

					// Switch into the content phase with whatever's left of this buffer
					Ok((ReadPhase::Content(content_length), bytes_available - end))
				} else {
					Err(eyre!("Unable to separate header from content!"))
				}
			} else {
				Err(eyre!("Invalid JSON-RPC header {:?}", str::from_utf8(buf)?))
			}
		},
		ReadPhase::Content(content_length) => {
			// Figure out how much of the content length remaining we can satisfy with this buffer
			let amount = bytes_available.min(content_length);
			// Extend the content buffer with that content
			content.extend_from_slice(&buf[0..amount]);

			// Calculate how many bytes are left unused in the buffer
			let buffer_remaining = bytes_available - amount;
			// And how many bytes are still needed to satisfy this content request
			let content_remaining = content_length - amount;

			trace!("{} bytes remaining", content_remaining);
			// If there's still more content needed, re-enter the content phase. buffer_remaining
			// will be 0.
			if content_remaining > 0 {
				Ok((ReadPhase::Content(content_remaining), 0))
			} else {
				// Otherwise, we completed extracting the content from the buffer, so deserialise it
				deserialise_message(
					content,
					sender,
					#[cfg(feature = "trace-server")]
					trace_sender,
				)?;
				// And switch back to the header phase with whatever's still reamining in the buffer
				// to be consumed
				Ok((ReadPhase::Header, buffer_remaining))
			}
		},
	}
}

fn parse_message(
	read: usize,
	buf: &[u8],
	content: &mut Vec<u8>,
	phase: &mut ReadPhase,
	sender: &UnboundedSender<Message>,
	shutdown_channel: &UnboundedSender<()>,
	#[cfg(feature = "trace-server")] trace_sender: &Option<UnboundedSender<Trace>>,
) -> eyre::Result<()> {
	let mut offset = 0;

	while offset < read {
		let (new_phase, buffer_remaining) = parse_chunk(
			read - offset,
			&buf[offset..read],
			content,
			*phase,
			sender,
			shutdown_channel,
			trace_sender,
		)?;
		*phase = new_phase;
		offset = read - buffer_remaining;
	}
	Ok(())
}

pub enum TransportType {
	Stdio,
	Socket(u16),
	Pipe(PathBuf),
}

pub trait LSPTransport: Sized {
	#[allow(async_fn_in_trait)]
	async fn create(
		self,
		cancellation_token: CancellationToken,
		shutdown_channel: UnboundedSender<()>,
		#[cfg(feature = "trace-server")] trace_transport: Option<TraceTransport>,
	) -> Result<(
		UnboundedReceiver<Message>,
		UnboundedSender<Message>,
		JoinSet<Result<()>>,
	)>;
}
