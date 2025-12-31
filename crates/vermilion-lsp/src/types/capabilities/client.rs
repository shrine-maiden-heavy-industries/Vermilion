/* SPDX-License-Identifier: BSD-3-Clause */

use crate::types::{
	CodeActionKind, CompletionItemKind, CompletionItemTag, CompletionListItemDefaults,
	DiagnosticTag, FailureHandlingKind, FoldingRangeKind, InsertTextMode, LspAny, MarkupKind,
	PositionEncodingKind, PrepareSupportDefaultBehavior, ResourceOperationKind,
	SemanticTokensFullRequestsType, StringProperties, SymbolKind, SymbolTag, TokenFormat, ValueSet,
};

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client capabilities for the linked editing range request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client capabilities specific to the moniker request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MonikerClientCapabilities {
	/// Whether moniker supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `MonikerRegistrationOptions` return value
	/// for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client capabilities specific to inline values.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueClientCapabilities {
	/// Whether implementation supports dynamic registration for inline value providers.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintResolveSupport {
	/// The properties that a client can resolve lazily.
	pub properties: Vec<String>,
}

/// Inlay hint client capabilities.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintClientCapabilities {
	/// Whether inlay hints support dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Indicates which properties a client can resolve lazily on an inlay hint..
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_support: Option<InlayHintResolveSupport>,
}

/// Client capabilities specific to diagnostic pull requests.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Whether the clients supports related documents for document diagnostic pulls.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub related_document_support: Option<bool>,
}

/// Client capabilities specific to inline completions.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionClientCapabilities {
	/// Whether implementation supports dynamic registration for inline completion providers.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Notebook specific client capabilities.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports sending execution summary data per cell.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub execution_summary_support: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItemCapabilities {
	/// Whether the client supports additional attributes which are preserved
	/// and send back to the server in the request's response.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub additional_properties_supported: Option<bool>,
}

/// Show message request client capabilities
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessageRequestClientCapabilities {
	/// Capabilities specific to the [`MessageActionItem`] type.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub message_action_item: Option<MessageActionItemCapabilities>,
}

/// Client capabilities specific to the used markdown parser.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownClientCapabilities {
	/// The name of the parser.
	pub parser: String,
	/// The version of the parser.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<String>,
	/// A list of HTML tags that the client allows/supports in Markdown.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub allowed_tags: Option<Vec<String>>,
}

/// Client capabilities specific to regular expressions.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RegularExpressionsClientCapabilities {
	/// The engine's name.
	pub engine: String,
	/// The engine's version.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<String>,
}

/// Client capabilities for the showDocument request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowDocumentClientCapabilities {
	/// The client has support for the showDocument request.
	pub support: bool,
}

/// Defines the capabilities provided by the client
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
	/// Workspace specific client capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub workspace: Option<WorkspaceClientCapabilities>,
	/// Text document specific client capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text_document: Option<TextDocumentClientCapabilities>,
	/// Capabilities specific to the notebook document support.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub notebook_document: Option<NotebookDocumentClientCapabilities>,
	/// Window specific client capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub window: Option<WindowClientCapabilities>,
	/// General client capabilities.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub general: Option<GeneralClientCapabilities>,
	/// Experimental client capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub experimental: Option<LspAny>,
}

