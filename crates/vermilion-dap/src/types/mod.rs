// SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;

pub mod breakpoint;
pub mod capabilities;
pub mod exception;
pub mod options;
pub mod params;
pub mod variable;

pub type Uri = fluent_uri::Uri<String>;

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum IntegerOrString {
	String(String),
	Integer(i64),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum StoppedReason {
	Step,
	Breakpoint,
	Exception,
	Pause,
	Entry,
	GoTo,
	#[serde(rename = "function breakpoint")]
	FunctionBreakpoint,
	#[serde(rename = "data breakpoint")]
	DataBreakpoint,
	#[serde(rename = "instruction breakpoint")]
	InstructionBreakpoint,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ThreadReason {
	Started,
	Exited,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum EvalContext {
	/// evaluate is called from a watch view context.
	Watch,
	/// evaluate is called from a REPL context.
	Repl,
	/// evaluate is called to generate the debug hover contents.
	///
	/// This value should only be used if the corresponding capability `supportsEvaluateForHovers`
	/// is true.
	Hover,
	/// evaluate is called to generate clipboard contents.
	///
	/// This value should only be used if the corresponding capability `supportsClipboardContext` is
	/// true.
	Clipboard,
	/// evaluate is called from a variables view context.
	Variables,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum OutputCategory {
	/// Show the output in the client's default message UI, e.g. a 'debug console'.
	///
	/// This category should only be used for informational output from the debugger (as opposed to
	/// the debuggee).
	Console,
	/// A hint for the client to show the output in the client's UI for important and highly visible
	/// information, e.g. as a popup notification.
	///
	/// This category should only be used for important messages from the debugger (as opposed to
	/// the debuggee).
	///
	/// Since this category value is a hint, clients might ignore the hint and assume the `console`
	/// category.
	Important,
	/// Show the output as normal program output from the debuggee.
	Stdout,
	/// Show the output as error program output from the debuggee.
	Stderr,
	/// Send the output to telemetry instead of showing it to the user.
	Telemetry,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum OutputGroup {
	/// Start a new group in expanded mode. Subsequent output events are members of the group and
	/// should be shown indented.\nThe `output` attribute becomes the name of the group and is not
	/// indented.
	Start,
	/// Start a new group in collapsed mode. Subsequent output events are members of the group and
	/// should be shown indented (as soon as the group is expanded).\nThe `output` attribute becomes
	/// the name of the group and is not indented.
	StartCollapsed,
	/// End the current group and decrease the indentation of subsequent output events.\nA non-empty
	/// `output` attribute is shown as the unindented end of the group.
	End,
}

/// The checksum of an item calculated by the specified algorithm.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Checksum {
	/// The algorithm used to calculate this checksum.
	pub(crate) algorithm: ChecksumAlgorithm,
	/// Value of the checksum, encoded as a hexadecimal value.
	pub(crate) checksum:  String,
}

/// Names of checksum algorithms that may be supported by a debug adapter.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum ChecksumAlgorithm {
	MD5,
	SHA1,
	SHA256,
	#[serde(rename = "timestamp")]
	Timestamp,
}

/// A [`ColumnDescriptor`] specifies what module attribute to show in a column of the modules view,
/// how to format it, and what the column's label should be.
///
/// It is only used if the underlying UI actually supports this level of customization.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDescriptor {
	/// Name of the attribute rendered in this column.
	pub(crate) attribute_name: String,
	/// Header UI label of column.
	pub(crate) label:          String,
	/// Format to use for the rendered values in this column.
	///
	/// TBD how the format strings looks like.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) format:         Option<String>,
	/// Datatype of values in this column. Defaults to [`ColumnType::String`] if not specified.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "type")]
	pub(crate) column_type:    Option<ColumnType>,
	/// Width of this column in characters (hint only).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) width:          Option<u32>,
}

#[derive(
	Clone,
	Copy,
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
#[serde(untagged, rename_all = "camelCase")]
pub enum ColumnType {
	#[default]
	String,
	Number,
	Boolean,
	#[serde(rename = "unixTimestampUTC")]
	UnixTimestampUTC,
}

/// `CompletionItems` are the suggestions returned from the `completions` request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
	/// The label of this completion item.
	///
	/// By default this is also the text that is inserted when selecting this completion.
	pub(crate) label:            String,
	/// If text is returned and not an empty string, then it is inserted instead of the label.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) text:             Option<String>,
	/// A string that should be used when comparing this item with other items.
	///
	/// If not returned or an empty string, the `label` is used instead.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) sort_text:        Option<String>,
	/// A human-readable string with additional information about this item, like type or symbol
	/// information.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) detail:           Option<String>,
	/// The item's type.
	///
	/// Typically the client uses this information to render the item in the UI with an icon.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "type")]
	pub(crate) completion_type:  Option<CompletionItemType>,
	/// Start position (within the `text` attribute of the `completions` request) where the
	/// completion text is added.
	///
	/// The position is measured in UTF-16 code units and the client
	/// capability `columnsStartAt1` determines whether it is 0- or 1-based.
	///
	/// If the start position is omitted the text is added at the location specified by the
	/// `column` attribute of the `completions` request.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) start:            Option<u32>,
	/// Length determines how many characters are overwritten by the completion text and it is
	/// measured in UTF-16 code units.
	///
	/// If missing the value 0 is assumed which results in the completion text being inserted.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) length:           Option<u32>,
	/// Determines the start of the new selection after the text has been inserted (or replaced).
	///
	/// [`CompletionItem::selection_start`] is measured in UTF-16 code units and must be in the
	/// range 0 and length of the completion text.
	///
	/// If omitted the selection starts at the end of the completion text.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) selection_start:  Option<u32>,
	/// Determines the length of the new selection after the text has been inserted (or replaced)
	/// and it is measured in UTF-16 code units.
	///
	/// The selection can not extend beyond the bounds of the completion text.
	///
	/// If omitted the length is assumed to be 0.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) selection_length: Option<u32>,
}

/// Some predefined types for the CompletionItem.
///
/// Please note that not all clients have specific icons for all of them.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CompletionItemType {
	Method,
	Function,
	Constructor,
	Field,
	Variable,
	Class,
	Interface,
	Module,
	Property,
	Unit,
	Value,
	Enum,
	Keyword,
	Snippet,
	Text,
	Color,
	File,
	Reference,
	#[serde(rename = "customcolor")]
	CustomColor,
}

