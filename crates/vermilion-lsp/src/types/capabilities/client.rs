/* SPDX-License-Identifier: BSD-3-Clause */

use crate::types::{
	CodeActionKind, CompletionItemKind, CompletionItemTag, CompletionListItemDefaults,
	DiagnosticTag, FailureHandlingKind, FoldingRangeKind, InsertTextMode, LspAny, MarkupKind,
	PositionEncodingKind, PrepareSupportDefaultBehavior, ResourceOperationKind,
	SemanticTokensFullRequestsType, StringProperties, SymbolKind, SymbolTag, TokenFormat, ValueSet,
};

/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client capabilities for the linked editing range request.
///
/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client capabilities specific to the moniker request.
///
/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MonikerClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client capabilities specific to inline values.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintResolveSupport {
	pub(crate) properties: Vec<String>,
}

/// Inlay hint client capabilities.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) resolve_support: Option<InlayHintResolveSupport>,
}

/// Client capabilities specific to diagnostic pull requests.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) related_document_support: Option<bool>,
}

/// Client capabilities specific to inline completions.
///
/// since: 3.18.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Notebook specific client capabilities.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) execution_summary_support: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItemCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) additional_properties_supported: Option<bool>,
}

/// Show message request client capabilities
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessageRequestClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message_action_item: Option<MessageActionItemCapabilities>,
}

/// Client capabilities specific to the used markdown parser.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownClientCapabilities {
	pub(crate) parser: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) allowed_tags: Option<Vec<String>>,
}

/// Client capabilities specific to regular expressions.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RegularExpressionsClientCapabilities {
	pub(crate) engine: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version: Option<String>,
}

/// Client capabilities for the showDocument request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowDocumentClientCapabilities {
	pub(crate) support: bool,
}

/// Defines the capabilities provided by the client
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) workspace: Option<WorkspaceClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text_document: Option<TextDocumentClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) notebook_document: Option<NotebookDocumentClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) window: Option<WindowClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) general: Option<GeneralClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) experimental: Option<LspAny>,
}

/// Workspace specific client capabilities.
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) apply_edit: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) workspace_edit: Option<WorkspaceEditClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_change_watched_files: Option<DidChangeWatchedFilesClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) symbol: Option<WorkspaceSymbolClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) execute_command: Option<ExecuteCommandClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) workspace_folders: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) configuration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) semantic_tokens: Option<SemanticTokensWorkspaceClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_lense: Option<CodeLensWorkspaceClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) file_operations: Option<FileOperationClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inline_value: Option<InlineValueWorkspaceClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inlay_hint: Option<InlayHintWorkspaceClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) diagnostics: Option<DiagnosticWorkspaceClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) folder_range: Option<FoldingRangeWorkspaceClientCapabilities>,
}

/// Text document specific client capabilities.
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) synchronization: Option<TextDocumentSyncClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) completion: Option<CompletionClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hover: Option<HoverClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) signature_help: Option<SignatureHelpClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) declaration: Option<DeclarationClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) definition: Option<DefinitionClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) type_definition: Option<TypeDefinitionClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) implementation: Option<ImplementationClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) references: Option<ReferenceClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_highlight: Option<DocumentHighlightClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_symbol: Option<DocumentSymbolClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_action: Option<CodeActionClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_lens: Option<CodeLensClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_link: Option<DocumentLinkClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) color_provider: Option<DocumentColorClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) formatting: Option<DocumentFormattingClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) range_formatting: Option<DocumentRangeFormattingClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) on_type_formatting: Option<DocumentOnTypeFormattingClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) rename: Option<RenameClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) folding_range: Option<FoldingRangeClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) selection_range: Option<SelectionRangeClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) call_hierarchy: Option<CallHierarchyClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) semantic_tokens: Option<SemanticTokensClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) linked_editing_range: Option<LinkedEditingRangeClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) moniker: Option<MonikerClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) type_hierarchy: Option<TypeHierarchyClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inline_value: Option<InlineValueClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inlay_hint: Option<InlayHintClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) diagnostic: Option<DiagnosticClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inline_completion: Option<InlineCompletionClientCapabilities>,
}

/// Capabilities specific to the notebook document support.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentClientCapabilities {
	pub(crate) synchronization: NotebookDocumentSyncClientCapabilities,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WindowClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) work_done_progress: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) show_message: Option<ShowMessageRequestClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) show_document: Option<ShowDocumentClientCapabilities>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StaleRequestClientCapabilities {
	pub(crate) cancel: bool,
	pub(crate) retry_on_content_modified: Vec<String>,
}

/// General client capabilities.
///
/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GeneralClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) stale_request_support: Option<StaleRequestClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) regular_expressions: Option<RegularExpressionsClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) markdown: Option<MarkdownClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) position_encodings: Option<Vec<PositionEncodingKind>>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceChangeAnnotationSupportClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) groups_on_label: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_changes: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) resource_operations: Option<Vec<ResourceOperationKind>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) failure_handling: Option<FailureHandlingKind>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) normalizes_line_endings: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) change_annotation_support:
		Option<WorkspaceChangeAnnotationSupportClientCapabilities>,
}
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub relative_pattern_support: Option<bool>,
}

/// Client capabilities for a [`WorkspaceSymbolRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) symbol_kind: Option<ValueSet<SymbolKind>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) tag_support: Option<ValueSet<SymbolTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) resolve_support: Option<StringProperties>,
}

/// Client capabilities for a [`ExecuteCommandRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensWorkspaceClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) refresh_support: Option<bool>,
}

/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensWorkspaceClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) refresh_support: Option<bool>,
}

/// Capabilities relating to events from file operations by the user in the client.
///
/// These events do not come from the file system, they come from user operations
/// like renaming a file in the UI.
///
/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_create: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) will_create: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_rename: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) will_rename: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_delete: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) will_delete: Option<bool>,
}

/// Client workspace capabilities specific to inline values.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueWorkspaceClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) refresh_support: Option<bool>,
}

/// Client workspace capabilities specific to inlay hints.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintWorkspaceClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) refresh_support: Option<bool>,
}

/// Workspace client capabilities specific to diagnostic pull requests.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticWorkspaceClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) refresh_support: Option<bool>,
}

/// Client workspace capabilities specific to folding ranges
///
/// since: 3.18.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeWorkspaceClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) refresh_support: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) will_save: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) will_save_wait_until: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_save: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) snippet_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) commit_characters_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) documentation_format: Option<Vec<MarkupKind>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) deprecated_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) preselect_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) tag_support: Option<ValueSet<CompletionItemTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) insert_replace_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) resolve_support: Option<StringProperties>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) insert_text_mode_support: Option<ValueSet<InsertTextMode>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) label_details_support: Option<bool>,
}

/// Completion client capabilities
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) completion_item: Option<CompletionItemClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) completion_item_kind: Option<ValueSet<CompletionItemKind>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) insert_text_mode: Option<InsertTextMode>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) context_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) completion_list: Option<CompletionListItemDefaults>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) content_format: Option<Vec<MarkupKind>>,
}

/// Client capabilities specific to parameter information.
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ParameterInformationClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) label_offset_support: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInformationClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) documentation_format: Option<Vec<MarkupKind>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) parameter_information: Option<ParameterInformationClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) active_parameter_support: Option<bool>,
}

/// Client Capabilities for a [`SignatureHelpRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) signature_information: Option<SignatureInformationClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) context_support: Option<bool>,
}

/// since: 3.14.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) link_support: Option<bool>,
}

/// Client Capabilities for a [`DefinitionRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) link_support: Option<bool>,
}

/// since: 3.6.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) link_support: Option<bool>,
}

/// since: 3.6.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) link_support: Option<bool>,
}

/// Client Capabilities for a [`ReferencesRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceClientCapabilities {
	/// Whether references supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client Capabilities for a [`DocumentHighlightRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightClientCapabilities {
	/// Whether document highlight supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client Capabilities for a [`CodeLensRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client Capabilities for a [`DocumentLinkRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tooltip_support: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client capabilities of a [`DocumentFormattingRequest`]
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

/// Client capabilities of a [`DocumentRangeFormattingRequest`]
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) ranges_support: Option<bool>,
}

/// Client capabilities of a [`DocumentOnTypeFormattingRequest`]
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) symbol_kind: Option<ValueSet<SymbolKind>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hierarchical_document_symbol_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tag_support: Option<ValueSet<SymbolTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) label_support: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) prepare_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) prepare_support_default_behavior: Option<PrepareSupportDefaultBehavior>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) honors_change_annotations: Option<bool>,
}

/// Specific options for the folding range.
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeRangeClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) collapsed_text: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) range_limit: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line_folding_only: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) folding_range_kind: Option<ValueSet<FoldingRangeKind>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) folding_range: Option<FoldingRangeRangeClientCapabilities>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionLiteralClientCapabilities {
	pub(crate) code_action_kind: ValueSet<CodeActionKind>,
}

/// The Client Capabilities of a [`CodeActionRequest`].
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_action_literal_support: Option<CodeActionLiteralClientCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) is_preferred_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) disabled_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) resolve_support: Option<StringProperties>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) honors_change_annotations: Option<bool>,
}

/// The publish diagnostic client capabilities.
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PublishDiagnosticsClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) related_information: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tag_support: Option<ValueSet<DiagnosticTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_description_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data_support: Option<bool>,
}

#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensRequestsClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) range: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) full: Option<SemanticTokensFullRequestsType>,
}

/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Default,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde::Deserialize,
	serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensClientCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) dynamic_registration: Option<bool>,
	pub(crate) requests: SemanticTokensRequestsClientCapabilities,
	pub(crate) token_types: Vec<String>,
	pub(crate) token_modifiers: Vec<String>,
	pub(crate) formats: Vec<TokenFormat>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) overlapping_token_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) multiline_token_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) server_cancel_support: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) augments_syntax_tokens: Option<bool>,
}

