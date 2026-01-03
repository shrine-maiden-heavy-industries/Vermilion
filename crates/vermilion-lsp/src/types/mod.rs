/* SPDX-License-Identifier: BSD-3-Clause */
// TODO(aki):
// There are many cases where structures have a `kind` member that are supposed to be a fixed value,
// We should try to turn these into common enums if possible.

pub mod capabilities;
pub mod options;
pub mod params;
pub mod semantic_tokens;

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
pub struct Position {
	pub(crate) line: u32,
	pub(crate) character: u32,
}

/// A range in a text document expressed as (zero-based) start and end positions.
///
/// A range is comparable to a selection in an editor. Therefore, the end position is exclusive.
/// If you want to specify a range that contains a line including the line ending character(s) then
/// use an end position denoting the start of the next line.
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
pub struct Range {
	pub(crate) start: Position,
	pub(crate) end: Position,
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
	pub(crate) uri: Uri,
	pub(crate) range: Range,
}

/// A workspace folder inside a client
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFolder {
	pub(crate) uri: Uri,
	pub(crate) name: String,
}

/// Represents a color range from a document
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInformation {
	pub(crate) range: Range,
	pub(crate) color: Color,
}

/// Represents a color in RGBA space.
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
	pub(crate) red: f32,
	pub(crate) green: f32,
	pub(crate) blue: f32,
	pub(crate) alpha: f32,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ColorPresentation {
	pub(crate) label: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text_edit: Option<TextEdit>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) additional_text_edits: Option<Vec<TextEdit>>,
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
	pub(crate) start_line: u32,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) start_character: Option<u32>,
	pub(crate) end_line: u32,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_character: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<FoldingRangeKind>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) collapsed_text: Option<String>,
}

/// A selection range represents a part of a selection hierarchy.
///
/// A selection range may have a parent selection range that contains it.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRange {
	pub(crate) range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) parent: Option<Box<SelectionRange>>,
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
	pub(crate) name: String,
	pub(crate) kind: SymbolKind,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tags: Option<Vec<SymbolTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) detail: Option<String>,
	pub(crate) uri: Uri,
	pub(crate) range: Range,
	pub(crate) selection_range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
}

/// Represents an incoming call, e.g. a caller of a method or constructor.
///
/// since: 3.16.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyIncomingCall {
	pub(crate) from: CallHierarchyItem,
	pub(crate) from_ranges: Vec<Range>,
}

/// Represents an outgoing call, e.g. calling a getter from a method or a method from a constructor etc.
///
/// since: 3.16.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyOutgoingCall {
	pub(crate) to: CallHierarchyItem,
	pub(crate) to_ranges: Vec<Range>,
}

/// The result of a showDocument request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShowDocumentResult {
	pub(crate) success: bool,
}

/// The result of a linked editing range request.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRanges {
	pub(crate) ranges: Vec<Range>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) word_pattern: Option<String>,
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
	pub(crate) kind: String,
	pub(crate) uri: Uri,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) options: Option<CreateFileOptions>,
	#[serde(flatten)]
	pub(crate) resource_operation: ResourceOperation,
}

/// Rename file operation
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RenameFile {
	pub(crate) kind: String,
	pub(crate) old_uri: Uri,
	pub(crate) new_uri: Uri,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) options: Option<RenameFileOptions>,
	#[serde(flatten)]
	pub(crate) resource_operation: ResourceOperation,
}

/// Delete file operation.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFile {
	pub(crate) kind: String,
	pub(crate) uri: Uri,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) options: Option<DeleteFileOptions>,
	#[serde(flatten)]
	pub(crate) resource_operation: ResourceOperation,
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
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEdit {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) changes: Option<HashMap<Uri, Vec<TextEdit>>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) document_changes: Option<Vec<DocumentChange>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) change_annotations: Option<HashMap<ChangeAnnotationIdentifier, ChangeAnnotation>>,
}

/// Moniker definition to match LSIF 0.5 moniker definition.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Moniker {
	pub(crate) scheme: String,
	pub(crate) identifier: String,
	pub(crate) unique: UniquenessLevel,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<MonikerKind>,
}

/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyItem {
	pub(crate) name: String,
	pub(crate) kind: SymbolKind,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tags: Option<Vec<SymbolTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) detail: Option<String>,
	pub(crate) uri: Uri,
	pub(crate) range: Range,
	pub(crate) selection_range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
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
	pub(crate) position: Position,
	pub(crate) label: InlayHintLabel,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<InlayHintKind>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text_edits: Option<Vec<TextEdit>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tooltip: Option<MarkupOrString>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) padding_left: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) padding_right: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
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
	pub(crate) related_documents: HashMap<Uri, DiagnosticReport>,
}

/// Cancellation data returned from a diagnostic request.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticServerCancellationData {
	pub(crate) retrigger_request: bool,
}

/// A workspace diagnostic report.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDiagnosticReport {
	pub(crate) items: Vec<WorkspaceDocumentDiagnosticReport>,
}

/// A partial result for a workspace diagnostic report.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDiagnosticReportPartialResult {
	pub(crate) items: Vec<WorkspaceDocumentDiagnosticReport>,
}

/// Represents a collection of [`InlineCompletionItem`]s to be presented in the editor.
///
/// since: 3.18.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionList {
	pub(crate) items: Vec<InlineCompletionItem>,
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
	pub(crate) insert_text: InsertText,
	/// A text that is used to decide if this inline completion should be shown.
	///
	/// When `falsy` the [`InlineCompletionItem::insert_text`] is used.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) filter_text: Option<String>,
	/// The range to replace. Must begin and end on the same line.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) range: Option<Range>,
	/// An optional [`Command`] that is executed *after* inserting this completion.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) command: Option<Command>,
}

/// The result returned from an initialize request.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
	pub(crate) capabilities: ServerCapabilities,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) server_info: Option<ServerInfo>,
}

/// Information about the server.
///
/// since: 3.15.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
	pub(crate) name: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version: Option<String>,
}

/// The data type of the ResponseError if the initialize request fails.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InitializeError {
	pub(crate) retry: bool,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItem {
	pub(crate) title: String,
}

/// A text edit applicable to a text document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
	pub(crate) range: Range,
	pub(crate) new_text: String,
}

/// A completion item represents a text snippet that is proposed to complete text that is being typed.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
	pub(crate) label: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) label_details: Option<CompletionItemLabelDetails>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<CompletionItemKind>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tags: Option<Vec<CompletionItemTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) detail: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) documentation: Option<MarkupOrString>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) deprecated: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) preselect: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) sort_text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) filter_text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) insert_text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) insert_text_format: Option<InsertTextFormat>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) insert_text_mode: Option<InsertTextMode>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text_edit: Option<TextEdit>, // XXX(aki), or InsertReplaceEdit
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text_edit_text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) additional_text_edits: Option<Vec<TextEdit>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) commit_characters: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) command: Option<Command>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDefaults {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub commit_characters: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub edit_range: Option<Range>, // XXX(aki) Or Position
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text_format: Option<InsertTextFormat>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub insert_text_mode: Option<InsertTextMode>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub data: Option<serde_json::Value>,
}

/// Represents a collection of [`CompletionItem`]s completion to be presented in the editor.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionList {
	pub(crate) is_incomplete: bool,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) item_defaults: Option<ItemDefaults>,
	pub(crate) items: Vec<CompletionItem>,
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
	pub(crate) contents: HoverContents,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) range: Option<Range>,
}

/// Signature help represents the signature of something callable.
///
/// There can be multiple signature but only one active and only one active parameter.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelp {
	pub(crate) signatures: Vec<SignatureInformation>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) active_signature: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) active_parameter: Option<u32>,
}

/// A document highlight is a range inside a text document which deserves special attention.
///
/// Usually a document highlight is visualized by changing the background color of its range.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlight {
	pub(crate) range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<DocumentHighlightKind>,
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
	pub(crate) name: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) detail: Option<String>,
	pub(crate) kind: SymbolKind,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tags: Option<Vec<SymbolTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) deprecated: Option<bool>,
	pub(crate) range: Range,
	pub(crate) selection_range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) children: Option<Vec<DocumentSymbol>>,
}

/// Represents a reference to a command.
///
/// Provides a title which will be used to represent a command in the UI and,
/// optionally,an array of arguments which will be passed to the command handler
/// function when invoked.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
	pub(crate) title: String,
	pub(crate) command: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) arguments: Option<Vec<serde_json::Value>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DisabledReason {
	pub(crate) reason: String,
}

/// A code action represents a change that can be performed in code, e.g. to fix a problem or to
/// refactor code.
///
/// A CodeAction must set either `edit` and/or a `command`. If both are supplied, the `edit` is applied first,
/// then the `command` is executed.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
	pub(crate) title: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<CodeActionKind>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) diagnostics: Option<Vec<Diagnostic>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) is_preferred: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) disabled: Option<DisabledReason>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) edit: Option<WorkspaceEdit>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) command: Option<Command>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
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
	pub(crate) range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) command: Option<Command>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
}

/// A document link is a range in a text document that links to an internal or external resource,
/// like another text document or a web site.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLink {
	pub(crate) range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) target: Option<Uri>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tooltip: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<serde_json::Value>,
}

/// The result returned from the apply workspace edit request.
///
/// since: 3.17.0 renamed from ApplyWorkspaceEditResponse
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ApplyWorkspaceEditResult {
	pub(crate) applied: bool,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) failure_reason: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) failed_change: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressBegin {
	pub(crate) kind: String,
	pub(crate) title: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) cancellable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) percentage: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressReport {
	pub(crate) kind: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) cancellable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) percentage: Option<u32>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressEnd {
	pub(crate) kind: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) message: Option<String>,
}

/// The workspace folder change event.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersChangeEvent {
	pub(crate) added: Vec<WorkspaceFolder>,
	pub(crate) removed: Vec<WorkspaceFolder>,
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
pub struct ConfigurationItem {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) scope_uri: Option<Uri>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) section: Option<String>,
}

/// A literal to identify a text document in the client.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
	pub(crate) uri: Uri,
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
pub struct TextDocumentFilter {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) language: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) scheme: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) pattern: Option<String>,
}

/// A notebook document filter denotes a notebook document by different properties.
///
/// The properties will be match against the notebook's URI (same as with documents)
///
/// See: [`TextDocumentFilter`]
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
pub struct NotebookDocumentFilter {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) notebook_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) scheme: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) pattern: Option<String>,
}

/// Represents information on a file/folder create.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileCreate {
	pub(crate) uri: String,
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
	pub(crate) annotation_id: ChangeAnnotationIdentifier,
	#[serde(flatten)]
	pub(crate) text_edit: TextEdit,
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
	pub(crate) text_document: OptionalVersionedTextDocumentIdentifier,
	pub(crate) edits: TextEditType,
}

/// A text document identifier to optionally denote a specific version of a text document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OptionalVersionedTextDocumentIdentifier {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version: Option<i32>,
	#[serde(flatten)]
	pub(crate) text_document_identifier: TextDocumentIdentifier,
}

/// Additional information that describes document changes.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnotation {
	pub(crate) label: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) needs_confirmation: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) description: Option<String>,
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
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) scheme: Option<String>,
	pub(crate) pattern: FileOperationPattern,
}

/// Represents information on a file/folder rename.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileRename {
	pub(crate) old_uri: String,
	pub(crate) new_uri: String,
}

/// Represents information on a file/folder delete.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileDelete {
	pub(crate) uri: String,
}

/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueContext {
	pub(crate) frame_id: i32,
	pub(crate) stopped_location: Range,
}

/// Provide inline value as text.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueText {
	pub(crate) range: Range,
	pub(crate) text: String,
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
	pub(crate) base_uri: FolderOrUri,
	pub(crate) pattern: Pattern,
}

/// An event describing a change to a text document.
///
/// If only a text is provided it is considered to be the full content of the document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentChangeEvent {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) range: Option<Range>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) range_length: Option<u32>,
	pub(crate) text: String,
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
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) origin_selection_range: Option<Range>,
	pub(crate) target_uri: Uri,
	pub(crate) target_range: Range,
	pub(crate) target_selection_range: Range,
}

/// An inlay hint label part allows for interactive and composite labels of inlay hints.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintLabelPart {
	pub(crate) value: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tooltip: Option<MarkupOrString>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) location: Option<Location>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) command: Option<Command>,
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
	pub(crate) kind: MarkupKind,
	pub(crate) value: String,
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
	pub(crate) range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) variable_name: Option<String>,
	pub(crate) case_sensitive_lookup: bool,
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
	pub(crate) range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) expression: Option<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionSummary {
	pub(crate) execution_order: u32,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) success: Option<bool>,
}

/// A notebook cell text document filter denotes a cell text document by different properties.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellTextDocumentFilter {
	pub(crate) notebook: NotebookDocumentFilter,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) language: Option<String>,
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
	pub(crate) location: Location,
	pub(crate) message: String,
}

/// Structure to capture a description for an error code.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CodeDescription {
	pub(crate) href: Uri,
}

/// Describes the currently selected completion item.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SelectionCompletionInfo {
	pub(crate) range: Range,
	pub(crate) text: String,
}

/// A change describing how to move a [`NotebookCell`] array from state S to S'.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellArrayChange {
	pub(crate) start: u32,
	pub(crate) delete_count: u32,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) cells: Option<Vec<NotebookCell>>,
}

/// A notebook cell.
///
/// A cell's document URI must be unique across ALL notebook\ncells and can therefore be used to uniquely identify a\nnotebook cell or the cell's text document.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCell {
	pub(crate) kind: NotebookCellKind,
	pub(crate) document: Uri,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) metadata: Option<LspObject>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) execution_summary: Option<ExecutionSummary>,
}

/// A pattern to describe in which file operation requests or notifications the server is interested in receiving.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationPattern {
	pub(crate) glob: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) matches: Option<FileOperationPatternKind>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) options: Option<FileOperationPatternOptions>,
}

/// A generic resource operation.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOperation {
	pub(crate) kind: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) annotation_id: Option<ChangeAnnotationIdentifier>,
}

/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensLegend {
	pub(crate) token_types: Vec<String>,
	pub(crate) token_modifiers: Vec<String>,
}

/// A base for all symbol information.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct BaseSymbolInformation {
	pub(crate) name: String,
	pub(crate) kind: SymbolKind,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub tags: Option<Vec<SymbolTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub container_name: Option<String>,
}

/// Value-object that contains additional information when requesting references.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceContext {
	pub(crate) include_declaration: bool,
}