/// Workspace specific client capabilities.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceClientCapabilities {
	/// The client supports applying batch edits to the workspace by supporting the request
	/// 'workspace/applyEdit'
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub apply_edit: Option<bool>,
	/// Capabilities specific to [`WorkspaceEdit`]s.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub workspace_edit: Option<WorkspaceEditClientCapabilities>,
	/// Capabilities specific to the `workspace/didChangeConfiguration` notification.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,
	/// Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_change_watched_files: Option<DidChangeWatchedFilesClientCapabilities>,
	/// Capabilities specific to the `workspace/symbol` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub symbol: Option<WorkspaceSymbolClientCapabilities>,
	/// Capabilities specific to the `workspace/executeCommand` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub execute_command: Option<ExecuteCommandClientCapabilities>,
	/// The client has support for workspace folders.
	///
	/// since: 3.6.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub workspace_folders: Option<bool>,
	/// The client supports `workspace/configuration` requests.
	///
	/// since 3.6.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub configuration: Option<bool>,
	/// Capabilities specific to the semantic token requests scoped to the workspace.
	///
	/// since: 3.16.0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub semantic_tokens: Option<SemanticTokensWorkspaceClientCapabilities>,
	/// Capabilities specific to the code lens requests scoped to the workspace.
	///
	/// since: 3.16.0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_lense: Option<CodeLensWorkspaceClientCapabilities>,
	/// The client has support for file notifications/requests for user operations on files.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub file_operations: Option<FileOperationClientCapabilities>,
	/// Capabilities specific to the inline values requests scoped to the workspace.
	///
	/// since: 3.17.0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inline_value: Option<InlineValueWorkspaceClientCapabilities>,
	/// Capabilities specific to the inlay hint requests scoped to the workspace.
	///
	/// since: 3.17.0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inlay_hint: Option<InlayHintWorkspaceClientCapabilities>,
	/// Capabilities specific to the diagnostic requests scoped to the workspace.
	///
	/// since: 3.17.0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub diagnostics: Option<DiagnosticWorkspaceClientCapabilities>,
	/// Capabilities specific to the folding range requests scoped to the workspace.
	///
	/// since: 3.18.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub folder_range: Option<FoldingRangeWorkspaceClientCapabilities>,
}

/// Text document specific client capabilities.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
	/// Defines which synchronization capabilities the client supports.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub synchronization: Option<TextDocumentSyncClientCapabilities>,
	/// Capabilities specific to the `textDocument/completion` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub completion: Option<CompletionClientCapabilities>,
	/// Capabilities specific to the `textDocument/hover` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub hover: Option<HoverClientCapabilities>,
	/// Capabilities specific to the `textDocument/signatureHelp` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub signature_help: Option<SignatureHelpClientCapabilities>,
	/// Capabilities specific to the `textDocument/declaration` request.
	///
	/// since: 3.14.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub declaration: Option<DeclarationClientCapabilities>,
	/// Capabilities specific to the `textDocument/definition` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub definition: Option<DefinitionClientCapabilities>,
	/// Capabilities specific to the `textDocument/typeDefinition` request.
	///
	/// since: 3.6.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub type_definition: Option<TypeDefinitionClientCapabilities>,
	/// Capabilities specific to the `textDocument/implementation` request.
	///
	/// since: 3.6.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub implementation: Option<ImplementationClientCapabilities>,
	/// Capabilities specific to the `textDocument/references` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub references: Option<ReferenceClientCapabilities>,
	/// Capabilities specific to the `textDocument/documentHighlight` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_highlight: Option<DocumentHighlightClientCapabilities>,
	/// Capabilities specific to the `textDocument/documentSymbol` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_symbol: Option<DocumentSymbolClientCapabilities>,
	/// Capabilities specific to the `textDocument/codeAction` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_action: Option<CodeActionClientCapabilities>,
	/// Capabilities specific to the `textDocument/codeLens` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_lens: Option<CodeLensClientCapabilities>,
	/// Capabilities specific to the `textDocument/documentLink` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_link: Option<DocumentLinkClientCapabilities>,
	/// Capabilities specific to the `textDocument/documentColor` and the `textDocument/colorPresentation` request.
	///
	/// since: 3.6.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub color_provider: Option<DocumentColorClientCapabilities>,
	/// Capabilities specific to the `textDocument/formatting` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub formatting: Option<DocumentFormattingClientCapabilities>,
	/// Capabilities specific to the `textDocument/rangeFormatting` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range_formatting: Option<DocumentRangeFormattingClientCapabilities>,
	/// Capabilities specific to the `textDocument/onTypeFormatting` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub on_type_formatting: Option<DocumentOnTypeFormattingClientCapabilities>,
	/// Capabilities specific to the `textDocument/rename` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub rename: Option<RenameClientCapabilities>,
	/// Capabilities specific to the `textDocument/foldingRange` request.
	///
	/// since: 3.10.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub folding_range: Option<FoldingRangeClientCapabilities>,
	/// Capabilities specific to the `textDocument/selectionRange` request.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub selection_range: Option<SelectionRangeClientCapabilities>,
	/// Capabilities specific to the `textDocument/publishDiagnostics` notification.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>,
	/// Capabilities specific to the various call hierarchy requests.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub call_hierarchy: Option<CallHierarchyClientCapabilities>,
	/// Capabilities specific to the various semantic token request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub semantic_tokens: Option<SemanticTokensClientCapabilities>,
	/// Capabilities specific to the `textDocument/linkedEditingRange` request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub linked_editing_range: Option<LinkedEditingRangeClientCapabilities>,
	/// Client capabilities specific to the `textDocument/moniker` request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub moniker: Option<MonikerClientCapabilities>,
	/// Capabilities specific to the various type hierarchy requests.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub type_hierarchy: Option<TypeHierarchyClientCapabilities>,
	/// Capabilities specific to the `textDocument/inlineValue` request.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inline_value: Option<InlineValueClientCapabilities>,
	/// Capabilities specific to the `textDocument/inlayHint` request.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inlay_hint: Option<InlayHintClientCapabilities>,
	/// Capabilities specific to the diagnostic pull model.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub diagnostic: Option<DiagnosticClientCapabilities>,
	/// Client capabilities specific to inline completions.
	///
	/// since: 3.18.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inline_completion: Option<InlineCompletionClientCapabilities>,
}