/// Represents a single disassembled instruction.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembledInstruction {
	/// The address of the instruction. Treated as a hex value if prefixed with `0x`, or as a
	/// decimal value otherwise.
	pub(crate) address:           String,
	/// Raw bytes representing the instruction and its operands, in an implementation-defined
	/// format.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) instruction_bytes: Option<String>,
	/// Text representing the instruction and its operands, in an implementation-defined format.
	pub(crate) instruction:       String,
	/// Name of the symbol that corresponds with the location of this instruction, if any.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) symbol:            Option<String>,
	/// Source location that corresponds to this instruction, if any.
	///
	/// Should always be set (if available) on the first instruction returned, but can be omitted
	/// afterwards if this instruction maps to the same source file as the previous instruction.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) location:          Option<Box<Source>>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	/// The line within the source location that corresponds to this instruction, if any.
	pub(crate) line:              Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	/// The column within the line that corresponds to this instruction, if any.
	pub(crate) column:            Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	/// The end line of the range that corresponds to this instruction, if any.
	pub(crate) end_line:          Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	/// The end column of the range that corresponds to this instruction, if any.
	pub(crate) end_column:        Option<u64>,
	/// A hint for how to present the instruction in the UI.
	///
	/// A value of `invalid` may be used to indicate this instruction is 'filler' and cannot be
	/// reached by the program. For example, unreadable memory addresses may be presented is
	/// 'invalid.'
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) presentation_hint: Option<InstructionPresentationHint>,
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum InstructionPresentationHint {
	Normal,
	Invalid,
}

/// A `GotoTarget` describes a code location that can be used as a target in the `goto`
/// request.
///
/// The possible goto targets can be determined via the `gotoTargets` request.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GotoTarget {
	/// Unique identifier for a goto target. This is used in the `goto` request.
	pub(crate) id: i32,
	/// The name of the goto target (shown in the UI).
	pub(crate) label: String,
	/// The line of the goto target.
	pub(crate) line: u64,
	/// The column of the goto target.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column: Option<u64>,
	/// The end line of the range covered by the goto target.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line: Option<u64>,
	/// The end column of the range covered by the goto target.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column: Option<u64>,
	/// A memory reference for the instruction pointer value represented by this target."
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) instruction_pointer_reference: Option<String>,
}

/// Logical areas that can be invalidated by the `invalidated` event.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum InvalidatedAreas {
	/// All previously fetched data has become invalid and needs to be refetched.
	All,
	/// Previously fetched stack related data has become invalid and needs to be refetched.
	Stacks,
	/// Previously fetched thread related data has become invalid and needs to be refetched.
	Threads,
	/// Previously fetched variable data has become invalid and needs to be refetched.
	Variables,
}

/// A structured message object. Used to return errors from requests.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
	/// Unique (within a debug adapter implementation) identifier for the message.
	///
	/// The purpose of these error IDs is to help extension authors that have the requirement that
	/// every user visible error message needs a corresponding error number, so that users or
	/// customer support can find information about the specific error more easily.
	pub(crate) id:             i32,
	/// A format string for the message. Embedded variables have the form `{name}`.
	///
	/// If variable name starts with an underscore character, the variable does not contain user
	/// data (PII) and can be safely used for telemetry purposes.
	pub(crate) format:         String,
	/// An object used as a dictionary for looking up the variables in the format string.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) variables:      Option<HashMap<String, String>>,
	/// If true send to telemetry.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) send_telemetry: Option<bool>,
	/// If true show user.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) show_user:      Option<bool>,
	/// A url where additional information about this message can be found.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) url:            Option<Uri>,
	/// A label that is presented to the user as the UI for opening the url.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) url_label:      Option<String>,
}

/// A Module object represents a row in the modules view.
///
/// The `id` attribute identifies a module in the modules view and is used in a `module` event for
/// identifying a module for adding, updating or deleting.
///
/// The `name` attribute is used to minimally render the module in the UI.
///
/// Additional attributes can be added to the module. They show up in the module view if they have a
/// corresponding `ColumnDescriptor`.
///
/// To avoid an unnecessary proliferation of additional attributes with similar semantics but
/// different names, we recommend to re-use attributes from the 'recommended' list below first, and
/// only introduce new attributes if nothing appropriate could be found.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Module {
	/// Unique identifier for the module.
	pub(crate) id:               IntegerOrString,
	/// A name of the module.
	pub(crate) name:             String,
	/// Logical full path to the module. The exact definition is implementation defined, but
	/// usually this would be a full path to the on-disk file for the module.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) path:             Option<String>,
	/// True if the module is optimized.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) is_optimized:     Option<bool>,
	/// True if the module is considered 'user code' by a debugger that supports 'Just My Code'.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) is_user_code:     Option<bool>,
	/// Version of Module.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) version:          Option<String>,
	/// User-understandable description of if symbols were found for the module (ex: 'Symbols
	/// Loaded', 'Symbols not found', etc.)
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) symbol_status:    Option<String>,
	/// Logical full path to the symbol file. The exact definition is implementation defined.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) symbol_file_path: Option<String>,
	/// Module created or modified, encoded as a RFC 3339 timestamp.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) date_time_stamp:  Option<String>,
	/// Address range covered by this module.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) address_range:    Option<String>,
}