/// Additional information about the context in which a signature help request was triggered
///
/// since: 3.15.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpContext {
	pub(crate) trigger_kind: SignatureHelpTriggerKind,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) trigger_character: Option<String>,
	pub(crate) is_retrigger: bool,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) active_signature_help: Option<SignatureHelp>,
}

/// A special text edit to provide an insert and a replace operation.
///
/// since: 3.16.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InsertReplaceEdit {
	pub(crate) new_text: String,
	pub(crate) insert: Range,
	pub(crate) replace: Range,
}

/// Additional details for a completion item label.
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
pub struct CompletionItemLabelDetails {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) detail: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) description: Option<String>,
}

/// Contains additional information about the context in which a completion request is triggered.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionContext {
	pub(crate) trigger_kind: CompletionTriggerKind,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) trigger_character: Option<String>,
}

/// Represents a diagnostic, such as a compiler error or warning.
///
/// Diagnostic objects are only valid in the scope of a resource.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
	pub(crate) range: Range,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) severity: Option<DiagnosticSeverity>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code: Option<ProgressToken>, // XXX(aki): Not the right type but right elements
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) code_description: Option<CodeDescription>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source: Option<String>,
	pub(crate) message: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) tags: Option<Vec<DiagnosticTag>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) related_information: Option<Vec<DiagnosticRelatedInformation>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<LspAny>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemWatcher {
	pub(crate) glob_pattern: GlobPattern,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind: Option<WatchKind>,
}

/// An event describing a file change.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FileEvent {
	pub(crate) uri: Uri,
	#[serde(rename = "type")]
	pub(crate) typ: FileChangeType,
}

/// General parameters to unregister a request or notification.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Unregistration {
	pub(crate) id: String,
	pub(crate) method: String,
}

/// General parameters to register for a notification or to register a provider.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
	pub(crate) id: String,
	pub(crate) method: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) register_options: Option<LspAny>,
}

/// A string value used as a snippet is a template which allows to insert text
/// and to control the editor cursor when insertion happens.
///
/// A snippet can define tab stops and placeholders with `$1`, `$2`\nand `${3:foo}`.
/// `$0` defines the final tab stop, it defaults to\nthe end of the snippet. Variables are defined
/// with `$name` and `${name:default value}`.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StringValue {
	pub(crate) kind: String,
	pub(crate) value: String,
}

/// Provides information about the context in which an inline completion was requested.
///
/// since: 3.18.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InlineCompletionContext {
	pub(crate) trigger_kind: InlineCompletionTriggerKind,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) selected_completion_info: Option<SelectionCompletionInfo>,
}

/// A literal to identify a notebook document in the client.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentIdentifier {
	pub(crate) uri: Uri,
}

/// A versioned notebook document identifier.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct VersionedNotebookDocumentIdentifier {
	pub(crate) version: i32,
	pub(crate) uri: Uri,
}

/// A text document identifier to denote a specific version of a text document.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct VersionedTextDocumentIdentifier {
	pub(crate) version: i32,
	#[serde(flatten)]
	pub(crate) text_document_identifier: TextDocumentIdentifier,
}

/// An item to transfer a text document from the client to the server.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
	pub(crate) uri: Uri,
	pub(crate) language_id: LanguageId,
	pub(crate) version: i32,
	pub(crate) text: String,
}

/// A notebook document.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocument {
	pub(crate) uri: Uri,
	pub(crate) notebook_type: String,
	pub(crate) version: i32,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) metadata: Option<LspObject>,
	pub(crate) cells: Vec<NotebookCell>,
}

/// A previous result id in a workspace pull request.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PreviousResultId {
	pub(crate) uri: Uri,
	pub(crate) value: String,
}

/// A diagnostic report indicating that the last returned report is still accurate.
///
/// since: 3.17.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct UnchangedDocumentDiagnosticReport {
	pub(crate) kind: String,
	pub(crate) result_id: String,
}

/// A diagnostic report with a full set of problems.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullDocumentDiagnosticReport {
	pub(crate) kind: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) result_id: Option<String>,
	pub(crate) items: Vec<Diagnostic>,
}

/// Represents information about programming constructs like variables, classes interfaces etc.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInformation {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) deprecated: Option<bool>,
	pub(crate) location: Location,
	#[serde(flatten)]
	pub(crate) base_symbol_info: BaseSymbolInformation,
}

/// Contains additional diagnostic information about the context in which
/// a [`CodeActionProvider::provide_code_actions`] is run.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionContext {
	pub(crate) diagnostics: Vec<Diagnostic>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) only: Option<Vec<CodeActionKind>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) trigger_kind: Option<CodeActionTriggerKind>,
}

/// A full diagnostic report with a set of related documents.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedFullDocumentDiagnosticReport {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) related_documents: Option<HashMap<Uri, DiagnosticReport>>,
	#[serde(flatten)]
	pub(crate) full_document_diagnostic_report: FullDocumentDiagnosticReport,
}

/// An unchanged diagnostic report with a set of related documents.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedUnchangedDocumentDiagnosticReport {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) related_documents: Option<HashMap<Uri, DiagnosticReport>>,
	#[serde(flatten)]
	pub(crate) unchanged_document_diagnostic_report: UnchangedDocumentDiagnosticReport,
}

/// A full document diagnostic report for a workspace diagnostic result.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFullDocumentDiagnosticReport {
	pub(crate) uri: Uri,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version: Option<i32>,
	#[serde(flatten)]
	pub(crate) full_document_diagnostic_report: FullDocumentDiagnosticReport,
}

/// An unchanged document diagnostic report for a workspace diagnostic result.
///
/// since: 3.17.0
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUnchangedDocumentDiagnosticReport {
	pub(crate) uri: Uri,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version: Option<i32>,
	#[serde(flatten)]
	pub(crate) unchanged_document_diagnostic_report: UnchangedDocumentDiagnosticReport,
}

/// Information about the LSP client
///
/// since: 3.15.0
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
	pub(crate) name: String,
	pub(crate) version: Option<String>,
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
	pub(crate) location: LocationOrUri,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<LspAny>,
	#[serde(flatten)]
	pub(crate) base_symbol_information: BaseSymbolInformation,
}

/// Changes to the cell structure to add or remove cells.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellStructureChange {
	pub(crate) array: NotebookCellArrayChange,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_open: Option<Vec<TextDocumentItem>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) did_close: Option<Vec<TextDocumentIdentifier>>,
}

/// Changes to the text content of notebook cells.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellTextChange {
	pub(crate) document: VersionedNotebookDocumentIdentifier,
	pub(crate) changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellsDiff {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) structure: Option<NotebookCellStructureChange>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) data: Option<Vec<NotebookCell>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text_context: Option<Vec<NotebookCellTextChange>>,
}

/// A change event for a notebook document.
///
/// since: 3.17.0
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentChangeEvent {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) metadata: Option<LspObject>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) cells: Option<NotebookCellsDiff>,
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
	pub label: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub documentation: Option<MarkupOrString>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub parameters: Option<Vec<ParameterInformation>>,
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
	pub(crate) language: String,
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
pub struct NotebookDocumentSyncOptionsSelector {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) notebook: Option<NotebookDocumentFilterOrString>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) cells: Option<Vec<NotebookCellLanguage>>,
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
	pub(crate) label: StringOrTuple<u32>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) documentation: Option<MarkupOrString>,
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
pub struct ValueSet<T> {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) value_set: Option<Vec<T>>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StringProperties {
	pub(crate) properties: Vec<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionListItemDefaults {
	pub(crate) item_defaults: Vec<String>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum SemanticTokensFullRequestsType {
	Bool(bool),
	Delta { delta: Option<bool> },
}

impl Position {
	pub fn new(line: u32, character: u32) -> Self {
		Self { line, character }
	}

	/// Line position in a document (zero-based).
	pub fn line(&self) -> &u32 {
		&self.line
	}

	/// Character offset on a line in a document (zero-based).
	/// The meaning of this offset is determined by the negotiated [`PositionEncodingKind`].
	///
	/// If the character value is greater than the line length it defaults back
	/// to the line length.
	pub fn character(&self) -> &u32 {
		&self.character
	}
}

impl Range {
	pub fn new(start: Position, end: Position) -> Self {
		Self { start, end }
	}

	/// The range's start position.
	pub fn start(&self) -> &Position {
		&self.start
	}

	/// The range's end position.
	pub fn end(&self) -> &Position {
		&self.end
	}
}

impl Location {
	pub fn new(uri: Uri, range: Range) -> Self {
		Self { uri, range }
	}

	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	pub fn range(&self) -> &Range {
		&self.range
	}
}

impl WorkspaceFolder {
	pub fn new(uri: Uri, name: String) -> Self {
		Self { uri, name }
	}

	/// The associated URI for this workspace folder
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The name of the workspace folder. Used to refer to this workspace folder in the user interface
	pub fn name(&self) -> &String {
		&self.name
	}
}

impl ColorInformation {
	pub fn new(range: Range, color: Color) -> Self {
		Self { range, color }
	}

	/// The range in the document where this color appears
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The actual color value for this color range
	pub fn color(&self) -> &Color {
		&self.color
	}
}

impl Color {
	pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Self { red, green, blue, alpha }
	}

	// TODO(aki): To/from hex

	/// The red component of this color in the range [0-1].
	pub fn red(&self) -> &f32 {
		&self.red
	}

	/// The green component of this color in the range [0-1].
	pub fn green(&self) -> &f32 {
		&self.green
	}

	/// The blue component of this color in the range [0-1].
	pub fn blue(&self) -> &f32 {
		&self.blue
	}

	/// The alpha component of this color in the range [0-1].
	pub fn alpha(&self) -> &f32 {
		&self.alpha
	}
}

impl ColorPresentation {
	pub fn new(label: String) -> Self {
		Self {
			label,
			text_edit: None,
			additional_text_edits: None,
		}
	}

	pub fn with_text_edit(mut self, text_edit: TextEdit) -> Self {
		self.text_edit = Some(text_edit);
		self
	}

	pub fn with_additional_text_edits(mut self, additional_text_edits: Vec<TextEdit>) -> Self {
		self.additional_text_edits = Some(additional_text_edits);
		self
	}

	/// The label of this color presentation. It will be shown on the color picker header.
	///
	/// By default this is also the text that is inserted when selecting this color presentation.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// A [`TextEdit`] which is applied to a document when selecting this presentation for the color.
	///
	/// When `falsy` the [`ColorPresentation::label`] is used.
	pub fn text_edit(&self) -> Option<&TextEdit> {
		self.text_edit.as_ref()
	}

	/// An optional array of additional [`TextEdit`]'s that are applied when selecting this color presentation.
	///
	/// Edits must not overlap with the main [`ColorPresentation::text_edit`] nor with themselves.
	pub fn additional_text_edits(&self) -> Option<&Vec<TextEdit>> {
		self.additional_text_edits.as_ref()
	}
}

impl FoldingRange {
	pub fn new(start_line: u32, end_line: u32) -> Self {
		Self {
			start_line,
			start_character: None,
			end_line,
			end_character: None,
			kind: None,
			collapsed_text: None,
		}
	}

	pub fn with_start_character(mut self, start_character: u32) -> Self {
		self.start_character = Some(start_character);
		self
	}

	pub fn with_end_character(mut self, end_character: u32) -> Self {
		self.end_character = Some(end_character);
		self
	}

	pub fn with_kind(mut self, kind: FoldingRangeKind) -> Self {
		self.kind = Some(kind);
		self
	}

	pub fn with_collapsed_text(mut self, collapsed_text: String) -> Self {
		self.collapsed_text = Some(collapsed_text);
		self
	}

	/// The zero-based start line of the range to fold.
	///
	/// The folded area starts after the line's last character. To be valid, the end must be zero
	/// or larger and smaller than the number of lines in the document.
	pub fn start_line(&self) -> &u32 {
		&self.start_line
	}

	/// The zero-based character offset from where the folded range starts.
	///
	/// If not defined, defaults to the length of the start line.
	pub fn start_character(&self) -> Option<&u32> {
		self.start_character.as_ref()
	}

	/// The zero-based end line of the range to fold.
	///
	/// The folded area ends with the line's last character. To be valid, the end must be zero
	/// or larger and smaller than the number of lines in the document.
	pub fn end_line(&self) -> &u32 {
		&self.end_line
	}

	/// The zero-based character offset before the folded range ends.
	///
	/// If not defined, defaults to the length of the end line.
	pub fn end_character(&self) -> Option<&u32> {
		self.end_character.as_ref()
	}

	/// Describes the kind of the folding range such as `comment' or 'region'.
	///
	/// The kind is used to categorize folding ranges and used by commands like 'Fold all comments'.
	///
	/// See [`FoldingRangeKind`] for an enumeration of standardized kinds.
	pub fn kind(&self) -> Option<&FoldingRangeKind> {
		self.kind.as_ref()
	}

	/// The text that the client should show when the specified range is collapsed.
	///
	/// If not defined or not supported by the client, a default will be chosen by the client.
	///
	/// since: 3.17.0
	pub fn collapsed_text(&self) -> Option<&String> {
		self.collapsed_text.as_ref()
	}
}

impl SelectionRange {
	pub fn new(range: Range) -> Self {
		Self { range, parent: None }
	}

	pub fn with_parent(mut self, parent: Box<SelectionRange>) -> Self {
		self.parent = Some(parent);
		self
	}

	/// The of this selection range
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The parent selection range containing this range.
	///
	/// Therefore `parent.range` must contain `this.range`.
	pub fn parent(&self) -> Option<&Box<SelectionRange>> {
		self.parent.as_ref()
	}
}

impl CallHierarchyItem {
	pub fn new(
		name: String,
		kind: SymbolKind,
		uri: Uri,
		range: Range,
		selection_range: Range,
	) -> Self {
		Self {
			name,
			kind,
			tags: None,
			detail: None,
			uri,
			range,
			selection_range,
			data: None,
		}
	}

	pub fn with_tags(mut self, tags: Vec<SymbolTag>) -> Self {
		self.tags = Some(tags);
		self
	}