impl CallHierarchyClientCapabilities {
	pub const fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl LinkedEditingRangeClientCapabilities {
	pub const fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl MonikerClientCapabilities {
	pub const fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether moniker supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `MonikerRegistrationOptions` return value
	/// for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl TypeHierarchyClientCapabilities {
	pub const fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl InlineValueClientCapabilities {
	pub const fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether implementation supports dynamic registration for inline value providers.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl InlayHintResolveSupport {
	pub fn new() -> Self {
		Self { properties: Vec::new() }
	}

	pub fn with_properties(mut self, properties: &[String]) -> Self {
		self.properties.extend_from_slice(properties);
		self
	}

	/// The properties that a client can resolve lazily.
	pub fn properties(&mut self) -> &mut Vec<String> {
		&mut self.properties
	}
}

impl InlayHintClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, resolve_support: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_resolve_support(mut self, resolve_support: InlayHintResolveSupport) -> Self {
		self.resolve_support = Some(resolve_support);
		self
	}

	/// Whether inlay hints support dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Indicates which properties a client can resolve lazily on an inlay hint..
	pub fn resolve_support(&self) -> Option<&InlayHintResolveSupport> {
		self.resolve_support.as_ref()
	}
}

impl DiagnosticClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			related_document_support: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_related_document_support(mut self, related_document_support: bool) -> Self {
		self.related_document_support = Some(related_document_support);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Whether the clients supports related documents for document diagnostic pulls.
	pub fn related_document_support(&self) -> Option<&bool> {
		self.related_document_support.as_ref()
	}
}

impl InlineCompletionClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether implementation supports dynamic registration for inline completion providers.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl NotebookDocumentSyncClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			execution_summary_support: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_execution_summary_support(mut self, execution_summary_support: bool) -> Self {
		self.execution_summary_support = Some(execution_summary_support);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
	/// return value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports sending execution summary data per cell.
	pub fn execution_summary_support(&self) -> Option<&bool> {
		self.execution_summary_support.as_ref()
	}
}

impl MessageActionItemCapabilities {
	pub fn new() -> Self {
		Self { additional_properties_supported: None }
	}

	pub fn with_additional_properties_supported(
		mut self,
		additional_properties_supported: bool,
	) -> Self {
		self.additional_properties_supported = Some(additional_properties_supported);
		self
	}

	/// Whether the client supports additional attributes which are preserved
	/// and send back to the server in the request's response.
	pub fn additional_properties_supported(&self) -> Option<&bool> {
		self.additional_properties_supported.as_ref()
	}
}

impl ShowMessageRequestClientCapabilities {
	pub fn new() -> Self {
		Self { message_action_item: None }
	}

	pub fn with_additional_properties_supported(
		mut self,
		message_action_item: MessageActionItemCapabilities,
	) -> Self {
		self.message_action_item = Some(message_action_item);
		self
	}

	/// Capabilities specific to the [`MessageActionItem`] type.
	pub fn message_action_item(&self) -> Option<&MessageActionItemCapabilities> {
		self.message_action_item.as_ref()
	}
}

impl MarkdownClientCapabilities {
	pub fn new(parser: String) -> Self {
		Self { parser, version: None, allowed_tags: None }
	}

	pub fn with_version(mut self, version: String) -> Self {
		self.version = Some(version);
		self
	}

	pub fn with_allowed_tags(mut self, allowed_tags: Vec<String>) -> Self {
		self.allowed_tags = Some(allowed_tags);
		self
	}

	/// The name of the parser.
	pub fn parser(&self) -> &String {
		&self.parser
	}

	/// The version of the parser.
	pub fn version(&self) -> Option<&String> {
		self.version.as_ref()
	}

	/// A list of HTML tags that the client allows/supports in Markdown.
	///
	/// since: 3.17.0
	pub fn allowed_tags(&self) -> Option<&Vec<String>> {
		self.allowed_tags.as_ref()
	}
}

impl RegularExpressionsClientCapabilities {
	pub fn new(engine: String) -> Self {
		Self { engine, version: None }
	}

	pub fn with_version(mut self, version: String) -> Self {
		self.version = Some(version);
		self
	}

	/// The engine's name.
	pub fn engine(&self) -> &String {
		&self.engine
	}

	/// The engine's version.
	pub fn version(&self) -> Option<&String> {
		self.version.as_ref()
	}
}

impl ShowDocumentClientCapabilities {
	pub fn new(support: bool) -> Self {
		Self { support }
	}

	/// The client has support for the showDocument request.
	pub fn support(&self) -> &bool {
		&self.support
	}
}

impl ClientCapabilities {
	pub fn new() -> Self {
		Self {
			workspace: None,
			text_document: None,
			notebook_document: None,
			window: None,
			general: None,
			experimental: None,
		}
	}

	pub fn with_workspace(mut self, workspace: WorkspaceClientCapabilities) -> Self {
		self.workspace = Some(workspace);
		self
	}

	pub fn with_text_document(mut self, text_document: TextDocumentClientCapabilities) -> Self {
		self.text_document = Some(text_document);
		self
	}

	pub fn with_notebook_document(
		mut self,
		notebook_document: NotebookDocumentClientCapabilities,
	) -> Self {
		self.notebook_document = Some(notebook_document);
		self
	}

	pub fn with_window(mut self, window: WindowClientCapabilities) -> Self {
		self.window = Some(window);
		self
	}

	pub fn with_general(mut self, general: GeneralClientCapabilities) -> Self {
		self.general = Some(general);
		self
	}

	pub fn with_experimental(mut self, experimental: LspAny) -> Self {
		self.experimental = Some(experimental);
		self
	}

	/// Workspace specific client capabilities.
	pub fn workspace(&self) -> Option<&WorkspaceClientCapabilities> {
		self.workspace.as_ref()
	}

	/// Text document specific client capabilities.
	pub fn text_document(&self) -> Option<&TextDocumentClientCapabilities> {
		self.text_document.as_ref()
	}

	/// Capabilities specific to the notebook document support.
	///
	/// since: 3.17.0
	pub fn notebook_document(&self) -> Option<&NotebookDocumentClientCapabilities> {
		self.notebook_document.as_ref()
	}

	/// Window specific client capabilities.
	pub fn window(&self) -> Option<&WindowClientCapabilities> {
		self.window.as_ref()
	}

	/// General client capabilities.
	///
	/// since: 3.16.0
	pub fn general(&self) -> Option<&GeneralClientCapabilities> {
		self.general.as_ref()
	}

	/// Experimental client capabilities.
	pub fn experimental(&self) -> Option<&LspAny> {
		self.experimental.as_ref()
	}
}

impl WorkspaceClientCapabilities {
	pub fn new() -> Self {
		Self {
			apply_edit: None,
			workspace_edit: None,
			did_change_configuration: None,
			did_change_watched_files: None,
			symbol: None,
			execute_command: None,
			workspace_folders: None,
			configuration: None,
			semantic_tokens: None,
			code_lense: None,
			file_operations: None,
			inline_value: None,
			inlay_hint: None,
			diagnostics: None,
			folder_range: None,
		}
	}

	pub fn with_apply_edit(mut self, apply_edit: bool) -> Self {
		self.apply_edit = Some(apply_edit);
		self
	}

	pub fn with_workspace_edit(mut self, workspace_edit: WorkspaceEditClientCapabilities) -> Self {
		self.workspace_edit = Some(workspace_edit);
		self
	}

	pub fn with_did_change_configuration(
		mut self,
		did_change_configuration: DidChangeConfigurationClientCapabilities,
	) -> Self {
		self.did_change_configuration = Some(did_change_configuration);
		self
	}

	pub fn with_did_change_watched_files(
		mut self,
		did_change_watched_files: DidChangeWatchedFilesClientCapabilities,
	) -> Self {
		self.did_change_watched_files = Some(did_change_watched_files);
		self
	}

	pub fn with_symbol(mut self, symbol: WorkspaceSymbolClientCapabilities) -> Self {
		self.symbol = Some(symbol);
		self
	}

	pub fn with_execute_command(
		mut self,
		execute_command: ExecuteCommandClientCapabilities,
	) -> Self {
		self.execute_command = Some(execute_command);
		self
	}

	pub fn with_workspace_folders(mut self, workspace_folders: bool) -> Self {
		self.workspace_folders = Some(workspace_folders);
		self
	}

	pub fn with_configuration(mut self, configuration: bool) -> Self {
		self.configuration = Some(configuration);
		self
	}

	pub fn with_semantic_tokens(
		mut self,
		semantic_tokens: SemanticTokensWorkspaceClientCapabilities,
	) -> Self {
		self.semantic_tokens = Some(semantic_tokens);
		self
	}

	pub fn with_code_lense(mut self, code_lense: CodeLensWorkspaceClientCapabilities) -> Self {
		self.code_lense = Some(code_lense);
		self
	}

	pub fn with_file_operations(
		mut self,
		file_operations: FileOperationClientCapabilities,
	) -> Self {
		self.file_operations = Some(file_operations);
		self
	}

	pub fn with_inline_value(
		mut self,
		inline_value: InlineValueWorkspaceClientCapabilities,
	) -> Self {
		self.inline_value = Some(inline_value);
		self
	}

	pub fn with_inlay_hint(mut self, inlay_hint: InlayHintWorkspaceClientCapabilities) -> Self {
		self.inlay_hint = Some(inlay_hint);
		self
	}

	pub fn with_diagnostics(mut self, diagnostics: DiagnosticWorkspaceClientCapabilities) -> Self {
		self.diagnostics = Some(diagnostics);
		self
	}

	pub fn with_folder_range(
		mut self,
		folder_range: FoldingRangeWorkspaceClientCapabilities,
	) -> Self {
		self.folder_range = Some(folder_range);
		self
	}

	/// The client supports applying batch edits to the workspace by supporting the request
	/// 'workspace/applyEdit'
	pub fn apply_edit(&self) -> Option<&bool> {
		self.apply_edit.as_ref()
	}

	/// Capabilities specific to [`WorkspaceEdit`]s.
	pub fn workspace_edit(&self) -> Option<&WorkspaceEditClientCapabilities> {
		self.workspace_edit.as_ref()
	}

	/// Capabilities specific to the `workspace/didChangeConfiguration` notification.
	pub fn did_change_configuration(&self) -> Option<&DidChangeConfigurationClientCapabilities> {
		self.did_change_configuration.as_ref()
	}

	/// Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
	pub fn did_change_watched_files(&self) -> Option<&DidChangeWatchedFilesClientCapabilities> {
		self.did_change_watched_files.as_ref()
	}

	/// Capabilities specific to the `workspace/symbol` request.
	pub fn symbol(&self) -> Option<&WorkspaceSymbolClientCapabilities> {
		self.symbol.as_ref()
	}

	/// Capabilities specific to the `workspace/executeCommand` request.
	pub fn execute_command(&self) -> Option<&ExecuteCommandClientCapabilities> {
		self.execute_command.as_ref()
	}

	/// The client has support for workspace folders.
	///
	/// since: 3.6.0
	pub fn workspace_folders(&self) -> Option<&bool> {
		self.workspace_folders.as_ref()
	}

	/// The client supports `workspace/configuration` requests.
	///
	/// since 3.6.0
	pub fn configuration(&self) -> Option<&bool> {
		self.configuration.as_ref()
	}

	/// Capabilities specific to the semantic token requests scoped to the workspace.
	///
	/// since: 3.16.0.
	pub fn semantic_tokens(&self) -> Option<&SemanticTokensWorkspaceClientCapabilities> {
		self.semantic_tokens.as_ref()
	}

	/// Capabilities specific to the code lens requests scoped to the workspace.
	///
	/// since: 3.16.0.
	pub fn code_lense(&self) -> Option<&CodeLensWorkspaceClientCapabilities> {
		self.code_lense.as_ref()
	}

	/// The client has support for file notifications/requests for user operations on files.
	///
	/// since: 3.16.0
	pub fn file_operations(&self) -> Option<&FileOperationClientCapabilities> {
		self.file_operations.as_ref()
	}

	/// Capabilities specific to the inline values requests scoped to the workspace.
	///
	/// since: 3.17.0.
	pub fn inline_value(&self) -> Option<&InlineValueWorkspaceClientCapabilities> {
		self.inline_value.as_ref()
	}

	/// Capabilities specific to the inlay hint requests scoped to the workspace.
	///
	/// since: 3.17.0.
	pub fn inlay_hint(&self) -> Option<&InlayHintWorkspaceClientCapabilities> {
		self.inlay_hint.as_ref()
	}

	/// Capabilities specific to the diagnostic requests scoped to the workspace.
	///
	/// since: 3.17.0.
	pub fn diagnostics(&self) -> Option<&DiagnosticWorkspaceClientCapabilities> {
		self.diagnostics.as_ref()
	}

	/// Capabilities specific to the folding range requests scoped to the workspace.
	///
	/// since: 3.18.0
	pub fn folder_range(&self) -> Option<&FoldingRangeWorkspaceClientCapabilities> {
		self.folder_range.as_ref()
	}
}

impl TextDocumentClientCapabilities {
	pub fn new() -> Self {
		Self {
			synchronization: None,
			completion: None,
			hover: None,
			signature_help: None,
			declaration: None,
			definition: None,
			type_definition: None,
			implementation: None,
			references: None,
			document_highlight: None,
			document_symbol: None,
			code_action: None,
			code_lens: None,
			document_link: None,
			color_provider: None,
			formatting: None,
			range_formatting: None,
			on_type_formatting: None,
			rename: None,
			folding_range: None,
			selection_range: None,
			publish_diagnostics: None,
			call_hierarchy: None,
			semantic_tokens: None,
			linked_editing_range: None,
			moniker: None,
			type_hierarchy: None,
			inline_value: None,
			inlay_hint: None,
			diagnostic: None,
			inline_completion: None,
		}
	}

	pub fn with_synchronization(
		mut self,
		synchronization: TextDocumentSyncClientCapabilities,
	) -> Self {
		self.synchronization = Some(synchronization);
		self
	}

	pub fn with_completion(mut self, completion: CompletionClientCapabilities) -> Self {
		self.completion = Some(completion);
		self
	}

	pub fn with_hover(mut self, hover: HoverClientCapabilities) -> Self {
		self.hover = Some(hover);
		self
	}

	pub fn with_signature_help(mut self, signature_help: SignatureHelpClientCapabilities) -> Self {
		self.signature_help = Some(signature_help);
		self
	}

	pub fn with_declaration(mut self, declaration: DeclarationClientCapabilities) -> Self {
		self.declaration = Some(declaration);
		self
	}

	pub fn with_definition(mut self, definition: DefinitionClientCapabilities) -> Self {
		self.definition = Some(definition);
		self
	}

	pub fn with_type_definition(
		mut self,
		type_definition: TypeDefinitionClientCapabilities,
	) -> Self {
		self.type_definition = Some(type_definition);
		self
	}

	pub fn with_implementation(mut self, implementation: ImplementationClientCapabilities) -> Self {
		self.implementation = Some(implementation);
		self
	}

	pub fn with_references(mut self, references: ReferenceClientCapabilities) -> Self {
		self.references = Some(references);
		self
	}

	pub fn with_document_highlight(
		mut self,
		document_highlight: DocumentHighlightClientCapabilities,
	) -> Self {
		self.document_highlight = Some(document_highlight);
		self
	}

	pub fn with_document_symbol(
		mut self,
		document_symbol: DocumentSymbolClientCapabilities,
	) -> Self {
		self.document_symbol = Some(document_symbol);
		self
	}

	pub fn with_code_action(mut self, code_action: CodeActionClientCapabilities) -> Self {
		self.code_action = Some(code_action);
		self
	}

	pub fn with_code_lens(mut self, code_lens: CodeLensClientCapabilities) -> Self {
		self.code_lens = Some(code_lens);
		self
	}

	pub fn with_document_link(mut self, document_link: DocumentLinkClientCapabilities) -> Self {
		self.document_link = Some(document_link);
		self
	}

	pub fn with_color_provider(mut self, color_provider: DocumentColorClientCapabilities) -> Self {
		self.color_provider = Some(color_provider);
		self
	}

	pub fn with_formatting(mut self, formatting: DocumentFormattingClientCapabilities) -> Self {
		self.formatting = Some(formatting);
		self
	}

	pub fn with_range_formatting(
		mut self,
		range_formatting: DocumentRangeFormattingClientCapabilities,
	) -> Self {
		self.range_formatting = Some(range_formatting);
		self
	}

	pub fn with_on_type_formatting(
		mut self,
		on_type_formatting: DocumentOnTypeFormattingClientCapabilities,
	) -> Self {
		self.on_type_formatting = Some(on_type_formatting);
		self
	}

	pub fn with_rename(mut self, rename: RenameClientCapabilities) -> Self {
		self.rename = Some(rename);
		self
	}

	pub fn with_folding_range(mut self, folding_range: FoldingRangeClientCapabilities) -> Self {
		self.folding_range = Some(folding_range);
		self
	}

	pub fn with_selection_range(
		mut self,
		selection_range: SelectionRangeClientCapabilities,
	) -> Self {
		self.selection_range = Some(selection_range);
		self
	}

	pub fn with_publish_diagnostics(
		mut self,
		publish_diagnostics: PublishDiagnosticsClientCapabilities,
	) -> Self {
		self.publish_diagnostics = Some(publish_diagnostics);
		self
	}

	pub fn with_call_hierarchy(mut self, call_hierarchy: CallHierarchyClientCapabilities) -> Self {
		self.call_hierarchy = Some(call_hierarchy);
		self
	}

	pub fn with_semantic_tokens(
		mut self,
		semantic_tokens: SemanticTokensClientCapabilities,
	) -> Self {
		self.semantic_tokens = Some(semantic_tokens);
		self
	}

	pub fn with_linked_editing_range(
		mut self,
		linked_editing_range: LinkedEditingRangeClientCapabilities,
	) -> Self {
		self.linked_editing_range = Some(linked_editing_range);
		self
	}

	pub fn with_moniker(mut self, moniker: MonikerClientCapabilities) -> Self {
		self.moniker = Some(moniker);
		self
	}

	pub fn with_type_hierarchy(mut self, type_hierarchy: TypeHierarchyClientCapabilities) -> Self {
		self.type_hierarchy = Some(type_hierarchy);
		self
	}

	pub fn with_inline_value(mut self, inline_value: InlineValueClientCapabilities) -> Self {
		self.inline_value = Some(inline_value);
		self
	}

	pub fn with_inlay_hint(mut self, inlay_hint: InlayHintClientCapabilities) -> Self {
		self.inlay_hint = Some(inlay_hint);
		self
	}

	pub fn with_diagnostic(mut self, diagnostic: DiagnosticClientCapabilities) -> Self {
		self.diagnostic = Some(diagnostic);
		self
	}

	pub fn with_inline_completion(
		mut self,
		inline_completion: InlineCompletionClientCapabilities,
	) -> Self {
		self.inline_completion = Some(inline_completion);
		self
	}

	/// Defines which synchronization capabilities the client supports.
	pub fn synchronization(&self) -> Option<&TextDocumentSyncClientCapabilities> {
		self.synchronization.as_ref()
	}

	/// Capabilities specific to the `textDocument/completion` request.
	pub fn completion(&self) -> Option<&CompletionClientCapabilities> {
		self.completion.as_ref()
	}

	/// Capabilities specific to the `textDocument/hover` request.
	pub fn hover(&self) -> Option<&HoverClientCapabilities> {
		self.hover.as_ref()
	}

	/// Capabilities specific to the `textDocument/signatureHelp` request.
	pub fn signature_help(&self) -> Option<&SignatureHelpClientCapabilities> {
		self.signature_help.as_ref()
	}

	/// Capabilities specific to the `textDocument/declaration` request.
	///
	/// since: 3.14.0
	pub fn declaration(&self) -> Option<&DeclarationClientCapabilities> {
		self.declaration.as_ref()
	}

	/// Capabilities specific to the `textDocument/definition` request.
	pub fn definition(&self) -> Option<&DefinitionClientCapabilities> {
		self.definition.as_ref()
	}

	/// Capabilities specific to the `textDocument/typeDefinition` request.
	///
	/// since: 3.6.0
	pub fn type_definition(&self) -> Option<&TypeDefinitionClientCapabilities> {
		self.type_definition.as_ref()
	}

	/// Capabilities specific to the `textDocument/implementation` request.
	///
	/// since: 3.6.0
	pub fn implementation(&self) -> Option<&ImplementationClientCapabilities> {
		self.implementation.as_ref()
	}

	/// Capabilities specific to the `textDocument/references` request.
	pub fn references(&self) -> Option<&ReferenceClientCapabilities> {
		self.references.as_ref()
	}

	/// Capabilities specific to the `textDocument/documentHighlight` request.
	pub fn document_highlight(&self) -> Option<&DocumentHighlightClientCapabilities> {
		self.document_highlight.as_ref()
	}

	/// Capabilities specific to the `textDocument/documentSymbol` request.
	pub fn document_symbol(&self) -> Option<&DocumentSymbolClientCapabilities> {
		self.document_symbol.as_ref()
	}

	/// Capabilities specific to the `textDocument/codeAction` request.
	pub fn code_action(&self) -> Option<&CodeActionClientCapabilities> {
		self.code_action.as_ref()
	}

	/// Capabilities specific to the `textDocument/codeLens` request.
	pub fn code_lens(&self) -> Option<&CodeLensClientCapabilities> {
		self.code_lens.as_ref()
	}

	/// Capabilities specific to the `textDocument/documentLink` request.
	pub fn document_link(&self) -> Option<&DocumentLinkClientCapabilities> {
		self.document_link.as_ref()
	}

	/// Capabilities specific to the `textDocument/documentColor` and the `textDocument/colorPresentation` request.
	///
	/// since: 3.6.0
	pub fn color_provider(&self) -> Option<&DocumentColorClientCapabilities> {
		self.color_provider.as_ref()
	}

	/// Capabilities specific to the `textDocument/formatting` request.
	pub fn formatting(&self) -> Option<&DocumentFormattingClientCapabilities> {
		self.formatting.as_ref()
	}

	/// Capabilities specific to the `textDocument/rangeFormatting` request.
	pub fn range_formatting(&self) -> Option<&DocumentRangeFormattingClientCapabilities> {
		self.range_formatting.as_ref()
	}

	/// Capabilities specific to the `textDocument/onTypeFormatting` request.
	pub fn on_type_formatting(&self) -> Option<&DocumentOnTypeFormattingClientCapabilities> {
		self.on_type_formatting.as_ref()
	}

	/// Capabilities specific to the `textDocument/rename` request.
	pub fn rename(&self) -> Option<&RenameClientCapabilities> {
		self.rename.as_ref()
	}

	/// Capabilities specific to the `textDocument/foldingRange` request.
	///
	/// since: 3.10.0
	pub fn folding_range(&self) -> Option<&FoldingRangeClientCapabilities> {
		self.folding_range.as_ref()
	}

	/// Capabilities specific to the `textDocument/selectionRange` request.
	///
	/// since: 3.15.0
	pub fn selection_range(&self) -> Option<&SelectionRangeClientCapabilities> {
		self.selection_range.as_ref()
	}

	/// Capabilities specific to the `textDocument/publishDiagnostics` notification.
	pub fn publish_diagnostics(&self) -> Option<&PublishDiagnosticsClientCapabilities> {
		self.publish_diagnostics.as_ref()
	}

	/// Capabilities specific to the various call hierarchy requests.
	///
	/// since: 3.16.0
	pub fn call_hierarchy(&self) -> Option<&CallHierarchyClientCapabilities> {
		self.call_hierarchy.as_ref()
	}

	/// Capabilities specific to the various semantic token request.
	///
	/// since: 3.16.0
	pub fn semantic_tokens(&self) -> Option<&SemanticTokensClientCapabilities> {
		self.semantic_tokens.as_ref()
	}

	/// Capabilities specific to the `textDocument/linkedEditingRange` request.
	///
	/// since: 3.16.0
	pub fn linked_editing_range(&self) -> Option<&LinkedEditingRangeClientCapabilities> {
		self.linked_editing_range.as_ref()
	}

	/// Client capabilities specific to the `textDocument/moniker` request.
	///
	/// since: 3.16.0
	pub fn moniker(&self) -> Option<&MonikerClientCapabilities> {
		self.moniker.as_ref()
	}

	/// Capabilities specific to the various type hierarchy requests.
	///
	/// since: 3.17.0
	pub fn type_hierarchy(&self) -> Option<&TypeHierarchyClientCapabilities> {
		self.type_hierarchy.as_ref()
	}

	/// Capabilities specific to the `textDocument/inlineValue` request.
	///
	/// since: 3.17.0
	pub fn inline_value(&self) -> Option<&InlineValueClientCapabilities> {
		self.inline_value.as_ref()
	}

	/// Capabilities specific to the `textDocument/inlayHint` request.
	///
	/// since: 3.17.0
	pub fn inlay_hint(&self) -> Option<&InlayHintClientCapabilities> {
		self.inlay_hint.as_ref()
	}

	/// Capabilities specific to the diagnostic pull model.
	///
	/// since: 3.17.0
	pub fn diagnostic(&self) -> Option<&DiagnosticClientCapabilities> {
		self.diagnostic.as_ref()
	}

	/// Client capabilities specific to inline completions.
	///
	/// since: 3.18.0
	pub fn inline_completion(&self) -> Option<&InlineCompletionClientCapabilities> {
		self.inline_completion.as_ref()
	}
}

impl NotebookDocumentClientCapabilities {
	pub fn new(synchronization: NotebookDocumentSyncClientCapabilities) -> Self {
		Self { synchronization }
	}

	/// Capabilities specific to notebook document synchronization
	///
	/// since: 3.17.0
	pub fn synchronization(&self) -> &NotebookDocumentSyncClientCapabilities {
		&self.synchronization
	}
}

impl WindowClientCapabilities {
	pub fn new() -> Self {
		Self {
			work_done_progress: None,
			show_message: None,
			show_document: None,
		}
	}

	pub fn with_work_done_progress(mut self, work_done_progress: bool) -> Self {
		self.work_done_progress = Some(work_done_progress);
		self
	}

	pub fn with_show_message(mut self, show_message: ShowMessageRequestClientCapabilities) -> Self {
		self.show_message = Some(show_message);
		self
	}

	pub fn with_show_document(mut self, show_document: ShowDocumentClientCapabilities) -> Self {
		self.show_document = Some(show_document);
		self
	}

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
	pub fn work_done_progress(&self) -> Option<&bool> {
		self.work_done_progress.as_ref()
	}

	/// Capabilities specific to the showMessage request.
	///
	/// since: 3.16.0
	pub fn show_message(&self) -> Option<&ShowMessageRequestClientCapabilities> {
		self.show_message.as_ref()
	}

	/// Capabilities specific to the showDocument request.
	///
	/// since: 3.16.0
	pub fn show_document(&self) -> Option<&ShowDocumentClientCapabilities> {
		self.show_document.as_ref()
	}
}

impl StaleRequestClientCapabilities {
	pub fn new(cancel: bool, retry_on_content_modified: Vec<String>) -> Self {
		Self { cancel, retry_on_content_modified }
	}

	/// The client will actively cancel the request.
	pub fn cancel(&self) -> &bool {
		&self.cancel
	}

	/// The list of requests for which the client will retry the request if it receives a
	/// response with error code [`crate::error::Code::ContentModified`]
	pub fn retry_on_content_modified(&self) -> &Vec<String> {
		&self.retry_on_content_modified
	}
}

impl GeneralClientCapabilities {
	pub fn new() -> Self {
		Self {
			stale_request_support: None,
			regular_expressions: None,
			markdown: None,
			position_encodings: None,
		}
	}

	pub fn with_stale_request_support(
		mut self,
		stale_request_support: StaleRequestClientCapabilities,
	) -> Self {
		self.stale_request_support = Some(stale_request_support);
		self
	}

	pub fn with_regular_expressions(
		mut self,
		regular_expressions: RegularExpressionsClientCapabilities,
	) -> Self {
		self.regular_expressions = Some(regular_expressions);
		self
	}

	pub fn with_markdown(mut self, markdown: MarkdownClientCapabilities) -> Self {
		self.markdown = Some(markdown);
		self
	}

	pub fn with_position_encodings(
		mut self,
		position_encodings: Vec<PositionEncodingKind>,
	) -> Self {
		self.position_encodings = Some(position_encodings);
		self
	}

	/// Client capability that signals how the client handles stale requests (e.g. a request
	/// for which the client will not process the response anymore since the information is outdated).
	///
	/// since: 3.17.0
	pub fn stale_request_support(&self) -> Option<&StaleRequestClientCapabilities> {
		self.stale_request_support.as_ref()
	}

	/// Client capabilities specific to regular expressions.
	///
	/// since: 3.16.0
	pub fn regular_expressions(&self) -> Option<&RegularExpressionsClientCapabilities> {
		self.regular_expressions.as_ref()
	}

	/// Client capabilities specific to the client's markdown parser.
	///
	/// since: 3.16.0
	pub fn markdown(&self) -> Option<&MarkdownClientCapabilities> {
		self.markdown.as_ref()
	}

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
	pub fn position_encodings(&self) -> Option<&Vec<PositionEncodingKind>> {
		self.position_encodings.as_ref()
	}
}

impl WorkspaceChangeAnnotationSupportClientCapabilities {
	pub fn new() -> Self {
		Self { groups_on_label: None }
	}

	pub fn with_groups_on_label(mut self, groups_on_label: bool) -> Self {
		self.groups_on_label = Some(groups_on_label);
		self
	}

	/// Whether the client groups edits with equal labels into tree nodes, for instance all edits
	/// labelled with "Changes in Strings" would be a tree node.
	pub fn groups_on_label(&self) -> Option<&bool> {
		self.groups_on_label.as_ref()
	}
}

impl WorkspaceEditClientCapabilities {
	pub fn new() -> Self {
		Self {
			document_changes: None,
			resource_operations: None,
			failure_handling: None,
			normalizes_line_endings: None,
			change_annotation_support: None,
		}
	}

	pub fn with_document_changes(mut self, document_changes: bool) -> Self {
		self.document_changes = Some(document_changes);
		self
	}

	pub fn with_resource_operations(
		mut self,
		resource_operations: Vec<ResourceOperationKind>,
	) -> Self {
		self.resource_operations = Some(resource_operations);
		self
	}

	pub fn with_failure_handling(mut self, failure_handling: FailureHandlingKind) -> Self {
		self.failure_handling = Some(failure_handling);
		self
	}

	pub fn with_normalizes_line_endings(mut self, normalizes_line_endings: bool) -> Self {
		self.normalizes_line_endings = Some(normalizes_line_endings);
		self
	}

	pub fn with_change_annotation_support(
		mut self,
		change_annotation_support: WorkspaceChangeAnnotationSupportClientCapabilities,
	) -> Self {
		self.change_annotation_support = Some(change_annotation_support);
		self
	}

	/// The client supports versioned document changes in [`WorkspaceEdit`]s
	pub fn document_changes(&self) -> Option<&bool> {
		self.document_changes.as_ref()
	}

	/// The resource operations the client supports.
	///
	/// Clients should at least support 'create', 'rename' and 'delete' files and folders.
	///
	/// since: 3.13.0
	pub fn resource_operations(&self) -> Option<&Vec<ResourceOperationKind>> {
		self.resource_operations.as_ref()
	}
	/// The failure handling strategy of a client if applying the workspace edit fails.
	///
	/// since: 3.13.0
	pub fn failure_handling(&self) -> Option<&FailureHandlingKind> {
		self.failure_handling.as_ref()
	}

	/// Whether the client normalizes line endings to the client specific setting.
	///
	/// If set to `true` the client will normalize line ending characters in a workspace
	/// edit to the client-specified new line character.
	///
	/// since: 3.16.0
	pub fn normalizes_line_endings(&self) -> Option<&bool> {
		self.normalizes_line_endings.as_ref()
	}

	/// Whether the client in general supports change annotations on text edits,create file,
	/// rename file and delete file changes.
	///
	/// since: 3.16.0
	pub fn change_annotation_support(
		&self,
	) -> Option<&WorkspaceChangeAnnotationSupportClientCapabilities> {
		self.change_annotation_support.as_ref()
	}
}

impl DidChangeConfigurationClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Did change configuration notification supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl DidChangeWatchedFilesClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			relative_pattern_support: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_relative_pattern_support(mut self, relative_pattern_support: bool) -> Self {
		self.relative_pattern_support = Some(relative_pattern_support);
		self
	}

	/// Did change watched files notification supports dynamic registration.
	///
	/// Please note that the current protocol doesn't support static configuration for file changes
	/// from the server side.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Whether the client has support for [`RelativePattern`] or not.
	///
	/// since: 3.17.0
	pub fn relative_pattern_support(&self) -> Option<&bool> {
		self.relative_pattern_support.as_ref()
	}
}

impl WorkspaceSymbolClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			symbol_kind: None,
			tag_support: None,
			resolve_support: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_symbol_kind(mut self, symbol_kind: ValueSet<SymbolKind>) -> Self {
		self.symbol_kind = Some(symbol_kind);
		self
	}

	pub fn with_tag_support(mut self, tag_support: ValueSet<SymbolTag>) -> Self {
		self.tag_support = Some(tag_support);
		self
	}

	pub fn with_resolve_support(mut self, resolve_support: StringProperties) -> Self {
		self.resolve_support = Some(resolve_support);
		self
	}

	/// Symbol request supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The symbol kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle
	/// values outside its set gracefully and falls back to a default value when unknown.
	///
	/// If this property is not present the client only supports the symbol kinds from
	/// `File` to `Array` as defined in the initial version of the protocol.
	pub fn symbol_kind(&self) -> Option<&ValueSet<SymbolKind>> {
		self.symbol_kind.as_ref()
	}

	/// The client supports tags on [`SymbolInformation`].
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// since: 3.16.0
	pub fn tag_support(&self) -> Option<&ValueSet<SymbolTag>> {
		self.tag_support.as_ref()
	}

	/// The client support partial workspace symbols.
	///
	/// The client will send the request `workspaceSymbol/resolve` to the server to resolve
	/// additional properties.
	///
	/// since: 3.17.0
	pub fn resolve_support(&self) -> Option<&StringProperties> {
		self.resolve_support.as_ref()
	}
}

impl ExecuteCommandClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Execute command supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl SemanticTokensWorkspaceClientCapabilities {
	pub fn new() -> Self {
		Self { refresh_support: None }
	}