/// A `Scope` is a named container for variables. Optionally a scope can map to a source or a range
/// within a source.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
	/// Name of the scope such as 'Arguments', 'Locals', or 'Registers'. This string is shown in
	/// the UI as is and can be translated.
	pub(crate) name:                String,
	/// A hint for how to present this scope in the UI. If this attribute is missing, the scope is
	/// shown with a generic UI.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) presentation_hint:   Option<ScopePresentationHint>,
	/// The variables of this scope can be retrieved by passing the value of `variablesReference`
	/// to the `variables` request as long as execution remains suspended.
	pub(crate) variables_reference: u32,
	/// The number of named variables in this scope.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) named_variables:     Option<u64>,
	/// The number of indexed variables in this scope.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) indexed_variables:   Option<u64>,
	/// If true, the number of variables in this scope is large or expensive to retrieve.
	pub(crate) expensive:           bool,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	/// The source for this scope.
	pub(crate) source:              Option<Box<Source>>,
	/// The start line of the range covered by this scope.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line:                Option<u64>,
	/// Start position of the range covered by the scope.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:              Option<u64>,
	/// The end line of the range covered by this scope.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line:            Option<u64>,
	/// End position of the range covered by the scope.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column:          Option<u64>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ScopePresentationHint {
	/// Scope contains method arguments.
	Arguments,
	/// Scope contains local variables.
	Locals,
	/// Scope contains registers. Only a single `registers` scope should be returned from a `scopes`
	/// request.
	Registers,
	/// Scope contains one or more return values.
	ReturnValue,
	#[serde(untagged)]
	Other(String),
}

/// A descriptor for source code.
///
/// It is returned from the debug adapter as part of a [`StackFrame`] and it is used by clients when
/// specifying breakpoints.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
	/// The short name of the source. Every source returned from the debug adapter has a
	/// name.
	///
	/// When sending a source to the debug adapter this name is optional.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) name:              Option<String>,
	/// The path of the source to be shown in the UI.
	///
	/// It is only used to locate and load the  content of the source if no `sourceReference` is
	/// specified (or its value is 0).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) path:              Option<String>,
	/// If the value > 0 the contents of the source must be retrieved through the `source`
	/// request (even if a path is specified).
	///
	/// Since a `sourceReference` is only valid for a session, it can not be used to persist a
	/// source.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source_reference:  Option<i32>,
	/// A hint for how to present the source in the UI.
	///
	/// A value of `deemphasize` can be used to indicate that the source is not available or that
	/// it is skipped on stepping.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) presentation_hint: Option<SourcePresentationHint>,
	/// The origin of this source. For example, 'internal module', 'inlined content from source
	/// map', etc.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) origin:            Option<String>,
	/// A list of sources that are related to this source. These may be the source that generated
	/// this source.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) sources:           Option<Vec<Self>>,
	/// Additional data that a debug adapter might want to loop through the client.
	///
	/// The client should leave the data intact and persist it across sessions.
	///
	/// The client should not interpret the data.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) adapter_data:      Option<serde_json::Value>,
	/// The checksums associated with this file.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) checksums:         Option<Vec<Checksum>>,
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SourcePresentationHint {
	Normal,
	Emphasize,
	Deemphasize,
}

/// A Stackframe contains the source location.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackFrame {
	/// An identifier for the stack frame. It must be unique across all threads.
	///
	/// This id can be used to retrieve the scopes of the frame with the `scopes` request or to
	/// restart the execution of a stack frame.
	pub(crate) id: i32,
	/// The name of the stack frame, typically a method name.
	pub(crate) name: String,
	/// The source of the frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) source: Option<Box<Source>>,
	/// The line within the source of the frame.
	///
	/// If the source attribute is missing or doesn't exist, `line` is 0 and should be ignored by
	/// the client.
	pub(crate) line: u64,
	/// Start position of the range covered by the stack frame.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	///
	/// If attribute `source` is missing or doesn't exist, `column` is 0 and should be ignored by
	/// the client.
	pub(crate) column: u64,
	/// The end line of the range covered by the stack frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line: Option<u64>,
	/// End position of the range covered by the stack frame.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column: Option<u64>,
	/// Indicates whether this frame can be restarted with the `restartFrame` request.
	///
	/// Clients should only use this if the debug adapter supports the `restart` request and the
	/// corresponding capability `supportsRestartFrame` is true. If a debug adapter has this
	/// capability, then `canRestart` defaults to `true` if the property is absent.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) can_restart: Option<bool>,
	/// A memory reference for the current instruction pointer in this frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) instruction_pointer_reference: Option<String>,
	/// The module associated with this frame, if any.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) module_id: Option<IntegerOrString>,
	/// A hint for how to present this frame in the UI.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) presentation_hint: Option<StackFramePresentationHint>,
}

/// A hint for how to present this frame in the UI.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum StackFramePresentationHint {
	Normal,
	/// Can be used to indicate that the frame is an artificial frame that is
	/// used as a visual label or separator.
	Label,
	Subtle,
}

/// Provides formatting information for a stack frame.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StackFrameFormat {
	#[serde(flatten)]
	pub(crate) variable_format:  ValueFormat,
	/// Displays parameters for the stack frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) parameters:       Option<bool>,
	/// Displays the types of parameters for the stack frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) parameter_types:  Option<bool>,
	/// Displays the names of parameters for the stack frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) parameter_names:  Option<bool>,
	/// Displays the values of parameters for the stack frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) parameter_values: Option<bool>,
	/// Displays the line number of the stack frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line:             Option<bool>,
	/// Displays the module of the stack frame.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) module:           Option<bool>,
	/// Includes all stack frames, including those the debug adapter might otherwise hide.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) include_all:      Option<bool>,
}

/// A `StepInTarget` can be used in the `stepIn` request and determines into which single target the
/// `stepIn` request should step.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StepInTarget {
	/// Unique identifier for a step-in target.
	pub(crate) id:         i32,
	/// The name of the step-in target (shown in the UI).
	pub(crate) label:      String,
	/// The line of the step-in target.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) line:       Option<u64>,
	/// Start position of the range covered by the step in target.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) column:     Option<u64>,
	/// The end line of the range covered by the step-in target.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_line:   Option<u64>,
	/// End position of the range covered by the step in target.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) end_column: Option<u64>,
}

