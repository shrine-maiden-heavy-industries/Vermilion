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
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) position_encoding: Option<PositionEncodingKind>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text_document_sync: Option<TextDocumentSyncServerCapability>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) notebook_document_sync: Option<NotebookDocumentSyncServerCapability>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) completion_provider: Option<CompletionOptions>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hover_provider: Option<HoverServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) signature_help_provider: Option<SignatureHelpOptions>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) declaration_provider: Option<DeclarationServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) definition_provider: Option<DefinitionServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) type_definition_provider: Option<TypeDefinitionServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) implementation_provider: Option<ImplementationServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) references_provider: Option<ReferencesServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_highlight_provider: Option<DocumentHighlightServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_symbol_provider: Option<DocumentSymbolServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_action_provider: Option<CodeActionServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_lense_provider: Option<CodeLenseOptions>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_link_provider: Option<DocumentLinkOptions>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) color_provider: Option<ColorServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) workspace_symbol_provider: Option<WorkspaceSymbolServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_formatting_provider: Option<DocumentFormattingServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_range_formatting_provider:
		Option<DocumentRangeFormattingServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) rename_provider: Option<RenameServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) folding_range_provider: Option<FoldingRangeServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) selection_range_provider: Option<SelectionRangeServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) execute_command_provider: Option<ExecuteCommandOptions>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) call_hierarchy_provider: Option<CallHierarchyServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) linked_editing_range_provider: Option<LinkedEditingRangeServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) semantic_tokens_provider: Option<SemanticTokensServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) moniker_provider: Option<MonikerServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) type_hierarchy_provider: Option<TypeHierarchyServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inline_value_provider: Option<InlineValueServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inlay_hint_provider: Option<InlayHintServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) diagnostic_provider: Option<DiagnosticServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) inline_completion_provider: Option<InlineCompletionServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) workspace: Option<WorkspaceServerCapabilities>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) experimental: Option<LspAny>,
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
pub struct WorkspaceFoldersServerCapabilities {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) supported: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) change_notification: Option<StringOrBool>,
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

impl ServerCapabilities {
	pub fn new() -> Self {
		Self {
			position_encoding: None,
			text_document_sync: None,
			notebook_document_sync: None,
			completion_provider: None,
			hover_provider: None,
			signature_help_provider: None,
			declaration_provider: None,
			definition_provider: None,
			type_definition_provider: None,
			implementation_provider: None,
			references_provider: None,
			document_highlight_provider: None,
			document_symbol_provider: None,
			code_action_provider: None,
			code_lense_provider: None,
			document_link_provider: None,
			color_provider: None,
			workspace_symbol_provider: None,
			document_formatting_provider: None,
			document_range_formatting_provider: None,
			document_on_type_formatting_provider: None,
			rename_provider: None,
			folding_range_provider: None,
			selection_range_provider: None,
			execute_command_provider: None,
			call_hierarchy_provider: None,
			linked_editing_range_provider: None,
			semantic_tokens_provider: None,
			moniker_provider: None,
			type_hierarchy_provider: None,
			inline_value_provider: None,
			inlay_hint_provider: None,
			diagnostic_provider: None,
			inline_completion_provider: None,
			workspace: None,
			experimental: None,
		}
	}

	pub fn with_position_encoding(mut self, position_encoding: PositionEncodingKind) -> Self {
		self.position_encoding = Some(position_encoding);
		self
	}

	pub fn with_text_document_sync(
		mut self,
		text_document_sync: TextDocumentSyncServerCapability,
	) -> Self {
		self.text_document_sync = Some(text_document_sync);
		self
	}

	pub fn with_notebook_document_sync(
		mut self,
		notebook_document_sync: NotebookDocumentSyncServerCapability,
	) -> Self {
		self.notebook_document_sync = Some(notebook_document_sync);
		self
	}

	pub fn with_completion_provider(mut self, completion_provider: CompletionOptions) -> Self {
		self.completion_provider = Some(completion_provider);
		self
	}

	pub fn with_hover_provider(mut self, hover_provider: HoverServerCapabilities) -> Self {
		self.hover_provider = Some(hover_provider);
		self
	}

	pub fn with_signature_help_provider(
		mut self,
		signature_help_provider: SignatureHelpOptions,
	) -> Self {
		self.signature_help_provider = Some(signature_help_provider);
		self
	}

	pub fn with_declaration_provider(
		mut self,
		declaration_provider: DeclarationServerCapabilities,
	) -> Self {
		self.declaration_provider = Some(declaration_provider);
		self
	}

	pub fn with_definition_provider(
		mut self,
		definition_provider: DefinitionServerCapabilities,
	) -> Self {
		self.definition_provider = Some(definition_provider);
		self
	}

