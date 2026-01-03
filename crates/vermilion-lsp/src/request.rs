// SPDX-License-Identifier: BSD-3-Clause

use crate::{
	message::Id,
	response::Response,
	types::{
		CodeAction, CodeLens, CompletionItem, DocumentLink, InlayHint, WorkspaceSymbol,
		params::{
			ApplyWorkspaceEditParams, CallHierarchyIncomingCallsParams,
			CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams, CodeActionParams,
			CodeLensParams, ColorPresentationParams, CompletionParams, ConfigurationParams,
			CreateFilesParams, DeclarationParams, DefinitionParams, DeleteFilesParams,
			DocumentColorParams, DocumentDiagnosticParams, DocumentFormattingParams,
			DocumentHighlightParams, DocumentLinkParams, DocumentOnTypeFormattingParams,
			DocumentRangeFormattingParams, DocumentRangesFormattingParams, DocumentSymbolParams,
			ExecuteCommandParams, FoldingRangeParams, HoverParams, ImplementationParams,
			InitializedParams, InlayHintParams, InlineCompletionParams, InlineValueParams,
			LinkedEditingRangeParams, MonikerParams, PrepareRenameParams, ReferenceParams,
			RegistrationParams, RenameFilesParams, RenameParams, SelectionRangeParams,
			SemanticTokensDeltaParams, SemanticTokensParams, SemanticTokensRangeParams,
			ShowDocumentParams, ShowMessageRequestParams, SignatureHelpParams,
			TypeDefinitionParams, TypeHierarchyPrepareParams, TypeHierarchySubtypesParams,
			TypeHierarchySupertypesParams, UnregistrationParams, WillSaveTextDocumentParams,
			WorkDoneProgressCreateParams, WorkspaceDiagnosticParams, WorkspaceSymbolParams,
		},
	},
};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Request {
	pub(crate) id:  Id,
	#[serde(flatten)]
	pub(crate) req: RequestType,
}

impl Request {
	pub fn new(id: Id, request: RequestType) -> Self {
		Self { id, req: request }
	}

	pub fn id(&self) -> &Id {
		&self.id
	}

	pub fn request(&self) -> &RequestType {
		&self.req
	}

