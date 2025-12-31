/* SPDX-License-Identifier: BSD-3-Clause */

use crate::types::{
	LspAny, OptionsOrBool, PositionEncodingKind, RegistrationOptionsOrBool, RegistrationOrOptions,
	StringOrBool, TextDocumentSyncKind,
	options::{
		CallHierarchyOptions, CallHierarchyRegistrationOptions, CodeActionOptions,
		CodeLenseOptions, CompletionOptions, DeclarationOptions, DeclarationRegistrationOptions,
		DefinitionOptions, DiagnosticOptions, DiagnosticRegistrationOptions, DocumentColorOptions,
		DocumentColorRegistrationOptions, DocumentFormattingOptions, DocumentHighlightOptions,
		DocumentLinkOptions, DocumentOnTypeFormattingOptions, DocumentRangeFormattingOptions,
		DocumentSymbolOptions, ExecuteCommandOptions, FileOperationOptions, FoldingRangeOptions,
		FoldingRangeRegistrationOptions, HoverOptions, ImplementationOptions,
		ImplementationRegistrationOptions, InlayHintOptions, InlayHintRegistrationOptions,
		InlineCompletionOptions, InlineValueOptions, InlineValueRegistrationOptions,
		LinkedEditingRangeOptions, LinkedEditingRangeRegistrationOptions, MonikerOptions,
		MonikerRegistrationOptions, NotebookDocumentSyncOptions,
		NotebookDocumentSyncRegistrationOptions, ReferenceOptions, RenameOptions,
		SelectionRangeOptions, SelectionRangeRegistrationOptions, SemanticTokensOptions,
		SemanticTokensRegistrationOptions, SignatureHelpOptions, TextDocumentSyncOptions,
		TypeDefinitionOptions, TypeDefinitionRegistrationOptions, TypeHierarchyOptions,
		TypeHierarchyRegistrationOptions, WorkspaceSymbolOptions,
	},
};

pub type HoverServerCapabilities = OptionsOrBool<HoverOptions>;
pub type DeclarationServerCapabilities =
	RegistrationOptionsOrBool<DeclarationOptions, DeclarationRegistrationOptions>;
pub type DefinitionServerCapabilities = OptionsOrBool<DefinitionOptions>;
pub type TypeDefinitionServerCapabilities =
	RegistrationOptionsOrBool<TypeDefinitionOptions, TypeDefinitionRegistrationOptions>;
pub type ImplementationServerCapabilities =
	RegistrationOptionsOrBool<ImplementationOptions, ImplementationRegistrationOptions>;
pub type ReferencesServerCapabilities = OptionsOrBool<ReferenceOptions>;
pub type DocumentHighlightServerCapabilities = OptionsOrBool<DocumentHighlightOptions>;
pub type DocumentSymbolServerCapabilities = OptionsOrBool<DocumentSymbolOptions>;
pub type CodeActionServerCapabilities = OptionsOrBool<CodeActionOptions>;
pub type ColorServerCapabilities =
	RegistrationOptionsOrBool<DocumentColorOptions, DocumentColorRegistrationOptions>;
pub type WorkspaceSymbolServerCapabilities = OptionsOrBool<WorkspaceSymbolOptions>;
pub type DocumentFormattingServerCapabilities = OptionsOrBool<DocumentFormattingOptions>;
pub type DocumentRangeFormattingServerCapabilities = OptionsOrBool<DocumentRangeFormattingOptions>;
pub type RenameServerCapabilities = OptionsOrBool<RenameOptions>;
pub type FoldingRangeServerCapabilities =
	RegistrationOptionsOrBool<FoldingRangeOptions, FoldingRangeRegistrationOptions>;
pub type SelectionRangeServerCapabilities =
	RegistrationOptionsOrBool<SelectionRangeOptions, SelectionRangeRegistrationOptions>;
pub type CallHierarchyServerCapabilities =
	RegistrationOptionsOrBool<CallHierarchyOptions, CallHierarchyRegistrationOptions>;
pub type LinkedEditingRangeServerCapabilities =
	RegistrationOptionsOrBool<LinkedEditingRangeOptions, LinkedEditingRangeRegistrationOptions>;
pub type SemanticTokensServerCapabilities =
	RegistrationOrOptions<SemanticTokensOptions, SemanticTokensRegistrationOptions>;
pub type MonikerServerCapabilities =
	RegistrationOptionsOrBool<MonikerOptions, MonikerRegistrationOptions>;
pub type TypeHierarchyServerCapabilities =
	RegistrationOptionsOrBool<TypeHierarchyOptions, TypeHierarchyRegistrationOptions>;
pub type InlineValueServerCapabilities =
	RegistrationOptionsOrBool<InlineValueOptions, InlineValueRegistrationOptions>;
pub type InlayHintServerCapabilities =
	RegistrationOptionsOrBool<InlayHintOptions, InlayHintRegistrationOptions>;
pub type DiagnosticServerCapabilities =
	RegistrationOrOptions<DiagnosticOptions, DiagnosticRegistrationOptions>;
pub type InlineCompletionServerCapabilities = OptionsOrBool<InlineCompletionOptions>;

