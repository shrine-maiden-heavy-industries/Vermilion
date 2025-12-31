/* SPDX-License-Identifier: BSD-3-Clause */

pub mod capabilities;
pub mod options;
pub mod params;

use std::collections::HashMap;

use crate::types::{
	capabilities::server::ServerCapabilities,
	options::{
		CreateFileOptions, DeleteFileOptions, FileOperationPatternOptions, RenameFileOptions,
	},
};

/// LSP object definition.
///
/// since: 3.17.0
pub type LspObject = serde_json::Map<String, serde_json::Value>;

/// The LSP any type.
///
/// Please note that strictly speaking a property with the value `undefined`
/// can't be converted into JSON preserving the property name.
///
/// However for convenience it is allowed and assumed that all these properties are
/// optional as well.
///
/// since; 3.17.0
pub type LspAny = serde_json::Value;

/// LSP arrays
///
/// since: 3.17.0
pub type LspArray = Vec<LspAny>;

/// The glob pattern to watch relative to the base path.
///
/// Glob patterns can have the following syntax:
/// - `*` to match zero or more characters in a path segment
/// - `?` to match on one character in a path segment
/// - `**` to match any number of path segments, including none
/// - `{}` to group conditions (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
///
/// since: 3.17.0
pub type Pattern = String;

/// Information about where a symbol is declared.
///
/// Provides additional metadata over normal [`Location`] declarations, including the range of
/// the declaring symbol.
///
/// Servers should prefer returning [`DeclarationLink`] over [`Declaration`] if supported by the client.
pub type DeclarationLink = LocationLink;

/// Information about where a symbol is defined.
///
/// Provides additional metadata over normal [`Location`] definitions, including the range of
/// the defining symbol.
pub type DefinitionLink = LocationLink;

/// The declaration of a symbol representation as one or many [`Location`]s.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum Declaration {
	Location(Location),
	LocationArray(Vec<Location>),
}

/// The definition of a symbol represented as one or many [`Location`]s.
///
/// For most programming languages there is only one location at which a symbol is
/// defined.
///
/// Servers should prefer returning [`DefinitionLink`] over [`Definition`] if supported
/// by the client.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum Definition {
	Location(Location),
	LocationArray(Vec<Location>),
}

/// The glob pattern.
///
/// Either a string pattern or a relative pattern.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum GlobPattern {
	Pattern(Pattern),
	RelativePattern(RelativePattern),
}

/// MarkedString can be used to render human readable text.
///
/// It is either a markdown string or a code-block that provides a language and a code snippet.
///
/// The language identifier is semantically equal to the optional language identifier in fenced code blocks
/// in GitHub issues.
///
/// See: https://help.github.com/articles/creating-and-highlighting-code-blocks/#syntax-highlighting
///
/// The pair of a language and a value is an equivalent to markdown:
///
/// \`\`\`${language}\n${value}\n\`\`\`
///
/// Note that markdown strings will be sanitized - that means html will be escaped.
///
/// deprecated: use MarkupContent instead.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum MarkedString {
	String(String),
	CodeBlock { language: String, value: String },
}

/// Position in a text document expressed as zero-based line and zero-based character offset.
///
/// A position is between two characters like an ‘insert’ cursor in an editor.
/// Special values like for example -1 to denote the end of a line are not supported.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Position {
	/// Line position in a document (zero-based).
	pub line: u32,
	/// Character offset on a line in a document (zero-based).
	/// The meaning of this offset is determined by the negotiated [`PositionEncodingKind`].
	///
	/// If the character value is greater than the line length it defaults back
	/// to the line length.
	pub character: u32,
}

/// A range in a text document expressed as (zero-based) start and end positions.
///
/// A range is comparable to a selection in an editor. Therefore, the end position is exclusive.
/// If you want to specify a range that contains a line including the line ending character(s) then
/// use an end position denoting the start of the next line.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Range {
	/// The range's start position.
	pub start: Position,
	/// The range's end position.
	pub end: Position,
}

/// A set of predefined token types.
///
/// This set is not fixed a client can specify additional token types via the
/// corresponding client capabilities.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum SemanticTokenType {
	Class,
	Comment,
	/// since: 3.17.0
	Decorator,
	Enum,
	EnumMember,
	Event,
	Function,
	Interface,
	Keyword,
	Macro,
	Method,
	Modifier,
	Namespace,
	Number,
	Operator,
	Parameter,
	Property,
	Regexp,
	String,
	Struct,
	/// Represents a generic type.
	///
	/// Acts as a fallback for types which can't be mapped to a specific type like class or enum.
	Type,
	TypeParameter,
	Variable,
	#[serde(untagged)]
	Other(String),
}

/// A set of predefined token modifiers.
///
/// This set is not fixed a client can specify additional token types via the
/// corresponding client capabilities.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum SemanticTokenModifiers {
	Abstract,
	Async,
	Declaration,
	DefaultLibrary,
	Definition,
	Deprecated,
	Documentation,
	Modification,
	#[serde(rename = "readonly")]
	ReadOnly,
	Static,
	#[serde(untagged)]
	Other(String),
}

/// The document diagnostic report kinds.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum DocumentDiagnosticReportKind {
	/// A diagnostic report with a full set of problems.
	Full,
	/// A report indicating that the last returned report is still accurate.
	Unchanged,
}

/// A set of predefined range kinds.
///
/// This set is not fixed a client can specify additional token types via the
/// corresponding client capabilities.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum FoldingRangeKind {
	/// Folding range for a comment
	Comment,
	/// Folding range for an import or include
	Imports,
	/// Folding range for a region (e.g. `#region`)
	Region,
	#[serde(untagged)]
	Other(String),
}

/// A symbol kind.
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum SymbolKind {
	Array = 18,
	Boolean = 17,
	Class = 5,
	Constant = 14,
	Constructor = 9,
	Enum = 10,
	EnumMember = 22,
	Event = 24,
	Field = 8,
	File = 1,
	Function = 12,
	Interface = 11,
	Key = 20,
	Method = 6,
	Module = 2,
	Namespace = 3,
	Null = 21,
	Number = 16,
	Object = 19,
	Operator = 25,
	Package = 4,
	Property = 7,
	String = 15,
	Struct = 23,
	TypeParameter = 26,
	Variable = 13,
}

/// Symbol tags are extra annotations that tweak the rendering of a symbol.
///
/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum SymbolTag {
	/// Render a symbol as obsolete, usually using a strike-out.
	Deprecated = 1,
}

/// Moniker uniqueness level to define scope of the moniker.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum UniquenessLevel {
	/// The moniker is only unique inside a document
	Document,
	/// The moniker is globally unique
	Global,
	/// The moniker is unique inside the group to which a project belongs
	Group,
	/// The moniker is unique inside a project for which a dump got created
	Project,
	/// The moniker is unique inside the moniker scheme
	Scheme,
}

/// The moniker kind
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum MonikerKind {
	/// The moniker represents a symbol that is exported from a project
	Export,
	/// The moniker represent a symbol that is imported into a project
	Import,
	/// The moniker represents a symbol that is local to a project
	/// (e.g. a local variable of a function, a class not visible outside the project)
	Local,
}

/// Inlay hint kinds
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum InlayHintKind {
	/// An inlay hint that is for a parameter
	Parameter = 2,
	/// An inlay hint that for a type annotation
	Type = 1,
}

/// The message type
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum MessageType {
	/// A debug message
	///
	/// since: 3.18.0
	Debug = 5,
	/// An error message
	Error = 1,
	/// An information message
	Info = 3,
	/// A log message
	Log = 4,
	/// A warning message
	Warning = 2,
}

/// Defines how the host (editor) should sync document changes to the language server.
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum TextDocumentSyncKind {
	/// Documents are synced by always sending the full content of the document.
	Full = 1,
	/// Documents are synced by sending the full content on open.
	///
	/// After that only incremental updates to the document are sent.
	Incremental = 2,
	/// Documents should not be synced at all
	None = 0,
}

/// The reason why a text document was saved
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum TextDocumentSaveReason {
	/// Automatic after a delay
	AfterDelay = 2,
	/// When the editor lost focus
	FocusOut = 3,
	/// Manually triggered, e.g. by the user pressing save, by starting debugging, or by an API call.
	Manual = 1,
}

/// The kind of a completion entry
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum CompletionItemKind {
	Class = 7,
	Color = 16,
	Constant = 21,
	Constructor = 4,
	Enum = 13,
	EnumMember = 20,
	Event = 23,
	Field = 5,
	File = 17,
	Folder = 19,
	Function = 3,
	Interface = 8,
	Keyword = 14,
	Method = 2,
	Module = 9,
	Operator = 24,
	Property = 10,
	Reference = 18,
	Snippet = 15,
	Struct = 22,
	Text = 1,
	TypeParameter = 25,
	Unit = 11,
	Value = 12,
	Variable = 6,
}

/// Completion item tags are extra annotations that tweak the rendering of a completion item.
///
/// since: 3.15.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum CompletionItemTag {
	/// Render a completion as obsolete, usually using a strike-out
	Deprecated = 1,
}

/// Defines whether the insert text in a completion item should be interpreted as plain text or a snippet.
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum InsertTextFormat {
	/// The primary text to be inserted is treated as a plain string
	PlainText = 1,
	/// The primary text to be inserted is treated as a snippet.
	///
	/// A snippet can define tab stops and placeholders with `$1`, `$2` and `${3:foo}`.
	/// `$0` defines the final tab stop, it defaults to the end of the snippet.
	///
	/// Placeholders with equal identifiers are linked, that is typing in one will update others too.
	///
	/// See also: https://microsoft.github.io/language-server-protocol/specifications/specification-current/#snippet_syntax
	Snippet = 2,
}