	pub fn with_type_definition_provider(
		mut self,
		type_definition_provider: TypeDefinitionServerCapabilities,
	) -> Self {
		self.type_definition_provider = Some(type_definition_provider);
		self
	}

	pub fn with_implementation_provider(
		mut self,
		implementation_provider: ImplementationServerCapabilities,
	) -> Self {
		self.implementation_provider = Some(implementation_provider);
		self
	}

	pub fn with_references_provider(
		mut self,
		references_provider: ReferencesServerCapabilities,
	) -> Self {
		self.references_provider = Some(references_provider);
		self
	}

	pub fn with_document_highlight_provider(
		mut self,
		document_highlight_provider: DocumentHighlightServerCapabilities,
	) -> Self {
		self.document_highlight_provider = Some(document_highlight_provider);
		self
	}

	pub fn with_document_symbol_provider(
		mut self,
		document_symbol_provider: DocumentSymbolServerCapabilities,
	) -> Self {
		self.document_symbol_provider = Some(document_symbol_provider);
		self
	}

	pub fn with_code_action_provider(
		mut self,
		code_action_provider: CodeActionServerCapabilities,
	) -> Self {
		self.code_action_provider = Some(code_action_provider);
		self
	}

	pub fn with_code_lense_provider(mut self, code_lense_provider: CodeLenseOptions) -> Self {
		self.code_lense_provider = Some(code_lense_provider);
		self
	}

	pub fn with_document_link_provider(
		mut self,
		document_link_provider: DocumentLinkOptions,
	) -> Self {
		self.document_link_provider = Some(document_link_provider);
		self
	}

	pub fn with_color_provider(mut self, color_provider: ColorServerCapabilities) -> Self {
		self.color_provider = Some(color_provider);
		self
	}

	pub fn with_workspace_symbol_provider(
		mut self,
		workspace_symbol_provider: WorkspaceSymbolServerCapabilities,
	) -> Self {
		self.workspace_symbol_provider = Some(workspace_symbol_provider);
		self
	}

	pub fn with_document_formatting_provider(
		mut self,
		document_formatting_provider: DocumentFormattingServerCapabilities,
	) -> Self {
		self.document_formatting_provider = Some(document_formatting_provider);
		self
	}

	pub fn with_document_range_formatting_provider(
		mut self,
		document_range_formatting_provider: DocumentRangeFormattingServerCapabilities,
	) -> Self {
		self.document_range_formatting_provider = Some(document_range_formatting_provider);
		self
	}

	pub fn with_document_on_type_formatting_provider(
		mut self,
		document_on_type_formatting_provider: DocumentOnTypeFormattingOptions,
	) -> Self {
		self.document_on_type_formatting_provider = Some(document_on_type_formatting_provider);
		self
	}

	pub fn with_rename_provider(mut self, rename_provider: RenameServerCapabilities) -> Self {
		self.rename_provider = Some(rename_provider);
		self
	}

	pub fn with_folding_range_provider(
		mut self,
		folding_range_provider: FoldingRangeServerCapabilities,
	) -> Self {
		self.folding_range_provider = Some(folding_range_provider);
		self
	}

	pub fn with_selection_range_provider(
		mut self,
		selection_range_provider: SelectionRangeServerCapabilities,
	) -> Self {
		self.selection_range_provider = Some(selection_range_provider);
		self
	}

	pub fn with_execute_command_provider(
		mut self,
		execute_command_provider: ExecuteCommandOptions,
	) -> Self {
		self.execute_command_provider = Some(execute_command_provider);
		self
	}

	pub fn with_call_hierarchy_provider(
		mut self,
		call_hierarchy_provider: CallHierarchyServerCapabilities,
	) -> Self {
		self.call_hierarchy_provider = Some(call_hierarchy_provider);
		self
	}

	pub fn with_linked_editing_range_provider(
		mut self,
		linked_editing_range_provider: LinkedEditingRangeServerCapabilities,
	) -> Self {
		self.linked_editing_range_provider = Some(linked_editing_range_provider);
		self
	}

	pub fn with_semantic_tokens_provider(
		mut self,
		semantic_tokens_provider: SemanticTokensServerCapabilities,
	) -> Self {
		self.semantic_tokens_provider = Some(semantic_tokens_provider);
		self
	}

	pub fn with_moniker_provider(mut self, moniker_provider: MonikerServerCapabilities) -> Self {
		self.moniker_provider = Some(moniker_provider);
		self
	}

	pub fn with_type_hierarchy_provider(
		mut self,
		type_hierarchy_provider: TypeHierarchyServerCapabilities,
	) -> Self {
		self.type_hierarchy_provider = Some(type_hierarchy_provider);
		self
	}