/// Capabilities specific to the notebook document support.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentClientCapabilities {
	/// Capabilities specific to notebook document synchronization
	///
	/// since: 3.17.0
	pub synchronization: NotebookDocumentSyncClientCapabilities,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WindowClientCapabilities {
	/// It indicates whether the client supports server initiated progress using the
	/// `window/workDoneProgress/create` request.
	///
	/// The capability also controls Whether client supports handling
	/// of progress notifications.
	///
	/// If set servers are allowed to report a `workDoneProgress` property in the request specific
	/// server capabilities.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub work_done_progress: Option<bool>,
	/// Capabilities specific to the showMessage request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub show_message: Option<ShowMessageRequestClientCapabilities>,
	/// Capabilities specific to the showDocument request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub show_document: Option<ShowDocumentClientCapabilities>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StaleRequestClientCapabilities {
	/// The client will actively cancel the request.
	pub cancel: bool,
	/// The list of requests for which the client will retry the request if it receives a
	/// response with error code [`crate::error::Code::ContentModified`]
	pub retry_on_content_modified: Vec<String>,
}

/// General client capabilities.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GeneralClientCapabilities {
	/// Client capability that signals how the client handles stale requests (e.g. a request
	/// for which the client will not process the response anymore since the information is outdated).
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub stale_request_support: Option<StaleRequestClientCapabilities>,
	/// Client capabilities specific to regular expressions.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub regular_expressions: Option<RegularExpressionsClientCapabilities>,
	/// Client capabilities specific to the client's markdown parser.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub markdown: Option<MarkdownClientCapabilities>,
	/// The position encodings supported by the client.
	///
	/// Client and server have to agree on the same position encoding to ensure
	/// that offsets (e.g. character position in a line) are interpreted the same on both sides.
	///
	/// To keep the protocol backwards compatible the following applies: if the value 'utf-16' is missing
	/// from the array of position encodings servers can assume that the client supports UTF-16. UTF-16 is therefore a
	/// mandatory encoding.
	///
	/// If omitted it defaults to [`PositionEncodingKind::Utf16`].
	///
	/// Implementation considerations: since the conversion from one encoding into another
	/// requires the content of the file/line the conversion is best done where the file is
	/// read which is usually on the server side.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub position_encodings: Option<Vec<PositionEncodingKind>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceChangeAnnotationSupportClientCapabilities {
	/// Whether the client groups edits with equal labels into tree nodes, for instance all edits
	/// labelled with "Changes in Strings" would be a tree node.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub groups_on_label: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditClientCapabilities {
	/// The client supports versioned document changes in [`WorkspaceEdit`]s
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_changes: Option<bool>,
	/// The resource operations the client supports.
	///
	/// Clients should at least support 'create', 'rename' and 'delete' files and folders.
	///
	/// since: 3.13.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resource_operations: Option<Vec<ResourceOperationKind>>,
	/// The failure handling strategy of a client if applying the workspace edit fails.
	///
	/// since: 3.13.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub failure_handling: Option<FailureHandlingKind>,
	/// Whether the client normalizes line endings to the client specific setting.
	///
	/// If set to `true` the client will normalize line ending characters in a workspace
	/// edit to the client-specified new line character.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub normalizes_line_endings: Option<bool>,
	/// Whether the client in general supports change annotations on text edits,create file,
	/// rename file and delete file changes.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub change_annotation_support: Option<WorkspaceChangeAnnotationSupportClientCapabilities>,
}
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationClientCapabilities {
	/// Did change configuration notification supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesClientCapabilities {
	/// Did change watched files notification supports dynamic registration.
	///
	/// Please note that the current protocol doesn't support static configuration for file changes
	/// from the server side.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Whether the client has support for [`RelativePattern`] or not.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub relative_pattern_support: Option<bool>,
}

