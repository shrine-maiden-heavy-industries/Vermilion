// SPDX-License-Identifier: BSD-3-Clause

/// A Variable is a name/value pair.
///
/// The `type` attribute is shown if space permits or when hovering over the variable's name.
///
/// The `kind` attribute is used to render additional properties of the variable, e.g. different
/// icons can be used to indicate that a variable is public or private.
///
/// If the value is structured (has children), a handle is provided to retrieve the children with
/// the `variables` request.
///
/// If the number of named or indexed children is large, the numbers should be returned via the
/// `namedVariables` and `indexedVariables` attributes.
///
/// The client can use this information to present the children in a paged UI and fetch them in
/// chunks.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
	/// The variable's name.
	pub(crate) name: String,
	/// The variable's value.
	///
	/// This can be a multi-line text, e.g. for a function the body of a function.
	///
	/// For structured variables (which do not have a simple value), it is recommended to provide a
	/// one-line representation of the structured object. This helps to identify the structured
	/// object in the collapsed state when its children are not yet visible.
	///
	/// An empty string can be used if no value should be shown in the UI.
	pub(crate) value: String,
	/// The type of the variable's value. Typically shown in the UI when hovering over the
	/// value.
	///
	/// This attribute should only be returned by a debug adapter if the corresponding capability
	/// `supportsVariableType` is true.
	#[serde(skip_serializing_if = "Option::is_none", default, rename = "type")]
	pub(crate) variable_type: Option<String>,
	/// Properties of a variable that can be used to determine how to render the variable in the
	/// UI.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) presentation_hint: Option<VariablePresentationHint>,
	/// The evaluatable name of this variable which can be passed to the `evaluate` request to
	/// fetch the variable's value.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) evaluate_name: Option<String>,
	/// If `variablesReference` is > 0, the variable is structured and its children can be
	/// retrieved by passing `variablesReference` to the `variables` request as long as execution
	/// remains suspended.
	pub(crate) variables_reference: u32,
	/// The number of named child variables.
	///
	/// The client can use this information to present the children in a paged UI and fetch them in
	/// chunks.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) named_variables: Option<i32>,
	/// The number of indexed child variables.
	///
	/// The client can use this information to present the children in a paged UI and fetch them in
	/// chunks.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) indexed_variables: Option<i32>,
	/// A memory reference associated with this variable.
	///
	/// For pointer type variables, this is generally a reference to the memory address contained
	/// in the pointer.
	///
	/// For executable data, this reference may later be used in a `disassemble` request.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) memory_reference: Option<String>,
	/// A reference that allows the client to request the location where the variable is declared.
	/// This should be present only if the adapter is likely to be able to resolve the
	/// location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) declaration_location_reference: Option<i32>,
	/// A reference that allows the client to request the location where the variable's value is
	/// declared. For example, if the variable contains a function pointer, the adapter may be able
	/// to look up the function's location. This should be present only if the adapter is likely to
	/// be able to resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) value_location_reference: Option<i32>,
}

/// Properties of a variable that can be used to determine how to render the variable in the UI.
#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct VariablePresentationHint {
	/// The kind of variable.
	///
	/// Before introducing additional values, try to use the listed values.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) kind:       Option<VariablePresentationHintKind>,
	/// Set of attributes represented as an array of strings.
	///
	/// Before introducing additional values, try to use the listed values.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) attributes: Option<Vec<VariablePresentationHintAttribute>>,
	/// Visibility of variable.
	///
	/// Before introducing additional values, try to use the listed values.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) visibility: Option<VariablePresentationHintVisibility>,
	/// If true, clients can present the variable with a UI that supports a specific gesture to
	/// trigger its evaluation.
	///
	/// This mechanism can be used for properties that require executing
	/// code when retrieving their value and where the code execution can be expensive and/or
	/// produce side-effects. A typical example are properties based on a getter function.
	///
	/// Please note that in addition to the `lazy` flag, the variable's `variablesReference` is
	/// expected to refer to a variable that will provide the value through another `variable`
	/// request
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub(crate) lazy:       Option<bool>,
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum VariablePresentationHintKind {
	/// Indicates that the object is a property.
	Property,
	/// Indicates that the object is a method.
	Method,
	/// Indicates that the object is a class.
	Class,
	/// Indicates that the object is data.
	Data,
	/// Indicates that the object is an event.
	Event,
	/// Indicates that the object is a base class.
	BaseClass,
	/// Indicates that the object is an inner class.
	InnerClass,
	/// Indicates that the object is an interface.
	Interface,
	/// Indicates that the object is the most derived class.
	MostDerivedClass,
	/// Indicates that the object is virtual, that means it is a synthetic object introduced by the
	/// adapter for rendering purposes, e.g. an index range for large arrays.
	Virtual,
	/// Indicates that a data breakpoint is registered for the object.
	#[deprecated = "The `hasDataBreakpoint` attribute should generally be used instead"]
	DataBreakpoint,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum VariablePresentationHintAttribute {
	/// Indicates that the object is static.
	Static,
	/// Indicates that the object is a constant.
	Constant,
	/// Indicates that the object is read only.
	ReadOnly,
	/// Indicates that the object is a raw string.
	RawString,
	/// Indicates that the object can have an Object ID created for it.
	#[deprecated = "This is a vestigial attribute that is used by some clients; 'Object ID's are \
	                not specified in the protocol"]
	HasObjectId,
	/// Indicates that the object has an Object ID associated with it.
	#[deprecated = "This is a vestigial attribute that is used by some clients; 'Object ID's are \
	                not specified in the protocol"]
	CanHaveObjectId,
	/// Indicates that the evaluation had side effects.
	HasSideEffects,
	/// Indicates that the object has its value tracked by a data breakpoint.
	HasDataBreakpoint,
	#[serde(untagged)]
	Other(String),
}

#[derive(
	Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "camelCase")]