	pub fn with_inline_value_provider(
		mut self,
		inline_value_provider: InlineValueServerCapabilities,
	) -> Self {
		self.inline_value_provider = Some(inline_value_provider);
		self
	}

	pub fn with_inlay_hint_provider(
		mut self,
		inlay_hint_provider: InlayHintServerCapabilities,
	) -> Self {
		self.inlay_hint_provider = Some(inlay_hint_provider);
		self
	}

	pub fn with_diagnostic_provider(
		mut self,
		diagnostic_provider: DiagnosticServerCapabilities,
	) -> Self {
		self.diagnostic_provider = Some(diagnostic_provider);
		self
	}

	pub fn with_inline_completion_provider(
		mut self,
		inline_completion_provider: InlineCompletionServerCapabilities,
	) -> Self {
		self.inline_completion_provider = Some(inline_completion_provider);
		self
	}

	pub fn with_workspace(mut self, workspace: WorkspaceServerCapabilities) -> Self {
		self.workspace = Some(workspace);
		self
	}

	pub fn with_experimental(mut self, experimental: serde_json::Value) -> Self {
		self.experimental = Some(experimental);
		self
	}

	/// The position encoding the server picked from the encodings offered by the client via the client capability
	/// `general.positionEncodings`.
	///
	/// If the client didn't provide any position encodings the only valid value that a server can return is 'utf-16'.
	///
	/// If omitted it defaults to 'utf-16'.
	///
	/// since: 3.17.0
	pub fn position_encoding(&self) -> Option<&PositionEncodingKind> {
		self.position_encoding.as_ref()
	}

	/// Defines how text documents are synced.
	///
	/// Is either a detailed structure defining each notification or for backwards
	/// compatibility the [`TextDocumentSyncKind`] number.
	pub fn text_document_sync(&self) -> Option<&TextDocumentSyncServerCapability> {
		self.text_document_sync.as_ref()
	}

	/// Defines how notebook documents are synced.
	///
	/// since: 3.17.0
	pub fn notebook_document_sync(&self) -> Option<&NotebookDocumentSyncServerCapability> {
		self.notebook_document_sync.as_ref()
	}

	/// The server provides completion support.
	pub fn completion_provider(&self) -> Option<&CompletionOptions> {
		self.completion_provider.as_ref()
	}

	/// The server provides hover support.
	pub fn hover_provider(&self) -> Option<&HoverServerCapabilities> {
		self.hover_provider.as_ref()
	}

	/// The server provides signature help support.
	pub fn signature_help_provider(&self) -> Option<&SignatureHelpOptions> {
		self.signature_help_provider.as_ref()
	}

	/// The server provides Goto Declaration support.
	pub fn declaration_provider(&self) -> Option<&DeclarationServerCapabilities> {
		self.declaration_provider.as_ref()
	}

	/// The server provides goto definition support.
	pub fn definition_provider(&self) -> Option<&DefinitionServerCapabilities> {
		self.definition_provider.as_ref()
	}

	/// The server provides Goto Type Definition support.
	pub fn type_definition_provider(&self) -> Option<&TypeDefinitionServerCapabilities> {
		self.type_definition_provider.as_ref()
	}

	/// The server provides Goto Implementation support.
	pub fn implementation_provider(&self) -> Option<&ImplementationServerCapabilities> {
		self.implementation_provider.as_ref()
	}

	/// The server provides find references support.
	pub fn references_provider(&self) -> Option<&ReferencesServerCapabilities> {
		self.references_provider.as_ref()
	}

	/// The server provides document highlight support.
	pub fn document_highlight_provider(&self) -> Option<&DocumentHighlightServerCapabilities> {
		self.document_highlight_provider.as_ref()
	}

	/// The server provides document symbol support.
	pub fn document_symbol_provider(&self) -> Option<&DocumentSymbolServerCapabilities> {
		self.document_symbol_provider.as_ref()
	}

	/// The server provides code actions.
	///
	/// [`CodeActionOptions`] may only be specified if the client states that it supports
	/// `codeActionLiteralSupport` in its initial `initialize` request.
	pub fn code_action_provider(&self) -> Option<&CodeActionServerCapabilities> {
		self.code_action_provider.as_ref()
	}

	/// The server provides code lens.
	pub fn code_lense_provider(&self) -> Option<&CodeLenseOptions> {
		self.code_lense_provider.as_ref()
	}

	/// The server provides document link support
	pub fn document_link_provider(&self) -> Option<&DocumentLinkOptions> {
		self.document_link_provider.as_ref()
	}

	/// The server provides color provider support.
	pub fn color_provider(&self) -> Option<&ColorServerCapabilities> {
		self.color_provider.as_ref()
	}

	/// The server provides workspace symbol support.
	pub fn workspace_symbol_provider(&self) -> Option<&WorkspaceSymbolServerCapabilities> {
		self.workspace_symbol_provider.as_ref()
	}

