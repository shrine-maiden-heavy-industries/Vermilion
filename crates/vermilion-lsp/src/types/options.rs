/* SPDX-License-Identifier: BSD-3-Clause */

use crate::types::{
	CodeActionKind, DocumentSelector, FileOperationFilter, FileSystemWatcher,
	NotebookDocumentSyncOptionsSelector, OptionsOrBool, TextDocumentSyncKind,
	semantic_tokens::SemanticTokensLegend,
};

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressOptions {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub work_done_progress: Option<bool>,
}

/// General text document registration options
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentRegistrationOptions {
	/// A document selector to identify the scope of the registration.
	///
	/// If set to null the document selector provided on the client side will be used
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_selector: Option<DocumentSelector>,
}

/// The options to register for file operations.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationRegistrationOptions {
	/// The actual filters
	pub filters: Vec<FileOperationFilter>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationRegistrationOptions {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub section: Option<Vec<String>>, // Or just String
}

/// Describe options to be used when registered for text document change events.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesRegistrationOptions {
	/// The watchers to register
	pub changes: Vec<FileSystemWatcher>,
}

/// Value-object describing what options formatting should use.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions {
	/// Size of a tab in spaces.
	pub tab_size: u32,
	/// Prefer spaces over tabs.
	pub insert_spaces: bool,
	/// Trim trailing whitespace on a line.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trim_trailing_whitespace: Option<bool>,
	/// Insert a newline character at the end of the file if one does not exist.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_final_newline: Option<bool>,
	/// Trim all newlines after the final newline at the end of the file.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trim_final_newlines: Option<bool>,
}

/// Matching options for the file operation pattern.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationPatternOptions {
	/// The pattern should be matched ignoring casing.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub ignore_case: Option<bool>,
}

/// Options for notifications/requests for user operations on files.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationOptions {
	/// The server is interested in receiving didCreateFiles notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_create: Option<FileOperationRegistrationOptions>,
	/// The server is interested in receiving willCreateFiles requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_create: Option<FileOperationRegistrationOptions>,
	/// The server is interested in receiving didRenameFiles notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_rename: Option<FileOperationRegistrationOptions>,
	/// The server is interested in receiving willRenameFiles requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_rename: Option<FileOperationRegistrationOptions>,
	/// The server is interested in receiving didDeleteFiles file notifications.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_delete: Option<FileOperationRegistrationOptions>,
	/// The server is interested in receiving willDeleteFiles file requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_delete: Option<FileOperationRegistrationOptions>,
}

/// Delete file options
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFileOptions {
	/// Delete the content recursively if a folder is denoted.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub recursive: Option<bool>,
	/// Ignore the operation if the file doesn't exist.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub ignore_if_not_exists: Option<bool>,
}

/// Rename file options
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameFileOptions {
	/// Overwrite target if existing.
	///
	/// Overwrite wins over [`RenameFileOptions::ignore_if_exists`]
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub overwrite: Option<bool>,
	/// Ignores if target exists.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub ignore_if_exists: Option<bool>,
}

/// Options to create a file.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileOptions {
	/// Overwrite existing file.
	///
	/// Overwrite wins over [`CreateFileOptions::ignore_if_exists`]
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub overwrite: Option<bool>,
	/// Ignore if exists.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub ignore_if_exists: Option<bool>,
}

/// Provider options for a [`DocumentOnTypeFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingOptions {
	/// A character on which formatting should be triggered, like `{`.
	pub first_trigger_character: String, // TODO(aki): Should this be a `char`?
	/// More trigger characters.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub more_trigger_character: Option<String>,
}

/// Save options.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SaveOptions {
	/// The client is supposed to include the content on save.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub include_text: Option<bool>,
}

/// Static registration options to be returned in the initialize request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StaticRegistrationOptions {
	/// The id used to register the request.
	///
	/// The id can be used to deregister the request again.
	///
	/// See also [`Registration::id`].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub id: Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub implementation_options: ImplementationOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub type_definition_options: TypeDefinitionOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_color_options: DocumentColorOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub folding_range_options: FoldingRangeOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub declaration_options: DeclarationOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub selection_range_options: SelectionRangeOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Call hierarchy options used during static or dynamic registration.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub call_hierarchy_options: CallHierarchyOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