/// Client capabilities for a [`WorkspaceSymbolRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolClientCapabilities {
	/// Symbol request supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The symbol kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle
	/// values outside its set gracefully and falls back to a default value when unknown.
	///
	/// If this property is not present the client only supports the symbol kinds from
	/// `File` to `Array` as defined in the initial version of the protocol.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub symbol_kind: Option<ValueSet<SymbolKind>>,
	/// The client supports tags on [`SymbolInformation`].
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag_support: Option<ValueSet<SymbolTag>>,
	/// The client support partial workspace symbols.
	///
	/// The client will send the request `workspaceSymbol/resolve` to the server to resolve
	/// additional properties.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_support: Option<StringProperties>,
}

/// Client capabilities for a [`ExecuteCommandRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandClientCapabilities {
	/// Execute command supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensWorkspaceClientCapabilities {
	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all semantic tokens currently
	/// shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example
	/// detects a project wide change that requires such a calculation.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub refresh_support: Option<bool>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensWorkspaceClientCapabilities {
	/// "Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all code lenses currently shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example
	/// detect a project wide change that requires such a calculation.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub refresh_support: Option<bool>,
}

/// Capabilities relating to events from file operations by the user in the client.
///
/// These events do not come from the file system, they come from user operations
/// like renaming a file in the UI.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationClientCapabilities {
	/// Whether the client supports dynamic registration for file requests/notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client has support for sending didCreateFiles notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_create: Option<bool>,
	/// The client has support for sending willCreateFiles requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_create: Option<bool>,
	/// The client has support for sending didRenameFiles notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_rename: Option<bool>,
	/// The client has support for sending willRenameFiles requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_rename: Option<bool>,
	/// The client has support for sending didDeleteFiles notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_delete: Option<bool>,
	/// The client has support for sending willDeleteFiles requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_delete: Option<bool>,
}

/// Client workspace capabilities specific to inline values.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueWorkspaceClientCapabilities {
	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all inline values currently shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub refresh_support: Option<bool>,
}

/// Client workspace capabilities specific to inlay hints.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintWorkspaceClientCapabilities {
	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all inlay hints currently shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub refresh_support: Option<bool>,
}

/// Workspace client capabilities specific to diagnostic pull requests.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticWorkspaceClientCapabilities {
	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all pulled diagnostics currently
	/// shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub refresh_support: Option<bool>,
}

