/* SPDX-License-Identifier: BSD-3-Clause */

use crate::types::{
	CallHierarchyItem, ClientInfo, CodeActionContext, Color, CompletionContext, ConfigurationItem,
	Diagnostic, FileCreate, FileDelete, FileEvent, FileRename, InlineCompletionContext,
	InlineValueContext, LspAny, MessageActionItem, MessageType, NotebookDocument,
	NotebookDocumentChangeEvent, NotebookDocumentIdentifier, Position, PreviousResultId,
	ProgressToken, Range, ReferenceContext, Registration, SignatureHelpContext,
	TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem,
	TextDocumentSaveReason, TraceValues, TypeHierarchyItem, Unregistration, Uri,
	VersionedNotebookDocumentIdentifier, VersionedTextDocumentIdentifier, WorkspaceEdit,
	WorkspaceFolder, WorkspaceFoldersChangeEvent, capabilities::client::ClientCapabilities,
	options::FormattingOptions,
};

/// The actual workspace folder change event
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWorkspaceFolderParams {
	/// The parameters of a `workspace/didChangeWorkspaceFolders` notification
	pub event: WorkspaceFoldersChangeEvent,
}

/// The parameters of a configuration request
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationParams {
	pub items: Vec<ConfigurationItem>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressCreateParams {
	/// The token to be used to report progress
	pub token: ProgressToken,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressCancelParams {
	/// The token to be used to report progress
	pub token: ProgressToken,
}

/// Params to show a resource in the UI.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowDocumentParams {
	/// The uri to show
	pub uri: Uri,
	/// Indicates to show the resource in an external program.
	///
	/// To show, for example, `https://gay.catgirl.stream/` in the default web browser set `external` to `tru
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub external: Option<bool>,
	/// An optional property to indicate whether the editor showing the document should take focus or not.
	///
	/// Clients might ignore this property if an external program is started.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub take_focus: Option<bool>,
	/// An optional selection range if the document is a text document.
	///
	/// Clients might ignore the property if an external program is started or the file is not
	/// a text file.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub selection: Option<Range>,
}

/// The parameters sent in notifications/requests for user-initiated creation of files.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CreateFilesParams {
	/// An array of all files/folders created in this operation.
	pub files: Vec<FileCreate>,
}

/// The parameters sent in notifications/requests for user-initiated renames of files.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameFilesParams {
	/// An array of all files/folders renamed in this operation. When a folder is renamed, only
	/// the folder will be included, and not its children.
	pub files: Vec<FileRename>,
}

/// The parameters sent in notifications/requests for user-initiated deletes of files.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFilesParams {
	/// An array of all files/folders deleted in this operation.
	pub files: Vec<FileDelete>,
}

/// The params sent in an open notebook document notification.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenNotebookDocumentParams {
	/// The notebook document that got opened
	pub notebook_document: NotebookDocument,
	/// The text documents that represent the content of a notebook cell.
	pub cell_text_documents: Vec<TextDocumentItem>,
}

/// The params sent in a change notebook document notification.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeNotebookDocumentParams {
	/// The notebook document that did change.
	///
	/// The version number points to the version after all provided changes have been applied.
	///
	/// If only the text document content of a cell changes the notebook version doesn't necessarily
	/// have to change.
	pub notebook_document: VersionedNotebookDocumentIdentifier,
	/// The actual changes to the notebook document.
	///
	/// The changes describe single state changes to the notebook document.
	/// So if there are two changes c1 (at array index 0) and c2 (at array index 1)
	/// for a notebook in state S then c1 moves the notebook from\nS to S' and c2 from S' to S''.
	/// So c1 is computed on the state S and\nc2 is computed on the state S'.
	///
	/// To mirror the content of a notebook using change events use the following approach:
	/// - start with the same initial content
	/// - apply the 'notebookDocument/didChange' notifications in the order you receive them.
	/// - apply the `NotebookChangeEvent`s in a single notification in the order you receive them.
	pub change: NotebookDocumentChangeEvent,
}

/// The params sent in a save notebook document notification.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidSaveNotebookDocumentParams {
	/// The notebook document that got saved
	pub notebook_document: NotebookDocumentIdentifier,
}