	pub fn with_refresh_support(mut self, refresh_support: bool) -> Self {
		self.refresh_support = Some(refresh_support);
		self
	}

	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all semantic tokens currently
	/// shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example
	/// detects a project wide change that requires such a calculation.
	pub fn refresh_support(&self) -> Option<&bool> {
		self.refresh_support.as_ref()
	}
}

impl CodeLensWorkspaceClientCapabilities {
	pub fn new() -> Self {
		Self { refresh_support: None }
	}

	pub fn with_refresh_support(mut self, refresh_support: bool) -> Self {
		self.refresh_support = Some(refresh_support);
		self
	}

	/// "Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all code lenses currently shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example
	/// detect a project wide change that requires such a calculation.
	pub fn refresh_support(&self) -> Option<&bool> {
		self.refresh_support.as_ref()
	}
}

impl FileOperationClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			did_create: None,
			will_create: None,
			did_rename: None,
			will_rename: None,
			did_delete: None,
			will_delete: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_did_create(mut self, did_create: bool) -> Self {
		self.did_create = Some(did_create);
		self
	}

	pub fn with_will_create(mut self, will_create: bool) -> Self {
		self.will_create = Some(will_create);
		self
	}

	pub fn with_did_rename(mut self, did_rename: bool) -> Self {
		self.did_rename = Some(did_rename);
		self
	}