/// Client workspace capabilities specific to folding ranges
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeWorkspaceClientCapabilities {
	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all folding ranges currently
	/// shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	///
	/// since: 3.18.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub refresh_support: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncClientCapabilities {
	/// Whether text document synchronization supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports sending will save notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_save: Option<bool>,
	/// The client supports sending a will save request and waits for a response providing
	/// text edits which will be applied to the document before it is saved.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_save_wait_until: Option<bool>,
	/// The client supports did save notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_save: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemClientCapabilities {
	/// Client supports snippets as insert text.
	///
	/// A snippet can define tab stops and placeholders with `$1`, `$2` and `${3:foo}`.
	///
	/// `$0` defines the final tab stop, it defaults to the end of the snippet.
	///
	/// Placeholders with equal identifiers are linked, that is typing in one will update others too.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub snippet_support: Option<bool>,
	/// Client supports commit characters on a completion item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub commit_characters_support: Option<bool>,
	/// Client supports the following content formats for the documentation property.
	///
	/// The order describes the preferred format of the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub documentation_format: Option<Vec<MarkupKind>>,
	/// Client supports the deprecated property on a completion item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub deprecated_support: Option<bool>,
	/// Client supports the preselect property on a completion item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub preselect_support: Option<bool>,
	/// Client supports the tag property on a completion item.
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// Clients especially need to preserve unknown tags when sending a completion item back
	/// to the server in a resolve call.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag_support: Option<ValueSet<CompletionItemTag>>,
	/// Client support insert replace edit to control different behavior if a
	/// completion item is inserted in the text or should replace text.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_replace_support: Option<bool>,
	/// Indicates which properties a client can resolve lazily on a completion item.
	///
	/// Before version 3.16.0 only the predefined properties `documentation` and `details`
	/// could be resolved lazily.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_support: Option<StringProperties>,
	/// The client supports the `insertTextMode` property on a completion item to override the
	/// whitespace handling mode as defined by the client (see `insertTextMode`).
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none")]
	pub insert_text_mode_support: Option<ValueSet<InsertTextMode>>,
	/// The client has support for completion item label details (see also `CompletionItemLabelDetails`).
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub label_details_support: Option<bool>,
}

/// Completion client capabilities
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionClientCapabilities {
	/// Whether completion supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports the following [`CompletionItem`] specific capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub completion_item: Option<CompletionItemClientCapabilities>,
	/// The completion item kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside
	/// its set gracefully and falls back to a default value when unknown.
	///
	/// If this property is not present the client only supports the completion items kinds from
	/// `Text` to `Reference` as defined in the initial version of the protocol.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completion_item_kind: Option<ValueSet<CompletionItemKind>>,
	/// Defines how the client handles whitespace and indentation when accepting a completion item
	/// that uses multi line text in either `insertText` or `textEdit`.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text_mode: Option<InsertTextMode>,
	/// The client supports to send additional context information for a `textDocument/completion` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub context_support: Option<bool>,
	/// The client supports the following itemDefaults on a completion list.
	///
	/// The value lists the supported property names of the `CompletionList.itemDefaults` object.
	///
	/// If omitted no properties are supported.
	///
	/// since 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub completion_list: Option<CompletionListItemDefaults>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
	/// Whether hover supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Client supports the following content formats for the content property.
	///
	/// The order describes the preferred format of the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub content_format: Option<Vec<MarkupKind>>,
}

/// Client capabilities specific to parameter information.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ParameterInformationClientCapabilities {
	/// The client supports processing label offsets instead of a simple label string.
	///
	/// since: 3.14.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub label_offset_support: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInformationClientCapabilities {
	/// Client supports the following content formats for the documentation property.
	///
	/// The order describes the preferred format of the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub documentation_format: Option<Vec<MarkupKind>>,
	/// Client capabilities specific to parameter information.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub parameter_information: Option<ParameterInformationClientCapabilities>,
	/// The client supports the `activeParameter` property on `SignatureInformation` literal.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub active_parameter_support: Option<bool>,
}

/// Client Capabilities for a [`SignatureHelpRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpClientCapabilities {
	/// Whether signature help supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports the following `SignatureInformation` specific properties.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub signature_information: Option<SignatureInformationClientCapabilities>,
	/// The client supports to send additional context information for a `textDocument/signatureHelp`
	/// request.
	///
	/// A client that opts into contextSupport will also support the `retriggerCharacters` on
	/// `SignatureHelpOptions`.
	///
	/// since 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub context_support: Option<bool>,
}

/// since: 3.14.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationClientCapabilities {
	/// Whether declaration supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`DeclarationRegistrationOptions`]
	/// return value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports additional metadata in the form of declaration links.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub link_support: Option<bool>,
}