/// The params sent in a close notebook document notification.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidCloseNotebookDocumentParams {
	/// The notebook document that got closed
	pub notebook_document: NotebookDocumentIdentifier,
	/// The text documents that represent the content of a notebook cell that got closed.
	pub cell_text_documents: Vec<TextDocumentIdentifier>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationParams {
	pub registrations: Vec<Registration>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct UnregistrationParams {
	pub unregistrations: Vec<Unregistration>,
}

/// The parameters of a change configuration notification.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationParams {
	/// The actual changed settings
	pub settings: serde_json::Value,
}

/// The parameters of a notification message
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessageParams {
	/// The message type. See [`MessageType`]
	#[serde(rename = "type")]
	pub typ: MessageType,
	/// The actual message.
	pub message: String,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessageRequestParams {
	/// The message type. See [`MessageType`]
	#[serde(rename = "type")]
	pub typ: MessageType,
	/// The actual message.
	pub message: String,
	/// The message action items to present.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub actions: Option<Vec<MessageActionItem>>,
}

///The log message parameters
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LogMessageParams {
	/// The message type. See [`MessageType`]
	#[serde(rename = "type")]
	pub typ: MessageType,
	/// The actual message.
	pub message: String,
}

/// The parameters sent in an open text document notification
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentParams {
	/// The document that was opened
	pub text_document: TextDocumentItem,
}

/// The change text document notification's parameters
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentParams {
	/// The document that did change.
	///
	/// The version number points to the version after all provided content changes have been applied.
	pub text_document: VersionedTextDocumentIdentifier,
	/// The actual content changes.
	///
	/// The content changes describe single state changes to the document. So if there are two content
	/// changes c1 (at array index 0) and c2 (at array index 1) for a document in state S then c1 moves
	/// the document from\nS to S' and c2 from S' to S''. So c1 is computed on the state S and c2 is
	/// computed on the state S'.
	///
	/// To mirror the content of a document using change events use the following approach:
	/// - start with the same initial content
	/// - apply the 'textDocument/didChange' notifications in the order you receive them.
	/// - apply the `TextDocumentContentChangeEvent`s in a single notification in the order you receive them.
	pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

/// The parameters sent in a close text document notification
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidCloseTextDocumentParams {
	/// The document that was closed
	pub text_document: TextDocumentIdentifier,
}

/// The parameters sent in a save text document notification
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidSaveTextDocumentParams {
	/// The document that was saved
	pub text_document: TextDocumentIdentifier,
	/// Optional the content when saved. Depends on the includeText value when the save notification was requested
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text: Option<String>,
}

/// The parameters sent in a will save text document notification
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WillSaveTextDocumentParams {
	/// The document that will be saved
	pub text_document: TextDocumentIdentifier,
	/// The reason for saving
	pub reason: TextDocumentSaveReason,
}

/// The watched files change notification's parameters
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesParams {
	/// The actual file events
	pub changes: Vec<FileEvent>,
}

/// The publish diagnostic notification's parameters.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishDiagnosticsParams {
	/// The URI for which diagnostic information is reported.
	pub uri: Uri,
	/// Optional the version number of the document the diagnostics are published for.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<i32>,
	/// An array of diagnostic information items
	pub diagnostics: Vec<Diagnostic>,
}

/// The parameters of a [`DocumentOnTypeFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingParams {
	/// The document to format.
	pub text_document: TextDocumentIdentifier,
	/// The position around which the on type formatting should happen.
	///
	/// This is not necessarily the exact position where the character denoted by the property `ch` got typed.
	pub position: Position,
	/// The character that has been typed that triggered the formatting on type request.
	///
	/// That is not necessarily the last character that got inserted into the document since the client
	/// could auto insert characters as well (e.g. like automatic brace completion)
	pub ch: String, // TODO(aki): Should this be a `char`?
	/// The formatting options.
	pub options: FormattingOptions,
}

