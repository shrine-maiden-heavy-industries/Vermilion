/* SPDX-License-Identifier: BSD-3-Clause */

use crate::types::{
	LspAny,
	params::{
		CancelParams, CreateFilesParams, DeleteFilesParams, DidChangeConfigurationParams,
		DidChangeNotebookDocumentParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
		DidChangeWorkspaceFoldersParams, DidCloseNotebookDocumentParams,
		DidCloseTextDocumentParams, DidOpenNotebookDocumentParams, DidOpenTextDocumentParams,
		DidSaveNotebookDocumentParams, DidSaveTextDocumentParams, InitializedParams,
		LogMessageParams, LogTraceParams, ProgressParams, PublishDiagnosticsParams,
		RenameFilesParams, SetTraceParams, ShowMessageParams, WillSaveTextDocumentParams,
		WorkDoneProgressCancelParams,
	},
};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "method", content = "params")]
pub enum Notification {
	// Message Direction: Client To Server
	/// The exit event is sent from the client to the server to ask the server to exit its process.
	#[serde(rename = "exit")]
	Exit,
	// Message Direction: Both
	#[serde(rename = "$/progress")]
	Progress(ProgressParams),
	// Message Direction: Both
	#[serde(rename = "$/cancelRequest")]
	Cancel(CancelParams),
	// Message Direction: Server To Client
	#[serde(rename = "$/logTrace")]
	LogTrace(LogTraceParams),
	// Message Direction: Client To Server
	#[serde(rename = "$/setTrace")]
	SetTrace(SetTraceParams),
	// Message Direction: Client To Server
	/// The initialized notification is sent from the client to the server after the client is fully
	/// initialized and the server is allowed to send requests from the server to the client.
	#[serde(rename = "initialized")]
	Initialized(serde_json::Value),
	// =======================
	// Message Direction: Client To Server
	/// The `workspace/didChangeWorkspaceFolders` notification is sent from the client to the server
	/// when the workspace folder configuration changes.
	#[serde(rename = "workspace/didChangeWorkspaceFolders")]
	WorkspaceFoldersChanged(DidChangeWorkspaceFoldersParams),
	// Message Direction: Client To Server
	/// The `window/workDoneProgress/cancel` notification is sent from  the client to the server to
	/// cancel a progress initiated on the server side.
	#[serde(rename = "window/workDoneProgress/cancel")]
	WindowCancelWorkDoneProgress(WorkDoneProgressCancelParams),
	/// The did create files notification is sent from the client to the server when files were
	/// created from within the client.
	///
	/// since: 3.16.0
	#[serde(rename = "workspace/didCreateFiles")]
	WorkspaceFilesCreated(CreateFilesParams), // FileOperationRegistrationOptions
	// Message Direction: Client To Server
	/// The did rename files notification is sent from the client to the server when files were
	/// renamed from within the client.
	///
	/// since: 3.16.0
	#[serde(rename = "workspace/didRenameFiles")]
	WorkspaceFilesRenamed(RenameFilesParams), // FileOperationRegistrationOptions
	// Message Direction: Client To Server
	/// The will delete files request is sent from the client to the server before files are actually
	/// deleted as long as the deletion is triggered from within the client.
	///
	/// since: 3.16.0",
	#[serde(rename = "workspace/didDeleteFiles")]
	WorkspaceFilesDeleted(DeleteFilesParams), // FileOperationRegistrationOptions
	// Message Direction: Client To Server
	/// A notification sent when a notebook opens.
	///
	/// since: 3.17.0
	#[serde(rename = "notebookDocument/didOpen")]
	NotebookOpened(DidOpenNotebookDocumentParams), // registration method: notebookDocument/sync
	// Message Direction: Client To Server
	#[serde(rename = "notebookDocument/didChange")]
	NotebookChanged(DidChangeNotebookDocumentParams), // registration method: notebookDocument/sync
	// Message Direction: Client To Server
	/// A notification sent when a notebook document is saved.
	///
	/// since: 3.17.0
	#[serde(rename = "notebookDocument/didSave")]
	NotebookSaved(DidSaveNotebookDocumentParams), // registration method: notebookDocument/sync
	// Message Direction: Client To Server
	/// A notification sent when a notebook closes.
	///
	/// since: 3.17.0
	#[serde(rename = "notebookDocument/didClose")]
	NotebookClosed(DidCloseNotebookDocumentParams), // registration method: notebookDocument/sync
	// Message Direction: Client To Server
	/// The configuration change notification is sent from the client to the server when the client's
	/// configuration has changed.
	///
	/// The notification contains the changed configuration as defined by the language client.
	#[serde(rename = "workspace/didChangeConfiguration")]
	WorkspaceConfigurationChanged(DidChangeConfigurationParams), // DidChangeConfigurationRegistrationOptions
	// Message Direction: Server To Client
	/// The show message notification is sent from a server to a client to ask the client
	/// to display a particular message in the user interface.
	#[serde(rename = "window/showMessage")]
	WindowShowMessage(ShowMessageParams),
	// Message Direction: Server To Client
	/// The log message notification is sent from the server to the client to ask the client
	/// to log a particular message.
	#[serde(rename = "window/logMessage")]
	WindowLogMessage(LogMessageParams),
	// Message Direction: Server To Client
	/// The telemetry event notification is sent from the server to the client to ask
	/// the client to log telemetry data.
	#[serde(rename = "telemetry/event")]
	TelemetryEven(LspAny),
	// Message Direction: Client To Server
	/// The document open notification is sent from the client to the server to signal newly opened
	/// text documents.
	///
	/// The document's truth is now managed by the client and the server must not try to read the
	/// document's truth using the document's uri. Open in this sense means it is managed by the
	/// client.
	///
	/// It doesn't necessarily mean that its content is presented in an editor.
	///
	/// An open notification must not be sent more than once without a corresponding close notification
	/// send before. This means open and close notification must be balanced and the max open count is one.
	#[serde(rename = "textDocument/didOpen")]
	TextDocumentOpened(DidOpenTextDocumentParams), // TextDocumentRegistrationOptions
	// Message Direction: Client To Server
	/// The document change notification is sent from the client to the server to signal changes to a
	/// text document.
	#[serde(rename = "textDocument/didChange")]
	TextDocumentChanged(DidChangeTextDocumentParams), // TextDocumentChangeRegistrationOptions
	// Message Direction: Client To Server
	/// The document close notification is sent from the client to the server when the document got
	/// closed in the client.
	///
	/// The document's truth now exists where the document's uri points to (e.g. if the document's
	/// uri is a file uri the truth now exists on disk).
	///
	/// As with the open notification the close notification is about managing the document's content.
	///
	/// Receiving a close notification doesn't mean that the document was open in an editor before.
	///
	/// A close notification requires a previous open notification to be sent.
	#[serde(rename = "textDocument/didClose")]
	TextDocumentClosed(DidCloseTextDocumentParams), // TextDocumentRegistrationOptions
	// Message Direction: Client To Server
	/// The document save notification is sent from the client to the server when the document got
	/// saved in the client.
	#[serde(rename = "textDocument/didSave")]
	TextDocumentSaved(DidSaveTextDocumentParams), // TextDocumentSaveRegistrationOptions
	// Message Direction: Client To Server
	/// A document will save notification is sent from the client to the server before the document is
	/// actually saved.
	#[serde(rename = "textDocument/willSave")]
	TextDocumentWillSave(WillSaveTextDocumentParams), // TextDocumentRegistrationOptions
	// Message Direction: Client To Server
	/// The watched files notification is sent from the client to the server when the client detects
	/// changes to file watched by the language client.
	#[serde(rename = "workspace/didChangeWatchedFiles")]
	WorkspaceWatchedFilesChanged(DidChangeWatchedFilesParams), // DidChangeWatchedFilesRegistrationOptions
	// Message Direction: Server To Client
	/// Diagnostics notification are sent from the server to the client to signal results of validation runs.
	#[serde(rename = "textDocument/publishDiagnostics")]
	TextDocumentPublishDiagnostics(PublishDiagnosticsParams),
}

#[cfg(test)]
mod tests {
	use super::*;
}