	pub fn with_detail(mut self, detail: String) -> Self {
		self.detail = Some(detail);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// The name of this item.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The kind of this item.
	pub fn kind(&self) -> &SymbolKind {
		&self.kind
	}

	/// Tags for this item.
	pub fn tags(&self) -> Option<&Vec<SymbolTag>> {
		self.tags.as_ref()
	}

	/// More detail for this item, e.g. the signature of a function.
	pub fn detail(&self) -> Option<&String> {
		self.detail.as_ref()
	}

	/// The resource identifier of this item.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The range enclosing this symbol not including leading/trailing whitespace but everything else,
	/// e.g. comments and code.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The range that should be selected and revealed when this symbol is being picked, e.g. the
	/// name of a function.
	///
	/// Must be contained by the [`CallHierarchyItem::range`].
	pub fn selection_range(&self) -> &Range {
		&self.selection_range
	}

	/// A data entry field that is preserved between a call hierarchy prepare and incoming calls
	/// or outgoing calls requests.
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl CallHierarchyIncomingCall {
	pub fn new(from: CallHierarchyItem, ranges: Vec<Range>) -> Self {
		Self { from, from_ranges: ranges }
	}

	/// The item that makes the call
	pub fn from(&self) -> &CallHierarchyItem {
		&self.from
	}

	/// The ranges at which the calls appear. This is relative to the caller denoted by [`CallHierarchyIncomingCall::from`]
	pub fn ranges(&self) -> &Vec<Range> {
		&self.from_ranges
	}
}

impl CallHierarchyOutgoingCall {
	pub fn new(to: CallHierarchyItem, ranges: Vec<Range>) -> Self {
		Self { to, to_ranges: ranges }
	}

	/// The item that is called
	pub fn to(&self) -> &CallHierarchyItem {
		&self.to
	}

	/// The range at which this item is called.
	///
	/// This is the range relative to the caller, e.g the item passed to [`CallHierarchyItemProvider::provide_call_hierarchy_outgoing_calls`]
	/// and not [`CallHierarchyOutgoingCall::to`].
	pub fn ranges(&self) -> &Vec<Range> {
		&self.to_ranges
	}
}

impl ShowDocumentResult {
	pub fn new(success: bool) -> Self {
		Self { success }
	}

	/// A boolean indicating if the show was successful.
	pub fn success(&self) -> &bool {
		&self.success
	}
}

impl LinkedEditingRanges {
	pub fn new(ranges: Vec<Range>) -> Self {
		Self { ranges, word_pattern: None }
	}

	pub fn with_word_pattern(mut self, word_pattern: String) -> Self {
		self.word_pattern = Some(word_pattern);
		self
	}

	/// A list of ranges that can be edited together.
	///
	/// The ranges must have identical length and contain identical text content. The ranges cannot overlap.
	pub fn ranges(&self) -> &Vec<Range> {
		&self.ranges
	}

	/// An optional word pattern (regular expression) that describes valid contents for the given ranges.
	///
	/// If no pattern is provided, the client configuration's word pattern will be used.
	pub fn word_pattern(&self) -> Option<&String> {
		self.word_pattern.as_ref()
	}
}

// TODO(aki): Re-factor the Create/Delete/Rename file ops
impl CreateFile {
	pub fn new(kind: String, uri: Uri, resource_operation: ResourceOperation) -> Self {
		Self { kind, uri, options: None, resource_operation }
	}

	pub fn with_options(mut self, options: CreateFileOptions) -> Self {
		self.options = Some(options);
		self
	}

	// `create`
	/// A create document change
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// The resource to create.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	pub fn resource_operation(&self) -> &ResourceOperation {
		&self.resource_operation
	}

	/// Additional options
	pub fn options(&self) -> Option<&CreateFileOptions> {
		self.options.as_ref()
	}
}

impl RenameFile {
	pub fn new(
		kind: String,
		old_uri: Uri,
		new_uri: Uri,
		resource_operation: ResourceOperation,
	) -> Self {
		Self {
			kind,
			old_uri,
			new_uri,
			options: None,
			resource_operation,
		}
	}

	pub fn with_options(mut self, options: RenameFileOptions) -> Self {
		self.options = Some(options);
		self
	}

	// `rename`
	/// A rename document change
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// The old (existing) location.
	pub fn old_uri(&self) -> &Uri {
		&self.old_uri
	}

	/// The new location.
	pub fn new_uri(&self) -> &Uri {
		&self.new_uri
	}

	pub fn resource_operation(&self) -> &ResourceOperation {
		&self.resource_operation
	}

	/// Additional options
	pub fn options(&self) -> Option<&RenameFileOptions> {
		self.options.as_ref()
	}
}

impl DeleteFile {
	pub fn new(kind: String, uri: Uri, resource_operation: ResourceOperation) -> Self {
		Self { kind, uri, options: None, resource_operation }
	}

	pub fn with_options(mut self, options: DeleteFileOptions) -> Self {
		self.options = Some(options);
		self
	}

	// `delete`
	/// A delete document change
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// The resource to delete.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	pub fn resource_operation(&self) -> &ResourceOperation {
		&self.resource_operation
	}

	/// Additional options
	pub fn options(&self) -> Option<&DeleteFileOptions> {
		self.options.as_ref()
	}
}

impl WorkspaceEdit {
	pub fn new() -> Self {
		Self {
			changes: None,
			document_changes: None,
			change_annotations: None,
		}
	}

	pub fn with_changes(
		mut self,
		changes: HashMap<fluent_uri::Uri<String>, Vec<TextEdit>>,
	) -> Self {
		self.changes = Some(changes);
		self
	}

	pub fn with_document_changes(mut self, document_changes: Vec<DocumentChange>) -> Self {
		self.document_changes = Some(document_changes);
		self
	}

	pub fn with_change_annotations(
		mut self,
		change_annotations: HashMap<String, ChangeAnnotation>,
	) -> Self {
		self.change_annotations = Some(change_annotations);
		self
	}

	/// Holds changes to existing resources.
	pub fn changes(&self) -> Option<&HashMap<Uri, Vec<TextEdit>>> {
		self.changes.as_ref()
	}

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
	pub fn document_changes(&self) -> Option<&Vec<DocumentChange>> {
		self.document_changes.as_ref()
	}

	/// A map of change annotations that can be referenced in [`AnnotatedTextEdit`]s or create, rename and
	/// delete file/folder operations.
	///
	/// Whether clients honor this property depends on the client capability `workspace.changeAnnotationSupport`.
	///
	/// since: 3.16.0
	pub fn change_annotations(
		&self,
	) -> Option<&HashMap<ChangeAnnotationIdentifier, ChangeAnnotation>> {
		self.change_annotations.as_ref()
	}
}

impl Moniker {
	pub fn new(scheme: String, identifier: String, unique: UniquenessLevel) -> Self {
		Self { scheme, identifier, unique, kind: None }
	}

	pub fn with_kind(mut self, kind: MonikerKind) -> Self {
		self.kind = Some(kind);
		self
	}

	/// The scheme of the moniker.
	///
	/// For example tsc or .Nets
	pub fn scheme(&self) -> &String {
		&self.scheme
	}
	/// The identifier of the moniker.
	///
	/// The value is opaque in LSIF however schema owners are allowed to define the structure if they want.
	pub fn identifier(&self) -> &String {
		&self.identifier
	}

	/// The scope in which the moniker is unique
	pub fn unique(&self) -> &UniquenessLevel {
		&self.unique
	}

	/// The moniker kind if known.
	pub fn kind(&self) -> Option<&MonikerKind> {
		self.kind.as_ref()
	}
}

impl TypeHierarchyItem {
	pub fn new(
		name: String,
		kind: SymbolKind,
		uri: Uri,
		range: Range,
		selection_range: Range,
	) -> Self {
		Self {
			name,
			kind,
			tags: None,
			detail: None,
			uri,
			range,
			selection_range,
			data: None,
		}
	}

	pub fn with_tags(mut self, tags: Vec<SymbolTag>) -> Self {
		self.tags = Some(tags);
		self
	}