/// How whitespace and indentation is handled during completion item insertion.
///
/// since: 3.16.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum InsertTextMode {
	/// The editor adjusts leading whitespace of new lines so that they match the
	/// indentation up to the cursor of the line for which the item is accepted.
	///
	/// Consider a line like this: <2tabs><cursor><3tabs>foo. Accepting a multi line
	/// completion item is indented using 2 tabs and all following lines inserted will
	/// be indented using 2 tabs as well
	AdjustIndentation = 2,
	/// The insertion or replace strings is taken as it is.
	///
	/// If the value is multi line the lines below the cursor will be inserted
	/// using the indentation defined in the string value.
	///
	/// The client will not apply any kind of adjustments to the string.
	AsIs = 1,
}

/// A document highlight kind
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum DocumentHighlightKind {
	/// Read-access of a symbol, like reading a variable
	Read = 2,
	/// A textual occurrence
	Text = 1,
	/// Write-access of a symbol, like writing to a variable
	Write = 3,
}

/// A set of predefined code action kinds
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum CodeActionKind {
	/// Empty kind
	#[serde(rename = "")]
	Empty,
	/// Base kind for quickfix actions: 'quickfix'
	QuickFix,
	/// Base kind for refactoring actions: 'refactor'
	Refactor,
	/// Base kind for refactoring extraction actions: 'refactor.extract'
	///
	/// Example extract actions:
	///
	/// - Extract method
	/// - Extract function
	/// - Extract variable
	/// - Extract interface from class
	/// - ...
	#[serde(rename = "refactor.extract")]
	RefactorExtract,
	/// "Base kind for refactoring inline actions: 'refactor.inline'
	///
	/// Example inline actions:
	///
	/// - Inline function
	/// - Inline variable
	/// - Inline constant
	/// - ...
	#[serde(rename = "refactor.inline")]
	RefactorInline,
	/// Base kind for refactoring rewrite actions: 'refactor.rewrite'
	///
	/// Example rewrite actions:
	///
	/// - Convert JavaScript function to class
	/// - Add or remove parameter
	/// - Encapsulate field
	/// - Make method static
	/// - Move method to base class\n- ..."
	#[serde(rename = "refactor.rewrite")]
	RefactorRewrite,
	/// Base kind for source actions: `source` Source code actions apply to the entire file.
	Source,
	/// Base kind for auto-fix source actions: `source.fixAll`.
	///
	/// Fix all actions automatically fix errors that have a clear fix that do not require user input.
	/// They should not suppress errors or perform unsafe fixes such as generating new types or classes.
	///
	/// since: 3.15.0
	#[serde(rename = "source.fixAll")]
	SourceFixAll,
	/// Base kind for an organize imports source action: `source.organizeImports`
	#[serde(rename = "source.organizeImports")]
	SourceOrganizeImports,
	#[serde(untagged)]
	Other(String),
}

/// Tracing value
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum TraceValues {
	/// Trace messages only
	Messages,
	/// Turn tracing off
	Off,
	/// Verbose message tracing
	Verbose,
}

/// Describes the content type that a client supports in various
/// result literals like [`Hover`], [`ParameterInfo`] or [`CompletionItem`].
///
/// Please note that [`MarkupKind`]s must not start with a `$`. These kinds are reserved for internal usage
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum MarkupKind {
	/// Markdown is supported as a content format
	Markdown,
	/// Plain text is supported as a content format
	PlainText,
}

/// Describes how an [`InlineCompletionItemProvider`] was triggered
///
/// since: 3.18.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum InlineCompletionTriggerKind {
	/// Completion was triggered automatically while editing
	Automatic = 1,
	/// Completion was triggered explicitly by a user gesture
	Invoked = 0,
}

/// A type indicating how positions are encoded, specifically what column offsets mean.
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
pub enum PositionEncodingKind {
	/// Character offsets count UTF-8 code units (e.g bytes).
	#[serde(rename = "utf-8")]
	Utf8,
	/// Character offsets count UTF-16 code units.
	///
	/// This is the default and must always be supported by servers.
	#[serde(rename = "utf-16")]
	#[default]
	Utf16,
	/// Character offsets count UTF-32 code units.
	///
	/// Implementation note: these are the same as Unicode code points, so
	/// this [`PositionEncodingKind`] may also be used for an encoding-agnostic
	/// representation of character offsets.
	#[serde(rename = "utf-32")]
	Utf32,
}

/// The file event type
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum FileChangeType {
	/// The file got changed
	Changed = 2,
	/// The file got created
	Created = 1,
	/// The file got deleted
	Deleted = 3,
}

/// The watch event type
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum WatchKind {
	/// Interested in change events
	Change = 2,
	/// Interested in create events
	Create = 1,
	/// Interested in delete events
	Delete = 4,
}

/// The diagnostic's severity
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum DiagnosticSeverity {
	/// Reports an error
	Error = 1,
	/// Reports a hint
	Hint = 4,
	/// Reports an information
	Information = 3,
	/// Reports a warning
	Warning = 2,
}

/// The diagnostic tags
///
/// since: 3.15.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum DiagnosticTag {
	/// Deprecated or obsolete code.
	///
	/// Clients are allowed to rendered diagnostics with this tag strike through.
	Deprecated = 2,
	/// Unused or unnecessary code.
	///
	/// Clients are allowed to render diagnostics with this tag faded out instead of having an error squiggle.
	Unnecessary = 1,
}

/// How a completion was triggered
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum CompletionTriggerKind {
	/// Completion was triggered by typing an identifier (24x7 code complete), manual invocation (e.g Ctrl+Space) or via API.
	Invoked = 1,
	/// Completion was triggered by a trigger character specified by the `triggerCharacters`
	/// properties of the [`CompletionRegistrationOptions`]
	TriggerCharacter = 2,
	/// Completion was re-triggered as current completion list is incomplete
	TriggerForIncompleteCompletions = 3,
}

/// How a signature help was triggered
///
/// since: 3.15.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum SignatureHelpTriggerKind {
	/// Signature help was triggered by the cursor moving or by the document content changing
	ContentChange = 3,
	/// Signature help was invoked manually by the user or by a command
	Invoked = 1,
	/// Signature help was triggered by a trigger character
	TriggerCharacter = 2,
}

/// The reason why code actions were requested
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum CodeActionTriggerKind {
	/// Code actions were requested automatically.
	///
	/// This typically happens when current selection in a file changes, but can
	/// also be triggered when file content changes
	Automatic = 2,
	/// Code actions were explicitly requested by the user or by an extension
	Invoked = 1,
}

/// A pattern kind describing if a glob pattern matches a file a folder or both
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum FileOperationPatternKind {
	/// The pattern matches a file only
	File,
	/// The pattern matches a folder only
	Folder,
}

/// A notebook cell kind
///
/// since: 3.17.0
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum NotebookCellKind {
	/// A code-cell is source code
	Code = 2,
	/// A markup-cell is formatted source that is used for display
	Markup = 1,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum ResourceOperationKind {
	/// Supports creating new files and folders
	Create,
	/// Supports deleting existing files and folders
	Delete,
	/// Supports renaming existing files and folders
	Rename,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum FailureHandlingKind {
	/// Applying the workspace change is simply aborted if one of the changes provided fails.
	///
	/// All operations executed before the failing operation stay executed.
	Abort,
	/// If the workspace edit contains only textual file changes they are executed transactional.
	///
	/// If resource changes (create, rename or delete file) are part of the change the failure
	/// handling strategy is abort.
	TextOnlyTransactional,
	/// All operations are executed transactional. That means they either all succeed or
	/// no changes at all are applied to the workspace.
	Transactional,
	/// The client tries to undo the operations already executed. But there is no guarantee that this is succeeding.
	Undo,
}

#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	serde_repr::Deserialize_repr,
	serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum PrepareSupportDefaultBehavior {
	/// The client's default behavior is to select the identifier according the to language's syntax rule.
	Identifier = 1,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum TokenFormat {
	Relative,
}

pub type Uri = fluent_uri::Uri<String>;

/// Represents a location inside a resource, such as a line inside a text file.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Location {
	pub uri: Uri,
	pub range: Range,
}

/// A workspace folder inside a client
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFolder {
	/// The associated URI for this workspace folder
	pub uri: Uri,
	/// The name of the workspace folder. Used to refer to this workspace folder in the user interface
	pub name: String,
}

/// Represents a color range from a document
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInformation {
	/// The range in the document where this color appears
	pub range: Range,
	/// The actual color value for this color range
	pub color: Color,
}

/// Represents a color in RGBA space.
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
	/// The red component of this color in the range [0-1].
	pub red: f32,
	/// The green component of this color in the range [0-1].
	pub green: f32,
	/// The blue component of this color in the range [0-1].
	pub blue: f32,
	/// The alpha component of this color in the range [0-1].
	pub alpha: f32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ColorPresentation {
	/// The label of this color presentation. It will be shown on the color\npicker header. By default this is also the text that is inserted when selecting\nthis color presentation.
	pub label: String,
	/// A [`TextEdit`] which is applied to a document when selecting this presentation for the color.
	///
	/// When `falsy` the [`ColorPresentation::label`] is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text_edit: Option<TextEdit>,
	/// An optional array of additional [`TextEdit`]'s that are applied when selecting this color presentation.
	///
	/// Edits must not overlap with the main [`ColorPresentation::text_edit`] nor with themselves.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub additional_text_edits: Option<Vec<TextEdit>>,
}

/// A document filter describes a top level text document or a notebook cell document.
///
/// since: 3.17.0 - proposed support for NotebookCellTextDocumentFilter.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum DocumentFilter {
	TextDocumentFilter(TextDocumentFilter),
	NotebookDocumentFilter(NotebookDocumentFilter),
}

/// A document selector is the combination of one or many document filters.
///
/// Example:
///
/// ```let sel:DocumentSelector = [{ language: 'typescript' }, { language: 'json', pattern: '**∕tsconfig.json' }]```
///
/// The use of a string as a document filter is deprecated
///
/// since: 3.16.0
pub type DocumentSelector = DocumentFilter;

/// Represents a folding range.
///
/// To be valid, start and end line must be bigger than zero and smaller
/// than the number of lines in the document.
///
/// Clients are free to ignore invalid ranges.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRange {
	/// The zero-based start line of the range to fold.
	///
	/// The folded area starts after the line's last character. To be valid, the end must be zero
	/// or larger and smaller than the number of lines in the document.
	pub start_line: u32,
	/// The zero-based character offset from where the folded range starts.
	///
	/// If not defined, defaults to the length of the start line.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub start_character: Option<u32>,
	/// The zero-based end line of the range to fold.
	///
	/// The folded area ends with the line's last character. To be valid, the end must be zero
	/// or larger and smaller than the number of lines in the document.
	pub end_line: u32,
	/// The zero-based character offset before the folded range ends.
	///
	/// If not defined, defaults to the length of the end line.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub end_character: Option<u32>,
	/// Describes the kind of the folding range such as `comment' or 'region'.
	///
	/// The kind is used to categorize folding ranges and used by commands like 'Fold all comments'.
	///
	/// See [`FoldingRangeKind`] for an enumeration of standardized kinds.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub kind: Option<FoldingRangeKind>,
	/// The text that the client should show when the specified range is collapsed.
	///
	/// If not defined or not supported by the client, a default will be chosen by the client.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub collapsed_text: Option<String>,
}