/// Call hierarchy options used during static registration.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub semantic_tokens_options: SemanticTokensOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensOptions {
	/// The legend used by the server
	pub legend: SemanticTokensLegend,
	/// Server supports providing semantic tokens for a specific range of a document.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range: Option<bool>,
	/// Server supports providing semantic tokens for a full document.
	pub full: Option<bool>, // XXX(aki): Or { "delta": bool }
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub linked_editing_range_options: LinkedEditingRangeOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MonikerRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub moniker_options: MonikerOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MonikerOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Type hierarchy options used during static or dynamic registration.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub type_hierarchy_options: TypeHierarchyOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

/// Type hierarchy options used during static registration.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Inline value options used during static or dynamic registration.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub inline_value_options: InlineValueOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

/// Inline value options used during static registration.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Inlay hint options used during static or dynamic registration.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub inlay_hint_options: InlayHintOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

/// Inlay hint options used during static registration.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintOptions {
	/// The server provides support to resolve additional information for an inlay hint item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_provider: Option<bool>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Diagnostic registration options.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub diagnostic_options: DiagnosticOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

/// Diagnostic options.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticOptions {
	/// An optional identifier under which the diagnostics are managed by the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub identifier: Option<bool>,
	/// Whether the language has inter file dependencies meaning that editing code in one file
	/// can result in a different diagnostic set in another file.
	///
	/// Inter file dependencies are common for most programming languages and typically uncommon
	/// for linters.
	pub inter_file_dependencies: bool,
	/// The server provides support for workspace diagnostics as well.
	pub workspace_diagnostics: bool,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Inline completion options used during static or dynamic registration.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub inline_completion_options: InlineCompletionOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

/// Inline completion options used during static registration.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Describe options to be used when registered for text document change events.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentChangeRegistrationOptions {
	/// How documents are synced to the server.
	pub sync_kind: TextDocumentSyncKind,
	#[serde(flatten)]
	pub options: TextDocumentRegistrationOptions,
}

/// Save registration options.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSaveRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub save_option: SaveOptions,
}

/// Registration options for a [`CompletionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub completion_options: CompletionOptions,
}

/// Registration options for a [`HoverRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct HoverRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub hover_options: HoverOptions,
}

/// Hover options.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct HoverOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`SignatureHelpRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub signature_help_options: SignatureHelpOptions,
}

/// Server Capabilities for a [`SignatureHelpRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpOptions {
	/// List of characters that trigger signature help automatically.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trigger_characters: Option<Vec<String>>, // XXX(aki): Should be `char`?
	/// List of characters that re-trigger signature help.
	///
	/// These trigger characters are only active when signature help is already showing.
	/// All trigger characters\nare also counted as re-trigger characters.
	///
	/// since: 3.15.0
	pub retrigger_characters: Option<Vec<String>>, // XXX(aki): Should be `char`?
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`DefinitionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub definition_options: DefinitionOptions,
}

/// Server Capabilities for a [`DefinitionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`ReferencesRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub reference_options: ReferenceOptions,
}

/// Reference options.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`DocumentHighlightRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_highlight_options: DocumentHighlightOptions,
}

/// Provider options for a [`DocumentHighlightRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`DocumentSymbolRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_symbol_options: DocumentSymbolOptions,
}

/// Provider options for a [`DocumentSymbolRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolOptions {
	/// A human-readable string that is shown when multiple outlines trees are shown for the same document.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub label: Option<String>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`CodeActionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub code_action_options: CodeActionOptions,
}

/// Provider options for a [`CodeActionRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionOptions {
	/// CodeActionKinds that this server may return.
	///
	/// The list of kinds may be generic, such as [`CodeActionKind::Refactor`], or the server may list out
	/// every specific kind they provide.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_action_kinds: Option<Vec<CodeActionKind>>,
	/// The server provides support to resolve additional information for a code action.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_provider: Option<bool>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`WorkspaceSymbolRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolRegistrationOptions {
	#[serde(flatten)]
	pub workspace_symbol_options: WorkspaceSymbolOptions,
}

/// Server capabilities for a [`WorkspaceSymbolRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolOptions {
	/// The server provides support to resolve additional information for a workspace symbol.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_provider: Option<bool>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`CodeLensRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub code_lense_options: CodeLenseOptions,
}

/// Code Lens provider options of a [`CodeLensRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeLenseOptions {
	/// Code lens has a resolve provider as well.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_provider: Option<bool>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`DocumentLinkRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_link_options: DocumentLinkOptions,
}