/// Workspace specific server capabilities.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceServerCapabilities {
	/// The server supports workspace folder.
	///
	/// since: 3.6.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub workspace_folders: Option<WorkspaceFoldersServerCapabilities>,
	/// The server is interested in notifications/requests for operations on files.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub file_operations: Option<FileOperationOptions>,
}

/// Defines the capabilities provided by a language server.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
	/// The position encoding the server picked from the encodings offered by the client via the client capability
	/// `general.positionEncodings`.
	///
	/// If the client didn't provide any position encodings the only valid value that a server can return is 'utf-16'.
	///
	/// If omitted it defaults to 'utf-16'.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub position_encoding: Option<PositionEncodingKind>,
	/// Defines how text documents are synced.
	///
	/// Is either a detailed structure defining each notification or for backwards
	/// compatibility the [`TextDocumentSyncKind`] number.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text_document_sync: Option<TextDocumentSyncServerCapability>,
	/// Defines how notebook documents are synced.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub notebook_document_sync: Option<NotebookDocumentSyncServerCapability>,
	/// The server provides completion support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub completion_provider: Option<CompletionOptions>,
	/// The server provides hover support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub hover_provider: Option<HoverServerCapabilities>,
	/// The server provides signature help support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub signature_help_provider: Option<SignatureHelpOptions>,
	/// The server provides Goto Declaration support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub declaration_provider: Option<DeclarationServerCapabilities>,
	/// The server provides goto definition support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub definition_provider: Option<DefinitionServerCapabilities>,
	/// The server provides Goto Type Definition support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub type_definition_provider: Option<TypeDefinitionServerCapabilities>,
	/// The server provides Goto Implementation support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub implementation_provider: Option<ImplementationServerCapabilities>,
	/// The server provides find references support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub references_provider: Option<ReferencesServerCapabilities>,
	/// The server provides document highlight support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_highlight_provider: Option<DocumentHighlightServerCapabilities>,
	/// The server provides document symbol support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_symbol_provider: Option<DocumentSymbolServerCapabilities>,
	/// The server provides code actions.
	///
	/// [`CodeActionOptions`] may only be specified if the client states that it supports
	/// `codeActionLiteralSupport` in its initial `initialize` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_action_provider: Option<CodeActionServerCapabilities>,
	/// The server provides code lens.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_lense_provider: Option<CodeLenseOptions>,
	/// The server provides document link support
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_link_provider: Option<DocumentLinkOptions>,
	/// The server provides color provider support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub color_provider: Option<ColorServerCapabilities>,
	/// The server provides workspace symbol support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub workspace_symbol_provider: Option<WorkspaceSymbolServerCapabilities>,
	/// The server provides document formatting.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_formatting_provider: Option<DocumentFormattingServerCapabilities>,
	/// The server provides document range formatting.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_range_formatting_provider: Option<DocumentRangeFormattingServerCapabilities>,
	/// The server provides document formatting on typing.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
	/// The server provides rename support.
	///
	/// [`RenameOptions`] may only be specified if the client states that it supports
	/// `prepareSupport` in its initial `initialize` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub rename_provider: Option<RenameServerCapabilities>,
	/// The server provides folding provider support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub folding_range_provider: Option<FoldingRangeServerCapabilities>,
	/// The server provides selection range support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub selection_range_provider: Option<SelectionRangeServerCapabilities>,
	/// The server provides execute command support.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub execute_command_provider: Option<ExecuteCommandOptions>,
	/// The server provides call hierarchy support.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub call_hierarchy_provider: Option<CallHierarchyServerCapabilities>,
	/// The server provides linked editing range support.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub linked_editing_range_provider: Option<LinkedEditingRangeServerCapabilities>,
	/// The server provides semantic tokens support.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub semantic_tokens_provider: Option<SemanticTokensServerCapabilities>,
	/// The server provides moniker support.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub moniker_provider: Option<MonikerServerCapabilities>,
	/// The server provides type hierarchy support.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub type_hierarchy_provider: Option<TypeHierarchyServerCapabilities>,
	/// The server provides inline values.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inline_value_provider: Option<InlineValueServerCapabilities>,
	/// The server provides inlay hints.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inlay_hint_provider: Option<InlayHintServerCapabilities>,
	/// The server has support for pull model diagnostics.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub diagnostic_provider: Option<DiagnosticServerCapabilities>,
	/// Inline completion options used during static registration.
	///
	/// since: 3.18.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub inline_completion_provider: Option<InlineCompletionServerCapabilities>,
	/// Workspace specific server capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub workspace: Option<WorkspaceServerCapabilities>,
	/// Experimental server capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub experimental: Option<LspAny>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersServerCapabilities {
	/// The server has support for workspace folders
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub supported: Option<bool>,
	/// Whether the server wants to receive workspace folder change notifications.
	///
	/// If a string is provided the string is treated as an ID under which the notification
	/// is registered on the client side. The ID can be used to unregister for these events
	/// using the `client/unregisterCapability` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub change_notification: Option<StringOrBool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum TextDocumentSyncServerCapability {
	Options(TextDocumentSyncOptions),
	Kind(TextDocumentSyncKind),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum NotebookDocumentSyncServerCapability {
	Options(NotebookDocumentSyncOptions),
	Kind(NotebookDocumentSyncRegistrationOptions),
}

#[cfg(test)]
mod tests {
	// use super::*;
}