	pub fn with_will_rename(mut self, will_rename: bool) -> Self {
		self.will_rename = Some(will_rename);
		self
	}

	pub fn with_did_delete(mut self, did_delete: bool) -> Self {
		self.did_delete = Some(did_delete);
		self
	}

	pub fn with_will_delete(mut self, will_delete: bool) -> Self {
		self.will_delete = Some(will_delete);
		self
	}

	/// Whether the client supports dynamic registration for file requests/notifications.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client has support for sending didCreateFiles notifications.
	pub fn did_create(&self) -> Option<&bool> {
		self.did_create.as_ref()
	}

	/// The client has support for sending willCreateFiles requests.
	pub fn will_create(&self) -> Option<&bool> {
		self.will_create.as_ref()
	}

	/// The client has support for sending didRenameFiles notifications.
	pub fn did_rename(&self) -> Option<&bool> {
		self.did_rename.as_ref()
	}

	/// The client has support for sending willRenameFiles requests.
	pub fn will_rename(&self) -> Option<&bool> {
		self.will_rename.as_ref()
	}

	/// The client has support for sending didDeleteFiles notifications.
	pub fn did_delete(&self) -> Option<&bool> {
		self.did_delete.as_ref()
	}

	/// The client has support for sending willDeleteFiles requests.
	pub fn will_delete(&self) -> Option<&bool> {
		self.will_delete.as_ref()
	}
}