pub enum VariablePresentationHintVisibility {
	Public,
	Private,
	Protected,
	Internal,
	Final,
	#[serde(untagged)]
	Other(String),
}

impl Variable {
	#[allow(clippy::too_many_arguments, reason = "Big structure, can't do much about it")]
	pub fn new(
		name: String,
		value: String,
		variable_type: Option<String>,
		presentation_hint: Option<VariablePresentationHint>,
		evaluate_name: Option<String>,
		variables_reference: u32,
		named_variables: Option<i32>,
		indexed_variables: Option<i32>,
		memory_reference: Option<String>,
		declaration_location_reference: Option<i32>,
		value_location_reference: Option<i32>,
	) -> Self {
		Self {
			name,
			value,
			variable_type,
			presentation_hint,
			evaluate_name,
			variables_reference,
			named_variables,
			indexed_variables,
			memory_reference,
			declaration_location_reference,
			value_location_reference,
		}
	}

	/// The variable's name.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The variable's value.
	///
	/// This can be a multi-line text, e.g. for a function the body of a function.
	///
	/// For structured variables (which do not have a simple value), it is recommended to provide a
	/// one-line representation of the structured object. This helps to identify the structured
	/// object in the collapsed state when its children are not yet visible.
	///
	/// An empty string can be used if no value should be shown in the UI.
	pub fn value(&self) -> &String {
		&self.value
	}

	/// The type of the variable's value. Typically shown in the UI when hovering over the
	/// value.
	///
	/// This attribute should only be returned by a debug adapter if the corresponding capability
	/// `supportsVariableType` is true.
	pub fn variable_type(&self) -> Option<&String> {
		self.variable_type.as_ref()
	}

	/// Properties of a variable that can be used to determine how to render the variable in the
	/// UI.
	pub fn presentation_hint(&self) -> Option<&VariablePresentationHint> {
		self.presentation_hint.as_ref()
	}

	/// The evaluatable name of this variable which can be passed to the `evaluate` request to
	/// fetch the variable's value.
	pub fn evaluate_name(&self) -> Option<&String> {
		self.evaluate_name.as_ref()
	}

	/// If `variablesReference` is > 0, the variable is structured and its children can be
	/// retrieved by passing `variablesReference` to the `variables` request as long as execution
	/// remains suspended.
	pub fn variables_reference(&self) -> u32 {
		self.variables_reference
	}

	/// The number of named child variables.
	///
	/// The client can use this information to present the children in a paged UI and fetch them in
	/// chunks.
	pub fn named_variables(&self) -> Option<i32> {
		self.named_variables
	}

	/// The number of indexed child variables.
	///
	/// The client can use this information to present the children in a paged UI and fetch them in
	/// chunks.
	pub fn indexed_variables(&self) -> Option<i32> {
		self.indexed_variables
	}

	/// A memory reference associated with this variable.
	///
	/// For pointer type variables, this is generally a reference to the memory address contained
	/// in the pointer.
	///
	/// For executable data, this reference may later be used in a `disassemble` request.
	///
	/// This attribute may be returned by a debug adapter if corresponding capability
	/// `supportsMemoryReferences` is true.
	pub fn memory_reference(&self) -> Option<&String> {
		self.memory_reference.as_ref()
	}

	/// A reference that allows the client to request the location where the variable is declared.
	/// This should be present only if the adapter is likely to be able to resolve the
	/// location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	pub fn declaration_location_reference(&self) -> Option<i32> {
		self.declaration_location_reference
	}

	/// A reference that allows the client to request the location where the variable's value is
	/// declared. For example, if the variable contains a function pointer, the adapter may be able
	/// to look up the function's location. This should be present only if the adapter is likely to
	/// be able to resolve the location.
	///
	/// This reference shares the same lifetime as the `variablesReference`.
	pub fn value_location_reference(&self) -> Option<i32> {
		self.value_location_reference
	}
}

impl VariablePresentationHint {
	pub fn new(
		kind: Option<VariablePresentationHintKind>,
		attributes: Option<Vec<VariablePresentationHintAttribute>>,
		visibility: Option<VariablePresentationHintVisibility>,
		lazy: Option<bool>,
	) -> Self {
		Self { kind, attributes, visibility, lazy }
	}

	/// The kind of variable.
	///
	/// Before introducing additional values, try to use the listed values.
	pub fn kind(&self) -> Option<&VariablePresentationHintKind> {
		self.kind.as_ref()
	}

	/// Set of attributes represented as an array of strings.
	///
	/// Before introducing additional values, try to use the listed values.
	pub fn attributes(&self) -> Option<&Vec<VariablePresentationHintAttribute>> {
		self.attributes.as_ref()
	}

	/// Visibility of variable.
	///
	/// Before introducing additional values, try to use the listed values.
	pub fn visibility(&self) -> Option<&VariablePresentationHintVisibility> {
		self.visibility.as_ref()
	}

	/// If true, clients can present the variable with a UI that supports a specific gesture to
	/// trigger its evaluation.
	///
	/// This mechanism can be used for properties that require executing
	/// code when retrieving their value and where the code execution can be expensive and/or
	/// produce side-effects. A typical example are properties based on a getter function.
	///
	/// Please note that in addition to the `lazy` flag, the variable's `variablesReference` is
	/// expected to refer to a variable that will provide the value through another `variable`
	/// request
	pub fn lazy(&self) -> Option<bool> {
		self.lazy
	}
}