/// The granularity of one 'step' in the stepping requests `next`, `stepIn`, `stepOut`, and
/// `stepBack`.
#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SteppingGranularity {
	/// The step should allow the program to run until the current statement has finished executing.
	///
	/// The meaning of a statement is determined by the adapter and it may be considered equivalent
	/// to a line.
	///
	/// For example `for(int i = 0; i > 10; i++)` could be considered to have 3 statements `int i =
	/// 0`, `i > 10`, and `i++`.
	Statement,
	/// The step should allow the program to run until the current source line has executed.
	Line,
	/// The step should allow one instruction to execute (e.g. one x86 instruction).
	Instruction,
}

/// A Thread
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
	/// Unique identifier for the thread.
	pub(crate) id:   i32,
	/// The name of the thread.
	pub(crate) name: String,
}

/// Provides formatting information for a value.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ValueFormat {
	/// Display the value in hex.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) hex: Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum EventReason {
	New,
	Changed,
	Removed,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ProcessStartMethod {
	/// Process was launched under the debugger.
	Launch,
	/// Debugger attached to an existing process.
	Attach,
	/// A project launcher component has launched a new process in a suspended state and then asked
	/// the debugger to attach.
	AttachForSuspendedLaunch,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum PathFormat {
	Path,
	Uri,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum VariablesFilter {
	Indexed,
	Named,
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum TerminalKind {
	Integrated,
	External,
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum StartDebuggingRequest {
	Launch,
	Attach,
}

#[derive(
	Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum OutputPresentation {
	Separate,
	MergeWithParent,
}

impl Checksum {
	pub fn new<T>(algorithm: ChecksumAlgorithm, checksum: T) -> Self
	where
		T: ToString,
	{
		Self { algorithm, checksum: checksum.to_string() }
	}

	/// The algorithm used to calculate this checksum.
	pub fn algorithm(&self) -> ChecksumAlgorithm {
		self.algorithm
	}

	/// Value of the checksum, encoded as a hexadecimal value.
	pub fn checksum(&self) -> &String {
		&self.checksum
	}
}

impl ColumnDescriptor {
	pub fn new<T, U>(attribute_name: T, label: U) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			attribute_name: attribute_name.to_string(),
			label:          label.to_string(),
			format:         None,
			column_type:    None,
			width:          None,
		}
	}

	pub fn with_format<T>(self, format: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.format = Some(format.to_string());
		this
	}

	pub fn with_column_type(self, column_type: ColumnType) -> Self {
		let mut this = self;
		this.column_type = Some(column_type);
		this
	}

	pub fn with_width(self, width: u32) -> Self {
		let mut this = self;
		this.width = Some(width);
		this
	}

	/// Name of the attribute rendered in this column.
	pub fn attribute_name(&self) -> &String {
		&self.attribute_name
	}

	/// Header UI label of column.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// Format to use for the rendered values in this column.
	///
	/// TBD how the format strings looks like.
	pub fn format(&self) -> Option<&String> {
		self.format.as_ref()
	}

	/// Datatype of values in this column. Defaults to [`ColumnType::String`] if not specified.
	pub fn column_type(&self) -> Option<ColumnType> {
		self.column_type
	}

	/// Width of this column in characters (hint only).
	pub fn width(&self) -> Option<u32> {
		self.width
	}
}

impl CompletionItem {
	pub fn new<T>(label: T) -> Self
	where
		T: ToString,
	{
		Self {
			label:            label.to_string(),
			text:             None,
			sort_text:        None,
			detail:           None,
			completion_type:  None,
			start:            None,
			length:           None,
			selection_start:  None,
			selection_length: None,
		}
	}

	pub fn with_text<T>(self, text: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.text = Some(text.to_string());
		this
	}

	pub fn with_sort_text<T>(self, sort_text: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.sort_text = Some(sort_text.to_string());
		this
	}

	pub fn with_detail<T>(self, detail: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.detail = Some(detail.to_string());
		this
	}

	pub fn with_completion_type(self, completion_type: CompletionItemType) -> Self {
		let mut this = self;
		this.completion_type = Some(completion_type);
		this
	}

	pub fn with_start(self, start: u32) -> Self {
		let mut this = self;
		this.start = Some(start);
		this
	}

	pub fn with_length(self, length: u32) -> Self {
		let mut this = self;
		this.length = Some(length);
		this
	}

	pub fn with_selection_start(self, selection_start: u32) -> Self {
		let mut this = self;
		this.selection_start = Some(selection_start);
		this
	}

	pub fn with_selection_length(self, selection_length: u32) -> Self {
		let mut this = self;
		this.selection_length = Some(selection_length);
		this
	}

	/// The label of this completion item.
	///
	/// By default this is also the text that is inserted when selecting this completion.
	pub fn label(&self) -> &String {
		&self.label
	}

	/// If text is returned and not an empty string, then it is inserted instead of the label.
	pub fn text(&self) -> Option<&String> {
		self.text.as_ref()
	}

	/// A string that should be used when comparing this item with other items.
	///
	/// If not returned or an empty string, the `label` is used instead.
	pub fn sort_text(&self) -> Option<&String> {
		self.sort_text.as_ref()
	}

	/// A human-readable string with additional information about this item, like type or symbol
	/// information.
	pub fn detail(&self) -> Option<&String> {
		self.detail.as_ref()
	}

	/// The item's type.
	///
	/// Typically the client uses this information to render the item in the UI with an icon.
	pub fn completion_type(&self) -> Option<CompletionItemType> {
		self.completion_type
	}

	/// Start position (within the `text` attribute of the `completions` request) where the
	/// completion text is added.
	///
	/// The position is measured in UTF-16 code units and the client
	/// capability `columnsStartAt1` determines whether it is 0- or 1-based.
	///
	/// If the start position is omitted the text is added at the location specified by the
	/// `column` attribute of the `completions` request.
	pub fn start(&self) -> Option<u32> {
		self.start
	}

	/// Length determines how many characters are overwritten by the completion text and it is
	/// measured in UTF-16 code units.
	///
	/// If missing the value 0 is assumed which results in the completion text being inserted.
	pub fn length(&self) -> Option<u32> {
		self.length
	}

	/// Determines the start of the new selection after the text has been inserted (or replaced).
	///
	/// [`CompletionItem::selection_start`] is measured in UTF-16 code units and must be in the
	/// range 0 and length of the completion text.
	///
	/// If omitted the selection starts at the end of the completion text.
	pub fn selection_start(&self) -> Option<u32> {
		self.selection_start
	}

	/// Determines the length of the new selection after the text has been inserted (or replaced)
	/// and it is measured in UTF-16 code units.
	///
	/// The selection can not extend beyond the bounds of the completion text.
	///
	/// If omitted the length is assumed to be 0.
	pub fn selection_length(&self) -> Option<u32> {
		self.selection_length
	}
}

impl DisassembledInstruction {
	pub fn new<T, U>(address: T, instruction: U) -> Self
	where
		T: ToString,
		U: ToString,
	{
		Self {
			address:           address.to_string(),
			instruction_bytes: None,
			instruction:       instruction.to_string(),
			symbol:            None,
			location:          None,
			line:              None,
			column:            None,
			end_line:          None,
			end_column:        None,
			presentation_hint: None,
		}
	}

	pub fn with_instruction_bytes<T>(self, instruction_bytes: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.instruction_bytes = Some(instruction_bytes.to_string());
		this
	}

	pub fn with_symbol<T>(self, symbol: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.symbol = Some(symbol.to_string());
		this
	}

	pub fn with_location(self, location: Box<Source>) -> Self {
		let mut this = self;
		this.location = Some(location);
		this
	}

	pub fn with_line(self, line: u64) -> Self {
		let mut this = self;
		this.line = Some(line);
		this
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	pub fn with_end_line(self, end_line: u64) -> Self {
		let mut this = self;
		this.end_line = Some(end_line);
		this
	}

	pub fn with_end_column(self, end_column: u64) -> Self {
		let mut this = self;
		this.end_column = Some(end_column);
		this
	}

	pub fn with_presentation_hint(self, presentation_hint: InstructionPresentationHint) -> Self {
		let mut this = self;
		this.presentation_hint = Some(presentation_hint);
		this
	}

	/// The address of the instruction. Treated as a hex value if prefixed with `0x`, or as a
	/// decimal value otherwise.
	pub fn address(&self) -> &String {
		&self.address
	}

	/// Raw bytes representing the instruction and its operands, in an implementation-defined
	/// format.
	pub fn instruction_bytes(&self) -> Option<&String> {
		self.instruction_bytes.as_ref()
	}

	/// Text representing the instruction and its operands, in an implementation-defined format.
	pub fn instruction(&self) -> &String {
		&self.instruction
	}

	/// Name of the symbol that corresponds with the location of this instruction, if any.
	pub fn symbol(&self) -> Option<&String> {
		self.symbol.as_ref()
	}

	/// Source location that corresponds to this instruction, if any.
	///
	/// Should always be set (if available) on the first instruction returned, but can be omitted
	/// afterwards if this instruction maps to the same source file as the previous instruction.
	pub fn location(&self) -> Option<&Source> {
		self.location.as_deref()
	}

	/// The line within the source location that corresponds to this instruction, if any.
	pub fn line(&self) -> Option<u64> {
		self.line
	}

	/// The column within the line that corresponds to this instruction, if any.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The end line of the range that corresponds to this instruction, if any.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// The end column of the range that corresponds to this instruction, if any.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}

	/// A hint for how to present the instruction in the UI.
	///
	/// A value of `invalid` may be used to indicate this instruction is 'filler' and cannot be
	/// reached by the program. For example, unreadable memory addresses may be presented is
	/// 'invalid.'
	pub fn presentation_hint(&self) -> Option<InstructionPresentationHint> {
		self.presentation_hint
	}
}

impl GotoTarget {
	pub fn new<T>(id: i32, label: T, line: u64) -> Self
	where
		T: ToString,
	{
		Self {
			id,
			label: label.to_string(),
			line,
			column: None,
			end_line: None,
			end_column: None,
			instruction_pointer_reference: None,
		}
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	pub fn with_end_line(self, end_line: u64) -> Self {
		let mut this = self;
		this.end_line = Some(end_line);
		this
	}

	pub fn with_end_column(self, end_column: u64) -> Self {
		let mut this = self;
		this.end_column = Some(end_column);
		this
	}

	pub fn with_instruction_pointer_reference<T>(self, instruction_pointer_reference: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.instruction_pointer_reference = Some(instruction_pointer_reference.to_string());
		this
	}

	/// Unique identifier for a goto target. This is used in the `goto` request.
	pub fn id(&self) -> i32 {
		self.id
	}

	/// The name of the goto target (shown in the UI).
	pub fn label(&self) -> &String {
		&self.label
	}

	/// The line of the goto target.
	pub fn line(&self) -> u64 {
		self.line
	}

	/// The column of the goto target.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The end line of the range covered by the goto target.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// The end column of the range covered by the goto target.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}

	/// A memory reference for the instruction pointer value represented by this target."
	pub fn instruction_pointer_reference(&self) -> Option<&String> {
		self.instruction_pointer_reference.as_ref()
	}
}

impl Message {
	pub fn new<T>(id: i32, format: T) -> Self
	where
		T: ToString,
	{
		Self {
			id,
			format: format.to_string(),
			variables: None,
			send_telemetry: None,
			show_user: None,
			url: None,
			url_label: None,
		}
	}

	pub fn with_variables(self, variables: Option<HashMap<String, String>>) -> Self {
		let mut this = self;
		this.variables = Some(variables.unwrap_or_default());
		this
	}

	pub fn with_send_telemetry(self, send_telemetry: bool) -> Self {
		let mut this = self;
		this.send_telemetry = Some(send_telemetry);
		this
	}

	pub fn with_show_user(self, show_user: bool) -> Self {
		let mut this = self;
		this.show_user = Some(show_user);
		this
	}

	pub fn with_url(self, url: Uri) -> Self {
		let mut this = self;
		this.url = Some(url);
		this
	}

	pub fn with_url_label<T>(self, url_label: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.url_label = Some(url_label.to_string());
		this
	}

	/// Unique (within a debug adapter implementation) identifier for the message.
	///
	/// The purpose of these error IDs is to help extension authors that have the requirement that
	/// every user visible error message needs a corresponding error number, so that users or
	/// customer support can find information about the specific error more easily.
	pub fn id(&self) -> i32 {
		self.id
	}

	/// A format string for the message. Embedded variables have the form `{name}`.
	///
	/// If variable name starts with an underscore character, the variable does not contain user
	/// data (PII) and can be safely used for telemetry purposes.
	pub fn format(&self) -> &String {
		&self.format
	}

	/// An object used as a dictionary for looking up the variables in the format string.
	pub fn variables(&mut self) -> Option<&mut HashMap<String, String>> {
		self.variables.as_mut()
	}

	/// If true send to telemetry.
	pub fn send_telemetry(&self) -> Option<bool> {
		self.send_telemetry
	}

	/// If true show user.
	pub fn show_user(&self) -> Option<bool> {
		self.show_user
	}

	/// A url where additional information about this message can be found.
	pub fn url(&self) -> Option<&Uri> {
		self.url.as_ref()
	}

	/// A label that is presented to the user as the UI for opening the url.
	pub fn url_label(&self) -> Option<&String> {
		self.url_label.as_ref()
	}
}

impl Module {
	pub fn new<T>(id: IntegerOrString, name: T) -> Self
	where
		T: ToString,
	{
		Self {
			id,
			name: name.to_string(),
			path: None,
			is_optimized: None,
			is_user_code: None,
			version: None,
			symbol_status: None,
			symbol_file_path: None,
			date_time_stamp: None,
			address_range: None,
		}
	}

	pub fn with_path<T>(self, path: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.path = Some(path.to_string());
		this
	}

	pub fn with_is_optimized(self, is_optimized: bool) -> Self {
		let mut this = self;
		this.is_optimized = Some(is_optimized);
		this
	}

	pub fn with_is_user_code(self, is_user_code: bool) -> Self {
		let mut this = self;
		this.is_user_code = Some(is_user_code);
		this
	}

	pub fn with_version<T>(self, version: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.version = Some(version.to_string());
		this
	}

	pub fn with_symbol_status<T>(self, symbol_status: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.symbol_status = Some(symbol_status.to_string());
		this
	}

	pub fn with_symbol_file_path<T>(self, symbol_file_path: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.symbol_file_path = Some(symbol_file_path.to_string());
		this
	}

	pub fn with_date_time_stamp<Tz>(self, date_time_stamp: chrono::DateTime<Tz>) -> Self
	where
		Tz: chrono::TimeZone,
	{
		let mut this = self;
		this.date_time_stamp = Some(date_time_stamp.to_rfc3339());
		this
	}

	pub fn with_address_range<T>(self, address_range: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.address_range = Some(address_range.to_string());
		this
	}

	/// Unique identifier for the module.
	pub fn id(&self) -> &IntegerOrString {
		&self.id
	}

	/// A name of the module.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// Logical full path to the module. The exact definition is implementation defined, but
	/// usually this would be a full path to the on-disk file for the module.
	pub fn path(&self) -> Option<&String> {
		self.path.as_ref()
	}

	/// True if the module is optimized.
	pub fn is_optimized(&self) -> Option<bool> {
		self.is_optimized
	}

	/// True if the module is considered 'user code' by a debugger that supports 'Just My Code'.
	pub fn is_user_code(&self) -> Option<bool> {
		self.is_user_code
	}

	/// Version of Module.
	pub fn version(&self) -> Option<&String> {
		self.version.as_ref()
	}

	/// User-understandable description of if symbols were found for the module (ex: 'Symbols
	/// Loaded', 'Symbols not found', etc.)
	pub fn symbol_status(&self) -> Option<&String> {
		self.symbol_status.as_ref()
	}

	/// Logical full path to the symbol file. The exact definition is implementation defined.
	pub fn symbol_file_path(&self) -> Option<&String> {
		self.symbol_file_path.as_ref()
	}

	/// Module created or modified, encoded as a RFC 3339 timestamp.
	pub fn date_time_stamp(&self) -> Option<&String> {
		self.date_time_stamp.as_ref()
	}

	/// Address range covered by this module.
	pub fn address_range(&self) -> Option<&String> {
		self.address_range.as_ref()
	}
}

impl Scope {
	pub fn new<T>(name: T, variables_reference: u32, expensive: bool) -> Self
	where
		T: ToString,
	{
		Self {
			name: name.to_string(),
			presentation_hint: None,
			variables_reference,
			named_variables: None,
			indexed_variables: None,
			expensive,
			source: None,
			line: None,
			column: None,
			end_line: None,
			end_column: None,
		}
	}

	pub fn with_presentation_hint(self, presentation_hint: ScopePresentationHint) -> Self {
		let mut this = self;
		this.presentation_hint = Some(presentation_hint);
		this
	}

	pub fn with_named_variables(self, named_variables: u64) -> Self {
		let mut this = self;
		this.named_variables = Some(named_variables);
		this
	}

	pub fn with_indexed_variables(self, indexed_variables: u64) -> Self {
		let mut this = self;
		this.indexed_variables = Some(indexed_variables);
		this
	}

	pub fn with_source(self, source: Box<Source>) -> Self {
		let mut this = self;
		this.source = Some(source);
		this
	}

	pub fn with_line(self, line: u64) -> Self {
		let mut this = self;
		this.line = Some(line);
		this
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	pub fn with_end_line(self, end_line: u64) -> Self {
		let mut this = self;
		this.end_line = Some(end_line);
		this
	}

	pub fn with_end_column(self, end_column: u64) -> Self {
		let mut this = self;
		this.end_column = Some(end_column);
		this
	}

	/// Name of the scope such as 'Arguments', 'Locals', or 'Registers'. This string is shown in
	/// the UI as is and can be translated.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// A hint for how to present this scope in the UI. If this attribute is missing, the scope is
	/// shown with a generic UI.
	pub fn presentation_hint(&self) -> Option<&ScopePresentationHint> {
		self.presentation_hint.as_ref()
	}

	/// The variables of this scope can be retrieved by passing the value of `variablesReference`
	/// to the `variables` request as long as execution remains suspended.
	pub fn variables_reference(&self) -> u32 {
		self.variables_reference
	}

	/// The number of named variables in this scope.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	pub fn named_variables(&self) -> Option<u64> {
		self.named_variables
	}

	/// The number of indexed variables in this scope.
	///
	/// The client can use this information to present the variables in a paged UI and fetch them
	/// in chunks.
	pub fn indexed_variables(&self) -> Option<u64> {
		self.indexed_variables
	}

	/// If true, the number of variables in this scope is large or expensive to retrieve.
	pub fn expensive(&self) -> bool {
		self.expensive
	}

	/// The source for this scope.
	pub fn source(&self) -> Option<&Source> {
		self.source.as_deref()
	}

	/// The start line of the range covered by this scope.
	pub fn line(&self) -> Option<u64> {
		self.line
	}

	/// Start position of the range covered by the scope.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The end line of the range covered by this scope.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// End position of the range covered by the scope.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}
}

impl Default for Source {
	fn default() -> Self {
		Self::new()
	}
}

impl Source {
	pub fn new() -> Self {
		Self {
			name:              None,
			path:              None,
			source_reference:  None,
			presentation_hint: None,
			origin:            None,
			sources:           None,
			adapter_data:      None,
			checksums:         None,
		}
	}

	pub fn with_name<T>(self, name: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.name = Some(name.to_string());
		this
	}

	pub fn with_path<T>(self, path: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.path = Some(path.to_string());
		this
	}

	pub fn with_source_reference(self, source_reference: i32) -> Self {
		let mut this = self;
		this.source_reference = Some(source_reference);
		this
	}

	pub fn with_presentation_hint(self, presentation_hint: SourcePresentationHint) -> Self {
		let mut this = self;
		this.presentation_hint = Some(presentation_hint);
		this
	}

	pub fn with_origin<T>(self, origin: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.origin = Some(origin.to_string());
		this
	}

	pub fn with_sources(self, sources: Vec<Self>) -> Self {
		let mut this = self;
		this.sources = Some(sources);
		this
	}

	pub fn with_adapter_data(self, adapter_data: serde_json::Value) -> Self {
		let mut this = self;
		this.adapter_data = Some(adapter_data);
		this
	}

	pub fn with_checksums(self, checksums: Vec<Checksum>) -> Self {
		let mut this = self;
		this.checksums = Some(checksums);
		this
	}

	/// The short name of the source. Every source returned from the debug adapter has a
	/// name.
	///
	/// When sending a source to the debug adapter this name is optional.
	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	/// The path of the source to be shown in the UI.
	///
	/// It is only used to locate and load the  content of the source if no `sourceReference` is
	/// specified (or its value is 0).
	pub fn path(&self) -> Option<&String> {
		self.path.as_ref()
	}

	/// If the value > 0 the contents of the source must be retrieved through the `source`
	/// request (even if a path is specified).
	///
	/// Since a `sourceReference` is only valid for a session, it can not be used to persist a
	/// source.
	///
	/// The value should be less than or equal to 2147483647 (2^31-1).
	pub fn source_reference(&self) -> Option<i32> {
		self.source_reference
	}

	/// A hint for how to present the source in the UI.
	///
	/// A value of `deemphasize` can be used to indicate that the source is not available or that
	/// it is skipped on stepping.
	pub fn presentation_hint(&self) -> Option<SourcePresentationHint> {
		self.presentation_hint
	}

	/// The origin of this source. For example, 'internal module', 'inlined content from source
	/// map', etc.
	pub fn origin(&self) -> Option<&String> {
		self.origin.as_ref()
	}

	/// A list of sources that are related to this source. These may be the source that generated
	/// this source.
	pub fn sources(&self) -> Option<&Vec<Self>> {
		self.sources.as_ref()
	}

	/// Additional data that a debug adapter might want to loop through the client.
	///
	/// The client should leave the data intact and persist it across sessions.
	///
	/// The client should not interpret the data.
	pub fn adapter_data(&self) -> Option<&serde_json::Value> {
		self.adapter_data.as_ref()
	}

	/// The checksums associated with this file.
	pub fn checksums(&self) -> Option<&Vec<Checksum>> {
		self.checksums.as_ref()
	}
}

impl StackFrame {
	pub fn new<T>(id: i32, name: T, line: u64, column: u64) -> Self
	where
		T: ToString,
	{
		Self {
			id,
			name: name.to_string(),
			source: None,
			line,
			column,
			end_line: None,
			end_column: None,
			can_restart: None,
			instruction_pointer_reference: None,
			module_id: None,
			presentation_hint: None,
		}
	}

	pub fn with_source(self, source: Box<Source>) -> Self {
		let mut this = self;
		this.source = Some(source);
		this
	}

	pub fn with_end_line(self, end_line: u64) -> Self {
		let mut this = self;
		this.end_line = Some(end_line);
		this
	}

	pub fn with_end_column(self, end_column: u64) -> Self {
		let mut this = self;
		this.end_column = Some(end_column);
		this
	}

	pub fn with_can_restart(self, can_restart: bool) -> Self {
		let mut this = self;
		this.can_restart = Some(can_restart);
		this
	}

	pub fn with_instruction_pointer_reference<T>(self, instruction_pointer_reference: T) -> Self
	where
		T: ToString,
	{
		let mut this = self;
		this.instruction_pointer_reference = Some(instruction_pointer_reference.to_string());
		this
	}

	pub fn with_module_id(self, module_id: IntegerOrString) -> Self {
		let mut this = self;
		this.module_id = Some(module_id);
		this
	}

	pub fn with_presentation_hint(self, presentation_hint: StackFramePresentationHint) -> Self {
		let mut this = self;
		this.presentation_hint = Some(presentation_hint);
		this
	}

	/// An identifier for the stack frame. It must be unique across all threads.
	///
	/// This id can be used to retrieve the scopes of the frame with the `scopes` request or to
	/// restart the execution of a stack frame.
	pub fn id(&self) -> i32 {
		self.id
	}

	/// The name of the stack frame, typically a method name.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The source of the frame.
	pub fn source(&self) -> Option<&Source> {
		self.source.as_deref()
	}

	/// The line within the source of the frame.
	///
	/// If the source attribute is missing or doesn't exist, `line` is 0 and should be ignored by
	/// the client.
	pub fn line(&self) -> u64 {
		self.line
	}

	/// Start position of the range covered by the stack frame.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	///
	/// If attribute `source` is missing or doesn't exist, `column` is 0 and should be ignored by
	/// the client.
	pub fn column(&self) -> u64 {
		self.column
	}

	/// The end line of the range covered by the stack frame.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// End position of the range covered by the stack frame.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}

	/// Indicates whether this frame can be restarted with the `restartFrame` request.
	///
	/// Clients should only use this if the debug adapter supports the `restart` request and the
	/// corresponding capability `supportsRestartFrame` is true. If a debug adapter has this
	/// capability, then `canRestart` defaults to `true` if the property is absent.
	pub fn can_restart(&self) -> Option<bool> {
		self.can_restart
	}

	/// A memory reference for the current instruction pointer in this frame.
	pub fn instruction_pointer_reference(&self) -> Option<&String> {
		self.instruction_pointer_reference.as_ref()
	}

	/// The module associated with this frame, if any.
	pub fn module_id(&self) -> Option<&IntegerOrString> {
		self.module_id.as_ref()
	}

	/// A hint for how to present this frame in the UI.
	pub fn presentation_hint(&self) -> Option<StackFramePresentationHint> {
		self.presentation_hint
	}
}

impl StackFrameFormat {
	pub fn new(variable_format: ValueFormat) -> Self {
		Self {
			variable_format,
			parameters: None,
			parameter_types: None,
			parameter_names: None,
			parameter_values: None,
			line: None,
			module: None,
			include_all: None,
		}
	}

	pub fn with_parameters(self, parameters: bool) -> Self {
		let mut this = self;
		this.parameters = Some(parameters);
		this
	}

	pub fn with_parameter_types(self, parameter_types: bool) -> Self {
		let mut this = self;
		this.parameter_types = Some(parameter_types);
		this
	}

	pub fn with_parameter_names(self, parameter_names: bool) -> Self {
		let mut this = self;
		this.parameter_names = Some(parameter_names);
		this
	}

	pub fn with_parameter_values(self, parameter_values: bool) -> Self {
		let mut this = self;
		this.parameter_values = Some(parameter_values);
		this
	}

	pub fn with_line(self, line: bool) -> Self {
		let mut this = self;
		this.line = Some(line);
		this
	}

	pub fn with_module(self, module: bool) -> Self {
		let mut this = self;
		this.module = Some(module);
		this
	}

	pub fn with_include_all(self, include_all: bool) -> Self {
		let mut this = self;
		this.include_all = Some(include_all);
		this
	}

	pub fn variable_format(&self) -> &ValueFormat {
		&self.variable_format
	}

	/// Displays parameters for the stack frame.
	pub fn parameters(&self) -> Option<bool> {
		self.parameters
	}

	/// Displays the types of parameters for the stack frame.
	pub fn parameter_types(&self) -> Option<bool> {
		self.parameter_types
	}

	/// Displays the names of parameters for the stack frame.
	pub fn parameter_names(&self) -> Option<bool> {
		self.parameter_names
	}

	/// Displays the values of parameters for the stack frame.
	pub fn parameter_values(&self) -> Option<bool> {
		self.parameter_values
	}

	/// Displays the line number of the stack frame.
	pub fn line(&self) -> Option<bool> {
		self.line
	}

	/// Displays the module of the stack frame.
	pub fn module(&self) -> Option<bool> {
		self.module
	}

	/// Includes all stack frames, including those the debug adapter might otherwise hide.
	pub fn include_all(&self) -> Option<bool> {
		self.include_all
	}
}

impl StepInTarget {
	pub fn new<T>(id: i32, label: T) -> Self
	where
		T: ToString,
	{
		Self {
			id,
			label: label.to_string(),
			line: None,
			column: None,
			end_line: None,
			end_column: None,
		}
	}

	pub fn with_line(self, line: u64) -> Self {
		let mut this = self;
		this.line = Some(line);
		this
	}

	pub fn with_column(self, column: u64) -> Self {
		let mut this = self;
		this.column = Some(column);
		this
	}

	pub fn with_end_line(self, end_line: u64) -> Self {
		let mut this = self;
		this.end_line = Some(end_line);
		this
	}

	pub fn with_end_column(self, end_column: u64) -> Self {
		let mut this = self;
		this.end_column = Some(end_column);
		this
	}

	/// Unique identifier for a step-in target.
	pub fn id(&self) -> i32 {
		self.id
	}

	/// The name of the step-in target (shown in the UI).
	pub fn label(&self) -> &String {
		&self.label
	}

	/// The line of the step-in target.
	pub fn line(&self) -> Option<u64> {
		self.line
	}

	/// Start position of the range covered by the step in target.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn column(&self) -> Option<u64> {
		self.column
	}

	/// The end line of the range covered by the step-in target.
	pub fn end_line(&self) -> Option<u64> {
		self.end_line
	}

	/// End position of the range covered by the step in target.
	///
	/// It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines
	/// whether it is 0- or 1-based.
	pub fn end_column(&self) -> Option<u64> {
		self.end_column
	}
}

impl Thread {
	pub fn new<T>(id: i32, name: T) -> Self
	where
		T: ToString,
	{
		Self { id, name: name.to_string() }
	}

	/// Unique identifier for the thread.
	pub fn id(&self) -> i32 {
		self.id
	}

	/// The name of the thread.
	pub fn name(&self) -> &String {
		&self.name
	}
}

impl Default for ValueFormat {
	fn default() -> Self {
		Self::new()
	}
}

impl ValueFormat {
	pub fn new() -> Self {
		Self { hex: None }
	}

	pub fn with_hex(self, hex: bool) -> Self {
		let mut this = self;
		this.hex = Some(hex);
		this
	}

	/// Display the value in hex.
	pub fn hex(&self) -> bool {
		self.hex.unwrap_or(false)
	}
}