/// A selection range represents a part of a selection hierarchy.
///
/// A selection range may have a parent selection range that contains it.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRange {
	/// The of this selection range
	pub range: Range,
	/// The parent selection range containing this range. Therefore `parent.range` must contain `this.range`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub parent: Option<Box<SelectionRange>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum ProgressToken {
	Integer(i32),
	String(String),
}

impl From<i32> for ProgressToken {
	fn from(value: i32) -> Self {
		ProgressToken::Integer(value)
	}
}

impl From<String> for ProgressToken {
	fn from(value: String) -> Self {
		ProgressToken::String(value)
	}
}

/// Represents programming constructs like functions or constructors in the context of call hierarchy.
///
/// since: 3.16.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyItem {
	/// The name of this item.
	pub name: String,
	/// The kind of this item.
	pub kind: SymbolKind,
	/// Tags for this item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tags: Option<Vec<SymbolTag>>,
	/// More detail for this item, e.g. the signature of a function.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub detail: Option<String>,
	/// The resource identifier of this item.
	pub uri: Uri,
	/// The range enclosing this symbol not including leading/trailing whitespace but everything else,
	/// e.g. comments and code.
	pub range: Range,
	/// The range that should be selected and revealed when this symbol is being picked, e.g. the
	/// name of a function.
	///
	/// Must be contained by the [`CallHierarchyItem::range`].
	pub selection_range: Range,
	/// A data entry field that is preserved between a call hierarchy prepare and incoming calls
	/// or outgoing calls requests.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

/// Represents an incoming call, e.g. a caller of a method or constructor.
///
/// since: 3.16.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyIncomingCall {
	/// The item that makes the call
	pub from: CallHierarchyItem,
	/// The ranges at which the calls appear. This is relative to the caller denoted by [`CallHierarchyIncomingCall::from`]
	pub from_ranges: Vec<Range>,
}

/// Represents an outgoing call, e.g. calling a getter from a method or a method from a constructor etc.
///
/// since: 3.16.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyOutgoingCall {
	/// The item that is called
	pub to: CallHierarchyItem,
	/// The range at which this item is called.
	///
	/// This is the range relative to the caller, e.g the item passed to [`CallHierarchyItemProvider::provide_call_hierarchy_outgoing_calls`]
	/// and not [`CallHierarchyOutgoingCall::to`].
	pub to_ranges: Vec<Range>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokens {
	/// An optional result id.
	///
	/// If provided and clients support delta updating the client will include the result id in the next
	/// semantic token request.
	///
	/// A server can then instead of computing all semantic tokens again simply send a delta.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub result_id: Option<String>,
	/// The actual tokens
	pub data: Vec<u32>,
}

/// See [`SemanticTokens`]
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensPartialResult {
	/// See [`SemanticTokens::data`]
	pub data: Vec<u32>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensDelta {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub result_id: Option<String>,
	///The semantic token edits to transform a previous result into a new result
	pub edits: Vec<SemanticTokensEdit>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensDeltaPartialResult {
	pub edits: Vec<SemanticTokensEdit>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensEdit {
	/// The start offset of the edit
	pub start: u32,
	/// The count of elements to remove
	pub delete_count: u32,
	/// The elements to insert
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<Vec<u32>>,
}

/// The result of a showDocument request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowDocumentResult {
	/// A boolean indicating if the show was successful.
	pub success: bool,
}

/// The result of a linked editing range request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRanges {
	/// A list of ranges that can be edited together.
	///
	/// The ranges must have identical length and contain identical text content. The ranges cannot overlap.
	pub ranges: Vec<Range>,
	/// An optional word pattern (regular expression) that describes valid contents for the given ranges.
	///
	/// If no pattern is provided, the client configuration's word pattern will be used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub word_pattern: Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum DocumentChange {
	TextDocumentEdit(TextDocumentEdit),
	CreateFile(CreateFile),
	RenameFile(RenameFile),
	DeleteFile(DeleteFile),
}

/// Create file operation.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CreateFile {
	// `create`
	/// A create document change
	pub kind: String,
	/// The resource to create.
	pub uri: Uri,
	/// Additional options
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub options: Option<CreateFileOptions>,
	#[serde(flatten)]
	pub resource_operation: ResourceOperation,
}

/// Rename file operation
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameFile {
	// `rename`
	/// A rename document change
	pub kind: String,
	/// The old (existing) location.
	pub old_uri: Uri,
	/// The new location.
	pub new_uri: Uri,
	/// Additional options
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub options: Option<RenameFileOptions>,
	#[serde(flatten)]
	pub resource_operation: ResourceOperation,
}

/// Delete file operation.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFile {
	// `delete`
	/// A delete document change
	pub kind: String,
	/// The resource to delete.
	pub uri: Uri,
	/// Additional options
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub options: Option<DeleteFileOptions>,
	#[serde(flatten)]
	pub resource_operation: ResourceOperation,
}

/// An identifier to refer to a change annotation stored with a workspace edit.
pub type ChangeAnnotationIdentifier = String;

/// A workspace edit represents changes to many resources managed in the workspace.
///
/// The edit should either provide `changes` or `documentChanges`.
///
/// If documentChanges are present they are preferred over `changes` if the client can handle versioned
/// document edits.
///
/// Since version 3.13.0 a workspace edit can contain resource operations as well.
///
/// If resource operations are present clients need to execute the operations in the order in which they
/// are provided. So a workspace edit for example can consist of the following two changes:
/// (1) a create file a.txt and (2) a text document edit which insert text into file a.txt.
///
/// An invalid sequence (e.g. (1) delete file a.txt and (2) insert text into file a.txt) will cause failure
/// of the operation.
///
/// How the client recovers from the failure is described by the client capability: `workspace.workspaceEdit.failureHandling`
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEdit {
	/// Holds changes to existing resources.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub changes: Option<HashMap<Uri, Vec<TextEdit>>>,
	/// Depending on the client capability `workspace.workspaceEdit.resourceOperations` document changes
	/// are either an array of `TextDocumentEdit`s to express changes to n different text documents
	/// where each text document edit addresses a specific version of a text document.
	///
	/// Or it can contain above [`TextDocumentEdit`]s mixed with create, rename and delete file/folder operations.
	///
	/// Whether a client supports versioned document edits is expressed via `workspace.workspaceEdit.documentChanges` client capability.
	///
	/// If a client neither supports `documentChanges` nor `workspace.workspaceEdit.resourceOperations` then
	/// only plain [`TextEdit`]s using the `changes` property are supported
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub document_changes: Option<Vec<DocumentChange>>,
	/// A map of change annotations that can be referenced in [`AnnotatedTextEdit`]s or create, rename and
	/// delete file/folder operations.
	///
	/// Whether clients honor this property depends on the client capability `workspace.changeAnnotationSupport`.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub change_annotations: Option<HashMap<ChangeAnnotationIdentifier, ChangeAnnotation>>,
}

/// Moniker definition to match LSIF 0.5 moniker definition.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Moniker {
	/// The scheme of the moniker.
	///
	/// For example tsc or .Net
	pub scheme: String,
	/// The identifier of the moniker.
	///
	/// The value is opaque in LSIF however schema owners are allowed to define the structure if they want.
	pub identifier: String,
	/// The scope in which the moniker is unique
	pub unique: UniquenessLevel,
	/// The moniker kind if known.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub kind: Option<MonikerKind>,
}

/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyItem {
	/// The name of this item.
	pub name: String,
	/// The kind of this item.
	pub kind: SymbolKind,
	/// Tags for this item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tags: Option<Vec<SymbolTag>>,
	/// More detail for this item, e.g. the signature of a function.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub detail: Option<String>,
	/// The resource identifier of this item.
	pub uri: Uri,
	/// The range enclosing this symbol not including leading/trailing whitespace but everything else,
	///
	/// e.g. comments and code.
	pub range: Range,
	/// The range that should be selected and revealed when this symbol is being picked,
	///
	/// e.g. the name of a function. Must be contained by the [`TypeHierarchyItem::range`].
	pub selection_range: Range,
	/// A data entry field that is preserved between a type hierarchy prepare and supertypes or subtypes
	/// requests.
	///
	/// It could also be used to identify the type hierarchy in the server, helping improve the performance
	/// on resolving supertypes and subtypes.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum InlayHintLabel {
	String(String),
	InlayHintLabelPart(InlayHintLabelPart),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum MarkupOrString {
	String(String),
	Markup(MarkupContent),
}

/// Inlay hint information
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayHint {
	/// The position of this hint.
	///
	/// If multiple hints have the same position, they will be shown in the order
	/// they appear in the response.
	pub position: Position,
	/// The label of this hint.
	///
	/// A human readable string or an array of [`InlayHintLabelPart`].
	///
	/// *Note* that neither the string nor the label part can be empty.
	pub label: InlayHintLabel,
	/// The kind of this hint.
	///
	/// Can be omitted in which case the client should fall back to a reasonable default.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub kind: Option<InlayHintKind>,
	/// Optional text edits that are performed when accepting this inlay hint.
	///
	/// *Note* that edits are expected to change the document so that the inlay hint (or its nearest variant)
	/// is now part of the document and the inlay hint itself is now obsolete.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text_edits: Option<Vec<TextEdit>>,
	/// The tooltip text when you hover over this item.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tooltip: Option<MarkupOrString>,
	/// Render padding before the hint.
	///
	/// Note: Padding should use the editor's background color, not the background color of the hint itself.
	/// That means padding can be used to visually align/separate an inlay hint.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub padding_left: Option<bool>,
	/// Render padding after the hint.
	///
	/// Note: Padding should use the editor's background color, not the background color of the hint itself.
	/// That means padding can be used\nto visually align/separate an inlay hint.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub padding_right: Option<bool>,
	/// A data entry field that is preserved on an inlay hint between a `textDocument/inlayHint` and a `inlayHint/resolve` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum DiagnosticReport {
	FullDocument(FullDocumentDiagnosticReport),
	UnchangedDocument(UnchangedDocumentDiagnosticReport),
}

/// A partial result for a document diagnostic report.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDiagnosticReportPartialResult {
	pub related_documents: HashMap<Uri, DiagnosticReport>,
}