impl InlineValueWorkspaceClientCapabilities {
	pub fn new() -> Self {
		Self { refresh_support: None }
	}

	pub fn with_refresh_support(mut self, refresh_support: bool) -> Self {
		self.refresh_support = Some(refresh_support);
		self
	}

	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all inline values currently shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	pub fn refresh_support(&self) -> Option<&bool> {
		self.refresh_support.as_ref()
	}
}

impl InlayHintWorkspaceClientCapabilities {
	pub fn new() -> Self {
		Self { refresh_support: None }
	}

	pub fn with_refresh_support(mut self, refresh_support: bool) -> Self {
		self.refresh_support = Some(refresh_support);
		self
	}

	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all inlay hints currently shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	pub fn refresh_support(&self) -> Option<&bool> {
		self.refresh_support.as_ref()
	}
}

impl DiagnosticWorkspaceClientCapabilities {
	pub fn new() -> Self {
		Self { refresh_support: None }
	}

	pub fn with_refresh_support(mut self, refresh_support: bool) -> Self {
		self.refresh_support = Some(refresh_support);
		self
	}

	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all pulled diagnostics currently
	/// shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	pub fn refresh_support(&self) -> Option<&bool> {
		self.refresh_support.as_ref()
	}
}

impl FoldingRangeWorkspaceClientCapabilities {
	pub fn new() -> Self {
		Self { refresh_support: None }
	}

	pub fn with_refresh_support(mut self, refresh_support: bool) -> Self {
		self.refresh_support = Some(refresh_support);
		self
	}