	pub fn with_detail(mut self, detail: String) -> Self {
		self.detail = Some(detail);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// The name of this item.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The kind of this item.
	pub fn kind(&self) -> &SymbolKind {
		&self.kind
	}

	/// Tags for this item.
	pub fn tags(&self) -> Option<&Vec<SymbolTag>> {
		self.tags.as_ref()
	}

	/// More detail for this item, e.g. the signature of a function.
	pub fn detail(&self) -> Option<&String> {
		self.detail.as_ref()
	}

	/// The resource identifier of this item.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The range enclosing this symbol not including leading/trailing whitespace but everything else,
	///
	/// e.g. comments and code.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The range that should be selected and revealed when this symbol is being picked,
	///
	/// e.g. the name of a function. Must be contained by the [`TypeHierarchyItem::range`].
	pub fn selection_range(&self) -> &Range {
		&self.selection_range
	}

	/// A data entry field that is preserved between a type hierarchy prepare and supertypes or subtypes
	/// requests.
	///
	/// It could also be used to identify the type hierarchy in the server, helping improve the performance
	/// on resolving supertypes and subtypes.
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl InlayHint {
	pub fn new(position: Position, label: InlayHintLabel) -> Self {
		Self {
			position,
			label,
			kind: None,
			text_edits: None,
			tooltip: None,
			padding_left: None,
			padding_right: None,
			data: None,
		}
	}

	pub fn with_kind(mut self, kind: InlayHintKind) -> Self {
		self.kind = Some(kind);
		self
	}

	pub fn with_text_edits(mut self, text_edits: Vec<TextEdit>) -> Self {
		self.text_edits = Some(text_edits);
		self
	}

	pub fn with_tooltip(mut self, tooltip: MarkupOrString) -> Self {
		self.tooltip = Some(tooltip);
		self
	}

	pub fn with_padding_left(mut self, padding_left: bool) -> Self {
		self.padding_left = Some(padding_left);
		self
	}

	pub fn with_padding_right(mut self, padding_right: bool) -> Self {
		self.padding_right = Some(padding_right);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// The position of this hint.
	///
	/// If multiple hints have the same position, they will be shown in the order
	/// they appear in the response.
	pub fn position(&self) -> &Position {
		&self.position
	}

	/// The label of this hint.
	///
	/// A human readable string or an array of [`InlayHintLabelPart`].
	///
	/// *Note* that neither the string nor the label part can be empty.
	pub fn label(&self) -> &InlayHintLabel {
		&self.label
	}

	/// The kind of this hint.
	///
	/// Can be omitted in which case the client should fall back to a reasonable default.
	pub fn kind(&self) -> Option<&InlayHintKind> {
		self.kind.as_ref()
	}

	/// Optional text edits that are performed when accepting this inlay hint.
	///
	/// *Note* that edits are expected to change the document so that the inlay hint (or its nearest variant)
	/// is now part of the document and the inlay hint itself is now obsolete.
	pub fn text_edits(&self) -> Option<&Vec<TextEdit>> {
		self.text_edits.as_ref()
	}

	/// The tooltip text when you hover over this item.
	pub fn tooltip(&self) -> Option<&MarkupOrString> {
		self.tooltip.as_ref()
	}

	/// Render padding before the hint.
	///
	/// Note: Padding should use the editor's background color, not the background color of the hint itself.
	/// That means padding can be used to visually align/separate an inlay hint.
	pub fn padding_left(&self) -> Option<&bool> {
		self.padding_left.as_ref()
	}

	/// Render padding after the hint.
	///
	/// Note: Padding should use the editor's background color, not the background color of the hint itself.
	/// That means padding can be used\nto visually align/separate an inlay hint.
	pub fn padding_right(&self) -> Option<&bool> {
		self.padding_right.as_ref()
	}

	/// A data entry field that is preserved on an inlay hint between a `textDocument/inlayHint` and a `inlayHint/resolve` request.
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl DocumentDiagnosticReportPartialResult {
	pub fn new(related_documents: HashMap<Uri, DiagnosticReport>) -> Self {
		Self { related_documents }
	}

	pub fn related_documents(&self) -> &HashMap<Uri, DiagnosticReport> {
		&self.related_documents
	}
}

impl DiagnosticServerCancellationData {
	pub fn new(retrigger_request: bool) -> Self {
		Self { retrigger_request }
	}

	pub fn retrigger_request(&self) -> &bool {
		&self.retrigger_request
	}
}

impl WorkspaceDiagnosticReport {
	pub fn new(items: Vec<WorkspaceDocumentDiagnosticReport>) -> Self {
		Self { items }
	}

	pub fn items(&self) -> &Vec<WorkspaceDocumentDiagnosticReport> {
		&self.items
	}
}

impl WorkspaceDiagnosticReportPartialResult {
	pub fn new(items: Vec<WorkspaceDocumentDiagnosticReport>) -> Self {
		Self { items }
	}

	pub fn items(&self) -> &Vec<WorkspaceDocumentDiagnosticReport> {
		&self.items
	}
}

impl InlineCompletionList {
	pub fn new(items: Vec<InlineCompletionItem>) -> Self {
		Self { items }
	}

	/// The inline completion items
	pub fn items(&self) -> &Vec<InlineCompletionItem> {
		&self.items
	}
}

impl InlineCompletionItem {
	pub fn new(insert_text: InsertText) -> Self {
		Self {
			insert_text,
			filter_text: None,
			range: None,
			command: None,
		}
	}

	pub fn with_filter_text(mut self, filter_text: String) -> Self {
		self.filter_text = Some(filter_text);
		self
	}

	pub fn with_range(mut self, range: Range) -> Self {
		self.range = Some(range);
		self
	}

	pub fn with_command(mut self, command: Command) -> Self {
		self.command = Some(command);
		self
	}

	/// The text to replace the range with. Must be set.
	pub fn insert_text(&self) -> &InsertText {
		&self.insert_text
	}

	/// A text that is used to decide if this inline completion should be shown.
	///
	/// When `falsy` the [`InlineCompletionItem::insert_text`] is used.
	pub fn filter_text(&self) -> Option<&String> {
		self.filter_text.as_ref()
	}

	/// The range to replace. Must begin and end on the same line.
	pub fn range(&self) -> Option<&Range> {
		self.range.as_ref()
	}

	/// An optional [`Command`] that is executed *after* inserting this completion.
	pub fn command(&self) -> Option<&Command> {
		self.command.as_ref()
	}
}

impl InitializeResult {
	pub fn new(capabilities: ServerCapabilities) -> Self {
		Self { capabilities, server_info: None }
	}

	pub fn with_server_info(mut self, server_info: ServerInfo) -> Self {
		self.server_info = Some(server_info);
		self
	}

	/// The capabilities the language server provides.
	pub fn capabilities(&self) -> &ServerCapabilities {
		&self.capabilities
	}

	/// Information about the server.
	///
	/// since: 3.15.0
	pub fn server_info(&self) -> Option<&ServerInfo> {
		self.server_info.as_ref()
	}
}

impl ServerInfo {
	pub fn new(name: String) -> Self {
		Self { name, version: None }
	}

	pub fn with_version(mut self, version: String) -> Self {
		self.version = Some(version);
		self
	}

	/// The name of the server as defined by the server.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The server's version as defined by the server.
	pub fn version(&self) -> Option<&String> {
		self.version.as_ref()
	}
}

impl InitializeError {
	pub fn new(retry: bool) -> Self {
		Self { retry }
	}

	/// Indicates whether the client execute the following retry logic:
	/// (1) show the message provided by the ResponseError to the user
	/// (2) user selects retry or cancel
	/// (3) if user selected retry the initialize method is sent again.
	pub fn retry(&self) -> &bool {
		&self.retry
	}
}

impl MessageActionItem {
	pub fn new(title: String) -> Self {
		Self { title }
	}

	/// A short title like 'Retry', 'Open Log' etc.
	pub fn title(&self) -> &String {
		&self.title
	}
}

impl TextEdit {
	pub fn new(range: Range, new_text: String) -> Self {
		Self { range, new_text }
	}

	/// The range of the text document to be manipulated.
	///
	/// To insert text into a document create a range where start === end.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The string to be inserted.
	///
	/// For delete operations use an empty string
	pub fn new_text(&self) -> &String {
		&self.new_text
	}
}

impl CompletionItem {
	pub fn new(label: String) -> Self {
		Self {
			label,
			label_details: None,
			kind: None,
			tags: None,
			detail: None,
			documentation: None,
			deprecated: None,
			preselect: None,
			sort_text: None,
			filter_text: None,
			insert_text: None,
			insert_text_format: None,
			insert_text_mode: None,
			text_edit: None,
			text_edit_text: None,
			additional_text_edits: None,
			commit_characters: None,
			command: None,
			data: None,
		}
	}

	pub fn with_label_details(mut self, label_details: CompletionItemLabelDetails) -> Self {
		self.label_details = Some(label_details);
		self
	}

	pub fn with_kind(mut self, kind: CompletionItemKind) -> Self {
		self.kind = Some(kind);
		self
	}

	pub fn with_tags(mut self, tags: Vec<CompletionItemTag>) -> Self {
		self.tags = Some(tags);
		self
	}

	pub fn with_detail(mut self, detail: String) -> Self {
		self.detail = Some(detail);
		self
	}

	pub fn with_documentation(mut self, documentation: MarkupOrString) -> Self {
		self.documentation = Some(documentation);
		self
	}

	pub fn with_deprecated(mut self, deprecated: bool) -> Self {
		self.deprecated = Some(deprecated);
		self
	}

	pub fn with_preselect(mut self, preselect: bool) -> Self {
		self.preselect = Some(preselect);
		self
	}

	pub fn with_sort_text(mut self, sort_text: String) -> Self {
		self.sort_text = Some(sort_text);
		self
	}

	pub fn with_filter_text(mut self, filter_text: String) -> Self {
		self.filter_text = Some(filter_text);
		self
	}

	pub fn with_insert_text(mut self, insert_text: String) -> Self {
		self.insert_text = Some(insert_text);
		self
	}

	pub fn with_insert_text_format(mut self, insert_text_format: InsertTextFormat) -> Self {
		self.insert_text_format = Some(insert_text_format);
		self
	}

	pub fn with_insert_text_mode(mut self, insert_text_mode: InsertTextMode) -> Self {
		self.insert_text_mode = Some(insert_text_mode);
		self
	}

	pub fn with_text_edit(mut self, text_edit: TextEdit) -> Self {
		self.text_edit = Some(text_edit);
		self
	}

	pub fn with_text_edit_text(mut self, text_edit_text: String) -> Self {
		self.text_edit_text = Some(text_edit_text);
		self
	}

	pub fn with_additional_text_edits(mut self, additional_text_edits: Vec<TextEdit>) -> Self {
		self.additional_text_edits = Some(additional_text_edits);
		self
	}

	pub fn with_commit_characters(mut self, commit_characters: Vec<String>) -> Self {
		self.commit_characters = Some(commit_characters);
		self
	}

	pub fn with_command(mut self, command: Command) -> Self {
		self.command = Some(command);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// The label of this completion item.
	///
	/// The label property is also by default the text that is inserted when selecting this completion.
	///
	/// If label details are provided the label itself should be an unqualified name of the completion item.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// Additional details for the label
	///
	/// since: 3.17.0
	pub fn label_details(&self) -> Option<&CompletionItemLabelDetails> {
		self.label_details.as_ref()
	}

	/// The kind of this completion item. Based of the kind an icon is chosen by the editor.
	pub fn kind(&self) -> Option<&CompletionItemKind> {
		self.kind.as_ref()
	}

	/// Tags for this completion item.
	///
	/// since: 3.15.0
	pub fn tags(&self) -> Option<&Vec<CompletionItemTag>> {
		self.tags.as_ref()
	}

	/// A human-readable string with additional information about this item, like type or symbol information.
	pub fn detail(&self) -> Option<&String> {
		self.detail.as_ref()
	}

	/// A human-readable string that represents a doc-comment.
	pub fn documentation(&self) -> Option<&MarkupOrString> {
		self.documentation.as_ref()
	}

	/// Indicates if this item is deprecated.
	///
	/// deprecated: Use [`CompletionItem::tags`] instead.
	pub fn deprecated(&self) -> Option<&bool> {
		self.deprecated.as_ref()
	}

	/// Select this item when showing.
	///
	/// *Note* that only one completion item can be selected and that the tool/client decides
	/// which item that is. The rule is that the *first* item of those that match best is selected.
	pub fn preselect(&self) -> Option<&bool> {
		self.preselect.as_ref()
	}

	/// A string that should be used when comparing this item with other items.
	///
	/// When `falsy` the [`CompletionItem::label`] is used.
	pub fn sort_text(&self) -> Option<&String> {
		self.sort_text.as_ref()
	}

	/// A string that should be used when filtering a set of completion items.
	///
	/// When `falsy` the [`CompletionItem::label`] is used.
	pub fn filter_text(&self) -> Option<&String> {
		self.filter_text.as_ref()
	}

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
	pub fn insert_text(&self) -> Option<&String> {
		self.insert_text.as_ref()
	}

	/// The format of the insert text. The format applies to both the
	/// `insert_text` property and the `new_text` property of a provided `text_edit`.
	///
	/// If omitted defaults to [`InsertTextFormat::PlainText`].
	///
	/// Please note that the `insert_text_format`` doesn't apply to `additional_text_edits`.
	pub fn insert_text_format(&self) -> Option<&InsertTextFormat> {
		self.insert_text_format.as_ref()
	}

	/// How whitespace and indentation is handled during completion item insertion.
	///
	/// If not provided the clients default value depends on the `textDocument.completion.insertTextMode`
	/// client capability.
	///
	/// since: 3.16.0
	pub fn insert_text_mode(&self) -> Option<&InsertTextMode> {
		self.insert_text_mode.as_ref()
	}

	/// A  [`TextEdit`] which is applied to a document when selecting this completion.
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
	pub fn text_edit(&self) -> Option<&TextEdit> {
		self.text_edit.as_ref() // XXX(aki), or InsertReplaceEdit
	}

	/// The edit text used if the completion item is part of a CompletionList and CompletionList
	/// defines an item default for the text edit range.
	///
	/// Clients will only honor this property if they opt into completion list item defaults using
	/// the capability `completionList.itemDefaults`.
	///
	/// If not provided and a list's default range is provided the label property is used as a text.
	///
	/// since: 3.17.0
	pub fn text_edit_text(&self) -> Option<&String> {
		self.text_edit_text.as_ref()
	}

	/// An optional array of additional [`TextEdit`] that are applied when selecting this completion.
	/// Edits must not overlap (including the same insert position) with the main [`CompletionItem::text_edit`]
	/// nor with themselves.
	///
	/// Additional text edits should be used to change text unrelated to the current cursor position
	/// (for example adding an import statement at the top of the file if the completion item will
	/// insert an unqualified type).
	pub fn additional_text_edits(&self) -> Option<&Vec<TextEdit>> {
		self.additional_text_edits.as_ref()
	}

	/// An optional set of characters that when pressed while this completion is active will accept it first and
	/// then type that character.
	///
	/// *Note* that all commit characters should have `length=1` and that superfluous characters will be ignored
	pub fn commit_characters(&self) -> Option<&Vec<String>> {
		self.commit_characters.as_ref()
	}

	/// An optional [`Command`] that is executed *after* inserting this completion.
	///
	/// *Note* that additional modifications to the current document should be described with the [`CompletionItem::additional_text_edits`]-property
	pub fn command(&self) -> Option<&Command> {
		self.command.as_ref()
	}

	/// A data entry field that is preserved on a completion item between a [`CompletionRequest`] and
	/// a [`CompletionResolveRequest`].
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl ItemDefaults {
	pub fn new() -> Self {
		Self {
			commit_characters: None,
			edit_range: None,
			insert_text_format: None,
			insert_text_mode: None,
			data: None,
		}
	}

	pub fn with_commit_characters(mut self, commit_characters: Vec<String>) -> Self {
		self.commit_characters = Some(commit_characters);
		self
	}

	pub fn with_edit_range(mut self, edit_range: Range) -> Self {
		self.edit_range = Some(edit_range);
		self
	}

	pub fn with_insert_text_format(mut self, insert_text_format: InsertTextFormat) -> Self {
		self.insert_text_format = Some(insert_text_format);
		self
	}

	pub fn with_insert_text_mode(mut self, insert_text_mode: InsertTextMode) -> Self {
		self.insert_text_mode = Some(insert_text_mode);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// A default commit character set.
	///
	/// since: 3.17.0
	pub fn commit_characters(&self) -> Option<&Vec<String>> {
		self.commit_characters.as_ref()
	}

	/// A default edit range.
	///
	/// since: 3.17.0
	pub fn edit_range(&self) -> Option<&Range> {
		self.edit_range.as_ref()
	}

	/// A default insert text format.
	///
	/// since: 3.17.0
	pub fn insert_text_format(&self) -> Option<&InsertTextFormat> {
		self.insert_text_format.as_ref()
	}

	/// A default insert text mode.
	///
	/// since: 3.17.0
	pub fn insert_text_mode(&self) -> Option<&InsertTextMode> {
		self.insert_text_mode.as_ref()
	}

	/// A default data value.
	///
	/// since: 3.17.0
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl CompletionList {
	pub fn new(is_incomplete: bool, items: Vec<CompletionItem>) -> Self {
		Self { is_incomplete, item_defaults: None, items }
	}

	pub fn with_item_defaults(mut self, item_defaults: ItemDefaults) -> Self {
		self.item_defaults = Some(item_defaults);
		self
	}

	/// This list it not complete.
	///
	/// Further typing results in recomputing this list.
	///
	/// Recomputed lists have all their items replaced (not appended) in the incomplete completion sessions.
	pub fn is_incomplete(&self) -> &bool {
		&self.is_incomplete
	}

	/// The completion items
	pub fn items(&self) -> &Vec<CompletionItem> {
		&self.items
	}

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
	pub fn item_defaults(&self) -> Option<&ItemDefaults> {
		self.item_defaults.as_ref()
	}
}

impl Hover {
	pub fn new(contents: HoverContents) -> Self {
		Self { contents, range: None }
	}

	pub fn with_range(mut self, range: Range) -> Self {
		self.range = Some(range);
		self
	}

	/// The hover's content
	pub fn contents(&self) -> &HoverContents {
		&self.contents
	}

	/// An optional range inside the text document that is used to visualize the hover,
	///
	/// e.g. by changing the background color.
	pub fn range(&self) -> Option<&Range> {
		self.range.as_ref()
	}
}

impl SignatureHelp {
	pub fn new(signatures: Vec<SignatureInformation>) -> Self {
		Self {
			signatures,
			active_signature: None,
			active_parameter: None,
		}
	}

	pub fn with_active_signature(mut self, active_signature: u32) -> Self {
		self.active_signature = Some(active_signature);
		self
	}

	pub fn with_active_parameter(mut self, active_parameter: u32) -> Self {
		self.active_parameter = Some(active_parameter);
		self
	}

	/// One or more signatures
	pub fn signatures(&self) -> &Vec<SignatureInformation> {
		&self.signatures
	}

	/// The active signature.
	///
	/// If omitted or the value lies outside the range of [`SignatureHelp::signatures`] the value defaults
	/// to zero or is ignored if the [`SignatureHelp`] has no signatures.
	///
	/// Whenever possible implementors should make an active decision about the active signature and
	/// shouldn't rely on a default value.
	///
	/// In future version of the protocol this property might become mandatory to better express this.
	pub fn active_signature(&self) -> Option<&u32> {
		self.active_signature.as_ref()
	}

	/// The active parameter of the active signature.
	///
	/// If omitted or the value lies outside the range of `signatures[activeSignature].parameters`
	/// defaults to 0 if the active signature has parameters.
	///
	/// If the active signature has no parameters it is ignored. In future version of the protocol this
	/// property might become mandatory to better express the active parameter if the active signature
	/// does have any.
	pub fn active_parameter(&self) -> Option<&u32> {
		self.active_parameter.as_ref()
	}
}

impl DocumentHighlight {
	pub fn new(range: Range) -> Self {
		Self { range, kind: None }
	}

	pub fn with_kind(mut self, kind: DocumentHighlightKind) -> Self {
		self.kind = Some(kind);
		self
	}

	/// The range this highlight applies to
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The highlight kind, default is [`DocumentHighlightKind::Text`]
	pub fn kind(&self) -> Option<&DocumentHighlightKind> {
		self.kind.as_ref()
	}
}

impl DocumentSymbol {
	pub fn new(name: String, kind: SymbolKind, range: Range, selection_range: Range) -> Self {
		Self {
			name,
			detail: None,
			kind,
			tags: None,
			deprecated: None,
			range,
			selection_range,
			children: None,
		}
	}

	pub fn with_detail(mut self, detail: String) -> Self {
		self.detail = Some(detail);
		self
	}

	pub fn with_tags(mut self, tags: Vec<SymbolTag>) -> Self {
		self.tags = Some(tags);
		self
	}

	pub fn with_deprecated(mut self, deprecated: bool) -> Self {
		self.deprecated = Some(deprecated);
		self
	}

	pub fn with_children(mut self, children: Vec<DocumentSymbol>) -> Self {
		self.children = Some(children);
		self
	}

	/// The name of this symbol.
	///
	/// Will be displayed in the user interface and therefore must not be an empty string or a string
	/// only consisting of white spaces.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// More detail for this symbol, e.g the signature of a function.
	pub fn detail(&self) -> Option<&String> {
		self.detail.as_ref()
	}

	/// The kind of this symbol.
	pub fn kind(&self) -> &SymbolKind {
		&self.kind
	}

	/// Tags for this document symbol.
	///
	/// since: 3.16.0
	pub fn tags(&self) -> Option<&Vec<SymbolTag>> {
		self.tags.as_ref()
	}

	/// Indicates if this symbol is deprecated.
	///
	/// deprecated: Use tags instead
	pub fn deprecated(&self) -> Option<&bool> {
		self.deprecated.as_ref()
	}

	/// The range enclosing this symbol not including leading/trailing whitespace but everything else
	/// like comments.
	///
	/// This information is typically used to determine if the clients cursor is inside the symbol
	/// to reveal in the symbol in the UI.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The range that should be selected and revealed when this symbol is being picked, e.g the name
	/// of a function.
	///
	/// Must be contained by the `range`.
	pub fn selection_range(&self) -> &Range {
		&self.selection_range
	}

	/// Children of this symbol, e.g. properties of a class.
	pub fn children(&self) -> Option<&Vec<DocumentSymbol>> {
		self.children.as_ref()
	}
}

impl Command {
	pub fn new(title: String, command: String) -> Self {
		Self { title, command, arguments: None }
	}

	pub fn with_arguments(mut self, arguments: Vec<serde_json::Value>) -> Self {
		self.arguments = Some(arguments);
		self
	}

	/// Title of the command, like `save`.
	pub fn title(&self) -> &String {
		&self.title
	}

	/// The identifier of the actual command handler.
	pub fn command(&self) -> &String {
		&self.command
	}

	/// Arguments that the command handler should be invoked with.
	pub fn arguments(&self) -> Option<&Vec<serde_json::Value>> {
		self.arguments.as_ref()
	}
}

impl DisabledReason {
	pub fn new(reason: String) -> Self {
		Self { reason }
	}

	/// Human readable description of why the code action is currently disabled.
	///
	/// This is displayed in the code actions UI.
	pub fn reason(&self) -> &String {
		&self.reason
	}
}

impl CodeAction {
	pub fn new(title: String) -> Self {
		Self {
			title,
			kind: None,
			diagnostics: None,
			is_preferred: None,
			disabled: None,
			edit: None,
			command: None,
			data: None,
		}
	}

	pub fn with_kind(mut self, kind: CodeActionKind) -> Self {
		self.kind = Some(kind);
		self
	}

	pub fn with_diagnostics(mut self, diagnostics: Vec<Diagnostic>) -> Self {
		self.diagnostics = Some(diagnostics);
		self
	}

	pub fn with_is_preferred(mut self, is_preferred: bool) -> Self {
		self.is_preferred = Some(is_preferred);
		self
	}

	pub fn with_disabled(mut self, disabled: DisabledReason) -> Self {
		self.disabled = Some(disabled);
		self
	}

	pub fn with_edit(mut self, edit: WorkspaceEdit) -> Self {
		self.edit = Some(edit);
		self
	}

	pub fn with_command(mut self, command: Command) -> Self {
		self.command = Some(command);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// A short, human-readable, title for this code action.
	pub fn title(&self) -> &String {
		&self.title
	}

	/// The kind of the code action.
	///
	/// Used to filter code actions.
	pub fn kind(&self) -> Option<&CodeActionKind> {
		self.kind.as_ref()
	}

	/// The diagnostics that this code action resolves.
	pub fn diagnostics(&self) -> Option<&Vec<Diagnostic>> {
		self.diagnostics.as_ref()
	}

	/// Marks this as a preferred action. Preferred actions are used by the `auto fix` command and can
	/// be targeted by keybindings.
	///
	/// A quick fix should be marked preferred if it properly addresses the underlying error.
	///
	/// A refactoring should be marked preferred if it is the most reasonable choice of actions to take.
	///
	/// since: 3.15.0
	pub fn is_preferred(&self) -> Option<&bool> {
		self.is_preferred.as_ref()
	}

	/// Marks that the code action cannot currently be applied.
	///
	/// Clients should follow the following guidelines regarding disabled code actions:
	/// - Disabled code actions are not shown in automatic [lightbulbs](https://code.visualstudio.com/docs/editor/editingevolved#_code-action) code action menus.
	/// - Disabled actions are shown as faded out in the code action menu when the user requests a more specific type of code action, such as refactorings.
	/// - If the user has a [keybinding](https://code.visualstudio.com/docs/editor/refactoring#_keybindings-for-code-actions) that auto applies a code action
	///   and only disabled code actions are returned, the client should show the user an error message with `reason` in the editor.
	///
	/// since: 3.16.0
	pub fn disabled(&self) -> Option<&DisabledReason> {
		self.disabled.as_ref()
	}

	/// The workspace edit this code action performs.
	pub fn edit(&self) -> Option<&WorkspaceEdit> {
		self.edit.as_ref()
	}

	/// A command this code action executes.
	///
	/// If a code action provides an edit and a command, first the edit is executed and then the command.
	pub fn command(&self) -> Option<&Command> {
		self.command.as_ref()
	}

	/// A data entry field that is preserved on a code action between\na `textDocument/codeAction` and a `codeAction/resolve` request.
	///
	/// @since 3.16.0
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl CodeLens {
	pub fn new(range: Range) -> Self {
		Self { range, command: None, data: None }
	}

	pub fn with_command(mut self, command: Command) -> Self {
		self.command = Some(command);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// The range in which this code lens is valid. Should only span a single line.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The command this code lens represents
	pub fn command(&self) -> Option<&Command> {
		self.command.as_ref()
	}

	/// A data entry field that is preserved on a code lens item between a [`CodeLensRequest`] and a [`CodeLensResolveRequest`]
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl DocumentLink {
	pub fn new(range: Range) -> Self {
		Self { range, target: None, tooltip: None, data: None }
	}

	pub fn with_target(mut self, target: fluent_uri::Uri<String>) -> Self {
		self.target = Some(target);
		self
	}

	pub fn with_tooltip(mut self, tooltip: String) -> Self {
		self.tooltip = Some(tooltip);
		self
	}

	pub fn with_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(data);
		self
	}

	/// The range this link applies to.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The uri this link points to.
	///
	/// If missing a resolve request is sent later.
	pub fn target(&self) -> Option<&Uri> {
		self.target.as_ref()
	}

	/// The tooltip text when you hover over this link.
	///
	/// If a tooltip is provided, is will be displayed in a string that includes instructions on how to
	/// trigger the link, such as `{0} (ctrl + click)`. The specific instructions vary depending on OS,
	/// user settings, and localization.
	///
	/// since: 3.15.0
	pub fn tooltip(&self) -> Option<&String> {
		self.tooltip.as_ref()
	}

	/// A data entry field that is preserved on a document link between a [`DocumentLinkRequest`] and
	/// a [`DocumentLinkResolveRequest`].
	pub fn data(&self) -> Option<&serde_json::Value> {
		self.data.as_ref()
	}
}

impl ApplyWorkspaceEditResult {
	pub fn new(applied: bool) -> Self {
		Self { applied, failure_reason: None, failed_change: None }
	}

	pub fn with_failure_reason(mut self, failure_reason: String) -> Self {
		self.failure_reason = Some(failure_reason);
		self
	}

	pub fn with_failed_change(mut self, failed_change: u32) -> Self {
		self.failed_change = Some(failed_change);
		self
	}

	/// Indicates whether the edit was applied or not.
	pub fn applied(&self) -> &bool {
		&self.applied
	}

	/// An optional textual description for why the edit was not applied.
	///
	/// This may be used by the server for diagnostic logging or to provide a suitable error for a
	/// request that triggered the edit.
	pub fn failure_reason(&self) -> Option<&String> {
		self.failure_reason.as_ref()
	}

	/// Depending on the client's failure handling strategy `failedChange` might
	/// contain the index of the change that failed.
	///
	/// This property is only available if the client signals a `failureHandlingStrategy` in its
	/// client capabilities.
	pub fn failed_change(&self) -> Option<&u32> {
		self.failed_change.as_ref()
	}
}

impl WorkDoneProgressBegin {
	pub fn new(title: String) -> Self {
		Self {
			kind: "begin".to_string(),
			title,
			cancellable: None,
			message: None,
			percentage: None,
		}
	}

	pub fn with_cancellable(mut self, cancellable: bool) -> Self {
		self.cancellable = Some(cancellable);
		self
	}

	pub fn with_message(mut self, message: String) -> Self {
		self.message = Some(message);
		self
	}

	pub fn with_percentage(mut self, percentage: u32) -> Self {
		self.percentage = Some(percentage);
		self
	}

	/// `begin`
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// Mandatory title of the progress operation.
	///
	/// Used to briefly inform about the kind of operation being performed.
	///
	/// Examples: "Indexing" or "Linking dependencies".
	pub fn title(&self) -> &String {
		&self.title
	}

	/// Controls if a cancel button should show to allow the user to cancel the
	/// long running operation.
	///
	/// Clients that don't support cancellation are allowed to ignore the setting.
	pub fn cancellable(&self) -> Option<&bool> {
		self.cancellable.as_ref()
	}

	/// Optional, more detailed associated progress message.
	///
	/// Contains complementary information to the `title`.
	///
	/// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
	///
	/// If unset, the previous progress message (if any) is still valid.
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	/// Optional progress percentage to display (value 100 is considered 100%).
	///
	/// If not provided infinite progress is assumed and clients are allowed
	/// to ignore the `percentage` value in subsequent report notifications.
	///
	/// The value should be steadily rising. Clients are free to ignore values
	/// that are not following this rule. The value range is [0, 100].
	pub fn percentage(&self) -> Option<&u32> {
		self.percentage.as_ref()
	}
}

impl WorkDoneProgressReport {
	pub fn new() -> Self {
		Self {
			kind: "report".to_string(),
			cancellable: None,
			message: None,
			percentage: None,
		}
	}

	pub fn with_cancellable(mut self, cancellable: bool) -> Self {
		self.cancellable = Some(cancellable);
		self
	}

	pub fn with_message(mut self, message: String) -> Self {
		self.message = Some(message);
		self
	}

	pub fn with_percentage(mut self, percentage: u32) -> Self {
		self.percentage = Some(percentage);
		self
	}

	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// Controls enablement state of a cancel button.
	///
	/// Clients that don't support cancellation or don't support controlling the button's
	/// enablement state are allowed to ignore the property.
	pub fn cancellable(&self) -> Option<&bool> {
		self.cancellable.as_ref()
	}

	/// Optional, more detailed associated progress message.
	///
	/// Contains complementary information to the `title`.
	///
	/// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
	///
	/// If unset, the previous progress message (if any) is still valid.
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	/// Optional progress percentage to display (value 100 is considered 100%).
	///
	/// If not provided infinite progress is assumed and clients are allowed
	/// to ignore the `percentage` value in subsequent report notifications.
	///
	/// The value should be steadily rising. Clients are free to ignore values
	/// that are not following this rule. The value range is [0, 100].
	pub fn percentage(&self) -> Option<&u32> {
		self.percentage.as_ref()
	}
}

impl WorkDoneProgressEnd {
	pub fn new() -> Self {
		Self { kind: "end".to_string(), message: None }
	}

	pub fn with_message(mut self, message: String) -> Self {
		self.message = Some(message);
		self
	}

	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// Optional, a final message indicating to for example indicate the outcome of the operation.
	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}
}

impl WorkspaceFoldersChangeEvent {
	pub fn new(added: Vec<WorkspaceFolder>, removed: Vec<WorkspaceFolder>) -> Self {
		Self { added, removed }
	}

	/// The array of added workspace folders
	pub fn added(&self) -> &Vec<WorkspaceFolder> {
		&self.added
	}

	/// The array of the removed workspace folders
	pub fn removed(&self) -> &Vec<WorkspaceFolder> {
		&self.removed
	}
}

impl ConfigurationItem {
	pub fn new() -> Self {
		Self { scope_uri: None, section: None }
	}

	pub fn with_scope_uri(mut self, scope_uri: Uri) -> Self {
		self.scope_uri = Some(scope_uri);
		self
	}

	pub fn with_section(mut self, section: String) -> Self {
		self.section = Some(section);
		self
	}

	/// The scope to get the configuration section for
	pub fn scope_uri(&self) -> Option<&Uri> {
		self.scope_uri.as_ref()
	}

	/// The configuration section asked for
	pub fn section(&self) -> Option<&String> {
		self.section.as_ref()
	}
}

impl TextDocumentIdentifier {
	pub fn new(uri: Uri) -> Self {
		Self { uri }
	}

	/// The text document's uri
	pub fn uri(&self) -> &Uri {
		&self.uri
	}
}

impl TextDocumentFilter {
	pub fn new() -> Self {
		Self { language: None, scheme: None, pattern: None }
	}

	pub fn with_language(mut self, language: String) -> Self {
		self.language = Some(language);
		self
	}

	pub fn with_scheme(mut self, scheme: String) -> Self {
		self.scheme = Some(scheme);
		self
	}

	pub fn with_pattern(mut self, pattern: String) -> Self {
		self.pattern = Some(pattern);
		self
	}

	/// A language id, like `typescript`.
	pub fn language(&self) -> Option<&String> {
		self.language.as_ref()
	}

	/// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
	pub fn scheme(&self) -> Option<&String> {
		self.scheme.as_ref()
	}

	/// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
	pub fn pattern(&self) -> Option<&String> {
		self.pattern.as_ref()
	}
}

impl NotebookDocumentFilter {
	pub fn new() -> Self {
		Self { notebook_type: None, scheme: None, pattern: None }
	}

	pub fn with_notebook_type(mut self, notebook_type: String) -> Self {
		self.notebook_type = Some(notebook_type);
		self
	}

	pub fn with_scheme(mut self, scheme: String) -> Self {
		self.scheme = Some(scheme);
		self
	}

	pub fn with_pattern(mut self, pattern: String) -> Self {
		self.pattern = Some(pattern);
		self
	}

	/// The type of the enclosing notebook.
	pub fn notebook_type(&self) -> Option<&String> {
		self.notebook_type.as_ref()
	}

	/// A Uri scheme, like `file` or `untitled`.
	pub fn scheme(&self) -> Option<&String> {
		self.scheme.as_ref()
	}

	/// A glob pattern.
	pub fn pattern(&self) -> Option<&String> {
		self.pattern.as_ref()
	}
}

impl FileCreate {
	pub fn new(uri: String) -> Self {
		Self { uri }
	}

	/// A `file://` URI for the location of the file/folder being created.
	pub fn uri(&self) -> &String {
		&self.uri
	}
}

impl AnnotatedTextEdit {
	pub fn new(annotation_id: ChangeAnnotationIdentifier, text_edit: TextEdit) -> Self {
		Self { annotation_id, text_edit }
	}

	/// The actual identifier of the change annotation
	pub fn annotation_id(&self) -> &ChangeAnnotationIdentifier {
		&self.annotation_id
	}

	pub fn text_edit(&self) -> &TextEdit {
		&self.text_edit
	}
}

impl TextDocumentEdit {
	pub fn new(
		text_document: OptionalVersionedTextDocumentIdentifier,
		edits: TextEditType,
	) -> Self {
		Self { text_document, edits }
	}

	/// The text document to change.
	pub fn text_document(&self) -> &OptionalVersionedTextDocumentIdentifier {
		&self.text_document
	}

	/// The edits to be applied.
	///
	/// since: 3.16.0 - support for AnnotatedTextEdit. This is guarded using a\nclient capability.
	pub fn edits(&self) -> &TextEditType {
		&self.edits
	}
}

impl OptionalVersionedTextDocumentIdentifier {
	pub fn new(text_document_identifier: TextDocumentIdentifier) -> Self {
		Self { version: None, text_document_identifier }
	}

	pub fn with_version(mut self, version: i32) -> Self {
		self.version = Some(version);
		self
	}

	pub fn text_document_identifier(&self) -> &TextDocumentIdentifier {
		&self.text_document_identifier
	}

	/// The version number of this document.
	///
	/// If a versioned text document identifier is sent from the server
	/// to the client and the file is not open in the editor (i.e. the server
	/// has not received an open notification before) the server can send `null`
	/// to indicate that the version is unknown and the content on disk is the
	/// truth (as specified with document content ownership).
	pub fn version(&self) -> Option<&i32> {
		self.version.as_ref()
	}
}

impl ChangeAnnotation {
	pub fn new(label: String) -> Self {
		Self { label, needs_confirmation: None, description: None }
	}

	pub fn with_needs_confirmation(mut self, needs_confirmation: bool) -> Self {
		self.needs_confirmation = Some(needs_confirmation);
		self
	}

	pub fn with_description(mut self, description: String) -> Self {
		self.description = Some(description);
		self
	}

	/// A human-readable string describing the actual change.
	///
	/// The string is rendered prominent in the user interface.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// A flag which indicates that user confirmation is needed before applying the change.
	pub fn needs_confirmation(&self) -> Option<&bool> {
		self.needs_confirmation.as_ref()
	}

	/// A human-readable string which is rendered less prominent in the user interface.
	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}
}

impl FileOperationFilter {
	pub fn new(pattern: FileOperationPattern) -> Self {
		Self { scheme: None, pattern }
	}

	pub fn with_scheme(mut self, scheme: String) -> Self {
		self.scheme = Some(scheme);
		self
	}

	/// The actual file operation pattern.
	pub fn pattern(&self) -> &FileOperationPattern {
		&self.pattern
	}

	/// A Uri scheme like `file` or `untitled`
	pub fn scheme(&self) -> Option<&String> {
		self.scheme.as_ref()
	}
}

impl FileRename {
	pub fn new(new_uri: String, old_uri: String) -> Self {
		Self { old_uri, new_uri }
	}

	/// A `file://` URI for the original location of the file/folder being renamed.
	pub fn new_uri(&self) -> &String {
		&self.new_uri
	}

	/// A `file://` URI for the new location of the file/folder being renamed.
	pub fn old_uri(&self) -> &String {
		&self.old_uri
	}
}

impl FileDelete {
	pub fn new(uri: String) -> Self {
		Self { uri }
	}

	/// A `file://`` URI for the location of the file/folder being deleted.
	pub fn uri(&self) -> &String {
		&self.uri
	}
}

impl InlineValueContext {
	pub fn new(frame_id: i32, stopped_location: Range) -> Self {
		Self { frame_id, stopped_location }
	}

	/// The stack frame (as a DAP Id) where the execution has stopped.
	pub fn frame_id(&self) -> &i32 {
		&self.frame_id
	}

	/// The document range where execution has stopped.
	///
	/// Typically the end position of the range denotes the line where the inline values are shown.
	pub fn stopped_location(&self) -> &Range {
		&self.stopped_location
	}
}

impl InlineValueText {
	pub fn new(text: String, range: Range) -> Self {
		Self { range, text }
	}

	/// The text of the inline value.
	pub fn text(&self) -> &String {
		&self.text
	}

	/// The document range for which the inline value applies.
	pub fn range(&self) -> &Range {
		&self.range
	}
}

impl RelativePattern {
	pub fn new(base_uri: FolderOrUri, pattern: Pattern) -> Self {
		Self { base_uri, pattern }
	}

	/// A workspace folder or a base URI to which this pattern will be matched against relatively.
	pub fn base_uri(&self) -> &FolderOrUri {
		&self.base_uri
	}

	/// The actual glob pattern
	pub fn pattern(&self) -> &Pattern {
		&self.pattern
	}
}

impl TextDocumentContentChangeEvent {
	pub fn new(text: String) -> Self {
		Self { range: None, range_length: None, text }
	}

	pub fn with_range(mut self, range: Range) -> Self {
		self.range = Some(range);
		self
	}

	pub fn with_range_length(mut self, range_length: u32) -> Self {
		self.range_length = Some(range_length);
		self
	}

	/// The new text for the provided range or if `range` is None the whole document.
	pub fn text(&self) -> &String {
		&self.text
	}

	/// The range of the document that changed.
	pub fn range(&self) -> Option<&Range> {
		self.range.as_ref()
	}

	/// The optional length of the range that got replaced.
	///
	/// deprecated: use `range` instead.
	pub fn range_length(&self) -> Option<&u32> {
		self.range_length.as_ref()
	}
}

impl LocationLink {
	pub fn new(target_uri: Uri, target_range: Range, target_selection_range: Range) -> Self {
		Self {
			origin_selection_range: None,
			target_uri,
			target_range,
			target_selection_range,
		}
	}

	pub fn with_origin_selection_range(mut self, origin_selection_range: Range) -> Self {
		self.origin_selection_range = Some(origin_selection_range);
		self
	}

	/// The target resource identifier of this link.
	pub fn target_uri(&self) -> &Uri {
		&self.target_uri
	}

	/// The full target range of this link.
	///
	/// If the target for example is a symbol then target range is the range
	/// enclosing this symbol not including leading/trailing whitespace but everything else
	/// like comments.
	///
	/// This information is typically used to highlight the range in the editor.
	pub fn target_range(&self) -> &Range {
		&self.target_range
	}

	/// The range that should be selected and revealed when this link is being followed,
	/// e.g the name of a function.
	///
	/// Must be contained by the [`LocationLink::target_range`].
	///
	/// See also [`DocumentSymbol::range`]
	pub fn target_selection_range(&self) -> &Range {
		&self.target_selection_range
	}

	/// Span of the origin of this link.
	///
	/// Used as the underlined span for mouse interaction.
	///
	/// Defaults to the word range at the definition position.
	pub fn origin_selection_range(&self) -> Option<&Range> {
		self.origin_selection_range.as_ref()
	}
}

impl InlayHintLabelPart {
	pub fn new(value: String) -> Self {
		Self {
			value,
			tooltip: None,
			location: None,
			command: None,
		}
	}

	pub fn with_tooltip(mut self, tooltip: MarkupOrString) -> Self {
		self.tooltip = Some(tooltip);
		self
	}

	pub fn with_location(mut self, location: Location) -> Self {
		self.location = Some(location);
		self
	}

	pub fn with_command(mut self, command: Command) -> Self {
		self.command = Some(command);
		self
	}

	/// The value of this label part.
	pub fn value(&self) -> &String {
		&self.value
	}

	/// The tooltip text when you hover over this label part.
	///
	/// Depending on the client capability `inlayHint.resolveSupport` clients might resolve
	/// this property late using the resolve request.
	pub fn tooltip(&self) -> Option<&MarkupOrString> {
		self.tooltip.as_ref()
	}
	/// An optional source code location that represents this label part.
	///
	/// The editor will use this location for the hover and for code navigation features:
	/// This part will become a clickable link that resolves to the definition of the symbol
	/// at the given location (not necessarily the location itself), it shows the hover that
	/// shows at the given location, and it shows a context menu with further code navigation commands.
	///
	/// Depending on the client capability `inlayHint.resolveSupport` clients might resolve this property
	/// late using the resolve request.
	pub fn location(&self) -> Option<&Location> {
		self.location.as_ref()
	}
	/// An optional command for this label part.
	///
	/// Depending on the client capability `inlayHint.resolveSupport` clients
	/// might resolve this property late using the resolve request.
	pub fn command(&self) -> Option<&Command> {
		self.command.as_ref()
	}
}

impl MarkupContent {
	pub fn new(kind: MarkupKind, value: String) -> Self {
		Self { kind, value }
	}

	/// The type of the Markup
	pub fn kind(&self) -> &MarkupKind {
		&self.kind
	}

	/// The content itself
	pub fn value(&self) -> &String {
		&self.value
	}
}

impl InlineValueVariableLookup {
	pub fn new(range: Range, case_sensitive_lookup: bool) -> Self {
		Self { range, variable_name: None, case_sensitive_lookup }
	}

	pub fn with_variable_name(mut self, variable_name: String) -> Self {
		self.variable_name = Some(variable_name);
		self
	}

	/// The document range for which the inline value applies.
	///
	/// The range is used to extract the variable name from the underlying document.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// How to perform the lookup.
	pub fn case_sensitive_lookup(&self) -> &bool {
		&self.case_sensitive_lookup
	}

	/// If specified the name of the variable to look up.
	pub fn variable_name(&self) -> Option<&String> {
		self.variable_name.as_ref()
	}
}

impl InlineValueEvaluatableExpression {
	pub fn new(range: Range) -> Self {
		Self { range, expression: None }
	}

	pub fn with_expression(mut self, expression: String) -> Self {
		self.expression = Some(expression);
		self
	}

	/// The document range for which the inline value applies.
	///
	/// The range is used to extract the evaluatable expression from the underlying document.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// If specified the expression overrides the extracted expression.
	pub fn expression(&self) -> Option<&String> {
		self.expression.as_ref()
	}
}

impl ExecutionSummary {
	pub fn new(execution_order: u32) -> Self {
		Self { execution_order, success: None }
	}

	pub fn with_success(mut self, success: bool) -> Self {
		self.success = Some(success);
		self
	}

	/// A strict monotonically increasing value indicating the execution order of a cell inside a notebook.
	pub fn execution_order(&self) -> &u32 {
		&self.execution_order
	}

	/// Whether the execution was successful or not if known by the client.
	pub fn success(&self) -> Option<&bool> {
		self.success.as_ref()
	}
}

impl NotebookCellTextDocumentFilter {
	pub fn new(notebook: NotebookDocumentFilter) -> Self {
		Self { notebook, language: None }
	}

	pub fn with_language(mut self, language: String) -> Self {
		self.language = Some(language);
		self
	}

	/// A filter that matches against the notebook containing the notebook cell.
	///
	/// If a string value is provided it matches against the notebook type. '*' matches every notebook.
	pub fn notebook(&self) -> &NotebookDocumentFilter {
		&self.notebook // XXX(aki) or String,
	}

	/// A language id like `python`.
	///
	/// Will be matched against the language id of the notebook cell document. '*' matches every language.
	pub fn language(&self) -> Option<&String> {
		self.language.as_ref()
	}
}

impl DiagnosticRelatedInformation {
	pub fn new(location: Location, message: String) -> Self {
		Self { location, message }
	}

	/// The location of this related diagnostic information.
	pub fn location(&self) -> &Location {
		&self.location
	}

	/// The message of this related diagnostic information.
	pub fn message(&self) -> &String {
		&self.message
	}
}

impl CodeDescription {
	pub fn new(href: Uri) -> Self {
		Self { href }
	}

	/// An URI to open with more information about the diagnostic error.
	pub fn href(&self) -> &Uri {
		&self.href
	}
}

impl SelectionCompletionInfo {
	pub fn new(range: Range, text: String) -> Self {
		Self { range, text }
	}

	/// The range that will be replaced if this completion item is accepted.
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The text the range will be replaced with if this completion is accepted.
	pub fn text(&self) -> &String {
		&self.text
	}
}

impl NotebookCellArrayChange {
	pub fn new(start: u32, delete_count: u32) -> Self {
		Self { start, delete_count, cells: None }
	}

	pub fn with_cells(mut self, cells: Vec<NotebookCell>) -> Self {
		self.cells = Some(cells);
		self
	}

	/// The start oftest of the cell that changed.
	pub fn start(&self) -> &u32 {
		&self.start
	}

	/// The deleted cells
	pub fn delete_count(&self) -> &u32 {
		&self.delete_count
	}

	/// The new cells, if any
	pub fn cells(&self) -> Option<&Vec<NotebookCell>> {
		self.cells.as_ref()
	}
}

impl NotebookCell {
	pub fn new(kind: NotebookCellKind, document: Uri) -> Self {
		Self {
			kind,
			document,
			metadata: None,
			execution_summary: None,
		}
	}

	pub fn with_metadata(mut self, metadata: LspObject) -> Self {
		self.metadata = Some(metadata);
		self
	}

	pub fn with_execution_summary(mut self, execution_summary: ExecutionSummary) -> Self {
		self.execution_summary = Some(execution_summary);
		self
	}

	/// The cell's kind
	pub fn kind(&self) -> &NotebookCellKind {
		&self.kind
	}

	/// The URI of the cell's text document content.
	pub fn document(&self) -> &Uri {
		&self.document
	}

	/// Additional metadata stored with the cell.
	///
	/// Note: should always be an object literal (e.g. LSPObject)
	pub fn metadata(&self) -> Option<&LspObject> {
		self.metadata.as_ref()
	}

	/// Additional execution summary information if supported by the client.
	pub fn execution_summary(&self) -> Option<&ExecutionSummary> {
		self.execution_summary.as_ref()
	}
}

impl FileOperationPattern {
	pub fn new(glob: String) -> Self {
		Self { glob, matches: None, options: None }
	}

	pub fn with_matches(mut self, matches: FileOperationPatternKind) -> Self {
		self.matches = Some(matches);
		self
	}

	pub fn with_options(mut self, options: FileOperationPatternOptions) -> Self {
		self.options = Some(options);
		self
	}

	/// The glob pattern to match.
	///
	/// Glob patterns can have the following syntax:
	/// - `*` to match zero or more characters in a path segment
	/// - `?` to match on one character in a path segment
	/// - `**` to match any number of path segments, including none
	/// - `{}` to group sub patterns into an OR expression. (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
	/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
	/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
	pub fn glob(&self) -> &String {
		&self.glob
	}

	/// Whether to match files or folders with this pattern. Matches both if undefined.
	pub fn matches(&self) -> Option<&FileOperationPatternKind> {
		self.matches.as_ref()
	}

	/// Additional options used during matching.
	pub fn options(&self) -> Option<&FileOperationPatternOptions> {
		self.options.as_ref()
	}
}

impl ResourceOperation {
	pub fn new(kind: String) -> Self {
		Self { kind, annotation_id: None }
	}

	pub fn with_annotation_id(mut self, annotation_id: String) -> Self {
		self.annotation_id = Some(annotation_id);
		self
	}

	/// The resource operation kind.
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// An optional annotation identifier describing the operation.
	///
	/// since: 3.16.0
	pub fn annotation_id(&self) -> Option<&ChangeAnnotationIdentifier> {
		self.annotation_id.as_ref()
	}
}

impl SemanticTokensLegend {
	pub fn new(token_types: Vec<String>, token_modifiers: Vec<String>) -> Self {
		Self { token_types, token_modifiers }
	}

	pub fn from_str_vec(token_types: Vec<&str>, token_modifiers: Vec<&str>) -> Self {
		Self::new(
			token_types.into_iter().map(String::from).collect(),
			token_modifiers.into_iter().map(String::from).collect()
		)
	}

	/// The token types a server uses.
	pub fn token_types(&self) -> &Vec<String> {
		&self.token_types
	}

	/// The token modifiers a server uses.
	pub fn token_modifiers(&self) -> &Vec<String> {
		&self.token_modifiers
	}
}

impl BaseSymbolInformation {
	pub fn new(name: String, kind: SymbolKind) -> Self {
		Self { name, kind, tags: None, container_name: None }
	}

	pub fn with_tags(mut self, tags: Vec<SymbolTag>) -> Self {
		self.tags = Some(tags);
		self
	}

	pub fn with_container_name(mut self, container_name: String) -> Self {
		self.container_name = Some(container_name);
		self
	}

	/// The name of this symbol.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The kind of this symbol.
	pub fn kind(&self) -> &SymbolKind {
		&self.kind
	}

	/// Tags for this symbol.
	///
	/// since: 3.16.0
	pub fn tags(&self) -> Option<&Vec<SymbolTag>> {
		self.tags.as_ref()
	}

	/// The name of the symbol containing this symbol.
	///
	/// This information is for user interface purposes (e.g. to render a qualifier in the user interface
	/// if necessary). It can't be used to re-infer a hierarchy for the document symbols.
	pub fn container_name(&self) -> Option<&String> {
		self.container_name.as_ref()
	}
}

impl ReferenceContext {
	pub fn new(include_declaration: bool) -> Self {
		Self { include_declaration }
	}

	/// Include the declaration of the current symbol.
	pub fn include_declaration(&self) -> &bool {
		&self.include_declaration
	}
}

impl SignatureHelpContext {
	pub fn new(trigger_kind: SignatureHelpTriggerKind, is_retrigger: bool) -> Self {
		Self {
			trigger_kind,
			trigger_character: None,
			is_retrigger,
			active_signature_help: None,
		}
	}

	pub fn with_trigger_character(mut self, trigger_character: String) -> Self {
		self.trigger_character = Some(trigger_character);
		self
	}

	pub fn with_active_signature_help(mut self, active_signature_help: SignatureHelp) -> Self {
		self.active_signature_help = Some(active_signature_help);
		self
	}

	/// Action that caused signature help to be triggered.
	pub fn trigger_kind(&self) -> &SignatureHelpTriggerKind {
		&self.trigger_kind
	}

	/// Character that caused signature help to be triggered.
	///
	/// This is undefined when `trigger_kind !== SignatureHelpTriggerKind.TriggerCharacter`
	pub fn trigger_character(&self) -> Option<&String> {
		self.trigger_character.as_ref()
	}

	/// `true` if signature help was already showing when it was triggered.
	///
	/// Retriggers occurs when the signature help is already active and can be caused by actions such as
	/// typing a trigger character, a cursor move, or document content changes.
	pub fn is_retrigger(&self) -> &bool {
		&self.is_retrigger
	}

	/// The currently active `SignatureHelp`.
	///
	/// The `activeSignatureHelp` has its `SignatureHelp.activeSignature` field updated based on
	/// the user navigating through available signatures.
	pub fn active_signature_help(&self) -> Option<&SignatureHelp> {
		self.active_signature_help.as_ref()
	}
}

impl InsertReplaceEdit {
	pub fn new(new_text: String, insert: Range, replace: Range) -> Self {
		Self { new_text, insert, replace }
	}

	/// The string to be inserted.
	pub fn new_text(&self) -> &String {
		&self.new_text
	}

	/// The range if the insert is requested
	pub fn insert(&self) -> &Range {
		&self.insert
	}

	/// The range if the replace is requested.
	pub fn replace(&self) -> &Range {
		&self.replace
	}
}

impl CompletionItemLabelDetails {
	pub fn new() -> Self {
		Self { detail: None, description: None }
	}

	pub fn with_detail(mut self, detail: String) -> Self {
		self.detail = Some(detail);
		self
	}

	pub fn with_description(mut self, description: String) -> Self {
		self.description = Some(description);
		self
	}

	/// An optional string which is rendered less prominently directly after [`CompletionItem::label`],
	/// without any spacing.
	///
	/// Should be used for function signatures and type annotations.
	pub fn detail(&self) -> Option<&String> {
		self.detail.as_ref()
	}

	/// An optional string which is rendered less prominently after [`CompletionItem::detail`]. Should be used
	/// for fully qualified names and file paths.
	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}
}

impl CompletionContext {
	pub fn new(trigger_kind: CompletionTriggerKind) -> Self {
		Self { trigger_kind, trigger_character: None }
	}

	pub fn with_trigger_character(mut self, trigger_character: String) -> Self {
		self.trigger_character = Some(trigger_character);
		self
	}

	/// How the completion was triggered.
	pub fn trigger_kind(&self) -> &CompletionTriggerKind {
		&self.trigger_kind
	}

	/// The trigger character (a single character) that has trigger code complete.
	///
	/// Is undefined if `triggerKind !== CompletionTriggerKind.TriggerCharacter`
	pub fn trigger_character(&self) -> Option<&String> {
		self.trigger_character.as_ref()
	}
}

impl Diagnostic {
	pub fn new(range: Range, message: String) -> Self {
		Self {
			range,
			severity: None,
			code: None,
			code_description: None,
			source: None,
			message,
			tags: None,
			related_information: None,
			data: None,
		}
	}

	pub fn with_severity(mut self, severity: DiagnosticSeverity) -> Self {
		self.severity = Some(severity);
		self
	}

	pub fn with_code(mut self, code: ProgressToken) -> Self {
		self.code = Some(code);
		self
	}

	pub fn with_code_description(mut self, code_description: CodeDescription) -> Self {
		self.code_description = Some(code_description);
		self
	}

	pub fn with_source(mut self, source: String) -> Self {
		self.source = Some(source);
		self
	}

	pub fn with_tags(mut self, tags: Vec<DiagnosticTag>) -> Self {
		self.tags = Some(tags);
		self
	}

	pub fn with_related_information(
		mut self,
		related_information: Vec<DiagnosticRelatedInformation>,
	) -> Self {
		self.related_information = Some(related_information);
		self
	}

	pub fn with_data(mut self, data: LspAny) -> Self {
		self.data = Some(data);
		self
	}

	/// The range at which the message applies
	pub fn range(&self) -> &Range {
		&self.range
	}

	/// The diagnostic's severity.
	///
	/// Can be omitted.
	///
	/// If omitted it is up to the client to interpret diagnostics as error, warning, info or hint.
	pub fn severity(&self) -> Option<&DiagnosticSeverity> {
		self.severity.as_ref()
	}

	/// The diagnostic's code, which usually appear in the user interface.
	pub fn code(&self) -> Option<&ProgressToken> {
		self.code.as_ref() // XXX(aki): Not the right type but right element
	}

	/// An optional property to describe the error code.
	///
	/// Requires the code field (above) to be present/not null.
	///
	/// since: 3.16.0
	pub fn code_description(&self) -> Option<&CodeDescription> {
		self.code_description.as_ref()
	}

	/// A human-readable string describing the source of this diagnostic,
	/// e.g. 'typescript' or 'super lint'. It usually appears in the user interface.
	pub fn source(&self) -> Option<&String> {
		self.source.as_ref()
	}

	/// The diagnostic's message. It usually appears in the user interface
	pub fn message(&self) -> &String {
		&self.message
	}

	/// Additional metadata about the diagnostic.
	///
	/// since: 3.15.0
	pub fn tags(&self) -> Option<&Vec<DiagnosticTag>> {
		self.tags.as_ref()
	}

	/// An array of related diagnostic information, e.g. when symbol-names within
	/// a scope collide all definitions can be marked via this property.
	pub fn related_information(&self) -> Option<&Vec<DiagnosticRelatedInformation>> {
		self.related_information.as_ref()
	}

	/// A data entry field that is preserved between a `textDocument/publishDiagnostics`
	/// notification and `textDocument/codeAction` request.
	///
	/// since: 3.16.0
	pub fn data(&self) -> Option<&LspAny> {
		self.data.as_ref()
	}
}

impl FileSystemWatcher {
	pub fn new(glob_pattern: GlobPattern) -> Self {
		Self { glob_pattern, kind: None }
	}

	pub fn with_kind(mut self, kind: WatchKind) -> Self {
		self.kind = Some(kind);
		self
	}

	/// The glob pattern to watch.
	///
	/// See [`GlobPattern`] for more detail.
	///
	/// since: 3.17.0 support for relative patterns.
	pub fn glob_pattern(&self) -> &GlobPattern {
		&self.glob_pattern
	}

	/// The kind of events of interest.
	///
	/// If omitted it defaults to WatchKind.Create | WatchKind.Change | WatchKind.Delete which is 7.
	pub fn kind(&self) -> Option<&WatchKind> {
		self.kind.as_ref()
	}
}

impl FileEvent {
	pub fn new(uri: Uri, typ: FileChangeType) -> Self {
		Self { uri, typ }
	}

	/// The file's uri.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The change type.
	pub fn typ(&self) -> &FileChangeType {
		&self.typ
	}
}

impl Unregistration {
	pub fn new(id: String, method: String) -> Self {
		Self { id, method }
	}

	/// The id used to unregister the request or notification.
	///
	/// Usually an id provided during the register request.
	pub fn id(&self) -> &String {
		&self.id
	}

	/// The method to unregister for.
	pub fn method(&self) -> &String {
		&self.method
	}
}

impl Registration {
	pub fn new(id: String, method: String) -> Self {
		Self { id, method, register_options: None }
	}

	/// The id used to register the request. The id can be used to deregister the request again.
	pub fn id(&self) -> &String {
		&self.id
	}

	/// The method/capability to register for.
	pub fn method(&self) -> &String {
		&self.method
	}

	/// Options necessary for the registration.
	pub fn register_options(&self) -> Option<&LspAny> {
		self.register_options.as_ref()
	}
}

impl StringValue {
	pub fn new(kind: String, value: String) -> Self {
		Self { kind, value }
	}

	// `snippet`
	/// The kind of string value.
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// The snippet string.
	pub fn value(&self) -> &String {
		&self.value
	}
}

impl InlineCompletionContext {
	pub fn new(trigger_kind: InlineCompletionTriggerKind) -> Self {
		Self { trigger_kind, selected_completion_info: None }
	}

	pub fn with_selected_completion_info(
		mut self,
		selected_completion_info: SelectionCompletionInfo,
	) -> Self {
		self.selected_completion_info = Some(selected_completion_info);
		self
	}

	/// Describes how the inline completion was triggered.
	pub fn trigger_kind(&self) -> &InlineCompletionTriggerKind {
		&self.trigger_kind
	}

	/// Provides information about the currently selected item in the autocomplete widget if it is visible.
	pub fn selected_completion_info(&self) -> Option<&SelectionCompletionInfo> {
		self.selected_completion_info.as_ref()
	}
}

impl NotebookDocumentIdentifier {
	pub fn new(uri: Uri) -> Self {
		Self { uri }
	}

	/// The notebook document's uri.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}
}

impl VersionedNotebookDocumentIdentifier {
	pub fn new(uri: Uri, version: i32) -> Self {
		Self { uri, version }
	}

	/// The version number of this notebook document.
	pub fn version(&self) -> &i32 {
		&self.version
	}

	/// The notebook document's uri.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}
}

impl VersionedTextDocumentIdentifier {
	pub fn new(text_document_identifier: TextDocumentIdentifier, version: i32) -> Self {
		Self { text_document_identifier, version }
	}

	/// The version number of this document.
	pub fn version(&self) -> &i32 {
		&self.version
	}

	pub fn text_document_identifier(&self) -> &TextDocumentIdentifier {
		&self.text_document_identifier
	}
}

impl TextDocumentItem {
	pub fn new(uri: Uri, language_id: LanguageId, version: i32, text: String) -> Self {
		Self { uri, language_id, version, text }
	}

	/// The text document's uri.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The text document's language identifier.
	pub fn language_id(&self) -> &LanguageId {
		&self.language_id
	}

	/// The version number of this document (it will increase after each change, including undo/redo).
	pub fn version(&self) -> &i32 {
		&self.version
	}

	/// The content of the opened text document.
	pub fn text(&self) -> &String {
		&self.text
	}
}

impl NotebookDocument {
	pub fn new(uri: Uri, notebook_type: String, version: i32, cells: Vec<NotebookCell>) -> Self {
		Self { uri, notebook_type, version, metadata: None, cells }
	}

	pub fn with_metadata(mut self, metadata: LspObject) -> Self {
		self.metadata = Some(metadata);
		self
	}

	/// The notebook document's uri.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The type of the notebook.
	pub fn notebook_type(&self) -> &String {
		&self.notebook_type
	}

	/// The version number of this document (it will increase after each change, including undo/redo).
	pub fn version(&self) -> &i32 {
		&self.version
	}

	/// Additional metadata stored with the notebook document.
	///
	/// Note: should always be an object literal (e.g. LSPObject)
	pub fn metadata(&self) -> Option<&LspObject> {
		self.metadata.as_ref()
	}

	/// The cells of a notebook.
	pub fn cells(&self) -> &Vec<NotebookCell> {
		&self.cells
	}
}

impl PreviousResultId {
	pub fn new(uri: Uri, value: String) -> Self {
		Self { uri, value }
	}

	/// The URI for which the client knowns a result id.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The value of the previous result id.
	pub fn value(&self) -> &String {
		&self.value
	}
}

impl UnchangedDocumentDiagnosticReport {
	pub fn new(kind: String, result_id: String) -> Self {
		Self { kind, result_id }
	}

	// `unchanged`
	/// A document diagnostic report indicating no changes to the last result.
	///
	/// A server can only return `unchanged` if result ids are provided.
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// A result id which will be sent on the next diagnostic request for the same document.
	pub fn result_id(&self) -> &String {
		&self.result_id
	}
}

impl FullDocumentDiagnosticReport {
	pub fn new(kind: String, items: Vec<Diagnostic>) -> Self {
		Self { kind, items, result_id: None }
	}

	// `full`
	/// A full document diagnostic report.
	pub fn kind(&self) -> &String {
		&self.kind
	}

	/// The actual items.
	pub fn items(&self) -> &Vec<Diagnostic> {
		&self.items
	}

	/// An optional result id.
	///
	/// If provided it will be sent on the next diagnostic request for the same document.
	pub fn result_id(&self) -> Option<&String> {
		self.result_id.as_ref()
	}
}

impl SymbolInformation {
	pub fn new(location: Location, base_symbol_info: BaseSymbolInformation) -> Self {
		Self { deprecated: None, location, base_symbol_info }
	}

	pub fn with_deprecated(mut self, deprecated: bool) -> Self {
		self.deprecated = Some(deprecated);
		self
	}

	/// Indicates if this symbol is deprecated.
	///
	/// deprecated: Use tags instead
	pub fn deprecated(&self) -> Option<&bool> {
		self.deprecated.as_ref()
	}

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
	pub fn location(&self) -> &Location {
		&self.location
	}
	pub fn base_symbol_info(&self) -> &BaseSymbolInformation {
		&self.base_symbol_info
	}
}

impl CodeActionContext {
	pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
		Self { diagnostics, only: None, trigger_kind: None }
	}

	pub fn with_only(mut self, only: Vec<CodeActionKind>) -> Self {
		self.only = Some(only);
		self
	}

	pub fn with_trigger_kind(mut self, trigger_kind: CodeActionTriggerKind) -> Self {
		self.trigger_kind = Some(trigger_kind);
		self
	}

	/// An array of diagnostics known on the client side overlapping the range provided to the
	/// `textDocument/codeAction` request. They are provided so that the server knows which
	/// errors are currently presented to the user for the given range.
	///
	/// There is no guarantee that these accurately reflect the error state of the resource.
	///
	/// The primary parameter to compute code actions is the provided range.
	pub fn diagnostics(&self) -> &Vec<Diagnostic> {
		&self.diagnostics
	}

	/// Requested kind of actions to return.
	///
	/// Actions not of this kind are filtered out by the client before being shown. So servers can omit computing them.
	pub fn only(&self) -> Option<&Vec<CodeActionKind>> {
		self.only.as_ref()
	}

	/// The reason why code actions were requested.
	///
	/// since: 3.17.0
	pub fn trigger_kind(&self) -> Option<&CodeActionTriggerKind> {
		self.trigger_kind.as_ref()
	}
}

impl RelatedFullDocumentDiagnosticReport {
	pub fn new(full_document_diagnostic_report: FullDocumentDiagnosticReport) -> Self {
		Self {
			related_documents: None,
			full_document_diagnostic_report,
		}
	}

	pub fn with_related_documents(
		mut self,
		related_documents: HashMap<fluent_uri::Uri<String>, DiagnosticReport>,
	) -> Self {
		self.related_documents = Some(related_documents);
		self
	}

	/// Diagnostics of related documents.
	///
	/// This information is useful in programming languages where code in a file A can generate
	/// diagnostics in a file B which A depends on.
	///
	/// An example of such a language is C and C++ where marco definitions in a file `a.cpp`` can result
	/// in errors in a header file `b.hpp`.
	///
	/// since: 3.17.0
	pub fn related_documents(&self) -> Option<&HashMap<Uri, DiagnosticReport>> {
		self.related_documents.as_ref()
	}

	pub fn full_document_diagnostic_report(&self) -> &FullDocumentDiagnosticReport {
		&self.full_document_diagnostic_report
	}
}

impl RelatedUnchangedDocumentDiagnosticReport {
	pub fn new(unchanged_document_diagnostic_report: UnchangedDocumentDiagnosticReport) -> Self {
		Self {
			related_documents: None,
			unchanged_document_diagnostic_report,
		}
	}

	pub fn with_related_documents(
		mut self,
		related_documents: HashMap<fluent_uri::Uri<String>, DiagnosticReport>,
	) -> Self {
		self.related_documents = Some(related_documents);
		self
	}

	/// Diagnostics of related documents.
	///
	/// This information is useful in programming languages where code in a file A can generate
	/// diagnostics in a file B which A depends on.
	///
	/// An example of such a language is C and C++ where marco definitions in a file `a.cpp`` can result
	/// in errors in a header file `b.hpp`.
	///
	/// since: 3.17.0
	pub fn related_documents(&self) -> Option<&HashMap<Uri, DiagnosticReport>> {
		self.related_documents.as_ref()
	}

	pub fn unchanged_document_diagnostic_report(&self) -> &UnchangedDocumentDiagnosticReport {
		&self.unchanged_document_diagnostic_report
	}
}

impl WorkspaceFullDocumentDiagnosticReport {
	pub fn new(uri: Uri, full_document_diagnostic_report: FullDocumentDiagnosticReport) -> Self {
		Self {
			uri,
			version: None,
			full_document_diagnostic_report,
		}
	}

	pub fn with_version(mut self, version: i32) -> Self {
		self.version = Some(version);
		self
	}

	/// The URI for which diagnostic information is reported.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The version number for which the diagnostics are reported.
	///
	/// If the document is not marked as open `null` can be provided.
	pub fn version(&self) -> Option<&i32> {
		self.version.as_ref()
	}

	pub fn full_document_diagnostic_report(&self) -> &FullDocumentDiagnosticReport {
		&self.full_document_diagnostic_report
	}
}

impl WorkspaceUnchangedDocumentDiagnosticReport {
	pub fn new(
		uri: Uri,
		unchanged_document_diagnostic_report: UnchangedDocumentDiagnosticReport,
	) -> Self {
		Self {
			uri,
			version: None,
			unchanged_document_diagnostic_report,
		}
	}

	pub fn with_version(mut self, version: i32) -> Self {
		self.version = Some(version);
		self
	}

	/// The URI for which diagnostic information is reported.
	pub fn uri(&self) -> &Uri {
		&self.uri
	}

	/// The version number for which the diagnostics are reported.
	///
	/// If the document is not marked as open `null` can be provided.
	pub fn version(&self) -> Option<&i32> {
		self.version.as_ref()
	}

	pub fn unchanged_document_diagnostic_report(&self) -> &UnchangedDocumentDiagnosticReport {
		&self.unchanged_document_diagnostic_report
	}
}

impl ClientInfo {
	pub fn new(name: String) -> Self {
		Self { name, version: None }
	}

	pub fn with_version(mut self, version: String) -> Self {
		self.version = Some(version);
		self
	}

	/// The name of the client as defined by the client.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The client's version as defined by the client.
	pub fn version(&self) -> Option<&String> {
		self.version.as_ref()
	}
}

impl WorkspaceSymbol {
	pub fn new(location: LocationOrUri, base_symbol_information: BaseSymbolInformation) -> Self {
		Self { location, data: None, base_symbol_information }
	}

	pub fn with_data(mut self, data: LspAny) -> Self {
		self.data = Some(data);
		self
	}

	/// The location of the symbol.
	///
	/// Whether a server is allowed to return a location without a range depends on the client
	/// capability `workspace.symbol.resolveSupport`.
	///
	/// See: [`SymbolInformation::location`] for more details.
	pub fn location(&self) -> &LocationOrUri {
		&self.location
	}

	/// A data entry field that is preserved on a workspace symbol between a workspace symbol
	/// request and a workspace symbol resolve request.
	pub fn data(&self) -> Option<&LspAny> {
		self.data.as_ref()
	}

	pub fn base_symbol_information(&self) -> &BaseSymbolInformation {
		&self.base_symbol_information
	}
}

impl NotebookCellStructureChange {
	pub fn new(array: NotebookCellArrayChange) -> Self {
		Self { array, did_open: None, did_close: None }
	}

	pub fn with_did_open(mut self, did_open: Vec<TextDocumentItem>) -> Self {
		self.did_open = Some(did_open);
		self
	}

	pub fn with_did_close(mut self, did_close: Vec<TextDocumentIdentifier>) -> Self {
		self.did_close = Some(did_close);
		self
	}

	/// The change to the cell array.
	pub fn array(&self) -> &NotebookCellArrayChange {
		&self.array
	}

	/// Additional opened cell text documents
	pub fn did_open(&self) -> Option<&Vec<TextDocumentItem>> {
		self.did_open.as_ref()
	}

	/// Additional closed cell text documents.
	pub fn did_close(&self) -> Option<&Vec<TextDocumentIdentifier>> {
		self.did_close.as_ref()
	}
}

impl NotebookCellTextChange {
	pub fn new(
		document: VersionedNotebookDocumentIdentifier,
		changes: Vec<TextDocumentContentChangeEvent>,
	) -> Self {
		Self { document, changes }
	}

	pub fn document(&self) -> &VersionedNotebookDocumentIdentifier {
		&self.document
	}

	pub fn changes(&self) -> &Vec<TextDocumentContentChangeEvent> {
		&self.changes
	}
}

impl NotebookCellsDiff {
	pub fn new() -> Self {
		Self { structure: None, data: None, text_context: None }
	}

	pub fn with_structure(mut self, structure: NotebookCellStructureChange) -> Self {
		self.structure = Some(structure);
		self
	}

	pub fn with_data(mut self, data: Vec<NotebookCell>) -> Self {
		self.data = Some(data);
		self
	}

	pub fn with_text_context(mut self, text_context: Vec<NotebookCellTextChange>) -> Self {
		self.text_context = Some(text_context);
		self
	}

	/// Changes to the cell structure to add or remove cells
	pub fn structure(&self) -> Option<&NotebookCellStructureChange> {
		self.structure.as_ref()
	}

	/// Changes to notebook cells properties like its kind, execution summary or metadata.
	pub fn data(&self) -> Option<&Vec<NotebookCell>> {
		self.data.as_ref()
	}

	/// Changes to the text content of notebook cells.
	pub fn text_context(&self) -> Option<&Vec<NotebookCellTextChange>> {
		self.text_context.as_ref()
	}
}

impl NotebookDocumentChangeEvent {
	pub fn new() -> Self {
		Self { metadata: None, cells: None }
	}

	pub fn with_metadata(mut self, metadata: LspObject) -> Self {
		self.metadata = Some(metadata);
		self
	}

	pub fn with_cells(mut self, cells: NotebookCellsDiff) -> Self {
		self.cells = Some(cells);
		self
	}

	/// The changed meta data if any.
	///
	/// Note: should always be an object literal (e.g. LSPObject)
	pub fn metadata(&self) -> Option<&LspObject> {
		self.metadata.as_ref()
	}

	/// Changes to cells
	pub fn cells(&self) -> Option<&NotebookCellsDiff> {
		self.cells.as_ref()
	}
}

impl SignatureInformation {
	pub fn new(label: String) -> Self {
		Self {
			label,
			documentation: None,
			parameters: None,
			active_parameter: None,
		}
	}

	pub fn with_documentation(mut self, documentation: MarkupOrString) -> Self {
		self.documentation = Some(documentation);
		self
	}

	pub fn with_parameters(mut self, parameters: Vec<ParameterInformation>) -> Self {
		self.parameters = Some(parameters);
		self
	}

	pub fn with_active_parameter(mut self, active_parameter: i32) -> Self {
		self.active_parameter = Some(active_parameter);
		self
	}

	/// The label of this signature.
	///
	/// Will be shown in the UI.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// The human-readable doc-comment of this signature.
	///
	/// Will be shown in the UI but can be omitted.
	pub fn documentation(&self) -> Option<&MarkupOrString> {
		self.documentation.as_ref()
	}

	/// The parameters of this signature.
	pub fn parameters(&self) -> Option<&Vec<ParameterInformation>> {
		self.parameters.as_ref()
	}

	/// The index of the active parameter.
	///
	/// If provided, this is used in place of [`SignatureHelp::active_parameter`].
	///
	/// since: 3.16.0
	pub fn active_parameter(&self) -> Option<&i32> {
		self.active_parameter.as_ref()
	}
}

impl NotebookCellLanguage {
	pub fn new(language: String) -> Self {
		Self { language }
	}

	pub fn language(&self) -> &String {
		&self.language
	}
}

impl NotebookDocumentSyncOptionsSelector {
	pub fn new() -> Self {
		Self { notebook: None, cells: None }
	}

	pub fn with_notebook(mut self, notebook: NotebookDocumentFilterOrString) -> Self {
		self.notebook = Some(notebook);
		self
	}

	pub fn with_cells(mut self, cells: Vec<NotebookCellLanguage>) -> Self {
		self.cells = Some(cells);
		self
	}

	/// The notebook to be synced If a string value is provided it matches against the notebook type.
	///
	///  '*' matches every notebook.
	pub fn notebook(&self) -> Option<&NotebookDocumentFilterOrString> {
		self.notebook.as_ref()
	}

	/// The cells of the matching notebook to be synced.
	pub fn cells(&self) -> Option<&Vec<NotebookCellLanguage>> {
		self.cells.as_ref()
	}
}

impl ParameterInformation {
	pub fn new(label: StringOrTuple<u32>) -> Self {
		Self { label, documentation: None }
	}

	pub fn with_documentation(mut self, documentation: MarkupOrString) -> Self {
		self.documentation = Some(documentation);
		self
	}

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
	pub fn label(&self) -> &StringOrTuple<u32> {
		&self.label
	}

	/// The human-readable doc-comment of this parameter.
	///
	/// Will be shown in the UI but can be omitted.
	pub fn documentation(&self) -> Option<&MarkupOrString> {
		self.documentation.as_ref()
	}
}

impl<T> ValueSet<T> {
	pub fn new() -> Self {
		Self { value_set: None }
	}

	pub fn with_values(mut self, values: Vec<T>) -> Self {
		self.value_set = Some(values);
		self
	}

	pub fn values(&self) -> Option<&Vec<T>> {
		self.value_set.as_ref()
	}
}

impl StringProperties {
	pub fn new(properties: Vec<String>) -> Self {
		Self { properties }
	}

	pub fn properties(&self) -> &Vec<String> {
		&self.properties
	}
}

impl CompletionListItemDefaults {
	pub fn new(item_defaults: Vec<String>) -> Self {
		Self { item_defaults }
	}

	pub fn item_defaults(&self) -> &Vec<String> {
		&self.item_defaults
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