/// Cancellation data returned from a diagnostic request.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticServerCancellationData {
	pub retrigger_request: bool,
}

/// A workspace diagnostic report.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDiagnosticReport {
	pub items: Vec<WorkspaceDocumentDiagnosticReport>,
}

/// A partial result for a workspace diagnostic report.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDiagnosticReportPartialResult {
	pub items: Vec<WorkspaceDocumentDiagnosticReport>,
}

/// Represents a collection of [`InlineCompletionItem`]s to be presented in the editor.
///
/// since: 3.18.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionList {
	/// The inline completion items
	pub items: Vec<InlineCompletionItem>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum InsertText {
	String(String),
	StringValue(StringValue),
}

/// An inline completion item represents a text snippet that is proposed inline to complete text that is being typed.
///
/// since: 3.18.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionItem {
	/// The text to replace the range with. Must be set.
	pub insert_text: InsertText,
	/// A text that is used to decide if this inline completion should be shown.
	///
	/// When `falsy` the [`InlineCompletionItem::insert_text`] is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub filter_text: Option<String>,
	/// The range to replace. Must begin and end on the same line.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range: Option<Range>,
	/// An optional [`Command`] that is executed *after* inserting this completion.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub command: Option<Command>,
}

/// The result returned from an initialize request.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
	/// The capabilities the language server provides.
	pub capabilities: ServerCapabilities,
	/// Information about the server.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub server_info: Option<ServerInfo>,
}

/// Information about the server.
///
/// since: 3.15.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
	/// The name of the server as defined by the server.
	pub name: String,
	/// The server's version as defined by the server.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<String>,
}

/// The data type of the ResponseError if the initialize request fails.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InitializeError {
	/// Indicates whether the client execute the following retry logic:
	/// (1) show the message provided by the ResponseError to the user
	/// (2) user selects retry or cancel
	/// (3) if user selected retry the initialize method is sent again.
	pub retry: bool,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItem {
	/// A short title like 'Retry', 'Open Log' etc.
	pub title: String,
}

/// A text edit applicable to a text document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
	/// The range of the text document to be manipulated.
	///
	/// To insert text into a document create a range where start === end.
	pub range: Range,
	/// The string to be inserted.
	///
	/// For delete operations use an empty string
	pub new_text: String,
}

/// A completion item represents a text snippet that is proposed to complete text that is being typed.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
	/// The label of this completion item.
	///
	/// The label property is also by default the text that is inserted when selecting this completion.
	///
	/// If label details are provided the label itself should be an unqualified name of the completion item.
	pub label: String,
	/// Additional details for the label
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub label_details: Option<CompletionItemLabelDetails>,
	/// The kind of this completion item. Based of the kind an icon is chosen by the editor.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub kind: Option<CompletionItemKind>,
	/// Tags for this completion item.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tags: Option<Vec<CompletionItemTag>>,
	/// A human-readable string with additional information about this item, like type or symbol information.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub detail: Option<String>,
	/// A human-readable string that represents a doc-comment.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub documentation: Option<MarkupOrString>, // XXX(aki): This is not the right type but same kind
	/// Indicates if this item is deprecated.
	///
	/// deprecated: Use [`CompletionItem::tags`] instead.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub deprecated: Option<bool>,
	/// Select this item when showing.
	///
	/// *Note* that only one completion item can be selected and that the tool/client decides
	/// which item that is. The rule is that the *first* item of those that match best is selected.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub preselect: Option<bool>,
	/// A string that should be used when comparing this item with other items.
	///
	/// When `falsy` the [`CompletionItem::label`] is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub sort_text: Option<String>,
	/// A string that should be used when filtering a set of completion items.
	///
	/// When `falsy` the [`CompletionItem::label`] is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub filter_text: Option<String>,
	/// A string that should be inserted into a document when selecting this completion.
	///
	/// When `falsy` the [`CompletionItem::label`] is used.
	///
	/// The `inser_text` is subject to interpretation by the client side. Some tools might not take the string literally.
	///
	/// For example VS Code when code complete is requested in this example `con<cursor position>` and a completion
	/// item with an `insert_text` of `console` is provided it will only insert `sole`.
	///
	/// Therefore it is recommended to use `text_edit` instead since it avoids additional client side interpretation.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text: Option<String>,
	/// The format of the insert text. The format applies to both the
	/// `insert_text` property and the `new_text` property of a provided `text_edit`.
	///
	/// If omitted defaults to [`InsertTextFormat::PlainText`].
	///
	/// Please note that the `insert_text_format`` doesn't apply to `additional_text_edits`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text_format: Option<InsertTextFormat>,
	/// How whitespace and indentation is handled during completion item insertion.
	///
	/// If not provided the clients default value depends on the `textDocument.completion.insertTextMode`
	/// client capability.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text_mode: Option<InsertTextMode>,
	/// An {@link TextEdit edit} which is applied to a document when selecting this completion.
	/// When an edit is provided the value of [`CompletionItem::insert_text`] is ignored.
	///
	/// Most editors support two different operations when accepting a completion item.
	/// One is to insert a completion text and the other is to replace an existing text with a
	/// completion text.
	///
	/// Since this can usually not be predetermined by a server it can report both ranges.
	/// Clients need to signal support for `InsertReplaceEdits` via the `textDocument.completion.insertReplaceSupport`
	/// client capability property.
	///
	/// *Note 1:* The text edit's range as well as both ranges from an insert replace edit must be a [single line] and they must contain the position
	/// at which completion has been requested.
	/// *Note 2:* If an `InsertReplaceEdit` is returned the edit's insert range must be a prefix of the edit's replace range, that means it must be
	/// contained and starting at the same position.
	///
	/// since: 3.16.0 additional type [`InsertReplaceEdit`]
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text_edit: Option<TextEdit>, // XXX(aki), or InsertReplaceEdit
	/// The edit text used if the completion item is part of a CompletionList and CompletionList
	/// defines an item default for the text edit range.
	///
	/// Clients will only honor this property if they opt into completion list item defaults using
	/// the capability `completionList.itemDefaults`.
	///
	/// If not provided and a list's default range is provided the label property is used as a text.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text_edit_text: Option<String>,
	/// An optional array of additional [`TextEdit`] that are applied when selecting this completion.
	/// Edits must not overlap (including the same insert position) with the main [`CompletionItem::text_edit`]
	/// nor with themselves.
	///
	/// Additional text edits should be used to change text unrelated to the current cursor position
	/// (for example adding an import statement at the top of the file if the completion item will
	/// insert an unqualified type).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub additional_text_edits: Option<Vec<TextEdit>>,
	/// An optional set of characters that when pressed while this completion is active will accept it first and
	/// then type that character.
	///
	/// *Note* that all commit characters should have `length=1` and that superfluous characters will be ignored
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub commit_characters: Option<Vec<String>>,
	/// An optional [`Command`] that is executed *after* inserting this completion.
	///
	/// *Note* that additional modifications to the current document should be described with the [`CompletionItem::additional_text_edits`]-property
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub command: Option<Command>,
	/// A data entry field that is preserved on a completion item between a [`CompletionRequest`] and
	/// a [`CompletionResolveRequest`].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDefaults {
	/// A default commit character set.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub commit_characters: Option<Vec<String>>,
	/// A default edit range.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub edit_range: Option<Range>, // XXX(aki) Or Position
	/// A default insert text format.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text_format: Option<InsertTextFormat>,
	/// A default insert text mode.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text_mode: Option<InsertTextMode>,
	/// A default data value.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

/// Represents a collection of [`CompletionItem`]s completion to be presented in the editor.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionList {
	/// This list it not complete.
	///
	/// Further typing results in recomputing this list.
	///
	/// Recomputed lists have all their items replaced (not appended) in the incomplete completion sessions.
	pub is_incomplete: bool,
	/// In many cases the items of an actual completion result share the same value for properties
	/// like `commitCharacters` or the range of a text edit.
	///
	/// A completion list can therefore define item defaults which will be used if a completion item
	/// itself doesn't specify the value.
	///
	/// If a completion list specifies a default value and a completion item also specifies a
	/// corresponding value the one from the item is used.
	///
	/// Servers are only allowed to return default values if the client signals support for this
	/// via the `completionList.itemDefaults` capability.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub item_defaults: Option<ItemDefaults>,
	/// The completion items
	pub items: Vec<CompletionItem>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum HoverContents {
	MarkupContent(MarkupContent),
	MarkedString(MarkedString),
	MarkedStrings(Vec<MarkedString>),
}