	/// Whether the client implementation supports a refresh request sent from the server to the client.
	///
	/// Note that this event is global and will force the client to refresh all folding ranges currently
	/// shown.
	///
	/// It should be used with absolute care and is useful for situation where a server for example detects
	/// a project wide change that requires such a calculation.
	///
	/// since: 3.18.0
	pub fn refresh_support(&self) -> Option<&bool> {
		self.refresh_support.as_ref()
	}
}

impl TextDocumentSyncClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			will_save: None,
			will_save_wait_until: None,
			did_save: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_will_save(mut self, will_save: bool) -> Self {
		self.will_save = Some(will_save);
		self
	}

	pub fn with_will_save_wait_until(mut self, will_save_wait_until: bool) -> Self {
		self.will_save_wait_until = Some(will_save_wait_until);
		self
	}

	pub fn with_did_save(mut self, did_save: bool) -> Self {
		self.did_save = Some(did_save);
		self
	}

	/// Whether text document synchronization supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports sending will save notifications.
	pub fn will_save(&self) -> Option<&bool> {
		self.will_save.as_ref()
	}

	/// The client supports sending a will save request and waits for a response providing
	/// text edits which will be applied to the document before it is saved.
	pub fn will_save_wait_until(&self) -> Option<&bool> {
		self.will_save_wait_until.as_ref()
	}

	/// The client supports did save notifications.
	pub fn did_save(&self) -> Option<&bool> {
		self.did_save.as_ref()
	}
}

impl CompletionItemClientCapabilities {
	pub fn new() -> Self {
		Self {
			snippet_support: None,
			commit_characters_support: None,
			documentation_format: None,
			deprecated_support: None,
			preselect_support: None,
			tag_support: None,
			insert_replace_support: None,
			resolve_support: None,
			insert_text_mode_support: None,
			label_details_support: None,
		}
	}

	pub fn with_snippet_support(mut self, snippet_support: bool) -> Self {
		self.snippet_support = Some(snippet_support);
		self
	}

	pub fn with_commit_characters_support(mut self, commit_characters_support: bool) -> Self {
		self.commit_characters_support = Some(commit_characters_support);
		self
	}

	pub fn with_documentation_format(mut self, documentation_format: Vec<MarkupKind>) -> Self {
		self.documentation_format = Some(documentation_format);
		self
	}

	pub fn with_deprecated_support(mut self, deprecated_support: bool) -> Self {
		self.deprecated_support = Some(deprecated_support);
		self
	}

	pub fn with_preselect_support(mut self, preselect_support: bool) -> Self {
		self.preselect_support = Some(preselect_support);
		self
	}

	pub fn with_tag_support(mut self, tag_support: ValueSet<CompletionItemTag>) -> Self {
		self.tag_support = Some(tag_support);
		self
	}

	pub fn with_insert_replace_support(mut self, insert_replace_support: bool) -> Self {
		self.insert_replace_support = Some(insert_replace_support);
		self
	}

	pub fn with_resolve_support(mut self, resolve_support: StringProperties) -> Self {
		self.resolve_support = Some(resolve_support);
		self
	}

	pub fn with_insert_text_mode_support(
		mut self,
		insert_text_mode_support: ValueSet<InsertTextMode>,
	) -> Self {
		self.insert_text_mode_support = Some(insert_text_mode_support);
		self
	}

	pub fn with_label_details_support(mut self, label_details_support: bool) -> Self {
		self.label_details_support = Some(label_details_support);
		self
	}

	/// Client supports snippets as insert text.
	///
	/// A snippet can define tab stops and placeholders with `$1`, `$2` and `${3:foo}`.
	///
	/// `$0` defines the final tab stop, it defaults to the end of the snippet.
	///
	/// Placeholders with equal identifiers are linked, that is typing in one will update others too.
	pub fn snippet_support(&self) -> Option<&bool> {
		self.snippet_support.as_ref()
	}

	/// Client supports commit characters on a completion item.
	pub fn commit_characters_support(&self) -> Option<&bool> {
		self.commit_characters_support.as_ref()
	}

	/// Client supports the following content formats for the documentation property.
	///
	/// The order describes the preferred format of the client.
	pub fn documentation_format(&self) -> Option<&Vec<MarkupKind>> {
		self.documentation_format.as_ref()
	}

	/// Client supports the deprecated property on a completion item.
	pub fn deprecated_support(&self) -> Option<&bool> {
		self.deprecated_support.as_ref()
	}

	/// Client supports the preselect property on a completion item.
	pub fn preselect_support(&self) -> Option<&bool> {
		self.preselect_support.as_ref()
	}

	/// Client supports the tag property on a completion item.
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// Clients especially need to preserve unknown tags when sending a completion item back
	/// to the server in a resolve call.
	///
	/// since: 3.15.0
	pub fn tag_support(&self) -> Option<&ValueSet<CompletionItemTag>> {
		self.tag_support.as_ref()
	}

	/// Client support insert replace edit to control different behavior if a
	/// completion item is inserted in the text or should replace text.
	///
	/// since: 3.16.0
	pub fn insert_replace_support(&self) -> Option<&bool> {
		self.insert_replace_support.as_ref()
	}

	/// Indicates which properties a client can resolve lazily on a completion item.
	///
	/// Before version 3.16.0 only the predefined properties `documentation` and `details`
	/// could be resolved lazily.
	///
	/// since: 3.16.0
	pub fn resolve_support(&self) -> Option<&StringProperties> {
		self.resolve_support.as_ref()
	}

	/// The client supports the `insertTextMode` property on a completion item to override the
	/// whitespace handling mode as defined by the client (see `insertTextMode`).
	///
	/// since: 3.16.0
	pub fn insert_text_mode_support(&self) -> Option<&ValueSet<InsertTextMode>> {
		self.insert_text_mode_support.as_ref()
	}

	/// The client has support for completion item label details (see also `CompletionItemLabelDetails`).
	///
	/// since: 3.17.0
	pub fn label_details_support(&self) -> Option<&bool> {
		self.label_details_support.as_ref()
	}
}

impl CompletionClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			completion_item: None,
			completion_item_kind: None,
			insert_text_mode: None,
			context_support: None,
			completion_list: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_completion_item(
		mut self,
		completion_item: CompletionItemClientCapabilities,
	) -> Self {
		self.completion_item = Some(completion_item);
		self
	}

	pub fn with_completion_item_kind(
		mut self,
		completion_item_kind: ValueSet<CompletionItemKind>,
	) -> Self {
		self.completion_item_kind = Some(completion_item_kind);
		self
	}

	pub fn with_insert_text_mode(mut self, insert_text_mode: InsertTextMode) -> Self {
		self.insert_text_mode = Some(insert_text_mode);
		self
	}

	pub fn with_context_support(mut self, context_support: bool) -> Self {
		self.context_support = Some(context_support);
		self
	}

	pub fn with_completion_list(mut self, completion_list: CompletionListItemDefaults) -> Self {
		self.completion_list = Some(completion_list);
		self
	}

	/// Whether completion supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports the following [`CompletionItem`] specific capabilities.
	pub fn completion_item(&self) -> Option<&CompletionItemClientCapabilities> {
		self.completion_item.as_ref()
	}

	/// The completion item kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside
	/// its set gracefully and falls back to a default value when unknown.
	///
	/// If this property is not present the client only supports the completion items kinds from
	/// `Text` to `Reference` as defined in the initial version of the protocol.
	pub fn completion_item_kind(&self) -> Option<&ValueSet<CompletionItemKind>> {
		self.completion_item_kind.as_ref()
	}

	/// Defines how the client handles whitespace and indentation when accepting a completion item
	/// that uses multi line text in either `insertText` or `textEdit`.
	///
	/// since: 3.17.0
	pub fn insert_text_mode(&self) -> Option<&InsertTextMode> {
		self.insert_text_mode.as_ref()
	}

	/// The client supports to send additional context information for a `textDocument/completion` request.
	pub fn context_support(&self) -> Option<&bool> {
		self.context_support.as_ref()
	}

	/// The client supports the following itemDefaults on a completion list.
	///
	/// The value lists the supported property names of the `CompletionList.itemDefaults` object.
	///
	/// If omitted no properties are supported.
	///
	/// since 3.17.0
	pub fn completion_list(&self) -> Option<&CompletionListItemDefaults> {
		self.completion_list.as_ref()
	}
}

impl HoverClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, content_format: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_content_format(mut self, content_format: Vec<MarkupKind>) -> Self {
		self.content_format = Some(content_format);
		self
	}

	/// Whether hover supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Client supports the following content formats for the content property.
	///
	/// The order describes the preferred format of the client.
	pub fn content_format(&self) -> Option<&Vec<MarkupKind>> {
		self.content_format.as_ref()
	}
}

impl ParameterInformationClientCapabilities {
	pub fn new() -> Self {
		Self { label_offset_support: None }
	}

	pub fn with_label_offset_support(mut self, label_offset_support: bool) -> Self {
		self.label_offset_support = Some(label_offset_support);
		self
	}

	/// The client supports processing label offsets instead of a simple label string.
	///
	/// since: 3.14.0
	pub fn label_offset_support(&self) -> Option<&bool> {
		self.label_offset_support.as_ref()
	}
}

impl SignatureInformationClientCapabilities {
	pub fn new() -> Self {
		Self {
			documentation_format: None,
			parameter_information: None,
			active_parameter_support: None,
		}
	}

	pub fn with_documentation_format(mut self, documentation_format: Vec<MarkupKind>) -> Self {
		self.documentation_format = Some(documentation_format);
		self
	}

	pub fn with_parameter_information(
		mut self,
		parameter_information: ParameterInformationClientCapabilities,
	) -> Self {
		self.parameter_information = Some(parameter_information);
		self
	}

	pub fn with_active_parameter_support(mut self, active_parameter_support: bool) -> Self {
		self.active_parameter_support = Some(active_parameter_support);
		self
	}

	/// Client supports the following content formats for the documentation property.
	///
	/// The order describes the preferred format of the client.
	pub fn documentation_format(&self) -> Option<&Vec<MarkupKind>> {
		self.documentation_format.as_ref()
	}