/// Client Capabilities for a [`DefinitionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionClientCapabilities {
	/// Whether definition supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports additional metadata in the form of definition links.
	///
	/// since: 3.14.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub link_support: Option<bool>,
}

/// since: 3.6.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`TypeDefinitionRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports additional metadata in the form of definition links.
	///
	/// since: 3.14.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub link_support: Option<bool>,
}

/// since: 3.6.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`ImplementationRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client supports additional metadata in the form of definition links.
	///
	/// since: 3.14.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub link_support: Option<bool>,
}

/// Client Capabilities for a [`ReferencesRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceClientCapabilities {
	/// Whether references supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client Capabilities for a [`DocumentHighlightRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightClientCapabilities {
	/// Whether document highlight supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client Capabilities for a [`CodeLensRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensClientCapabilities {
	/// Whether code lens supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client Capabilities for a [`DocumentLinkRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkClientCapabilities {
	/// Whether document link supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Whether the client supports the `tooltip` property on `DocumentLink`.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tooltip_support: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`DocumentColorRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client capabilities of a [`DocumentFormattingRequest`]
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingClientCapabilities {
	/// Whether formatting supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

/// Client capabilities of a [`DocumentRangeFormattingRequest`]
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingClientCapabilities {
	/// Whether range formatting supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Whether the client supports formatting multiple ranges at once.
	///
	/// since: 3.18.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub ranges_support: Option<bool>,
}

/// Client capabilities of a [`DocumentOnTypeFormattingRequest`]
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingClientCapabilities {
	/// Whether on type formatting supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeClientCapabilities {
	/// Whether implementation supports dynamic registration for selection range providers.
	///
	/// If this is set to `true` the client supports the new [`SelectionRangeRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolClientCapabilities {
	/// Whether document symbol supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The symbol kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside its set
	/// gracefully and falls back to a default value when unknown.
	///
	/// If this property is not present the client only supports the symbol kinds from `File` to `Array`
	/// as defined in the initial version of the protocol.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub symbol_kind: Option<ValueSet<SymbolKind>>,
	/// The client supports hierarchical document symbols.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub hierarchical_document_symbol_support: Option<bool>,
	/// The client supports tags on [`SymbolInformation`].
	///
	/// Tags are supported on [`DocumentSymbol`] if `hierarchicalDocumentSymbolSupport` is set to true.
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tag_support: Option<ValueSet<SymbolTag>>,
	/// The client supports an additional label presented in the UI when registering a document symbol provider.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub label_support: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameClientCapabilities {
	/// Whether rename supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Client supports testing for validity of rename operations before execution.
	///
	/// since: 3.12.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub prepare_support: Option<bool>,
	/// Client supports the default behavior result.
	///
	/// The value indicates the default behavior used by the client.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub prepare_support_default_behavior: Option<PrepareSupportDefaultBehavior>,
	/// Whether the client honors the change annotations in text edits and resource operations returned
	/// via the rename request's workspace edit by for example presenting the workspace edit in the
	/// user interface and asking\nfor confirmation.
	///
	/// since 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub honors_change_annotations: Option<bool>,
}

/// Specific options for the folding range.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeRangeClientCapabilities {
	/// If set, the client signals that it supports setting collapsedText on
	/// folding ranges to display custom labels instead of the default text.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub collapsed_text: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeClientCapabilities {
	/// Whether implementation supports dynamic registration for folding range providers.
	///
	/// If this is set to `true` the client supports the new [`FoldingRangeRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The maximum number of folding ranges that the client prefers to receive per document.
	///
	// The value serves as a hint, servers are free to follow the limit.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range_limit: Option<u32>,
	/// If set, the client signals that it only supports folding complete lines.
	///
	/// If set, client will ignore specified `startCharacter` and `endCharacter` properties in a
	/// [`FoldingRange`].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub line_folding_only: Option<bool>,
	/// The folding range kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside its
	/// set gracefully and falls back to a default value when unknown.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub folding_range_kind: Option<ValueSet<FoldingRangeKind>>,
	/// Specific options for the folding range.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub folding_range: Option<FoldingRangeRangeClientCapabilities>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionLiteralClientCapabilities {
	/// The code action kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside
	/// its set gracefully and falls back to a default value when unknown.
	pub code_action_kind: ValueSet<CodeActionKind>,
}