	pub fn response(&self) -> Response {
		Response::new(self.id.clone())
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "method", content = "params")]
pub enum RequestType {
	// clientToServer
	// Result: Some(Definition) | Some(Vec<DefinitionLink>) | None
	// Partial Result: Vec<Location> | Vec<DefinitionLink>
	// Registration Options: ImplementationRegistrationOptions
	/// A request to resolve the implementation locations of a symbol at a given text document
	/// position.
	///
	/// The request's parameter is of type [`TextDocumentPositionParams`] the response is of type
	/// [`Definition`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/implementation")]
	TextDocumentImplementation(ImplementationParams),
	// clientToServer
	// Result: Some(Definition) | Some(Vec<DefinitionLink>) | None
	// Partial Result: Vec<Location> | Vec<DefinitionLink>
	// Registration Options: TypeDefinitionRegistrationOptions
	/// A request to resolve the type definition locations of a symbol at a given text document
	/// position.
	///
	/// The request's parameter is of type [`TextDocumentPositionParams`] the response is of type
	/// [`Definition`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/typeDefinition")]
	TextDocumentTypeDefinition(TypeDefinitionParams),
	// serverToClient
	// Result: Some(Vec<WorkspaceFolder>) | None
	/// The `workspace/workspaceFolders` is sent from the server to the client to fetch the open
	/// workspace folders.
	#[serde(rename = "workspace/workspaceFolders")]
	WorkspaceWorkspaceFolders,
	// serverToClient
	// Result: Vec<LspAny>
	/// The 'workspace/configuration' request is sent from the server to the client to fetch a
	/// certain configuration setting.
	///
	/// This pull model replaces the old push model where the client signaled configuration change
	/// via an event. If the server still needs to react to configuration changes (since the server
	/// caches the result of `workspace/configuration` requests) the server should register for an
	/// empty configuration change event and empty the cache if such an event is received.
	#[serde(rename = "workspace/configuration")]
	WorkspaceConfiguration(ConfigurationParams),
	// clientToServer
	// Result: Vec<ColorInformation>
	// Partial Result: Vec<ColorInformation>
	// Registration Options: DocumentColorRegistrationOptions
	/// A request to list all color symbols found in a given text document.
	///
	/// The request's parameter is of type [`DocumentColorParams`] the response is of type
	/// [`ColorInformation`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/documentColor")]
	TextDocumentDocumentColor(DocumentColorParams),
	// clientToServer
	// Result: Vec<ColorPresentation>
	// Partial Result: Vec<ColorPresentation>
	// Registration Options: WorkDoneProgressOptions & TextDocumentRegistrationOptions
	/// A request to list all presentation for a color.
	///
	/// The request's parameter is of type [`ColorPresentationParams`] the response is of type
	/// [`ColorPresentation`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/colorPresentation")]
	TextDocumentColorPresentation(ColorPresentationParams),
	// clientToServer
	// Result: Some(Vec<FoldingRange>)
	// Partial Result: Vec<FoldingRange>
	// Registration Options: FoldingRangeRegistrationOptions
	/// A request to provide folding ranges in a document.
	///
	/// The request's parameter is of type [`FoldingRangeParams`], the response is of type
	/// [`FoldingRangeList`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/foldingRange")]
	TextDocumentFoldingRange(FoldingRangeParams),
	// serverToClient
	// Result: None
	/// since: 3.18.0
	#[serde(rename = "workspace/foldingRange/refresh")]
	WorkspaceFoldingRangeRefresh,
	// clientToServer
	// Result: Some(Definition) | Some(Vec<DefinitionLink>) | None
	// Partial Result: Vec<Location> | Vec<DefinitionLink>
	// Registration Options: DeclarationRegistrationOptions
	/// A request to resolve the type definition locations of a symbol at a given text document
	/// position.
	///
	/// The request's parameter is of type [`TextDocumentPositionParams`] the response is of type
	/// [`Declaration`] or a typed array of [`DeclarationLink`] or a Thenable that resolves to such
	#[serde(rename = "textDocument/declaration")]
	TextDocumentDeclaration(DeclarationParams),
	// clientToServer
	// Result: Some(Vec<SelectionRange>) | None
	// Partial Result: Vec<SelectionRange>
	// Registration Options: SelectionRangeRegistrationOptions
	/// A request to provide selection ranges in a document.
	///
	/// The request's parameter is of type [`SelectionRangeParams`], the response is of type
	/// [`SelectionRange`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/selectionRange")]
	TextDocumentSelectionRange(SelectionRangeParams),
	// serverToClient
	// Result: None
	/// The `window/workDoneProgress/create` request is sent from the server to the client to
	/// initiate progress reporting from the server.
	#[serde(rename = "window/workDoneProgress/create")]
	WindowWorkDoneProgressCreate(WorkDoneProgressCreateParams),
	// clientToServer
	// Result: Some(Vec<CallHierarchyItem>) | None
	// Registration Options: CallHierarchyRegistrationOptions
	/// A request to result a `CallHierarchyItem` in a document at a given position.
	///
	/// Can be used as an input to an incoming or outgoing call hierarchy.
	///
	/// since: 3.16.0
	#[serde(rename = "textDocument/prepareCallHierarchy")]
	TextDocumentPrepareCallHierarchy(CallHierarchyPrepareParams),
	// clientToServer
	// Result: Some(Vec<CallHierarchyIncomingCall>) | None
	// Partial Result: Vec<CallHierarchyIncomingCall>
	/// A request to resolve the incoming calls for a given `CallHierarchyItem`.
	///
	/// since: 3.16.0
	#[serde(rename = "callHierarchy/incomingCalls")]
	CallHierarchyIncomingCalls(CallHierarchyIncomingCallsParams),
	// clientToServer
	// Result: Some(Vec<CallHierarchyOutgoingCall>) | None
	// Partial Result: Vec<CallHierarchyOutgoingCall>
	/// A request to resolve the outgoing calls for a given `CallHierarchyItem`.
	///
	/// since: 3.16.0
	#[serde(rename = "callHierarchy/outgoingCalls")]
	CallHierarchyOutgoingCalls(CallHierarchyOutgoingCallsParams),
	// clientToServer
	// Result: Some(SemanticTokens) | None
	// Partial Result: SemanticTokensPartialResult
	// Registration Options: SemanticTokensRegistrationOptions
	// Registration Method: textDocument/semanticTokens
	/// since: 3.16.0
	#[serde(rename = "textDocument/semanticTokens/full")]
	TextDocumentSemanticTokensFull(SemanticTokensParams),
	// clientToServer
	// Result: Some(SemanticTokens) | Some(SemanticTokensDelta) | None
	// Partial Result: SemanticTokensDeltaPartialResult | SemanticTokensPartialResult
	// Registration Options: SemanticTokensRegistrationOptions
	// Registration Method: textDocument/semanticTokens
	/// since: 3.16.0
	#[serde(rename = "textDocument/semanticTokens/full/delta")]
	TextDocumentSemanticTokensFullDelta(SemanticTokensDeltaParams),
	// clientToServer
	// Result: Some(SemanticTokens) | None
	// Partial Result: SemanticTokensPartialResult
	// Registration Method: textDocument/semanticTokens
	/// since: 3.16.0
	#[serde(rename = "textDocument/semanticTokens/range")]
	TextDocumentSemanticTokensRange(SemanticTokensRangeParams),
	// serverToClient
	// Result: None
	/// since: 3.16.0
	#[serde(rename = "workspace/semanticTokens/refresh")]
	WorkspaceSemanticTokensRefresh,
	// serverToClient
	// Result: ShowDocumentResult
	/// A request to show a document.
	///
	/// This request might open an external program depending on the value of the URI to open.
	///
	/// For example a request to open `https://catgirl.stream/` will very likely open the URI in a web browser.
	///
	/// since: 3.16.0
	#[serde(rename = "window/showDocument")]
	WindowShowDocument(ShowDocumentParams),
	// clientToServer
	// Result: Some(LinkedEditingRanges) | None
	// Registration Options: LinkedEditingRangeRegistrationOptions
	/// A request to provide ranges that can be edited together.
	///
	/// since: 3.16.0
	#[serde(rename = "textDocument/linkedEditingRange")]
	TextDocumentLinkedEditingRange(LinkedEditingRangeParams),
	// clientToServer
	// Result: Some(WorkspaceEdit) | None
	// Registration Options: FileOperationRegistrationOptions
	/// The will create files request is sent from the client to the server before files are
	/// actually created as long as the creation is triggered from within the client.
	///
	/// The request can return a [`WorkspaceEdit`] which will be applied to workspace before the
	/// files are created. Hence the [`WorkspaceEdit`] can not manipulate the content of the file to
	/// be created.
	///
	/// since: 3.16.0
	#[serde(rename = "workspace/willCreateFiles")]
	WorkspaceWillCreateFiles(CreateFilesParams),
	// clientToServer
	// Result: Some(WorkspaceEdit) | None
	// Registration Options: FileOperationRegistrationOptions
	/// The will rename files request is sent from the client to the server before files are
	/// actually renamed as long as the rename is triggered from within the client.
	///
	/// since: 3.16.0
	#[serde(rename = "workspace/willRenameFiles")]
	WorkspaceWillRenameFiles(RenameFilesParams),
	// clientToServer
	// Result: Some(WorkspaceEdit) | None
	// Registration Options: FileOperationRegistrationOptions
	/// The did delete files notification is sent from the client to the server when
	/// files were deleted from within the client.
	///
	/// since: 3.16.0
	#[serde(rename = "workspace/willDeleteFiles")]
	WorkspaceWillDeleteFiles(DeleteFilesParams),
	// clientToServer
	// Result: Some(Vec<Moniker>) | None
	// Partial Result: Vec<Moniker>
	// Registration Options: MonikerRegistrationOptions
	/// A request to get the moniker of a symbol at a given text document position.
	///
	/// The request parameter is of type [`TextDocumentPositionParams`].
	///
	/// The response is of type [`Moniker`] or `null`.
	#[serde(rename = "textDocument/moniker")]
	TextDocumentMoniker(MonikerParams),
	// clientToServer
	// Result: Some(Vec<TypeHierarchyItem>) | None
	// Registration Options: TypeHierarchyRegistrationOptions
	/// A request to result a [`TypeHierarchyItem`] in a document at a given position.
	///
	/// Can be used as an input to a subtypes or supertypes type hierarchy.
	///
	/// since: 3.17.0
	#[serde(rename = "textDocument/prepareTypeHierarchy")]
	TextDocumentPrepareTypeHierarchy(TypeHierarchyPrepareParams),
	// clientToServer
	// Result: Some(Vec<TypeHierarchyItem>) | None
	// Partial Result: Vec<TypeHierarchyItem>
	/// A request to resolve the supertypes for a given [`TypeHierarchyItem`].
	///
	/// since: 3.17.0
	#[serde(rename = "typeHierarchy/supertypes")]
	TypeHierarchySupertypes(TypeHierarchySupertypesParams),
	// clientToServer
	// Result: Some(Vec<TypeHierarchyItem>) | None
	// Partial Result: Vec<TypeHierarchyItem>
	/// A request to resolve the subtypes for a given [`TypeHierarchyItem`].
	///
	/// since: 3.17.0
	#[serde(rename = "typeHierarchy/subtypes")]
	TypeHierarchySubtypes(TypeHierarchySubtypesParams),
	// clientToServer
	// Result: Some(Vec<InlineValue>) | None
	// Partial Result: Vec<InlineValue>
	// Registration Options: InlineValueRegistrationOptions
	/// A request to provide inline values in a document.
	///
	/// The request's parameter is of type [`InlineValueParams`], the response is of type
	/// [`InlineValue`] or a Thenable that resolves to such.
	///
	/// since: 3.17.0
	#[serde(rename = "textDocument/inlineValue")]
	TextDocumentInlineValue(InlineValueParams),
	// serverToClient
	// Result: None
	/// since: 3.17.0
	#[serde(rename = "workspace/inlineValue/refresh")]
	WorkspaceInlineValueRefresh,
	// clientToServer
	// Result: Some(Vec<InlayHint>)
	// Partial Result: Vec<InlayHint>
	// Registration Options: InlayHintRegistrationOptions
	/// A request to provide inlay hints in a document.
	///
	/// The request's parameter is of type [`InlayHintsParams`], the response is of type
	/// [`InlayHint`]or a Thenable that resolves to such.
	///
	/// since: 3.17.0
	#[serde(rename = "textDocument/inlayHint")]
	TextDocumentInlayHint(InlayHintParams),
	// clientToServer
	// Result: InlayHint
	/// A request to resolve additional properties for an inlay hint.
	///
	/// The request's parameter is of type [`InlayHint`], the response is of type [`InlayHint`] or a
	/// Thenable that resolves to such.
	///
	/// since: 3.17.0
	#[serde(rename = "inlayHint/resolve")]
	InlayHintResolve(InlayHint),
	// serverToClient
	// Result: None
	/// since: 3.17.0
	#[serde(rename = "workspace/inlayHint/refresh")]
	InlayHintRefresh,
	// clientToServer
	// Result: DocumentDiagnosticReport
	// Partial Result: DocumentDiagnosticReportPartialResult
	// Error Data: DiagnosticServerCancellationData
	// Registration Options: DiagnosticRegistrationOptions
	/// The document diagnostic request definition.
	///
	/// since: 3.17.0
	#[serde(rename = "textDocument/diagnostic")]
	TextDocumentDiagnostic(DocumentDiagnosticParams),
	// clientToServer
	// Result: WorkspaceDiagnosticReport
	// Partial Result: WorkspaceDiagnosticReportPartialResult
	// Error Data: DiagnosticServerCancellationData
	/// The workspace diagnostic request definition.
	///
	/// since: 3.17.0
	#[serde(rename = "workspace/diagnostic")]
	WorkspaceDiagnostic(WorkspaceDiagnosticParams),
	// serverToClient
	// Result: None
	/// The diagnostic refresh request definition.
	///
	/// since: 3.17.0
	#[serde(rename = "workspace/diagnostic/refresh")]
	WorkspaceDiagnosticRefresh,
	// clientToServer
	// Result: Some(Vec<InlineCompletionItem>) | Some(InlineCompletionList) | None
	// Partial Result: Vec<InlineCompletionItem>
	// Registration Options: InlineCompletionRegistrationOptions
	/// A request to provide inline completions in a document.
	///
	/// The request's parameter is of\ntype [`InlineCompletionParams`], the response is of type
	/// [`InlineCompletion`] or a Thenable that resolves to such.
	///
	/// since: 3.18.0
	#[serde(rename = "textDocument/inlineCompletion")]
	TextDocumentInlineCompletion(InlineCompletionParams),
	// serverToClient
	// Result: None
	/// The `client/registerCapability` request is sent from the server to the client to register a
	/// new capability handler on the client side.
	#[serde(rename = "client/registerCapability")]
	ClientRegisterCapability(RegistrationParams),
	// serverToClient
	// Result: None
	/// The `client/unregisterCapability` request is sent from the server to the client to
	/// unregister a previously registered capability handler on the client side.
	#[serde(rename = "client/unregisterCapability")]
	ClientUnregisterCapability(UnregistrationParams),
	// clientToServer
	// Result: InitializeResult
	// Error Data: InitializeError
	/// The initialize request is sent from the client to the server.
	///
	/// It is sent once as the request after starting up the server.
	///
	/// The requests parameter is of type [`InitializeParams`] the response if of type
	/// [`InitializeResult`] of a Thenable that resolves to such.
	#[serde(rename = "initialize")]
	Initialize(Box<InitializedParams>),
	// clientToServer
	// Result: None
	/// A shutdown request is sent from the client to the server.
	///
	/// It is sent once when the client decides to shutdown the server.
	///
	/// The only notification that is sent after a shutdown request is the exit event.
	#[serde(rename = "shutdown")]
	Shutdown,
	// serverToClient
	// Result: MessageActionItem
	/// The show message request is sent from the server to the client to show a message and a
	/// set of options actions to the user.
	#[serde(rename = "window/showMessageRequest")]
	WindowShowMessageRequest(ShowMessageRequestParams),
	// clientToServer
	// Result: Some(Vec<TextEdit>) | None
	// Registration Options: TextDocumentRegistrationOptions
	/// A document will save request is sent from the client to the server before the document is
	/// actually saved.
	///
	/// The request can return an array of [`TextEdits`] which will be applied to the text document
	/// before it is saved.
	///
	/// Please note that clients might drop results if computing the text edits took too long or if
	/// a server constantly fails on this request.
	///
	/// This is done to keep the save fast and reliable.
	#[serde(rename = "textDocument/willSaveWaitUntil")]
	TextDocumentWillSaveWaitUntil(WillSaveTextDocumentParams),
	// clientToServer
	// Result: Some(Vec<CompletionItem>) | Some(CompletionList) | None
	// Partial Result: Vec<CompletionItem>
	// Registration Options: CompletionRegistrationOptions
	/// Request to request completion at a given text document position.
	///
	/// The request's parameter is of type [`TextDocumentPosition`] the response is of type
	/// [`CompletionItem`] or [`CompletionList`] or a Thenable that resolves to such.
	///
	/// The request can delay the computation of the [`CompletionItem`] and [`CompletionItem`]
	/// properties to the `completionItem/resolve` request.
	///
	/// However, properties that are needed for the initial sorting and filtering, like `sortText`,
	/// `filterText`, `insertText`, and `textEdit`, must not be changed during resolve.
	#[serde(rename = "textDocument/completion")]
	TextDocumentCompletion(CompletionParams),
	// clientToServer
	// Result: CompletionItem
	/// Request to resolve additional information for a given completion item.The request's
	/// parameter is of type [`CompletionItem`] the response is of type [`CompletionItem`] or a
	/// Thenable that resolves to such.
	#[serde(rename = "completionItem/resolve")]
	CompletionItemResolve(CompletionItem),
	// clientToServer
	// Result: Some(Hover) | None
	// Registration Options: HoverRegistrationOptions
	/// Request to request hover information at a given text document position.
	///
	/// The request's parameter is of type [`TextDocumentPosition`] the response is of type
	/// [`Hover`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/hover")]
	TextDocumentHover(HoverParams),
	// clientToServe
	// Result: Some(SignatureHelp) | None
	// Registration Options: SignatureHelpRegistrationOptions
	#[serde(rename = "textDocument/signatureHelp")]
	TextDocumentSignatureHelp(SignatureHelpParams),
	// clientToServer
	// Result: Some(Vec<DefinitionLink>) | Some(Vec<Location>) | None
	// Partial Result: Vec<DefinitionLink> | Vec<Location>
	// Registration Options: DefinitionRegistrationOptions
	/// A request to resolve the definition location of a symbol at a given text document position.
	///
	/// The request's parameter is of type [`TextDocumentPosition`] the response is of either type
	/// [`Definition`] or a typed array of [`DefinitionLink`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/definition")]
	TextDocumentDefinition(DefinitionParams),
	// clientToServer
	// Result: Some(Vec<Location>) | None
	// Partial Result: Vec<Location>
	// Registration Options: ReferenceRegistrationOptions
	/// A request to resolve project-wide references for the symbol denoted by the given text
	/// document position.
	///
	/// The request's parameter is of type [`ReferenceParams`] the response is of type [`Location`]
	/// or a Thenable that resolves to such.
	#[serde(rename = "textDocument/references")]
	TextDocumentReferences(ReferenceParams),
	// clientToServer
	// Result: Some(Vec<DocumentHighlight>) | None
	// Partial Result: Vec<DocumentHighlight>
	// Registration Options: DocumentHighlightRegistrationOptions
	/// Request to resolve a [`DocumentHighlight`] for a given text document position.
	///
	/// The request's parameter is of type [`TextDocumentPosition`] the request response is an array
	/// of type [`DocumentHighlight`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/documentHighlight")]
	TextDocumentDocumentHighlight(DocumentHighlightParams),
	// clientToServer
	// Result: Some(Vec<SymbolInformation>) | Some(Vec<DocumentSymbol>) | None
	// Partial Result: Vec<SymbolInformation> | Vec<DocumentSymbol>
	// Registration Options: DocumentSymbolRegistrationOptions
	/// A request to list all symbols found in a given text document.
	///
	/// The request's parameter is of type [`TextDocumentIdentifier`] the response is of type
	/// [`SymbolInformation`] or a Thenable that resolves to such.
	#[serde(rename = "textDocument/documentSymbol")]
	TextDocumentDocumentSymbol(DocumentSymbolParams),
	// clientToServer
	// Result: Some(Vec<CodeAction | Command>)) | None
	// Partial Result: Vec<CodeAction | Command>
	// Registration Options: CodeActionRegistrationOptions
	/// A request to provide commands for the given text document and range.
	#[serde(rename = "textDocument/codeAction")]
	TextDocumentCodeAction(CodeActionParams),
	// clientToServer
	// Result: CodeAction
	/// Request to resolve additional information for a given code action.
	///
	/// The request's parameter is of type [`CodeAction`] the response is of type [`CodeAction`] or
	/// a Thenable that resolves to such.
	#[serde(rename = "codeAction/resolve")]
	CodeActionResolve(CodeAction),
	// clientToServer
	// Result: Some(Vec<SymbolInformation>) | Some(Vec<WorkspaceSymbol>) | None
	// Partial Result: Vec<SymbolInformation> | Vec<WorkspaceSymbol>
	// Registration Options: WorkspaceSymbolRegistrationOptions
	/// A request to list project-wide symbols matching the query string given by the
	/// [`WorkspaceSymbolParams`].
	///
	/// The response is of type [`SymbolInformation`] or a Thenable that resolves to such.
	///
	/// since: 3.17.0 - support for [`WorkspaceSymbol`] in the returned data.
	///
	/// Clients need to advertise support for WorkspaceSymbols via the client capability
	///  `workspace.symbol.resolveSupport`.
	#[serde(rename = "workspace/symbol")]
	WorkspaceSymbol(WorkspaceSymbolParams),
	// clientToServer
	// Result: WorkspaceSymbol
	/// A request to resolve the range inside the workspace symbol's location.
	///
	/// since: 3.17.0
	#[serde(rename = "workspaceSymbol/resolve")]
	WorkspaceSymbolResolve(WorkspaceSymbol),
	// clientToServer
	// Result: Some(Vec<CodeLens>) | None
	// Partial Result: Vec<CodeLens>
	// Registration Options: CodeLensRegistrationOptions
	/// A request to provide code lens for the given text document.
	#[serde(rename = "textDocument/codeLens")]
	TextDocumentCodeLens(CodeLensParams),
	// clientToServer
	// Result: CodeLens
	/// A request to resolve a command for a given code lens.
	#[serde(rename = "codeLens/resolve")]
	CodeLensResolve(CodeLens),
	// serverToClient
	// Result: None
	/// A request to refresh all code actions
	///
	/// since: 3.16.0
	#[serde(rename = "workspace/codeLens/refresh")]
	CodeLensRefresh,
	// clientToServer
	// Result: Some(Vec<DocumentLink>) | None
	// Partial Result: Vec<DocumentLink>
	// Registration Options: DocumentLinkRegistrationOptions
	/// A request to provide document links
	#[serde(rename = "textDocument/documentLink")]
	TextDocumentDocumentLink(DocumentLinkParams),
	// clientToServer
	// Result: DocumentLink
	/// Request to resolve additional information for a given document link.
	///
	/// The request's parameter is of type [`DocumentLink`] the response is of type [`DocumentLink`]
	/// or a Thenable that resolves to such.
	#[serde(rename = "documentLink/resolve")]
	DocumentLinkResolve(DocumentLink),
	// clientToServer
	// Result: Some(Vec<TextEdit>) | None
	// Registration Options: DocumentFormattingRegistrationOptions
	/// A request to format a whole document.
	#[serde(rename = "textDocument/formatting")]
	TextDocumentFormatting(DocumentFormattingParams),
	// clientToServer
	// Result: Some(Vec<TextEdit>) | None
	// Registration Options: DocumentRangeFormattingRegistrationOptions
	/// A request to format a range in a document.
	#[serde(rename = "textDocument/rangeFormatting")]
	TextDocumentRangeFormatting(DocumentRangeFormattingParams),
	// clientToServer
	// Result: Some(Vec<TextEdit>) | None
	// Registration Options: DocumentRangeFormattingRegistrationOptions
	/// A request to format ranges in a document.
	///
	/// since: 3.18.0
	#[serde(rename = "textDocument/rangesFormatting")]
	TextDocumentRangesFormatting(DocumentRangesFormattingParams),
	// clientToServer
	// Result: Some(Vec<TextEdit>) | None
	// Registration Options: DocumentOnTypeFormattingRegistrationOptions
	/// A request to format a document on type.
	#[serde(rename = "textDocument/onTypeFormatting")]
	TextDocumentOnTypeFormatting(DocumentOnTypeFormattingParams),
	// clientToServer
	// Result: Some(WorkspaceEdit) | None
	// Registration Options: RenameRegistrationOptions
	/// A request to rename a symbol.
	#[serde(rename = "textDocument/rename")]
	TextDocumentRename(RenameParams),
	// clientToServer
	// Result: Some(PrepareRenameResult) | None
	/// A request to test and perform the setup necessary for a rename.
	///
	/// since: 3.16 - support for default behavior"
	#[serde(rename = "textDocument/prepareRename")]
	TextDocumentPrepareRename(PrepareRenameParams),
	// clientToServer
	// Result: Some(LspAny) | None
	// Registration Options: ExecuteCommandRegistrationOptions
	/// A request send from the client to the server to execute a command.
	///
	/// The request might return a workspace edit which the client will apply to the workspace.
	#[serde(rename = "workspace/executeCommand")]
	WorkspaceExecuteCommand(ExecuteCommandParams),
	// serverToClient
	// Result: ApplyWorkspaceEditResult
	/// A request sent from the server to the client to modified certain resources.
	#[serde(rename = "workspace/applyEdit")]
	WorkspaceApplyEdit(ApplyWorkspaceEditParams),
}

impl Request {
	pub fn is_initialize(&self) -> bool {
		matches!(self.req, RequestType::Initialize(_))
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