	/// Client capabilities specific to parameter information.
	pub fn parameter_information(&self) -> Option<&ParameterInformationClientCapabilities> {
		self.parameter_information.as_ref()
	}

	/// The client supports the `activeParameter` property on `SignatureInformation` literal.
	///
	/// since: 3.16.0
	pub fn active_parameter_support(&self) -> Option<&bool> {
		self.active_parameter_support.as_ref()
	}
}

impl SignatureHelpClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			signature_information: None,
			context_support: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_signature_information(
		mut self,
		signature_information: SignatureInformationClientCapabilities,
	) -> Self {
		self.signature_information = Some(signature_information);
		self
	}

	pub fn with_context_support(mut self, context_support: bool) -> Self {
		self.context_support = Some(context_support);
		self
	}

	/// Whether signature help supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports the following `SignatureInformation` specific properties.
	pub fn signature_information(&self) -> Option<&SignatureInformationClientCapabilities> {
		self.signature_information.as_ref()
	}

	/// The client supports to send additional context information for a `textDocument/signatureHelp`
	/// request.
	///
	/// A client that opts into contextSupport will also support the `retriggerCharacters` on
	/// `SignatureHelpOptions`.
	///
	/// since 3.15.0
	pub fn context_support(&self) -> Option<&bool> {
		self.context_support.as_ref()
	}
}

impl DeclarationClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, link_support: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_link_support(mut self, link_support: bool) -> Self {
		self.link_support = Some(link_support);
		self
	}

	/// Whether declaration supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`DeclarationRegistrationOptions`]
	/// return value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports additional metadata in the form of declaration links.
	pub fn link_support(&self) -> Option<&bool> {
		self.link_support.as_ref()
	}
}

impl DefinitionClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, link_support: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_link_support(mut self, link_support: bool) -> Self {
		self.link_support = Some(link_support);
		self
	}

	/// Whether definition supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports additional metadata in the form of definition links.
	///
	/// since: 3.14.0
	pub fn link_support(&self) -> Option<&bool> {
		self.link_support.as_ref()
	}
}

impl TypeDefinitionClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, link_support: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_link_support(mut self, link_support: bool) -> Self {
		self.link_support = Some(link_support);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`TypeDefinitionRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports additional metadata in the form of definition links.
	///
	/// since: 3.14.0
	pub fn link_support(&self) -> Option<&bool> {
		self.link_support.as_ref()
	}
}

impl ImplementationClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, link_support: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_link_support(mut self, link_support: bool) -> Self {
		self.link_support = Some(link_support);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`ImplementationRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client supports additional metadata in the form of definition links.
	///
	/// since: 3.14.0
	pub fn link_support(&self) -> Option<&bool> {
		self.link_support.as_ref()
	}
}

impl ReferenceClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether references supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl DocumentHighlightClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether document highlight supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl CodeLensClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether code lens supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl DocumentLinkClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, tooltip_support: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_tooltip_support(mut self, tooltip_support: bool) -> Self {
		self.tooltip_support = Some(tooltip_support);
		self
	}

	/// Whether document link supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Whether the client supports the `tooltip` property on `DocumentLink`.
	///
	/// since: 3.15.0
	pub fn tooltip_support(&self) -> Option<&bool> {
		self.tooltip_support.as_ref()
	}
}

impl DocumentColorClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new [`DocumentColorRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl DocumentFormattingClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether formatting supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl DocumentRangeFormattingClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None, ranges_support: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_ranges_support(mut self, ranges_support: bool) -> Self {
		self.ranges_support = Some(ranges_support);
		self
	}

	/// Whether range formatting supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Whether the client supports formatting multiple ranges at once.
	///
	/// since: 3.18.0
	pub fn ranges_support(&self) -> Option<&bool> {
		self.ranges_support.as_ref()
	}
}

impl DocumentOnTypeFormattingClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether on type formatting supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl SelectionRangeClientCapabilities {
	pub fn new() -> Self {
		Self { dynamic_registration: None }
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	/// Whether implementation supports dynamic registration for selection range providers.
	///
	/// If this is set to `true` the client supports the new [`SelectionRangeRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}
}

impl DocumentSymbolClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			symbol_kind: None,
			hierarchical_document_symbol_support: None,
			tag_support: None,
			label_support: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_symbol_kind(mut self, symbol_kind: ValueSet<SymbolKind>) -> Self {
		self.symbol_kind = Some(symbol_kind);
		self
	}

	pub fn with_hierarchical_document_symbol_support(
		mut self,
		hierarchical_document_symbol_support: bool,
	) -> Self {
		self.hierarchical_document_symbol_support = Some(hierarchical_document_symbol_support);
		self
	}

	pub fn with_tag_support(mut self, tag_support: ValueSet<SymbolTag>) -> Self {
		self.tag_support = Some(tag_support);
		self
	}

	pub fn with_label_support(mut self, label_support: bool) -> Self {
		self.label_support = Some(label_support);
		self
	}

	/// Whether document symbol supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The symbol kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside its set
	/// gracefully and falls back to a default value when unknown.
	///
	/// If this property is not present the client only supports the symbol kinds from `File` to `Array`
	/// as defined in the initial version of the protocol.
	pub fn symbol_kind(&self) -> Option<&ValueSet<SymbolKind>> {
		self.symbol_kind.as_ref()
	}

	/// The client supports hierarchical document symbols.
	pub fn hierarchical_document_symbol_support(&self) -> Option<&bool> {
		self.hierarchical_document_symbol_support.as_ref()
	}

	/// The client supports tags on [`SymbolInformation`].
	///
	/// Tags are supported on [`DocumentSymbol`] if `hierarchicalDocumentSymbolSupport` is set to true.
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// since: 3.16.0
	pub fn tag_support(&self) -> Option<&ValueSet<SymbolTag>> {
		self.tag_support.as_ref()
	}

	/// The client supports an additional label presented in the UI when registering a document symbol provider.
	///
	/// since: 3.16.0
	pub fn label_support(&self) -> Option<&bool> {
		self.label_support.as_ref()
	}
}

impl RenameClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			prepare_support: None,
			prepare_support_default_behavior: None,
			honors_change_annotations: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_prepare_support(mut self, prepare_support: bool) -> Self {
		self.prepare_support = Some(prepare_support);
		self
	}

	pub fn with_prepare_support_default_behavior(
		mut self,
		prepare_support_default_behavior: PrepareSupportDefaultBehavior,
	) -> Self {
		self.prepare_support_default_behavior = Some(prepare_support_default_behavior);
		self
	}

	pub fn with_honors_change_annotations(mut self, honors_change_annotations: bool) -> Self {
		self.honors_change_annotations = Some(honors_change_annotations);
		self
	}

	/// Whether rename supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Client supports testing for validity of rename operations before execution.
	///
	/// since: 3.12.0
	pub fn prepare_support(&self) -> Option<&bool> {
		self.prepare_support.as_ref()
	}

	/// Client supports the default behavior result.
	///
	/// The value indicates the default behavior used by the client.
	///
	/// since: 3.16.0
	pub fn prepare_support_default_behavior(&self) -> Option<&PrepareSupportDefaultBehavior> {
		self.prepare_support_default_behavior.as_ref()
	}

	/// Whether the client honors the change annotations in text edits and resource operations returned
	/// via the rename request's workspace edit by for example presenting the workspace edit in the
	/// user interface and asking\nfor confirmation.
	///
	/// since 3.16.0
	pub fn honors_change_annotations(&self) -> Option<&bool> {
		self.honors_change_annotations.as_ref()
	}
}

impl FoldingRangeRangeClientCapabilities {
	pub fn new() -> Self {
		Self { collapsed_text: None }
	}

	pub fn with_collapsed_text(mut self, collapsed_text: bool) -> Self {
		self.collapsed_text = Some(collapsed_text);
		self
	}

	/// If set, the client signals that it supports setting collapsedText on
	/// folding ranges to display custom labels instead of the default text.
	///
	/// since: 3.17.0
	pub fn collapsed_text(&self) -> Option<&bool> {
		self.collapsed_text.as_ref()
	}
}

impl FoldingRangeClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			range_limit: None,
			line_folding_only: None,
			folding_range_kind: None,
			folding_range: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_range_limit(mut self, range_limit: u32) -> Self {
		self.range_limit = Some(range_limit);
		self
	}

	pub fn with_line_folding_only(mut self, line_folding_only: bool) -> Self {
		self.line_folding_only = Some(line_folding_only);
		self
	}

	pub fn with_folding_range_kind(
		mut self,
		folding_range_kind: ValueSet<FoldingRangeKind>,
	) -> Self {
		self.folding_range_kind = Some(folding_range_kind);
		self
	}

	pub fn with_folding_range(
		mut self,
		folding_range: FoldingRangeRangeClientCapabilities,
	) -> Self {
		self.folding_range = Some(folding_range);
		self
	}

	/// Whether implementation supports dynamic registration for folding range providers.
	///
	/// If this is set to `true` the client supports the new [`FoldingRangeRegistrationOptions`] return
	/// value for the corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The maximum number of folding ranges that the client prefers to receive per document.
	///
	// The value serves as a hint, servers are free to follow the limit.
	pub fn range_limit(&self) -> Option<&u32> {
		self.range_limit.as_ref()
	}

	/// If set, the client signals that it only supports folding complete lines.
	///
	/// If set, client will ignore specified `startCharacter` and `endCharacter` properties in a
	/// [`FoldingRange`].
	pub fn line_folding_only(&self) -> Option<&bool> {
		self.line_folding_only.as_ref()
	}

	/// The folding range kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside its
	/// set gracefully and falls back to a default value when unknown.
	pub fn folding_range_kind(&self) -> Option<&ValueSet<FoldingRangeKind>> {
		self.folding_range_kind.as_ref()
	}

	/// Specific options for the folding range.
	///
	/// since: 3.17.0
	pub fn folding_range(&self) -> Option<&FoldingRangeRangeClientCapabilities> {
		self.folding_range.as_ref()
	}
}