	/// The server provides document formatting.
	pub fn document_formatting_provider(&self) -> Option<&DocumentFormattingServerCapabilities> {
		self.document_formatting_provider.as_ref()
	}

	/// The server provides document range formatting.
	pub fn document_range_formatting_provider(
		&self,
	) -> Option<&DocumentRangeFormattingServerCapabilities> {
		self.document_range_formatting_provider.as_ref()
	}

	/// The server provides document formatting on typing.
	pub fn document_on_type_formatting_provider(&self) -> Option<&DocumentOnTypeFormattingOptions> {
		self.document_on_type_formatting_provider.as_ref()
	}

	/// The server provides rename support.
	///
	/// [`RenameOptions`] may only be specified if the client states that it supports
	/// `prepareSupport` in its initial `initialize` request.
	pub fn rename_provider(&self) -> Option<&RenameServerCapabilities> {
		self.rename_provider.as_ref()
	}

	/// The server provides folding provider support.
	pub fn folding_range_provider(&self) -> Option<&FoldingRangeServerCapabilities> {
		self.folding_range_provider.as_ref()
	}

	/// The server provides selection range support.
	pub fn selection_range_provider(&self) -> Option<&SelectionRangeServerCapabilities> {
		self.selection_range_provider.as_ref()
	}

	/// The server provides execute command support.
	pub fn execute_command_provider(&self) -> Option<&ExecuteCommandOptions> {
		self.execute_command_provider.as_ref()
	}

	/// The server provides call hierarchy support.
	///
	/// since: 3.16.0
	pub fn call_hierarchy_provider(&self) -> Option<&CallHierarchyServerCapabilities> {
		self.call_hierarchy_provider.as_ref()
	}

	/// The server provides linked editing range support.
	///
	/// since: 3.16.0
	pub fn linked_editing_range_provider(&self) -> Option<&LinkedEditingRangeServerCapabilities> {
		self.linked_editing_range_provider.as_ref()
	}

	/// The server provides semantic tokens support.
	///
	/// since: 3.16.0
	pub fn semantic_tokens_provider(&self) -> Option<&SemanticTokensServerCapabilities> {
		self.semantic_tokens_provider.as_ref()
	}

	/// The server provides moniker support.
	///
	/// since: 3.16.0
	pub fn moniker_provider(&self) -> Option<&MonikerServerCapabilities> {
		self.moniker_provider.as_ref()
	}

	/// The server provides type hierarchy support.
	///
	/// since: 3.17.0
	pub fn type_hierarchy_provider(&self) -> Option<&TypeHierarchyServerCapabilities> {
		self.type_hierarchy_provider.as_ref()
	}

	/// The server provides inline values.
	///
	/// since: 3.17.0
	pub fn inline_value_provider(&self) -> Option<&InlineValueServerCapabilities> {
		self.inline_value_provider.as_ref()
	}

	/// The server provides inlay hints.
	///
	/// since: 3.17.0
	pub fn inlay_hint_provider(&self) -> Option<&InlayHintServerCapabilities> {
		self.inlay_hint_provider.as_ref()
	}

	/// The server has support for pull model diagnostics.
	///
	/// since: 3.17.0
	pub fn diagnostic_provider(&self) -> Option<&DiagnosticServerCapabilities> {
		self.diagnostic_provider.as_ref()
	}

	/// Inline completion options used during static registration.
	///
	/// since: 3.18.0
	pub fn inline_completion_provider(&self) -> Option<&InlineCompletionServerCapabilities> {
		self.inline_completion_provider.as_ref()
	}

	/// Workspace specific server capabilities.
	pub fn workspace(&self) -> Option<&WorkspaceServerCapabilities> {
		self.workspace.as_ref()
	}

	/// Experimental server capabilities.
	pub fn experimental(&self) -> Option<&LspAny> {
		self.experimental.as_ref()
	}
}

impl WorkspaceFoldersServerCapabilities {
	pub fn new() -> Self {
		Self { supported: None, change_notification: None }
	}

	pub fn with_supported(mut self, supported: bool) -> Self {
		self.supported = Some(supported);
		self
	}

	pub fn with_change_notification(mut self, change_notification: StringOrBool) -> Self {
		self.change_notification = Some(change_notification);
		self
	}

	/// The server has support for workspace folders
	pub fn supported(&self) -> Option<&bool> {
		self.supported.as_ref()
	}

	/// Whether the server wants to receive workspace folder change notifications.
	///
	/// If a string is provided the string is treated as an ID under which the notification
	/// is registered on the client side. The ID can be used to unregister for these events
	/// using the `client/unregisterCapability` request.
	pub fn change_notification(&self) -> Option<&StringOrBool> {
		self.change_notification.as_ref()
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