/// The result of a hover request
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Hover {
	/// The hover's content
	pub contents: HoverContents,
	/// An optional range inside the text document that is used to visualize the hover,
	///
	/// e.g. by changing the background color.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range: Option<Range>,
}

/// Signature help represents the signature of something callable.
///
/// There can be multiple signature but only one active and only one active parameter.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelp {
	/// One or more signatures
	pub signatures: Vec<SignatureInformation>,
	/// The active signature.
	///
	/// If omitted or the value lies outside the range of [`SignatureHelp::signatures`] the value defaults
	/// to zero or is ignored if the [`SignatureHelp`] has no signatures.
	///
	/// Whenever possible implementors should make an active decision about the active signature and
	/// shouldn't rely on a default value.
	///
	/// In future version of the protocol this property might become mandatory to better express this.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub active_signature: Option<u32>,
	/// The active parameter of the active signature.
	///
	/// If omitted or the value\nlies outside the range of `signatures[activeSignature].parameters`
	/// defaults to 0 if the active signature has parameters.
	///
	/// If the active signature has no parameters it is ignored. In future version of the protocol this
	/// property might become mandatory to better express the active parameter if the active signature
	/// does have any.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub active_parameter: Option<u32>,
}

/// A document highlight is a range inside a text document which deserves special attention.
///
/// Usually a document highlight is visualized by changing the background color of its range.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlight {
	/// The range this highlight applies to
	pub range: Range,
	/// The highlight kind, default is [`DocumentHighlightKind::Text`]
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub kind: Option<DocumentHighlightKind>,
}

/// Represents programming constructs like variables, classes, interfaces etc. that appear in a document.
///
/// Document symbols can be hierarchical and they have two ranges: one that encloses its definition and one that
/// points to its most interesting range, e.g. the range of an identifier.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
	/// The name of this symbol.
	///
	/// Will be displayed in the user interface and therefore must not be an empty string or a string
	/// only consisting of white spaces.
	pub name: String,
	/// More detail for this symbol, e.g the signature of a function.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub detail: Option<String>,
	/// The kind of this symbol.
	pub kind: SymbolKind,
	/// Tags for this document symbol.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tags: Option<Vec<SymbolTag>>,
	/// Indicates if this symbol is deprecated.
	///
	/// deprecated: Use tags instead
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub deprecated: Option<bool>,
	/// The range enclosing this symbol not including leading/trailing whitespace but everything else
	/// like comments.
	///
	/// This information is typically used to determine if the clients cursor is inside the symbol
	/// to reveal in the symbol in the UI.
	pub range: Range,
	/// The range that should be selected and revealed when this symbol is being picked, e.g the name
	/// of a function.
	///
	/// Must be contained by the `range`.
	pub selection_range: Range,
	/// Children of this symbol, e.g. properties of a class.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub children: Option<Vec<Box<DocumentSymbol>>>,
}

/// Represents a reference to a command.
///
/// Provides a title which will be used to represent a command in the UI and,
/// optionally,an array of arguments which will be passed to the command handler
/// function when invoked.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
	/// Title of the command, like `save`.
	pub title: String,
	/// The identifier of the actual command handler.
	pub command: String,
	/// Arguments that the command handler should be invoked with.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub arguments: Option<Vec<serde_json::Value>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DisabledReason {
	/// Human readable description of why the code action is currently disabled.
	///
	/// This is displayed in the code actions UI.
	pub reason: String,
}

/// A code action represents a change that can be performed in code, e.g. to fix a problem or to
/// refactor code.
///
/// A CodeAction must set either `edit` and/or a `command`. If both are supplied, the `edit` is applied first,
/// then the `command` is executed.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
	/// A short, human-readable, title for this code action.
	pub title: String,
	/// The kind of the code action.
	///
	/// Used to filter code actions.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub kind: Option<CodeActionKind>,
	/// The diagnostics that this code action resolves.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub diagnostics: Option<Vec<Diagnostic>>,
	/// Marks this as a preferred action. Preferred actions are used by the `auto fix` command and can
	/// be targeted by keybindings.
	///
	/// A quick fix should be marked preferred if it properly addresses the underlying error.
	///
	/// A refactoring should be marked preferred if it is the most reasonable choice of actions to take.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub is_preferred: Option<bool>,
	/// Marks that the code action cannot currently be applied.
	///
	/// Clients should follow the following guidelines regarding disabled code actions:
	/// - Disabled code actions are not shown in automatic [lightbulbs](https://code.visualstudio.com/docs/editor/editingevolved#_code-action) code action menus.
	/// - Disabled actions are shown as faded out in the code action menu when the user requests a more specific type of code action, such as refactorings.
	/// - If the user has a [keybinding](https://code.visualstudio.com/docs/editor/refactoring#_keybindings-for-code-actions) that auto applies a code action
	///   and only disabled code actions are returned, the client should show the user an error message with `reason` in the editor.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub disabled: Option<DisabledReason>,
	/// The workspace edit this code action performs.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub edit: Option<WorkspaceEdit>,
	/// A command this code action executes.
	///
	/// If a code action provides an edit and a command, first the edit is executed and then the command.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub command: Option<Command>,
	/// A data entry field that is preserved on a code action between\na `textDocument/codeAction` and a `codeAction/resolve` request.
	///
	/// @since 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

/// A code lens represents a [`Command`] that should be shown along with source text, like the number
/// of references, a way to run tests, etc.
///
/// A code lens is _unresolved_ when no command is associated to it.
///
/// For performance reasons the creation of a code lens and resolving should be done in two stages.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeLens {
	/// The range in which this code lens is valid. Should only span a single line.
	pub range: Range,
	/// The command this code lens represents
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub command: Option<Command>,
	/// A data entry field that is preserved on a code lens item between a [`CodeLensRequest`] and a [`CodeLensResolveRequest`]
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

/// A document link is a range in a text document that links to an internal or external resource,
/// like another text document or a web site.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLink {
	/// The range this link applies to.
	pub range: Range,
	/// The uri this link points to.
	///
	/// If missing a resolve request is sent later.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub target: Option<Uri>,
	/// The tooltip text when you hover over this link.
	///
	/// If a tooltip is provided, is will be displayed in a string that includes instructions on how to
	/// trigger the link, such as `{0} (ctrl + click)`. The specific instructions vary depending on OS,
	/// user settings, and localization.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tooltip: Option<String>,
	/// A data entry field that is preserved on a document link between a [`DocumentLinkRequest`] and
	/// a [`DocumentLinkResolveRequest`].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

/// The result returned from the apply workspace edit request.
///
/// since: 3.17.0 renamed from ApplyWorkspaceEditResponse
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ApplyWorkspaceEditResult {
	/// Indicates whether the edit was applied or not.
	pub applied: bool,
	/// An optional textual description for why the edit was not applied.
	///
	/// This may be used by the server for diagnostic logging or to provide a suitable error for a
	/// request that triggered the edit.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub failure_reason: Option<String>,
	/// Depending on the client's failure handling strategy `failedChange` might
	/// contain the index of the change that failed.
	///
	/// This property is only available if the client signals a `failureHandlingStrategy` in its
	/// client capabilities.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub failed_change: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressBegin {
	/// `begin`
	pub kind: String,
	/// Mandatory title of the progress operation.
	///
	/// Used to briefly inform about the kind of operation being performed.
	///
	/// Examples: "Indexing" or "Linking dependencies".
	pub title: String,
	/// Controls if a cancel button should show to allow the user to cancel the
	/// long running operation.
	///
	/// Clients that don't support cancellation are allowed to ignore the setting.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub cancellable: Option<bool>,
	/// Optional, more detailed associated progress message.
	///
	/// Contains complementary information to the `title`.
	///
	/// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
	///
	/// If unset, the previous progress message (if any) is still valid.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub message: Option<String>,
	/// Optional progress percentage to display (value 100 is considered 100%).
	///
	/// If not provided infinite progress is assumed and clients are allowed
	/// to ignore the `percentage` value in subsequent report notifications.
	///
	/// The value should be steadily rising. Clients are free to ignore values
	/// that are not following this rule. The value range is [0, 100].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub percentage: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressReport {
	/// `report`
	pub kind: String,
	/// Controls enablement state of a cancel button.
	///
	/// Clients that don't support cancellation or don't support controlling the button's
	/// enablement state are allowed to ignore the property.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub cancellable: Option<bool>,
	/// Optional, more detailed associated progress message.
	///
	/// Contains complementary information to the `title`.
	///
	/// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
	///
	/// If unset, the previous progress message (if any) is still valid.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub message: Option<String>,
	/// Optional progress percentage to display (value 100 is considered 100%).
	///
	/// If not provided infinite progress is assumed and clients are allowed
	/// to ignore the `percentage` value in subsequent report notifications.
	///
	/// The value should be steadily rising. Clients are free to ignore values
	/// that are not following this rule. The value range is [0, 100].
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub percentage: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressEnd {
	/// `end`
	pub kind: String,
	/// Optional, a final message indicating to for example indicate the outcome of the operation.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub message: Option<String>,
}

/// The workspace folder change event.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersChangeEvent {
	/// The array of added workspace folders
	pub added: Vec<WorkspaceFolder>,
	/// The array of the removed workspace folders
	pub removed: Vec<WorkspaceFolder>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationItem {
	/// The scope to get the configuration section for
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub scope_uri: Option<Uri>,
	/// The configuration section asked for
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub section: Option<String>,
}

/// A literal to identify a text document in the client.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
	/// The text document's uri
	pub uri: Uri,
}

/// A document filter denotes a document by different properties like the [`TextDocument::language_id`],
/// the Uri.scheme of its resource, or a glob-pattern that is applied to the [`TextDocument::file_name`]
///
/// Glob patterns can have the following syntax:
/// - `*` to match zero or more characters in a path segment
/// - `?` to match on one character in a path segment
/// - `**` to match any number of path segments, including none
/// - `{}` to group sub patterns into an OR expression. (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
///
/// Examples:
/// A language filter that applies to typescript files on disk:
/// ```json
/// { language: 'typescript', scheme: 'file' }`
/// ```
/// A language filter that applies to all package.json paths:
/// ```json
/// { language: 'json', pattern: '**package.json' }
/// ```
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentFilter {
	/// A language id, like `typescript`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub language: Option<String>,
	/// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub scheme: Option<String>,
	/// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub pattern: Option<String>,
}