/// The parameters passed via an apply workspace edit request
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyWorkspaceEditParams {
	/// An optional label of the workspace edit.
	///
	/// This label is presented in the user interface for example on an undo stack to undo the workspace edit.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub label: Option<String>,
	/// The edits to apply.
	pub edit: WorkspaceEdit,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SetTraceParams {
	pub value: TraceValues,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LogTraceParams {
	pub message: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub verbose: Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CancelParams {
	/// The request ID to cancel
	pub id: ProgressToken, // XXX(aki): Not the proper type, but same elements
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressParams {
	/// The progress token provided by the client or server.
	pub token: ProgressToken,
	/// The progress data.
	pub value: serde_json::Value,
}

/// A parameter literal used in requests to pass a text document and a position inside that document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentPositionParams {
	/// The text document
	pub text_document: TextDocumentIdentifier,
	/// The position inside the text document
	pub position: Position,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersInitializeParams {
	/// The workspace folders configured in the client when the server starts.
	///
	/// This property is only available if the client supports workspace folders.
	///
	/// It can be `null` if the client supports workspace folders but none are configured.
	///
	/// since: 3.6.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PartialResultParams {
	/// An optional token that a server can use to report partial results (e.g. streaming) to the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub partial_result_token: Option<ProgressToken>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressParams {
	/// An optional token that a server can use to report work done progress.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub work_done_token: Option<ProgressToken>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_param: PartialResultParams,
}

/// Parameters for a [`DocumentColorRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorPresentationParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	/// The color to request presentations for.
	pub color: Color,
	/// The range where the color would be inserted. Serves as a context.
	pub range: Range,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Parameters for a [`FoldingRangeRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// A parameter literal used in selection range requests.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	/// The positions inside the text document.
	pub positions: Vec<Position>,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameter of a `textDocument/prepareCallHierarchy` request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyPrepareParams {
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
}

/// The parameter of a `callHierarchy/incomingCalls` request.
///
/// since: 3.16.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyIncomingCallsParams {
	pub item: CallHierarchyItem,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameter of a `callHierarchy/outgoingCalls` request.
///
/// since: 3.16.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyOutgoingCallsParams {
	pub item: CallHierarchyItem,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensDeltaParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	/// The result id of a previous response.
	///
	/// The result Id can either point to a full response
	/// or a delta response depending on what was received last.
	pub previous_result_id: String,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensRangeParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	/// The range the semantic tokens are requested for.
	pub range: Range,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MonikerParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameter of a `textDocument/prepareTypeHierarchy` request.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyPrepareParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// The parameter of a `typeHierarchy/supertypes` request.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchySupertypesParams {
	pub item: TypeHierarchyItem,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameter of a `typeHierarchy/subtypes` request.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchySubtypesParams {
	pub item: TypeHierarchyItem,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// A parameter literal used in inline value requests.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	/// The document range for which inline values should be computed.
	pub range: Range,
	/// Additional information about the context in which inline values were requested.
	pub context: InlineValueContext,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// A parameter literal used in inlay hint requests.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	/// The document range for which inlay hints should be computed.
	pub range: Range,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// Parameters of the document diagnostic request.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDiagnosticParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	/// The additional identifier  provided during registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub identifier: Option<String>,
	/// The result id of a previous response if provided.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub previous_result_id: Option<String>,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Parameters of the workspace diagnostic request.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDiagnosticParams {
	/// The additional identifier provided during registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub identifier: Option<bool>,
	/// The currently known diagnostic reports with their previous result ids.
	pub previous_result_ids: Vec<PreviousResultId>,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// A parameter literal used in inline completion requests.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionParams {
	/// Additional information about the context in which inline completions were requested.
	pub context: InlineCompletionContext,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Completion parameters
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	/// The completion context. This is only available if the client specifies\nto send this using the client capability `textDocument.completion.contextSupport === true`
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub context: Option<CompletionContext>,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Parameters for a [`HoverRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct HoverParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// Parameters for a [`SignatureHelpRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpParams {
	/// The signature help context. This is only available if the client specifies\nto send this using the client capability `textDocument.signatureHelp.contextSupport === true`
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub context: Option<SignatureHelpContext>,
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// Parameters for a [`DefinitionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Parameters for a [`ReferencesRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceParams {
	pub context: ReferenceContext,
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Parameters for a [`DocumentHighlightRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Parameters for a [`DocumentSymbolRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// Parameters for a [`CodeActionRequest`].
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionParams {
	/// The document in which the command was invoked.
	pub text_document: TextDocumentIdentifier,
	/// The range for which the command was invoked.
	pub range: Range,
	/// Context carrying additional information.
	pub context: CodeActionContext,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameters of a [`WorkspaceSymbolRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolParams {
	/// A query string to filter symbols by. Clients may send an empty string here to request all symbols.
	pub query: String,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameters of a [`CodeLensRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensParams {
	/// The document to request code lens for.
	pub text_document: TextDocumentIdentifier,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameters of a [`DocumentLinkRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkParams {
	/// The document to provide document links for.
	pub text_document: TextDocumentIdentifier,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub partial_result_params: PartialResultParams,
}

/// The parameters of a [`DocumentFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingParams {
	/// The document to format.
	pub text_document: TextDocumentIdentifier,
	/// The format options.
	pub options: FormattingOptions,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// The parameters of a [`DocumentRangeFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingParams {
	/// The document to format.
	pub text_document: TextDocumentIdentifier,
	/// The range to format.
	pub range: Range,
	/// The format options.
	pub options: FormattingOptions,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// The parameters of a [`DocumentRangesFormattingRequest`].
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangesFormattingParams {
	/// The document to format.
	pub text_document: TextDocumentIdentifier,
	/// The ranges to format.
	pub range: Vec<Range>,
	/// The format options.
	pub options: FormattingOptions,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// The parameters of a [`RenameRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameParams {
	/// The document to format.
	pub text_document: TextDocumentIdentifier,
	/// The position at which this request was sent.
	pub position: Position,
	/// The new name of the symbol.
	///
	/// If the given name is not valid the request must return a [`ResponseError`] with an
	/// appropriate message set.
	pub new_name: String,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PrepareRenameParams {
	#[serde(flatten)]
	pub text_document_position_params: TextDocumentPositionParams,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// The parameters of a [`ExecuteCommandRequest`].
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandParams {
	/// The identifier of the actual command handler.
	pub command: String,
	/// Arguments that the command should be invoked with.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub arguments: Option<Vec<LspAny>>,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
}

/// The initialize parameters
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializedParams {
	/// The process ID of the parent process that started the server.
	///
	/// Is `null` if the process has not been started by another process.
	///
	/// If the parent process is not alive then the server should exit.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub process_id: Option<i32>,
	/// Information about the LSP client
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub client_info: Option<ClientInfo>,
	/// The locale the client is currently showing the user interface in.
	///
	/// This must not necessarily be the locale of the operating system.
	///
	/// Uses IETF language tags as the value's syntax see: https://en.wikipedia.org/wiki/IETF_language_tag)
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub locale: Option<String>,
	/// The rootPath of the workspace.
	///
	/// Is `null`l if no folder is open.
	///
	/// deprecated: in favour of [`InitializedParams::root_uri`] .
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub root_path: Option<String>,
	/// The root URI of the workspace.
	///
	/// Is null if no folder is open.
	///
	/// If both [`InitializedParams::root_path`] and [`InitializedParams::root_uri`] are set
	/// [`InitializedParams::root_uri`] wins.
	///
	/// deprecated: in favour of [`WorkspaceFoldersInitializeParams::workspace_folders`] .
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub root_uri: Option<Uri>, // BUG(aki): This should be serialized as `null` instead of skipped
	/// The capabilities provided by the client (editor or tool)
	pub capabilities: ClientCapabilities,
	/// User provided initialization options.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub initialization_options: Option<LspAny>,
	/// The initial trace setting.
	///
	/// If omitted trace is disabled ('off').
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trace: Option<TraceValues>,
	#[serde(flatten)]
	pub work_done_progress_params: WorkDoneProgressParams,
	#[serde(flatten)]
	pub workspace_folder_initialize_params: WorkspaceFoldersInitializeParams,
}