/// The Client Capabilities of a [`CodeActionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionClientCapabilities {
	/// Whether code action supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// The client support code action literals of type [`CodeAction`] as a valid response of the
	/// `textDocument/codeAction` request.
	///
	/// If the property is not set the request can only return [`Command`] literals.
	///
	/// since 3.8.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_action_literal_support: Option<CodeActionLiteralClientCapabilities>,
	/// Whether code action supports the `isPreferred` property.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub is_preferred_support: Option<bool>,
	/// Whether code action supports the `disabled` property.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub disabled_support: Option<bool>,
	/// Whether code action supports the `data` property which is preserved between a
	/// `textDocument/codeAction` and a `codeAction/resolve` request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data_support: Option<bool>,
	/// Whether the client supports resolving additional code action properties via a separate
	/// `codeAction/resolve` request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_support: Option<StringProperties>,
	/// Whether the client honors the change annotations in\ntext edits and resource operations returned via the\n`CodeAction#edit` property by for example presenting\nthe workspace edit in the user interface and asking\nfor confirmation.\n\n@since 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub honors_change_annotations: Option<bool>,
}

/// The publish diagnostic client capabilities.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PublishDiagnosticsClientCapabilities {
	/// Whether the clients accepts diagnostics with related information.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub related_information: Option<bool>,
	/// Client supports the tag property to provide meta data about a diagnostic.
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// since:s 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tag_support: Option<ValueSet<DiagnosticTag>>,
	/// Whether the client interprets the version property of the `textDocument/publishDiagnostics`
	/// notification's parameter.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version_support: Option<bool>,
	/// Client supports a codeDescription property.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_description_support: Option<bool>,
	/// Whether code action supports the `data` property which is preserved between a
	/// `textDocument/publishDiagnostics` and `textDocument/codeAction` request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data_support: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensRequestsClientCapabilities {
	/// The client will send the `textDocument/semanticTokens/range` request if
	/// the server provides a corresponding handler.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range: Option<bool>,
	/// The client will send the `textDocument/semanticTokens/full` request if
	/// the server provides a corresponding handler.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub full: Option<SemanticTokensFullRequestsType>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensClientCapabilities {
	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new
	/// `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return value for the
	/// corresponding server capability as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	/// Which requests the client supports and might send to the server depending on the server's capability.
	///
	/// Please note that clients might not show semantic tokens or degrade some of the user experience
	/// if a range or full request is advertised by the client but not provided by the server.
	///
	/// If for example the client capability `requests.full` and `request.range` are both set
	/// to true but the server only provides a range provider the client might not render a minimap
	/// correctly or might even decide to not show any semantic tokens at all.
	pub requests: SemanticTokensRequestsClientCapabilities,
	/// The token types that the client supports.
	pub token_types: Vec<String>,
	/// The token modifiers that the client supports.
	pub token_modifiers: Vec<String>,
	/// The token formats the clients supports.
	pub formats: Vec<TokenFormat>,
	/// Whether the client supports tokens that can overlap each other.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub overlapping_token_support: Option<bool>,
	/// Whether the client supports tokens that can span multiple lines.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub multiline_token_support: Option<bool>,
	/// Whether the client allows the server to actively cancel a semantic token request,
	/// e.g. supports returning [`crate::error::Code::ServerCancelled`].
	///
	/// If a server does the client needs to retrigger the request.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub server_cancel_support: Option<bool>,
	/// Whether the client uses semantic tokens to augment existing syntax tokens.
	///
	/// If set to `true` client side created syntax tokens and semantic tokens are both used for
	/// colorization.
	///
	/// If set to `false` the client only uses the returned semantic tokens for colorization.
	///
	/// If the value is `undefined` then the client behavior is not specified.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub augments_syntax_tokens: Option<bool>,
}