impl CodeActionLiteralClientCapabilities {
	pub fn new(code_action_kind: ValueSet<CodeActionKind>) -> Self {
		Self { code_action_kind }
	}

	/// The code action kind values the client supports.
	///
	/// When this property exists the client also guarantees that it will handle values outside
	/// its set gracefully and falls back to a default value when unknown.
	pub fn code_action_kind(&self) -> &ValueSet<CodeActionKind> {
		&self.code_action_kind
	}
}

impl CodeActionClientCapabilities {
	pub fn new() -> Self {
		Self {
			dynamic_registration: None,
			code_action_literal_support: None,
			is_preferred_support: None,
			disabled_support: None,
			data_support: None,
			resolve_support: None,
			honors_change_annotations: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_code_action_literal_support(
		mut self,
		code_action_literal_support: CodeActionLiteralClientCapabilities,
	) -> Self {
		self.code_action_literal_support = Some(code_action_literal_support);
		self
	}

	pub fn with_is_preferred_support(mut self, is_preferred_support: bool) -> Self {
		self.is_preferred_support = Some(is_preferred_support);
		self
	}

	pub fn with_disabled_support(mut self, disabled_support: bool) -> Self {
		self.disabled_support = Some(disabled_support);
		self
	}

	pub fn with_data_support(mut self, data_support: bool) -> Self {
		self.data_support = Some(data_support);
		self
	}

	pub fn with_resolve_support(mut self, resolve_support: StringProperties) -> Self {
		self.resolve_support = Some(resolve_support);
		self
	}

	pub fn with_honors_change_annotations(mut self, honors_change_annotations: bool) -> Self {
		self.honors_change_annotations = Some(honors_change_annotations);
		self
	}

	/// Whether code action supports dynamic registration.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// The client support code action literals of type [`CodeAction`] as a valid response of the
	/// `textDocument/codeAction` request.
	///
	/// If the property is not set the request can only return [`Command`] literals.
	///
	/// since 3.8.0
	pub fn code_action_literal_support(&self) -> Option<&CodeActionLiteralClientCapabilities> {
		self.code_action_literal_support.as_ref()
	}

	/// Whether code action supports the `isPreferred` property.
	///
	/// since: 3.15.0
	pub fn is_preferred_support(&self) -> Option<&bool> {
		self.is_preferred_support.as_ref()
	}

	/// Whether code action supports the `disabled` property.
	///
	/// since: 3.16.0
	pub fn disabled_support(&self) -> Option<&bool> {
		self.disabled_support.as_ref()
	}

	/// Whether code action supports the `data` property which is preserved between a
	/// `textDocument/codeAction` and a `codeAction/resolve` request.
	///
	/// since: 3.16.0
	pub fn data_support(&self) -> Option<&bool> {
		self.data_support.as_ref()
	}

	/// Whether the client supports resolving additional code action properties via a separate
	/// `codeAction/resolve` request.
	///
	/// since: 3.16.0
	pub fn resolve_support(&self) -> Option<&StringProperties> {
		self.resolve_support.as_ref()
	}

	/// Whether the client honors the change annotations in text edits and resource operations returned
	/// via the [`CodeAction::edit`] property by for example presenting the workspace edit in the user
	/// interface and asking for confirmation.
	///
	/// since: 3.16.0
	pub fn honors_change_annotations(&self) -> Option<&bool> {
		self.honors_change_annotations.as_ref()
	}
}

impl PublishDiagnosticsClientCapabilities {
	pub fn new() -> Self {
		Self {
			related_information: None,
			tag_support: None,
			version_support: None,
			code_description_support: None,
			data_support: None,
		}
	}

	pub fn with_related_information(mut self, related_information: bool) -> Self {
		self.related_information = Some(related_information);
		self
	}

	pub fn with_tag_support(mut self, tag_support: ValueSet<DiagnosticTag>) -> Self {
		self.tag_support = Some(tag_support);
		self
	}

	pub fn with_version_support(mut self, version_support: bool) -> Self {
		self.version_support = Some(version_support);
		self
	}

	pub fn with_code_description_support(mut self, code_description_support: bool) -> Self {
		self.code_description_support = Some(code_description_support);
		self
	}

	pub fn with_data_support(mut self, data_support: bool) -> Self {
		self.data_support = Some(data_support);
		self
	}

	/// Whether the clients accepts diagnostics with related information.
	pub fn related_information(&self) -> Option<&bool> {
		self.related_information.as_ref()
	}

	/// Client supports the tag property to provide meta data about a diagnostic.
	///
	/// Clients supporting tags have to handle unknown tags gracefully.
	///
	/// since:s 3.15.0
	pub fn tag_support(&self) -> Option<&ValueSet<DiagnosticTag>> {
		self.tag_support.as_ref()
	}

	/// Whether the client interprets the version property of the `textDocument/publishDiagnostics`
	/// notification's parameter.
	///
	/// since: 3.15.0
	pub fn version_support(&self) -> Option<&bool> {
		self.version_support.as_ref()
	}

	/// Client supports a codeDescription property.
	///
	/// since: 3.16.0
	pub fn code_description_support(&self) -> Option<&bool> {
		self.code_description_support.as_ref()
	}

	/// Whether code action supports the `data` property which is preserved between a
	/// `textDocument/publishDiagnostics` and `textDocument/codeAction` request.
	///
	/// since: 3.16.0
	pub fn data_support(&self) -> Option<&bool> {
		self.data_support.as_ref()
	}
}

impl SemanticTokensRequestsClientCapabilities {
	pub fn new() -> Self {
		Self { range: None, full: None }
	}

	pub fn with_range(mut self, range: bool) -> Self {
		self.range = Some(range);
		self
	}

	pub fn with_full(mut self, full: SemanticTokensFullRequestsType) -> Self {
		self.full = Some(full);
		self
	}

	/// The client will send the `textDocument/semanticTokens/range` request if
	/// the server provides a corresponding handler.
	pub fn range(&self) -> Option<&bool> {
		self.range.as_ref()
	}

	/// The client will send the `textDocument/semanticTokens/full` request if
	/// the server provides a corresponding handler.
	pub fn full(&self) -> Option<&SemanticTokensFullRequestsType> {
		self.full.as_ref()
	}
}

impl SemanticTokensClientCapabilities {
	pub fn new(
		requests: SemanticTokensRequestsClientCapabilities,
		token_types: Vec<String>,
		token_modifiers: Vec<String>,
		formats: Vec<TokenFormat>,
	) -> Self {
		Self {
			dynamic_registration: None,
			requests,
			token_types,
			token_modifiers,
			formats,
			overlapping_token_support: None,
			multiline_token_support: None,
			server_cancel_support: None,
			augments_syntax_tokens: None,
		}
	}

	pub fn with_dynamic_registration(mut self, dynamic_registration: bool) -> Self {
		self.dynamic_registration = Some(dynamic_registration);
		self
	}

	pub fn with_overlapping_token_support(mut self, overlapping_token_support: bool) -> Self {
		self.overlapping_token_support = Some(overlapping_token_support);
		self
	}

	pub fn with_multiline_token_support(mut self, multiline_token_support: bool) -> Self {
		self.multiline_token_support = Some(multiline_token_support);
		self
	}

	pub fn with_server_cancel_support(mut self, server_cancel_support: bool) -> Self {
		self.server_cancel_support = Some(server_cancel_support);
		self
	}

	pub fn with_augments_syntax_tokens(mut self, augments_syntax_tokens: bool) -> Self {
		self.augments_syntax_tokens = Some(augments_syntax_tokens);
		self
	}

	/// Whether implementation supports dynamic registration.
	///
	/// If this is set to `true` the client supports the new
	/// `(TextDocumentRegistrationOptions & StaticRegistrationOptions)` return value for the
	/// corresponding server capability as well.
	pub fn dynamic_registration(&self) -> Option<&bool> {
		self.dynamic_registration.as_ref()
	}

	/// Which requests the client supports and might send to the server depending on the server's capability.
	///
	/// Please note that clients might not show semantic tokens or degrade some of the user experience
	/// if a range or full request is advertised by the client but not provided by the server.
	///
	/// If for example the client capability `requests.full` and `request.range` are both set
	/// to true but the server only provides a range provider the client might not render a minimap
	/// correctly or might even decide to not show any semantic tokens at all.
	pub fn requests(&self) -> &SemanticTokensRequestsClientCapabilities {
		&self.requests
	}

	/// The token types that the client supports.
	pub fn token_types(&self) -> &Vec<String> {
		&self.token_types
	}

	/// The token modifiers that the client supports.
	pub fn token_modifiers(&self) -> &Vec<String> {
		&self.token_modifiers
	}

	/// The token formats the clients supports.
	pub fn formats(&self) -> &Vec<TokenFormat> {
		&self.formats
	}

	/// Whether the client supports tokens that can overlap each other.
	pub fn overlapping_token_support(&self) -> Option<&bool> {
		self.overlapping_token_support.as_ref()
	}

	/// Whether the client supports tokens that can span multiple lines.
	pub fn multiline_token_support(&self) -> Option<&bool> {
		self.multiline_token_support.as_ref()
	}

	/// Whether the client allows the server to actively cancel a semantic token request,
	/// e.g. supports returning [`crate::error::Code::ServerCancelled`].
	///
	/// If a server does the client needs to retrigger the request.
	///
	/// since: 3.17.0
	pub fn server_cancel_support(&self) -> Option<&bool> {
		self.server_cancel_support.as_ref()
	}

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
	pub fn augments_syntax_tokens(&self) -> Option<&bool> {
		self.augments_syntax_tokens.as_ref()
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