/// Provider options for a [`DocumentLinkRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkOptions {
	/// Document links have a resolve provider as well
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_provider: Option<bool>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`DocumentFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_formatting_options: DocumentFormattingOptions,
}

/// Provider options for a [`DocumentFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingOptions {
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`DocumentRangeFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_range_formatting_options: DocumentRangeFormattingOptions,
}

/// Provider options for a [`DocumentRangeFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingOptions {
	/// Whether the server supports formatting multiple ranges at once.
	///
	/// since: 3.18.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub ranges_support: Option<bool>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for a [`DocumentOnTypeFormattingRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_on_type_formatting_options: DocumentOnTypeFormattingOptions,
}

/// Registration options for a [`RenameRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameRegistrationOptions {
	#[serde(flatten)]
	pub text_document_registration_options: TextDocumentRegistrationOptions,
	#[serde(flatten)]
	pub document_on_type_formatting_options: RenameOptions,
}

/// Provider options for a [`RenameRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameOptions {
	/// Renames should be checked and tested before being executed.
	///
	/// since: 3.12.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub prepare_provider: Option<bool>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Provider options for a [`ExecuteCommandRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandRegistrationOptions {
	#[serde(flatten)]
	pub work_done_progress_options: ExecuteCommandOptions,
}

/// The server capabilities of a [`ExecuteCommandRequest`].
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandOptions {
	/// The commands to be executed on the server
	pub commands: Vec<String>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemOptions {
	/// The server has support for completion item label details (see also `CompletionItemLabelDetails`) when
	/// receiving a completion item in a resolve call.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub label_details_support: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions {
	/// Most tools trigger completion request automatically without explicitly requesting
	/// it using a keyboard shortcut (e.g. Ctrl+Space).
	///
	/// Typically they do so when the user starts to type an identifier.
	///
	/// For example if the user types `c` in a JavaScript file code complete will automatically pop up
	/// present `console` besides others as a completion item.
	///
	/// Characters that make up identifiers don't need to be listed here.
	///
	/// If code complete should automatically be trigger on characters not being valid inside
	/// an identifier (for example `.` in JavaScript) list them in `trigger_characters`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trigger_characters: Option<Vec<String>>, // XXX(aki): Should this be `char`?
	/// The list of all possible characters that commit a completion.
	///
	/// This field can be used\nif clients don't support individual commit characters per completion item.
	///
	/// See `ClientCapabilities.textDocument.completion.completionItem.commitCharactersSupport`
	///
	/// If a server provides both `allCommitCharacters` and commit characters on an individual
	/// completion item the ones on the completion item win.
	///
	/// since: 3.2.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub all_commit_characters: Option<Vec<String>>, // XXX(aki): Should this be `char`?
	/// The server provides support to resolve additional information for a completion item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub resolve_provider: Option<bool>,
	/// The server supports the following [`CompletionItem`] specific capabilities.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub completion_item: Option<CompletionItemOptions>,
	#[serde(flatten)]
	pub work_done_progress_options: WorkDoneProgressOptions,
}

pub type TextDocumentSaveOptions = OptionsOrBool<SaveOptions>;

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncOptions {
	/// Open and close notifications are sent to the server.
	///
	/// If omitted open close notification should not be sent.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub open_close: Option<bool>,
	/// Change notifications are sent to the server.
	///
	/// See [`TextDocumentSyncKind::None`], [`TextDocumentSyncKind::Full`] and [`TextDocumentSyncKind::Incremental`].
	///
	/// If omitted it defaults to [`TextDocumentSyncKind::None`].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub change: Option<TextDocumentSyncKind>,
	/// If present will save notifications are sent to the server.
	///
	/// If omitted the notification should not be sent.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_save: Option<bool>,
	/// If present will save wait until requests are sent to the server.
	///
	/// If omitted the request should not be sent.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub will_save_wait_until: Option<bool>,
	/// If present save notifications are sent to the server.
	///
	/// If omitted the notification should not be sent.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub save: Option<TextDocumentSaveOptions>,
}

/// Options specific to a notebook plus its cells to be synced to the server.
///
/// If a selector provides a notebook document filter but no cell selector all cells of a
/// matching notebook document will be synced.
///
/// If a selector provides no notebook document filter but only a cell selector all notebook
/// document that contain at least one matching cell will be synced.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncOptions {
	/// Whether save notification should be forwarded to\ the server.
	///
	/// Will only be honored if mode === `notebook`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub save: Option<bool>,
	/// The notebooks to be synced
	pub notebook_selector: NotebookDocumentSyncOptionsSelector,
}

/// Registration options specific to a notebook.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncRegistrationOptions {
	#[serde(flatten)]
	pub notebook_document_sync_options: NotebookDocumentSyncOptions,
	#[serde(flatten)]
	pub static_registration_options: StaticRegistrationOptions,
}

#[cfg(test)]
mod tests {
	// use super::*;
}