/// A notebook document filter denotes a notebook document by different properties.
///
/// The properties will be match against the notebook's URI (same as with documents)
///
/// See: [`TextDocumentFilter`]
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentFilter {
	/// The type of the enclosing notebook.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub notebook_type: Option<String>,
	/// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub scheme: Option<String>,
	/// A glob pattern.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub pattern: Option<String>,
}

/// Represents information on a file/folder create.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileCreate {
	/// A `file://` URI for the location of the file/folder being created.
	pub uri: String,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum TextEditType {
	TextEdit(TextEdit),
	AnnotatedTextEdit(AnnotatedTextEdit),
}

/// A special text edit with an additional change annotation.
///
/// since: 3.16.0.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct AnnotatedTextEdit {
	/// The actual identifier of the change annotation
	pub annotation_id: ChangeAnnotationIdentifier,
	#[serde(flatten)]
	pub text_edit: TextEdit,
}

/// Describes textual changes on a text document.
///
/// A [`TextDocumentEdit`] describes all changes on a document version Si and after they
/// are applied move the document to version Si+1.
///
/// So the creator of a [`TextDocumentEdit`] doesn't need to sort the array of edits or do any
/// kind of ordering. However the edits must be non overlapping.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentEdit {
	/// The text document to change.
	pub text_document: OptionalVersionedTextDocumentIdentifier,
	/// The edits to be applied.
	///
	/// since: 3.16.0 - support for AnnotatedTextEdit. This is guarded using a\nclient capability.
	pub edits: TextEditType,
}

/// A text document identifier to optionally denote a specific version of a text document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OptionalVersionedTextDocumentIdentifier {
	/// The version number of this document.
	///
	/// If a versioned text document identifier is sent from the server
	/// to the client and the file is not open in the editor (i.e. the server
	/// has not received an open notification before) the server can send `null`
	/// to indicate that the version is unknown and the content on disk is the
	/// truth (as specified with document content ownership).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<i32>,
	#[serde(flatten)]
	pub text_document_identifier: TextDocumentIdentifier,
}

/// Additional information that describes document changes.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnotation {
	/// A human-readable string describing the actual change.
	///
	/// The string is rendered prominent in the user interface.
	pub label: String,
	/// A flag which indicates that user confirmation is needed before applying the change.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub needs_confirmation: Option<bool>,
	/// A human-readable string which is rendered less prominent in the user interface.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub description: Option<String>,
}

/// A filter to describe in which file operation requests or notifications
/// the server is interested in receiving.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationFilter {
	/// A Uri scheme like `file` or `untitled`
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub scheme: Option<String>,
	/// The actual file operation pattern.
	pub pattern: FileOperationPattern,
}

/// Represents information on a file/folder rename.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileRename {
	/// A `file://` URI for the original location of the file/folder being renamed.
	pub old_uri: String,
	/// A `file://` URI for the new location of the file/folder being renamed.
	pub new_uri: String,
}

/// Represents information on a file/folder delete.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileDelete {
	/// A `file://`` URI for the location of the file/folder being deleted.
	pub uri: String,
}

/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueContext {
	/// The stack frame (as a DAP Id) where the execution has stopped.
	pub frame_id: i32,
	/// The document range where execution has stopped.
	///
	/// Typically the end position of the range denotes the line where the inline values are shown.
	pub stopped_location: Range,
}

/// Provide inline value as text.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueText {
	/// The document range for which the inline value applies.
	pub range: Range,
	/// The text of the inline value.
	pub text: String,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum FolderOrUri {
	Folder(WorkspaceFolder),
	Uri(Uri),
}

/// A relative pattern is a helper to construct glob patterns that are matched relatively to a base URI.
///
/// The common value for a `base_uri` is a workspace folder root,
/// but it can be another absolute URI as well.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RelativePattern {
	/// A workspace folder or a base URI to which this pattern will be matched against relatively.
	pub base_uri: FolderOrUri,
	/// The actual glob pattern
	pub pattern: Pattern,
}

/// An event describing a change to a text document.
///
/// If only a text is provided it is considered to be the full content of the document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentChangeEvent {
	/// The range of the document that changed.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range: Option<Range>,
	/// The optional length of the range that got replaced.
	///
	/// deprecated: use `range` instead.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub range_length: Option<u32>,
	/// The new text for the provided range or if `range` is None the whole document.
	pub text: String,
}

/// A workspace diagnostic document report.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum WorkspaceDocumentDiagnosticReport {
	Full(WorkspaceFullDocumentDiagnosticReport),
	Unchanged(WorkspaceUnchangedDocumentDiagnosticReport),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum PrepareRenameResult {
	Range(Range),
	RangeWithPlaceholder { range: Range, placeholder: String },
	Default { default_behaviour: bool },
}

/// The result of a document diagnostic pull request.
///
/// A report can either be a full report containing all diagnostics for the
/// requested document or an unchanged report indicating that nothing
/// has changed in terms of diagnostics in comparison to the last
/// pull request.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum DocumentDiagnosticReport {
	Full(RelatedFullDocumentDiagnosticReport),
	Unchanged(RelatedUnchangedDocumentDiagnosticReport),
}

/// Inline value information can be provided by different means:
/// - directly as a text value (class InlineValueText).
/// - as a name to use for a variable lookup (class InlineValueVariableLookup)
/// - as an evaluatable expression (class InlineValueEvaluatableExpression)
///
/// The InlineValue types combines all inline value types into one type.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum InlineValue {
	Text(InlineValueText),
	VariableLookup(InlineValueVariableLookup),
	EvaluatableExpression(InlineValueEvaluatableExpression),
}

/// Represents the connection of two locations.
///
/// Provides additional metadata over normal [`Location`], including an origin range.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LocationLink {
	/// Span of the origin of this link.
	///
	/// Used as the underlined span for mouse interaction.
	///
	/// Defaults to the word range at the definition position.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub origin_selection_range: Option<Range>,
	/// The target resource identifier of this link.
	pub target_uri: Uri,
	/// The full target range of this link.
	///
	/// If the target for example is a symbol then target range is the range
	/// enclosing this symbol not including leading/trailing whitespace but everything else
	/// like comments.
	///
	/// This information is typically used to highlight the range in the editor.
	pub target_range: Range,
	/// The range that should be selected and revealed when this link is being followed,
	/// e.g the name of a function.
	///
	/// Must be contained by the [`LocationLink::target_range`].
	///
	/// See also [`DocumentSymbol::range`]
	pub target_selection_range: Range,
}

/// An inlay hint label part allows for interactive and composite labels of inlay hints.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintLabelPart {
	/// The value of this label part.
	pub value: String,
	/// The tooltip text when you hover over this label part.
	///
	/// Depending on the client capability `inlayHint.resolveSupport` clients might resolve
	/// this property late using the resolve request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tooltip: Option<MarkupOrString>,
	/// An optional source code location that represents this label part.
	///
	/// The editor will use this location for the hover and for code navigation features:
	/// This part will become a clickable link that resolves to the definition of the symbol
	/// at the given location (not necessarily the location itself), it shows the hover that
	/// shows at the given location, and it shows a context menu with further code navigation commands.
	///
	/// Depending on the client capability `inlayHint.resolveSupport` clients might resolve this property
	/// late using the resolve request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub location: Option<Location>,
	/// An optional command for this label part.
	///
	/// Depending on the client capability `inlayHint.resolveSupport` clients
	/// might resolve this property late using the resolve request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub command: Option<Command>,
}

/// A [`MarkupContent`] literal represents a string value which content is interpreted base on its
/// kind flag.
///
/// Currently the protocol supports `plaintext` and `markdown` as markup kinds.
///
/// If the kind is `markdown` then the value can contain fenced code blocks like in GitHub issues.
/// See: https://help.github.com/articles/creating-and-highlighting-code-blocks/#syntax-highlighting
///
/// *Please Note* that clients might sanitize the return markdown. A client could decide to
/// remove HTML from the markdown to avoid script execution.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MarkupContent {
	/// The type of the Markup
	pub kind: MarkupKind,
	/// The content itself
	pub value: String,
}

/// Provide inline value through a variable lookup.
///
/// If only a range is specified, the variable name will be extracted from the underlying document.
///
/// An optional variable name can be used to override the extracted name.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueVariableLookup {
	/// The document range for which the inline value applies.
	///
	/// The range is used to extract the variable name from the underlying document.
	pub range: Range,
	/// If specified the name of the variable to look up.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub variable_name: Option<String>,
	/// How to perform the lookup.
	pub case_sensitive_lookup: bool,
}

/// Provide an inline value through an expression evaluation.
///
/// If only a range is specified, the expression will be extracted from the underlying document.
/// An optional expression can be used to override the extracted expression.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueEvaluatableExpression {
	/// The document range for which the inline value applies.
	///
	/// The range is used to extract the evaluatable expression from the underlying document.
	pub range: Range,
	/// If specified the expression overrides the extracted expression.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub expression: Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionSummary {
	/// A strict monotonically increasing value indicating the execution order of a cell inside a notebook.
	pub execution_order: u32,
	/// Whether the execution was successful or not if known by the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub success: Option<bool>,
}

/// A notebook cell text document filter denotes a cell text document by different properties.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellTextDocumentFilter {
	/// A filter that matches against the notebook containing the notebook cell.
	///
	/// If a string value is provided it matches against the notebook type. '*' matches every notebook.
	pub notebook: NotebookDocumentFilter, // XXX(aki) or String,
	/// A language id like `python`.
	///
	/// Will be matched against the language id of the notebook cell document. '*' matches every language.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub language: Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum LanguageId {
	Abap,
	Bat,
	BibTeX,
	Clojure,
	Coffeescript,
	C,
	Cpp,
	CSharp,
	Css,
	Diff,
	Dart,
	Dockerfile,
	Elixir,
	#[serde(rename = "git-commit")]
	GitCommit,
	#[serde(rename = "git-rebase")]
	GitRebase,
	Go,
	Groovy,
	Handelbars,
	Html,
	Ini,
	Java,
	JavaScript,
	JavaScriptReact,
	Json,
	LaTeX,
	Less,
	Lua,
	Makefile,
	Markdown,
	#[serde(rename = "objective-c")]
	ObjectiveC,
	#[serde(rename = "objective-cpp")]
	ObjectiveCPP,
	Perl,
	Perl6,
	Php,
	Powershell,
	#[serde(rename = "jade")]
	Pug,
	Python,
	R,
	Razor,
	Ruby,
	Rust,
	Scss,
	Sass,
	Scala,
	ShaderLab,
	ShellScript,
	Sql,
	Swift,
	TypeScript,
	TypeScriptReact,
	#[serde(rename = "vb")]
	VisualBasic,
	Xml,
	Xsl,
	Yaml,
	#[serde(untagged)]
	Other(String),
}

/// Represents a related message and source code location for a diagnostic.
///
/// This should be used to point to code locations that cause or related to a diagnostics,
/// e.g when duplicating a symbol in a scope.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRelatedInformation {
	/// The location of this related diagnostic information.
	pub location: Location,
	/// The message of this related diagnostic information.
	pub message: String,
}

/// Structure to capture a description for an error code.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeDescription {
	/// An URI to open with more information about the diagnostic error.
	pub href: Uri,
}

/// Describes the currently selected completion item.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionCompletionInfo {
	/// The range that will be replaced if this completion item is accepted.
	pub range: Range,
	/// The text the range will be replaced with if this completion is accepted.
	pub text: String,
}

/// A change describing how to move a [`NotebookCell`] array from state S to S'.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellArrayChange {
	/// The start oftest of the cell that changed.
	pub start: u32,
	/// The deleted cells
	pub delete_count: u32,
	/// The new cells, if any
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub cells: Option<Vec<NotebookCell>>,
}

/// A notebook cell.
///
/// A cell's document URI must be unique across ALL notebook\ncells and can therefore be used to uniquely identify a\nnotebook cell or the cell's text document.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCell {
	/// The cell's kind
	pub kind: NotebookCellKind,
	/// The URI of the cell's text document content.
	pub document: Uri,
	/// Additional metadata stored with the cell.
	///
	/// Note: should always be an object literal (e.g. LSPObject)
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub metadata: Option<LspObject>,
	/// Additional execution summary information if supported by the client.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub execution_summary: Option<ExecutionSummary>,
}

/// A pattern to describe in which file operation requests or notifications the server is interested in receiving.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationPattern {
	/// The glob pattern to match.
	///
	/// Glob patterns can have the following syntax:
	/// - `*` to match zero or more characters in a path segment
	/// - `?` to match on one character in a path segment
	/// - `**` to match any number of path segments, including none
	/// - `{}` to group sub patterns into an OR expression. (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
	/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
	/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
	pub glob: String,
	/// Whether to match files or folders with this pattern. Matches both if undefined.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub matches: Option<FileOperationPatternKind>,
	/// Additional options used during matching.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub options: Option<FileOperationPatternOptions>,
}

/// A generic resource operation.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOperation {
	/// The resource operation kind.
	pub kind: String,
	/// An optional annotation identifier describing the operation.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensLegend {
	/// The token types a server uses.
	pub token_types: Vec<String>,
	/// The token modifiers a server uses.
	pub token_modifiers: Vec<String>,
}

/// A base for all symbol information.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct BaseSymbolInformation {
	/// The name of this symbol.
	pub name: String,
	/// The kind of this symbol.
	pub kind: SymbolKind,
	/// Tags for this symbol.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tags: Option<Vec<SymbolTag>>,
	/// The name of the symbol containing this symbol.
	///
	/// This information is for user interface purposes (e.g. to render a qualifier in the user interface
	/// if necessary). It can't be used to re-infer a hierarchy for the document symbols.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub container_name: Option<String>,
}

/// Value-object that contains additional information when requesting references.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceContext {
	/// Include the declaration of the current symbol.
	pub include_declaration: bool,
}

/// Additional information about the context in which a signature help request was triggered
///
/// since: 3.15.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpContext {
	/// Action that caused signature help to be triggered.
	pub trigger_kind: SignatureHelpTriggerKind,
	/// Character that caused signature help to be triggered.
	///
	/// This is undefined when `trigger_kind !== SignatureHelpTriggerKind.TriggerCharacter`
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trigger_character: Option<String>,
	/// `true` if signature help was already showing when it was triggered.
	///
	/// Retriggers occurs when the signature help is already active and can be caused by actions such as
	/// typing a trigger character, a cursor move, or document content changes.
	pub is_retrigger: bool,
	/// The currently active `SignatureHelp`.
	///
	/// The `activeSignatureHelp` has its `SignatureHelp.activeSignature` field updated based on
	/// the user navigating through available signatures.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub active_signature_help: Option<SignatureHelp>,
}

/// A special text edit to provide an insert and a replace operation.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InsertReplaceEdit {
	/// The string to be inserted.
	pub new_text: String,
	/// The range if the insert is requested
	pub insert: Range,
	/// The range if the replace is requested.
	pub replace: Range,
}

/// Additional details for a completion item label.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemLabelDetails {
	/// An optional string which is rendered less prominently directly after [`CompletionItem::label`],
	/// without any spacing.
	///
	/// Should be used for function signatures and type annotations.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub detail: Option<String>,
	/// An optional string which is rendered less prominently after [`CompletionItem::detail`]. Should be used
	/// for fully qualified names and file paths.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub description: Option<String>,
}

/// Contains additional information about the context in which a completion request is triggered.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionContext {
	/// How the completion was triggered.
	pub trigger_kind: CompletionTriggerKind,
	/// The trigger character (a single character) that has trigger code complete.
	///
	/// Is undefined if `triggerKind !== CompletionTriggerKind.TriggerCharacter`
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trigger_character: Option<String>,
}

/// Represents a diagnostic, such as a compiler error or warning.
///
/// Diagnostic objects are only valid in the scope of a resource.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
	/// The range at which the message applies
	pub range: Range,
	/// The diagnostic's severity.
	///
	/// Can be omitted.
	///
	/// If omitted it is up to the client to interpret diagnostics as error, warning, info or hint.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub severity: Option<DiagnosticSeverity>,
	/// The diagnostic's code, which usually appear in the user interface.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code: Option<ProgressToken>, // XXX(aki): Not the right type but right elements
	/// An optional property to describe the error code.
	///
	/// Requires the code field (above) to be present/not null.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub code_description: Option<CodeDescription>,
	/// A human-readable string describing the source of this diagnostic,
	/// e.g. 'typescript' or 'super lint'. It usually appears in the user interface.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub source: Option<String>,
	/// The diagnostic's message. It usually appears in the user interface
	pub message: String,
	/// Additional metadata about the diagnostic.
	///
	/// since: 3.15.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tags: Option<Vec<DiagnosticTag>>,
	/// An array of related diagnostic information, e.g. when symbol-names within
	/// a scope collide all definitions can be marked via this property.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
	/// A data entry field that is preserved between a `textDocument/publishDiagnostics`
	/// notification and `textDocument/codeAction` request.
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<LspAny>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemWatcher {
	/// The glob pattern to watch.
	///
	/// See [`GlobPattern`] for more detail.
	///
	/// since: 3.17.0 support for relative patterns.
	pub glob_pattern: GlobPattern,
	/// The kind of events of interest.
	///
	/// If omitted it defaults to WatchKind.Create | WatchKind.Change | WatchKind.Delete which is 7.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub kind: Option<WatchKind>,
}

/// An event describing a file change.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileEvent {
	/// The file's uri.
	pub uri: Uri,
	/// The change type.
	#[serde(rename = "type")]
	pub typ: FileChangeType,
}

/// General parameters to unregister a request or notification.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Unregistration {
	/// The id used to unregister the request or notification.
	///
	/// Usually an id provided during the register request.
	pub id: String,
	/// The method to unregister for.
	pub method: String,
}

/// General parameters to register for a notification or to register a provider.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
	/// The id used to register the request. The id can be used to deregister the request again.
	pub id: String,
	/// The method/capability to register for.
	pub method: String,
	/// Options necessary for the registration.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub register_options: Option<LspAny>,
}

/// A string value used as a snippet is a template which allows to insert text
/// and to control the editor cursor when insertion happens.
///
/// A snippet can define tab stops and placeholders with `$1`, `$2`\nand `${3:foo}`.
/// `$0` defines the final tab stop, it defaults to\nthe end of the snippet. Variables are defined
/// with `$name` and\n`${name:default value}`.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StringValue {
	// `snippet`
	/// The kind of string value.
	pub kind: String,
	/// The snippet string.
	pub value: String,
}

/// Provides information about the context in which an inline completion was requested.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionContext {
	/// Describes how the inline completion was triggered.
	pub trigger_kind: InlineCompletionTriggerKind,
	/// Provides information about the currently selected item in the autocomplete widget if it is visible.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub selected_completion_info: Option<SelectionCompletionInfo>,
}

/// A literal to identify a notebook document in the client.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentIdentifier {
	/// The notebook document's uri.
	pub uri: Uri,
}

/// A versioned notebook document identifier.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct VersionedNotebookDocumentIdentifier {
	/// The version number of this notebook document.
	pub version: i32,
	/// The notebook document's uri.
	pub uri: Uri,
}

/// A text document identifier to denote a specific version of a text document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct VersionedTextDocumentIdentifier {
	/// The version number of this document.
	pub version: i32,
	#[serde(flatten)]
	pub text_document_identifier: TextDocumentIdentifier,
}

/// An item to transfer a text document from the client to the server.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
	/// The text document's uri.
	pub uri: Uri,
	/// The text document's language identifier.
	pub language_id: LanguageId,
	/// The version number of this document (it will increase after each change, including undo/redo).
	pub version: i32,
	/// The content of the opened text document.
	pub text: String,
}

/// A notebook document.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocument {
	/// The notebook document's uri.
	pub uri: Uri,
	/// The type of the notebook.
	pub notebook_type: String,
	/// The version number of this document (it will increase after each change, including undo/redo).
	pub version: i32,
	/// Additional metadata stored with the notebook document.
	///
	/// Note: should always be an object literal (e.g. LSPObject)
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub metadata: Option<LspObject>,
	/// The cells of a notebook.
	pub cells: Vec<NotebookCell>,
}

/// A previous result id in a workspace pull request.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PreviousResultId {
	/// The URI for which the client knowns a result id.
	pub uri: Uri,
	/// The value of the previous result id.
	pub value: String,
}

/// A diagnostic report indicating that the last returned report is still accurate.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct UnchangedDocumentDiagnosticReport {
	// `unchanged`
	/// A document diagnostic report indicating no changes to the last result.
	///
	/// A server can only return `unchanged` if result ids are provided.
	pub kind: String,
	/// A result id which will be sent on the next diagnostic request for the same document.
	pub result_id: String,
}

/// A diagnostic report with a full set of problems.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullDocumentDiagnosticReport {
	// `full`
	/// A full document diagnostic report.
	pub kind: String,
	/// An optional result id.
	///
	/// If provided it will be sent on the next diagnostic request for the same document.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub result_id: Option<String>,
	/// The actual items.
	pub items: Vec<Diagnostic>,
}

/// Represents information about programming constructs like variables, classes interfaces etc.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInformation {
	/// Indicates if this symbol is deprecated.
	///
	/// deprecated: Use tags instead
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub deprecated: Option<bool>,
	/// The location of this symbol.
	///
	/// The location's range is used by a tool to reveal the location in the editor.
	///
	/// If the symbol is selected in the tool the range's start information is used
	/// to position the cursor. So the range usually spans more than the actual
	/// symbol's name and does normally include things like visibility modifiers.
	///
	/// The range doesn't have to denote a node range in the sense of an abstract syntax tree.
	/// It can therefore not be used to re-construct a hierarchy of the symbols.
	pub location: Location,
	#[serde(flatten)]
	pub base_symbol_info: BaseSymbolInformation,
}

/// Contains additional diagnostic information about the context in which
/// a [`CodeActionProvider::provide_code_actions`] is run.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionContext {
	/// An array of diagnostics known on the client side overlapping the range provided to the\n`textDocument/codeAction` request. They are provided so that the server knows which\nerrors are currently presented to the user for the given range. There is no guarantee\nthat these accurately reflect the error state of the resource. The primary parameter\nto compute code actions is the provided range.
	pub diagnostics: Vec<Diagnostic>,
	/// Requested kind of actions to return.
	///
	/// Actions not of this kind are filtered out by the client before being shown. So servers\ncan omit computing them.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub only: Option<Vec<CodeActionKind>>,
	/// The reason why code actions were requested.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub trigger_kind: Option<CodeActionTriggerKind>,
}

/// A full diagnostic report with a set of related documents.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedFullDocumentDiagnosticReport {
	/// Diagnostics of related documents.
	///
	/// This information is useful in programming languages where code in a file A can generate
	/// diagnostics in a file B which A depends on.
	///
	/// An example of such a language is C and C++ where marco definitions in a file `a.cpp`` can result
	/// in errors in a header file `b.hpp`.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub related_documents: Option<HashMap<Uri, DiagnosticReport>>,
	#[serde(flatten)]
	pub full_document_diagnostic_report: FullDocumentDiagnosticReport,
}

/// An unchanged diagnostic report with a set of related documents.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedUnchangedDocumentDiagnosticReport {
	/// Diagnostics of related documents.
	///
	/// This information is useful in programming languages where code in a file A can generate
	/// diagnostics in a file B which A depends on.
	///
	/// An example of such a language is C and C++ where marco definitions in a file `a.cpp`` can result
	/// in errors in a header file `b.hpp`.
	///
	/// since: 3.17.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub related_documents: Option<HashMap<Uri, DiagnosticReport>>,
	#[serde(flatten)]
	pub unchanged_document_diagnostic_report: UnchangedDocumentDiagnosticReport,
}

/// A full document diagnostic report for a workspace diagnostic result.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFullDocumentDiagnosticReport {
	/// The URI for which diagnostic information is reported.
	pub uri: Uri,
	/// The version number for which the diagnostics are reported.
	///
	/// If the document is not marked as open `null` can be provided.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<i32>,
	#[serde(flatten)]
	pub full_document_diagnostic_report: FullDocumentDiagnosticReport,
}

/// An unchanged document diagnostic report for a workspace diagnostic result.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUnchangedDocumentDiagnosticReport {
	/// The URI for which diagnostic information is reported.
	pub uri: Uri,
	/// The version number for which the diagnostics are reported.
	///
	/// If the document is not marked as open `null` can be provided.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub version: Option<i32>,
	#[serde(flatten)]
	pub unchanged_document_diagnostic_report: UnchangedDocumentDiagnosticReport,
}

/// Information about the LSP client
///
/// since: 3.15.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
	/// The name of the client as defined by the client.
	pub name: String,
	/// The client's version as defined by the client.
	pub version: Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum LocationOrUri {
	Location(Location),
	Uri { uri: Uri },
}

/// A special workspace symbol that supports locations without a range.
///
/// See also: [`SymbolInformation`]
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbol {
	/// The location of the symbol.
	///
	/// Whether a server is allowed to return a location without a range depends on the client
	/// capability `workspace.symbol.resolveSupport`.
	///
	/// See: [`SymbolInformation::location`] for more details.
	pub location: LocationOrUri,
	/// A data entry field that is preserved on a workspace symbol between a workspace symbol
	/// request and a workspace symbol resolve request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<LspAny>,
	#[serde(flatten)]
	pub base_symbol_information: BaseSymbolInformation,
}

/// Changes to the cell structure to add or remove cells.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellStructureChange {
	/// The change to the cell array.
	pub array: NotebookCellArrayChange,
	/// Additional opened cell text documents
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_open: Option<Vec<TextDocumentItem>>,
	/// Additional closed cell text documents.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub did_close: Option<Vec<TextDocumentIdentifier>>,
}

/// Changes to the text content of notebook cells.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellTextChange {
	pub document: VersionedTextDocumentIdentifier,
	pub changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellsDiff {
	/// Changes to the cell structure to add or remove cells
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub structure: Option<NotebookCellStructureChange>,
	/// Changes to notebook cells properties like its kind, execution summary or metadata.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<Vec<NotebookCell>>,
	/// Changes to the text content of notebook cells.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub text_context: Option<Vec<NotebookCellTextChange>>,
}

/// A change event for a notebook document.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentChangeEvent {
	/// The changed meta data if any.
	///
	/// Note: should always be an object literal (e.g. LSPObject)
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub metadata: Option<LspObject>,
	/// Changes to cells
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub cells: Option<NotebookCellsDiff>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum OptionsOrBool<T> {
	Options(T),
	Bool(bool),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum RegistrationOptionsOrBool<T, U> {
	Options(T),
	RegistrationOptions(U),
	Bool(bool),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum RegistrationOrOptions<T, U> {
	Options(T),
	RegistrationOptions(U),
}

/// Represents the signature of something callable.
///
/// A signature can have a label, like a function-name, a doc-comment, and a set of parameters.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInformation {
	/// The label of this signature.
	///
	/// Will be shown in the UI.
	pub label: String,
	/// The human-readable doc-comment of this signature.
	///
	/// Will be shown in the UI but can be omitted.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub documentation: Option<MarkupOrString>,
	/// The parameters of this signature.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub parameters: Option<Vec<ParameterInformation>>,
	/// The index of the active parameter.
	///
	/// If provided, this is used in place of [`SignatureHelp::active_parameter`].
	///
	/// since: 3.16.0
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub active_parameter: Option<i32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum NotebookDocumentFilterOrString {
	String(String),
	Filter(NotebookDocumentFilter),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellLanguage {
	pub language: String,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncOptionsSelector {
	/// The notebook to be synced If a string\nvalue is provided it matches against the\nnotebook type. '*' matches every notebook.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub notebook: Option<NotebookDocumentFilterOrString>,
	/// The cells of the matching notebook to be synced.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub cells: Option<Vec<NotebookCellLanguage>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum StringOrBool {
	String(String),
	Bool(bool),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum StringOrTuple<T> {
	String(String),
	Tuple((T, T)),
}

/// Represents a parameter of a callable-signature.
///
/// A parameter can have a label and a doc-comment.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ParameterInformation {
	/// The label of this parameter information.
	///
	/// Either a string or an inclusive start and exclusive end offsets within its containing
	/// signature label. (see [`SignatureInformation::label`]).
	///
	/// The offsets are based on a UTF-16 string representation as [`Position`] and [`Range`] does.
	///
	/// *Note*: a label of type string should be a substring of its containing signature label.
	///
	/// Its intended use case is to highlight the parameter label part in the [`SignatureInformation::label`].
	pub label: StringOrTuple<u32>,
	/// The human-readable doc-comment of this parameter.
	///
	/// Will be shown in the UI but can be omitted.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub documentation: Option<MarkupOrString>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet<T> {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value_set: Option<Vec<T>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StringProperties {
	pub properties: Vec<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionListItemDefaults {
	pub item_defaults: Vec<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum SemanticTokensFullRequestsType {
	Bool(bool),
	Delta { delta: Option<bool> },
}

#[cfg(test)]
mod tests {
	// use super::*;
}
