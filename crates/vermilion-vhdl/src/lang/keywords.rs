// SPDX-License-Identifier: BSD-3-Clause

use phf::{phf_map, phf_set};

use crate::{LanguageStd, lang::tokenizer::token::Keyword};

/// IEEE 1076-1987 (VHDL 1987) Keyword to [`Keyword`] token map
pub static VHDL_87_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"in" => Keyword::In,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"register" => Keyword::Register,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"signal" => Keyword::Signal,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xor" => Keyword::Xor,
};

/// IEEE 1076-1987 (VHDL 1987) Keyword set
pub static VHDL_87_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",          "configuration", "inout",   "or",        "then",
	"access",       "constant",      "is",      "others",    "to",
	"after",        "disconnect",    "label",   "out",       "transport",
	"alias",        "downto",        "library", "package",   "type",
	"all",          "else",          "linkage", "port",      "units",
	"and",          "elsif",         "loop",    "procedure", "until",
	"architecture", "end",           "map",     "process",   "use",
	"array",        "entity",        "mod",     "range",     "variable",
	"assert",       "exit",          "nand",    "record",    "wait",
	"attribute",    "file",          "new",     "register",  "when",
	"begin",        "for",           "next",    "rem",       "while",
	"block",        "function",      "nor",     "report",    "with",
	"body",         "generate",      "not",     "return",    "xor",
	"buffer",       "generic",       "null",    "select",
	"bus",          "guarded",       "of",      "severity",
	"case",         "if",            "on",      "signal",
	"component",    "in",            "open",    "subtype",

};

/// IEEE 1076-1993 (VHDL 1993) Keyword to [`Keyword`] token map
pub static VHDL_93_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1076-1993 (VHDL 1993) Keyword set
pub static VHDL_93_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",          "downto",   "library", "postponed", "srl",
	"access",       "else",     "linkage", "procedure", "subtype",
	"after",        "elsif",    "literal", "process",   "then",
	"alias",        "end",      "loop",    "pure",      "to",
	"all",          "entity",   "map",     "range",     "transport",
	"and",          "exit",     "mod",     "record",    "type",
	"architecture", "file",     "nand",    "register",  "unaffected",
	"array",        "for",      "new",     "reject",    "units",
	"assert",       "function", "next",    "rem",       "until",
	"attribute",    "generate", "nor",     "report",    "use",
	"begin",        "generic",  "not",     "return",    "variable",
	"block",        "group",    "null",    "rol",       "wait",
	"body",         "guarded",  "of",      "ror",       "when",
	"buffer",       "if",       "on",      "select",    "while",
	"bus",          "impure",   "open",    "severity",  "with",
	"case",         "in",       "or",      "shared",    "xnor",
	"component",    "inertial", "others",  "signal",    "xor",
	"configuration","inout",    "out",     "sla",
	"constant",     "is",       "package", "sll",
	"disconnect",   "label",    "port",    "sra",
};

/// IEEE 1076-2000 (VHDL 2000) Keyword to [`Keyword`] token map
pub static VHDL_2K_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1076-2000 (VHDL 2000) Keyword set
pub static VHDL_2K_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "downto",   "library", "postponed",  "sla",
	"access",        "else",     "linkage", "procedural", "sll",
	"after",         "elsif",    "literal", "procedure",  "sra",
	"alias",         "end",      "loop",    "process",    "srl",
	"all",           "entity",   "map",     "protected",  "subtype",
	"and",           "exit",     "mod",     "pure",       "then",
	"architecture",  "file",     "nand",    "range",      "to",
	"array",         "for",      "new",     "record",     "transport",
	"assert",        "function", "next",    "reference",  "type",
	"attribute",     "generate", "nor",     "register",   "unaffected",
	"begin",         "generic",  "not",     "reject",     "units",
	"block",         "group",    "null",    "rem",        "until",
	"body",          "guarded",  "of",      "report",     "use",
	"buffer",        "if",       "on",      "return",     "variable",
	"bus",           "impure",   "open",    "rol",        "wait",
	"case",          "in",       "or",      "ror",        "when",
	"component",     "inertial", "others",  "select",     "while",
	"configuration", "inout",    "out",     "severity",   "with",
	"constant",      "is",       "package", "shared",     "xnor",
	"disconnect",    "label",    "port",    "signal",     "xor",
};

// NOTE(aki): Identical to `VHDL_2K_KEYWORD_MAP`
/// IEEE 1076-2002 (VHDL 2002) Keyword to [`Keyword`] token map
pub static VHDL_02_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `VHDL_2K_KEYWORD_SET`
/// IEEE 1076-2002 (VHDL 2002) Keyword set
pub static VHDL_02_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "downto",   "library", "postponed",  "sla",
	"access",        "else",     "linkage", "procedural", "sll",
	"after",         "elsif",    "literal", "procedure",  "sra",
	"alias",         "end",      "loop",    "process",    "srl",
	"all",           "entity",   "map",     "protected",  "subtype",
	"and",           "exit",     "mod",     "pure",       "then",
	"architecture",  "file",     "nand",    "range",      "to",
	"array",         "for",      "new",     "record",     "transport",
	"assert",        "function", "next",    "reference",  "type",
	"attribute",     "generate", "nor",     "register",   "unaffected",
	"begin",         "generic",  "not",     "reject",     "units",
	"block",         "group",    "null",    "rem",        "until",
	"body",          "guarded",  "of",      "report",     "use",
	"buffer",        "if",       "on",      "return",     "variable",
	"bus",           "impure",   "open",    "rol",        "wait",
	"case",          "in",       "or",      "ror",        "when",
	"component",     "inertial", "others",  "select",     "while",
	"configuration", "inout",    "out",     "severity",   "with",
	"constant",      "is",       "package", "shared",     "xnor",
	"disconnect",    "label",    "port",    "signal",     "xor",
};

// NOTE(aki): Identical to `VHDL_2K_KEYWORD_MAP`
/// IEC 61691-1-1:2004 (VHDL 2004) Keyword to [`Keyword`] token map
pub static VHDL_04_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `VHDL_2K_KEYWORD_SET`
/// IEC 61691-1-1:2004 (VHDL 2004) Keyword set
pub static VHDL_04_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "downto",   "library", "postponed",  "sla",
	"access",        "else",     "linkage", "procedural", "sll",
	"after",         "elsif",    "literal", "procedure",  "sra",
	"alias",         "end",      "loop",    "process",    "srl",
	"all",           "entity",   "map",     "protected",  "subtype",
	"and",           "exit",     "mod",     "pure",       "then",
	"architecture",  "file",     "nand",    "range",      "to",
	"array",         "for",      "new",     "record",     "transport",
	"assert",        "function", "next",    "reference",  "type",
	"attribute",     "generate", "nor",     "register",   "unaffected",
	"begin",         "generic",  "not",     "reject",     "units",
	"block",         "group",    "null",    "rem",        "until",
	"body",          "guarded",  "of",      "report",     "use",
	"buffer",        "if",       "on",      "return",     "variable",
	"bus",           "impure",   "open",    "rol",        "wait",
	"case",          "in",       "or",      "ror",        "when",
	"component",     "inertial", "others",  "select",     "while",
	"configuration", "inout",    "out",     "severity",   "with",
	"constant",      "is",       "package", "shared",     "xnor",
	"disconnect",    "label",    "port",    "signal",     "xor",
};

// NOTE(aki): Identical to `VHDL_2K_KEYWORD_MAP`
/// IEEE 1076-2007 (VHDL 2007) Keyword to [`Keyword`] token map
pub static VHDL_07_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `VHDL_2K_KEYWORD_SET`
/// IEEE 1076-2007 (VHDL 2007) Keyword set
pub static VHDL_07_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "downto",   "library", "postponed",  "sla",
	"access",        "else",     "linkage", "procedural", "sll",
	"after",         "elsif",    "literal", "procedure",  "sra",
	"alias",         "end",      "loop",    "process",    "srl",
	"all",           "entity",   "map",     "protected",  "subtype",
	"and",           "exit",     "mod",     "pure",       "then",
	"architecture",  "file",     "nand",    "range",      "to",
	"array",         "for",      "new",     "record",     "transport",
	"assert",        "function", "next",    "reference",  "type",
	"attribute",     "generate", "nor",     "register",   "unaffected",
	"begin",         "generic",  "not",     "reject",     "units",
	"block",         "group",    "null",    "rem",        "until",
	"body",          "guarded",  "of",      "report",     "use",
	"buffer",        "if",       "on",      "return",     "variable",
	"bus",           "impure",   "open",    "rol",        "wait",
	"case",          "in",       "or",      "ror",        "when",
	"component",     "inertial", "others",  "select",     "while",
	"configuration", "inout",    "out",     "severity",   "with",
	"constant",      "is",       "package", "shared",     "xnor",
	"disconnect",    "label",    "port",    "signal",     "xor",
};

/// IEEE 1076-2008 (VHDL 2008) Keyword to [`Keyword`] token map
pub static VHDL_08_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"assume" => Keyword::Assume,
	"assume_guarantee" => Keyword::AssumeGuarantee,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"context" => Keyword::Context,
	"cover" => Keyword::Cover,
	"default" => Keyword::Default,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"fairness" => Keyword::Fairness,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"parameter" => Keyword::Parameter,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"release" => Keyword::Release,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"restrict" => Keyword::Restrict,
	"restrict_guarantee" => Keyword::RestrictGuarantee,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"sequence" => Keyword::Sequence,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"strong" => Keyword::Strong,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"vmode" => Keyword::Vmode,
	"vprop" => Keyword::Vprop,
	"vunit" => Keyword::Vunit,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1076-2008 (VHDL 2008) Keyword set
pub static VHDL_08_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",              "default",    "label",     "procedure",          "sll",
	"access",           "disconnect", "library",   "process",            "sra",
	"after",            "downto",     "linkage",   "property",           "srl",
	"alias",            "else",       "literal",   "protected",          "strong",
	"all",              "elsif",      "loop",      "pure",               "subtype",
	"and",              "end",        "map",       "range",              "then",
	"architecture",     "entity",     "mod",       "record",             "to",
	"array",            "exit",       "nand",      "register",           "transport",
	"assert",           "fairness",   "new",       "reject",             "type",
	"assume",           "file",       "next",      "release",            "unaffected",
	"assume_guarantee", "for",        "nor",       "rem",                "units",
	"attribute",        "force",      "not",       "report",             "until",
	"begin",            "function",   "null",      "restrict",           "use",
	"block",            "generate",   "of",        "restrict_guarantee", "variable",
	"body",             "generic",    "on",        "return",             "vmode",
	"buffer",           "group",      "open",      "rol",                "vprop",
	"bus",              "guarded",    "or",        "ror",                "vunit",
	"case",             "if",         "others",    "select",             "wait",
	"component",        "impure",     "out",       "sequence",           "when",
	"configuration",    "in",         "package",   "severity",           "while",
	"constant",         "inertial",   "parameter", "shared",             "with",
	"context",          "inout",      "port",      "signal",             "xnor",
	"cover",            "is",         "postponed", "sla",                "xor",
};

// NOTE(aki): Identical to `VHDL_08_KEYWORD_MAP`
/// IEC 61691-1-1:2011 (VHDL 2011) Keyword to [`Keyword`] token map
pub static VHDL_11_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"assume" => Keyword::Assume,
	"assume_guarantee" => Keyword::AssumeGuarantee,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"context" => Keyword::Context,
	"cover" => Keyword::Cover,
	"default" => Keyword::Default,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"fairness" => Keyword::Fairness,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"parameter" => Keyword::Parameter,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"release" => Keyword::Release,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"restrict" => Keyword::Restrict,
	"restrict_guarantee" => Keyword::RestrictGuarantee,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"sequence" => Keyword::Sequence,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"strong" => Keyword::Strong,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"vmode" => Keyword::Vmode,
	"vprop" => Keyword::Vprop,
	"vunit" => Keyword::Vunit,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `VHDL_08_KEYWORD_SET`
/// IEC 61691-1-1:2011 (VHDL 2011) Keyword set
pub static VHDL_11_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",              "default",    "label",     "procedure",          "sll",
	"access",           "disconnect", "library",   "process",            "sra",
	"after",            "downto",     "linkage",   "property",           "srl",
	"alias",            "else",       "literal",   "protected",          "strong",
	"all",              "elsif",      "loop",      "pure",               "subtype",
	"and",              "end",        "map",       "range",              "then",
	"architecture",     "entity",     "mod",       "record",             "to",
	"array",            "exit",       "nand",      "register",           "transport",
	"assert",           "fairness",   "new",       "reject",             "type",
	"assume",           "file",       "next",      "release",            "unaffected",
	"assume_guarantee", "for",        "nor",       "rem",                "units",
	"attribute",        "force",      "not",       "report",             "until",
	"begin",            "function",   "null",      "restrict",           "use",
	"block",            "generate",   "of",        "restrict_guarantee", "variable",
	"body",             "generic",    "on",        "return",             "vmode",
	"buffer",           "group",      "open",      "rol",                "vprop",
	"bus",              "guarded",    "or",        "ror",                "vunit",
	"case",             "if",         "others",    "select",             "wait",
	"component",        "impure",     "out",       "sequence",           "when",
	"configuration",    "in",         "package",   "severity",           "while",
	"constant",         "inertial",   "parameter", "shared",             "with",
	"context",          "inout",      "port",      "signal",             "xnor",
	"cover",            "is",         "postponed", "sla",                "xor",
};

/// IEEE 1076-2019 (VHDL 2019) Keyword to [`Keyword`] token map
pub static VHDL_19_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"assume" => Keyword::Assume,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"context" => Keyword::Context,
	"cover" => Keyword::Cover,
	"default" => Keyword::Default,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"fairness" => Keyword::Fairness,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"parameter" => Keyword::Parameter,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"private" => Keyword::Private,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"release" => Keyword::Release,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"sequence" => Keyword::Sequence,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"strong" => Keyword::Strong,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"view" => Keyword::View,
	"vmode" => Keyword::Vmode,
	"vpkg" => Keyword::Vpkg,
	"vprop" => Keyword::Vprop,
	"vunit" => Keyword::Vunit,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1076-2019 (VHDL 2019) Keyword set
pub static VHDL_19_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "downto",   "literal",    "property",  "strong",
	"access",        "else",     "loop",       "protected", "subtype",
	"after",         "elsif",    "map",        "pure",      "then",
	"alias",         "end",      "mod",        "range",     "to",
	"all",           "entity",   "nand",       "record",    "transport",
	"and",           "exit",     "new",        "reference", "type",
	"architecture",  "fairness", "next",       "register",  "unaffected",
	"array",         "file",     "nor",        "reject",    "units",
	"assert",        "for",      "not",        "release",   "until",
	"assume",        "force",    "null",       "rem",       "use",
	"attribute",     "function", "of",         "report",    "variable",
	"begin",         "generate", "on",         "restrict",  "view",
	"block",         "generic",  "open",       "return",    "vmode",
	"body",          "group",    "or",         "rol",       "vpkg",
	"buffer",        "guarded",  "others",     "ror",       "vprop",
	"bus",           "if",       "out",        "select",    "vunit",
	"case",          "impure",   "package",    "sequence",  "wait",
	"component",     "in",       "parameter",  "severity",  "when",
	"configuration", "inertial", "port",       "shared",    "while",
	"constant",      "inout",    "postponed",  "signal",    "with",
	"context",       "is",       "private",    "sla",       "xnor",
	"cover",         "label",    "procedural", "sll",       "xor",
	"default",       "library",  "procedure",  "sra",
	"disconnect",    "linkage",  "process",    "srl",
};

// NOTE(aki): Identical to `VHDL_19_KEYWORD_MAP`
/// IEC 61691-1-1:2023 (VHDL 2023) Keyword to [`Keyword`] token map
pub static VHDL_23_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"assume" => Keyword::Assume,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"context" => Keyword::Context,
	"cover" => Keyword::Cover,
	"default" => Keyword::Default,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"fairness" => Keyword::Fairness,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"parameter" => Keyword::Parameter,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"private" => Keyword::Private,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"release" => Keyword::Release,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"sequence" => Keyword::Sequence,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"strong" => Keyword::Strong,
	"subtype" => Keyword::Subtype,
	"then" => Keyword::Then,
	"to" => Keyword::To,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"view" => Keyword::View,
	"vmode" => Keyword::Vmode,
	"vpkg" => Keyword::Vpkg,
	"vprop" => Keyword::Vprop,
	"vunit" => Keyword::Vunit,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `VHDL_19_KEYWORD_SET`
/// IEC 61691-1-1:2023 (VHDL 2023) Keyword set
pub static VHDL_23_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "downto",   "literal",    "property",  "strong",
	"access",        "else",     "loop",       "protected", "subtype",
	"after",         "elsif",    "map",        "pure",      "then",
	"alias",         "end",      "mod",        "range",     "to",
	"all",           "entity",   "nand",       "record",    "transport",
	"and",           "exit",     "new",        "reference", "type",
	"architecture",  "fairness", "next",       "register",  "unaffected",
	"array",         "file",     "nor",        "reject",    "units",
	"assert",        "for",      "not",        "release",   "until",
	"assume",        "force",    "null",       "rem",       "use",
	"attribute",     "function", "of",         "report",    "variable",
	"begin",         "generate", "on",         "restrict",  "view",
	"block",         "generic",  "open",       "return",    "vmode",
	"body",          "group",    "or",         "rol",       "vpkg",
	"buffer",        "guarded",  "others",     "ror",       "vprop",
	"bus",           "if",       "out",        "select",    "vunit",
	"case",          "impure",   "package",    "sequence",  "wait",
	"component",     "in",       "parameter",  "severity",  "when",
	"configuration", "inertial", "port",       "shared",    "while",
	"constant",      "inout",    "postponed",  "signal",    "with",
	"context",       "is",       "private",    "sla",       "xnor",
	"cover",         "label",    "procedural", "sll",       "xor",
	"default",       "library",  "procedure",  "sra",
	"disconnect",    "linkage",  "process",    "srl",
};

/// IEEE 1076.1-1999 (VHDL-AMS 1999) Keyword to [`Keyword`] token map
pub static VHDL_AMS_99_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"across" => Keyword::Across,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"break" => Keyword::Break,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"limit" => Keyword::Limit,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"nature" => Keyword::Nature,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"noise" => Keyword::Noise,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"pure" => Keyword::Pure,
	"quantity" => Keyword::Quantity,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"spectrum" => Keyword::Spectrum,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subnature" => Keyword::SubNature,
	"subtype" => Keyword::Subtype,
	"terminal" => Keyword::Terminal,
	"then" => Keyword::Then,
	"through" => Keyword::Through,
	"to" => Keyword::To,
	"tolerance" => Keyword::Tolerance,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1076.1-1999 (VHDL-AMS 1999) Keyword set
pub static VHDL_AMS_99_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "downto",   "linkage",   "procedural", "sra",
	"access",        "else",     "literal",   "procedure",  "srl",
	"across",        "elsif",    "loop",      "process",    "subnature",
	"after",         "end",      "map",       "pure",       "subtype",
	"alias",         "entity",   "mod",       "quantity",   "terminal",
	"all",           "exit",     "nand",      "range",      "then",
	"and",           "file",     "nature",    "record",     "through",
	"architecture",  "for",      "new",       "reference",  "to",
	"array",         "function", "next",      "register",   "tolerance",
	"assert",        "generate", "noise",     "reject",     "transport",
	"attribute",     "generic",  "nor",       "rem",        "type",
	"begin",         "group",    "not",       "report",     "unaffected",
	"block",         "guarded",  "null",      "return",     "units",
	"body",          "if",       "of",        "rol",        "until",
	"break",         "impure",   "on",        "ror",        "use",
	"buffer",        "in",       "open",      "select",     "variable",
	"bus",           "inertial", "or",        "severity",   "wait",
	"case",          "inout",    "others",    "shared",     "when",
	"component",     "is",       "out",       "signal",     "while",
	"configuration", "label",    "package",   "sla",        "with",
	"constant",      "library",  "port",      "sll",        "xnor",
	"disconnect",    "limit",    "postponed", "spectrum",   "xor",
};

/// IEEE 1076.1-2007 (VHDL-AMS 2007) Keyword to [`Keyword`] token map
pub static VHDL_AMS_07_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"across" => Keyword::Across,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"break" => Keyword::Break,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"limit" => Keyword::Limit,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"nature" => Keyword::Nature,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"noise" => Keyword::Noise,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"quantity" => Keyword::Quantity,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"spectrum" => Keyword::Spectrum,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subnature" => Keyword::SubNature,
	"subtype" => Keyword::Subtype,
	"terminal" => Keyword::Terminal,
	"then" => Keyword::Then,
	"through" => Keyword::Through,
	"to" => Keyword::To,
	"tolerance" => Keyword::Tolerance,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1076.1-2007 (VHDL-AMS 2007) Keyword set
pub static VHDL_AMS_07_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "else",     "loop",       "protected", "subtype",
	"access",        "elsif",    "map",        "pure",      "terminal",
	"across",        "end",      "mod",        "quantity",  "then",
	"after",         "entity",   "nand",       "range",     "through",
	"alias",         "exit",     "nature",     "record",    "to",
	"all",           "file",     "new",        "reference", "tolerance",
	"and",           "for",      "next",       "register",  "transport",
	"architecture",  "function", "noise",      "reject",    "type",
	"array",         "generate", "nor",        "rem",       "unaffected",
	"assert",        "generic",  "not",        "report",    "units",
	"attribute",     "group",    "null",       "return",    "until",
	"begin",         "guarded",  "of",         "rol",       "use",
	"block",         "if",       "on",         "ror",       "variable",
	"body",          "impure",   "open",       "select",    "wait",
	"break",         "in",       "or",         "severity",  "when",
	"buffer",        "inertial", "others",     "shared",    "while",
	"bus",           "inout",    "out",        "signal",    "with",
	"case",          "is",       "package",    "sla",       "xnor",
	"component",     "label",    "port",       "sll",       "xor",
	"configuration", "library",  "postponed",  "spectrum",
	"constant",      "limit",    "procedural", "sra",
	"disconnect",    "linkage",  "procedure",  "srl",
	"downto",        "literal",  "process",    "subnature",
};

// NOTE(aki): Identical to `VHDL_AMS_07_KEYWORD_MAP`
/// IEC 61691-6:2009 (VHDL-AMS 2009) Keyword to [`Keyword`] token map
pub static VHDL_AMS_09_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"across" => Keyword::Across,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"break" => Keyword::Break,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"limit" => Keyword::Limit,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"nature" => Keyword::Nature,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"noise" => Keyword::Noise,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"quantity" => Keyword::Quantity,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"spectrum" => Keyword::Spectrum,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"subnature" => Keyword::SubNature,
	"subtype" => Keyword::Subtype,
	"terminal" => Keyword::Terminal,
	"then" => Keyword::Then,
	"through" => Keyword::Through,
	"to" => Keyword::To,
	"tolerance" => Keyword::Tolerance,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `VHDL_AMS_07_KEYWORD_SET`
/// IEC 61691-6:2009 (VHDL-AMS 2009) Keyword set
pub static VHDL_AMS_09_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",           "else",     "loop",       "protected", "subtype",
	"access",        "elsif",    "map",        "pure",      "terminal",
	"across",        "end",      "mod",        "quantity",  "then",
	"after",         "entity",   "nand",       "range",     "through",
	"alias",         "exit",     "nature",     "record",    "to",
	"all",           "file",     "new",        "reference", "tolerance",
	"and",           "for",      "next",       "register",  "transport",
	"architecture",  "function", "noise",      "reject",    "type",
	"array",         "generate", "nor",        "rem",       "unaffected",
	"assert",        "generic",  "not",        "report",    "units",
	"attribute",     "group",    "null",       "return",    "until",
	"begin",         "guarded",  "of",         "rol",       "use",
	"block",         "if",       "on",         "ror",       "variable",
	"body",          "impure",   "open",       "select",    "wait",
	"break",         "in",       "or",         "severity",  "when",
	"buffer",        "inertial", "others",     "shared",    "while",
	"bus",           "inout",    "out",        "signal",    "with",
	"case",          "is",       "package",    "sla",       "xnor",
	"component",     "label",    "port",       "sll",       "xor",
	"configuration", "library",  "postponed",  "spectrum",
	"constant",      "limit",    "procedural", "sra",
	"disconnect",    "linkage",  "procedure",  "srl",
	"downto",        "literal",  "process",    "subnature",
};

/// IEEE 1076.1-2017 (VHDL-AMS 2017) Keyword to [`Keyword`] token map
pub static VHDL_AMS_17_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"across" => Keyword::Across,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"assume_guarantee" => Keyword::AssumeGuarantee,
	"assume" => Keyword::Assume,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"break" => Keyword::Break,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"context" => Keyword::Context,
	"cover" => Keyword::Cover,
	"default" => Keyword::Default,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"fairness" => Keyword::Fairness,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"limit" => Keyword::Limit,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"nature" => Keyword::Nature,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"noise" => Keyword::Noise,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"parameter" => Keyword::Parameter,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"quantity" => Keyword::Quantity,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"release" => Keyword::Release,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"restrict_guarantee" => Keyword::RestrictGuarantee,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"sequence" => Keyword::Sequence,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"spectrum" => Keyword::Spectrum,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"strong" => Keyword::Strong,
	"subnature" => Keyword::SubNature,
	"subtype" => Keyword::Subtype,
	"terminal" => Keyword::Terminal,
	"then" => Keyword::Then,
	"through" => Keyword::Through,
	"to" => Keyword::To,
	"tolerance" => Keyword::Tolerance,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"vmode" => Keyword::Vmode,
	"vprop" => Keyword::Vprop,
	"vunit" => Keyword::Vunit,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

/// IEEE 1076.1-2017 (VHDL-AMS 2017) Keyword set
pub static VHDL_AMS_17_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",              "disconnect", "literal",    "protected",          "strong",
	"access",           "downto",     "loop",       "pure",               "subnature",
	"across",           "else",       "map",        "quantity",           "subtype",
	"after",            "elsif",      "mod",        "range",              "terminal",
	"alias",            "end",        "nand",       "record",             "then",
	"all",              "entity",     "nature",     "reference",          "through",
	"and",              "exit",       "new",        "register",           "to",
	"architecture",     "fairness",   "next",       "reject",             "tolerance",
	"array",            "file",       "noise",      "release",            "transport",
	"assert",           "for",        "nor",        "rem",                "type",
	"assume_guarantee", "force",      "not",        "report",             "unaffected",
	"assume",           "function",   "null",       "restrict_guarantee", "units",
	"attribute",        "generate",   "of",         "restrict",           "until",
	"begin",            "generic",    "on",         "return",             "use",
	"block",            "group",      "open",       "rol",                "variable",
	"body",             "guarded",    "or",         "ror",                "vmode",
	"break",            "if",         "others",     "select",             "vprop",
	"buffer",           "impure",     "out",        "sequence",           "vunit",
	"bus",              "in",         "package",    "severity",           "wait",
	"case",             "inertial",   "parameter",  "shared",             "when",
	"component",        "inout",      "port",       "signal",             "while",
	"configuration",    "is",         "postponed",  "sla",                "with",
	"constant",         "label",      "procedural", "sll",                "xnor",
	"context",          "library",    "procedure",  "spectrum",           "xor",
	"cover",            "limit",      "process",    "sra",
	"default",          "linkage",    "property",   "srl",
};

// NOTE(aki): Identical to `VHDL_AMS_17_KEYWORD_MAP`
/// IEC 61691-6:2021 (VHDL-AMS 2021) Keyword to [`Keyword`] token map
pub static VHDL_AMS_21_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"abs" => Keyword::Abs,
	"access" => Keyword::Access,
	"across" => Keyword::Across,
	"after" => Keyword::After,
	"alias" => Keyword::Alias,
	"all" => Keyword::All,
	"and" => Keyword::And,
	"architecture" => Keyword::Architecture,
	"array" => Keyword::Array,
	"assert" => Keyword::Assert,
	"assume_guarantee" => Keyword::AssumeGuarantee,
	"assume" => Keyword::Assume,
	"attribute" => Keyword::Attribute,
	"begin" => Keyword::Begin,
	"block" => Keyword::Block,
	"body" => Keyword::Body,
	"break" => Keyword::Break,
	"buffer" => Keyword::Buffer,
	"bus" => Keyword::Bus,
	"case" => Keyword::Case,
	"component" => Keyword::Component,
	"configuration" => Keyword::Configuration,
	"constant" => Keyword::Constant,
	"context" => Keyword::Context,
	"cover" => Keyword::Cover,
	"default" => Keyword::Default,
	"disconnect" => Keyword::Disconnect,
	"downto" => Keyword::DownTo,
	"else" => Keyword::Else,
	"elsif" => Keyword::Elsif,
	"end" => Keyword::End,
	"entity" => Keyword::Entity,
	"exit" => Keyword::Exit,
	"fairness" => Keyword::Fairness,
	"file" => Keyword::File,
	"for" => Keyword::For,
	"force" => Keyword::Force,
	"function" => Keyword::Function,
	"generate" => Keyword::Generate,
	"generic" => Keyword::Generic,
	"group" => Keyword::Group,
	"guarded" => Keyword::Guarded,
	"if" => Keyword::If,
	"impure" => Keyword::Impure,
	"in" => Keyword::In,
	"inertial" => Keyword::Inertial,
	"inout" => Keyword::InOut,
	"is" => Keyword::Is,
	"label" => Keyword::Label,
	"library" => Keyword::Library,
	"limit" => Keyword::Limit,
	"linkage" => Keyword::Linkage,
	"literal" => Keyword::Literal,
	"loop" => Keyword::Loop,
	"map" => Keyword::Map,
	"mod" => Keyword::Mod,
	"nand" => Keyword::Nand,
	"nature" => Keyword::Nature,
	"new" => Keyword::New,
	"next" => Keyword::Next,
	"noise" => Keyword::Noise,
	"nor" => Keyword::Nor,
	"not" => Keyword::Not,
	"null" => Keyword::Null,
	"of" => Keyword::Of,
	"on" => Keyword::On,
	"open" => Keyword::Open,
	"or" => Keyword::Or,
	"others" => Keyword::Others,
	"out" => Keyword::Out,
	"package" => Keyword::Package,
	"parameter" => Keyword::Parameter,
	"port" => Keyword::Port,
	"postponed" => Keyword::Postponed,
	"procedural" => Keyword::Procedural,
	"procedure" => Keyword::Procedure,
	"process" => Keyword::Process,
	"property" => Keyword::Property,
	"protected" => Keyword::Protected,
	"pure" => Keyword::Pure,
	"quantity" => Keyword::Quantity,
	"range" => Keyword::Range,
	"record" => Keyword::Record,
	"reference" => Keyword::Reference,
	"register" => Keyword::Register,
	"reject" => Keyword::Reject,
	"release" => Keyword::Release,
	"rem" => Keyword::Rem,
	"report" => Keyword::Report,
	"restrict_guarantee" => Keyword::RestrictGuarantee,
	"restrict" => Keyword::Restrict,
	"return" => Keyword::Return,
	"rol" => Keyword::Rol,
	"ror" => Keyword::Ror,
	"select" => Keyword::Select,
	"sequence" => Keyword::Sequence,
	"severity" => Keyword::Severity,
	"shared" => Keyword::Shared,
	"signal" => Keyword::Signal,
	"sla" => Keyword::Sla,
	"sll" => Keyword::Sll,
	"spectrum" => Keyword::Spectrum,
	"sra" => Keyword::Sra,
	"srl" => Keyword::Srl,
	"strong" => Keyword::Strong,
	"subnature" => Keyword::SubNature,
	"subtype" => Keyword::Subtype,
	"terminal" => Keyword::Terminal,
	"then" => Keyword::Then,
	"through" => Keyword::Through,
	"to" => Keyword::To,
	"tolerance" => Keyword::Tolerance,
	"transport" => Keyword::Transport,
	"type" => Keyword::Type,
	"unaffected" => Keyword::Unaffected,
	"units" => Keyword::Units,
	"until" => Keyword::Until,
	"use" => Keyword::Use,
	"variable" => Keyword::Variable,
	"vmode" => Keyword::Vmode,
	"vprop" => Keyword::Vprop,
	"vunit" => Keyword::Vunit,
	"wait" => Keyword::Wait,
	"when" => Keyword::When,
	"while" => Keyword::While,
	"with" => Keyword::With,
	"xnor" => Keyword::Xnor,
	"xor" => Keyword::Xor,
};

// NOTE(aki): Identical to `VHDL_AMS_17_KEYWORD_SET`
/// IEC 61691-6:2021 (VHDL-AMS 2021) Keyword set
pub static VHDL_AMS_21_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"abs",              "disconnect", "literal",    "protected",          "strong",
	"access",           "downto",     "loop",       "pure",               "subnature",
	"across",           "else",       "map",        "quantity",           "subtype",
	"after",            "elsif",      "mod",        "range",              "terminal",
	"alias",            "end",        "nand",       "record",             "then",
	"all",              "entity",     "nature",     "reference",          "through",
	"and",              "exit",       "new",        "register",           "to",
	"architecture",     "fairness",   "next",       "reject",             "tolerance",
	"array",            "file",       "noise",      "release",            "transport",
	"assert",           "for",        "nor",        "rem",                "type",
	"assume_guarantee", "force",      "not",        "report",             "unaffected",
	"assume",           "function",   "null",       "restrict_guarantee", "units",
	"attribute",        "generate",   "of",         "restrict",           "until",
	"begin",            "generic",    "on",         "return",             "use",
	"block",            "group",      "open",       "rol",                "variable",
	"body",             "guarded",    "or",         "ror",                "vmode",
	"break",            "if",         "others",     "select",             "vprop",
	"buffer",           "impure",     "out",        "sequence",           "vunit",
	"bus",              "in",         "package",    "severity",           "wait",
	"case",             "inertial",   "parameter",  "shared",             "when",
	"component",        "inout",      "port",       "signal",             "while",
	"configuration",    "is",         "postponed",  "sla",                "with",
	"constant",         "label",      "procedural", "sll",                "xnor",
	"context",          "library",    "procedure",  "spectrum",           "xor",
	"cover",            "limit",      "process",    "sra",
	"default",          "linkage",    "property",   "srl",
};

/// Get the IEEE 1076-1987 (VHDL 1987) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_87_keyword(ident: &str) -> Option<Keyword> {
	VHDL_87_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076-1987 (VHDL 1987) keyword
///
/// This is used rather than [`get_vhdl_87_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_87_keyword(ident: &str) -> bool {
	VHDL_87_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076-1993 (VHDL 1993) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_93_keyword(ident: &str) -> Option<Keyword> {
	VHDL_93_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076-1993 (VHDL 1993) keyword
///
/// This is used rather than [`get_vhdl_93_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_93_keyword(ident: &str) -> bool {
	VHDL_93_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076-2000 (VHDL 2000) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_2k_keyword(ident: &str) -> Option<Keyword> {
	VHDL_2K_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076-2000 (VHDL 2000) keyword
///
/// This is used rather than [`get_vhdl_2k_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_2k_keyword(ident: &str) -> bool {
	VHDL_2K_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076-2002 (VHDL 2002) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_02_keyword(ident: &str) -> Option<Keyword> {
	VHDL_02_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076-2002 (VHDL 2002) keyword
///
/// This is used rather than [`get_vhdl_02_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_02_keyword(ident: &str) -> bool {
	VHDL_02_KEYWORD_SET.contains(ident)
}

/// Get the IEC 61691-1-1:2004 (VHDL 2004) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_04_keyword(ident: &str) -> Option<Keyword> {
	VHDL_04_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEC 61691-1-1:2004 (VHDL 2004) keyword
///
/// This is used rather than [`get_vhdl_04_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_04_keyword(ident: &str) -> bool {
	VHDL_04_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076-2007 (VHDL 2007) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_07_keyword(ident: &str) -> Option<Keyword> {
	VHDL_07_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076-2007 (VHDL 2007) keyword
///
/// This is used rather than [`get_vhdl_07_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_07_keyword(ident: &str) -> bool {
	VHDL_07_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076-2008 (VHDL 2008) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_08_keyword(ident: &str) -> Option<Keyword> {
	VHDL_08_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076-2008 (VHDL 2008) keyword
///
/// This is used rather than [`get_vhdl_08_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_08_keyword(ident: &str) -> bool {
	VHDL_08_KEYWORD_SET.contains(ident)
}

/// Get the IEC 61691-1-1:2011 (VHDL 2011) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_11_keyword(ident: &str) -> Option<Keyword> {
	VHDL_11_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEC 61691-1-1:2011 (VHDL 2011) keyword
///
/// This is used rather than [`get_vhdl_11_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_11_keyword(ident: &str) -> bool {
	VHDL_11_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076-2019 (VHDL 2019) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_19_keyword(ident: &str) -> Option<Keyword> {
	VHDL_19_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076-2019 (VHDL 2019) keyword
///
/// This is used rather than [`get_vhdl_19_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_19_keyword(ident: &str) -> bool {
	VHDL_19_KEYWORD_SET.contains(ident)
}

/// Get the IEC 61691-1-1:2023 (VHDL 2023) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_23_keyword(ident: &str) -> Option<Keyword> {
	VHDL_23_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEC 61691-1-1:2023 (VHDL 2023) keyword
///
/// This is used rather than [`get_vhdl_23_keyword`] to test if the
/// found identifier for a lower VHDL standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_23_keyword(ident: &str) -> bool {
	VHDL_23_KEYWORD_SET.contains(ident)
}

/// Get the VHDL keyword for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Vh87 => get_vhdl_87_keyword(ident),
		LanguageStd::Vh93 => get_vhdl_93_keyword(ident),
		LanguageStd::Vh2k => get_vhdl_2k_keyword(ident),
		LanguageStd::Vh02 => get_vhdl_02_keyword(ident),
		LanguageStd::Vh04 => get_vhdl_04_keyword(ident),
		LanguageStd::Vh07 => get_vhdl_07_keyword(ident),
		LanguageStd::Vh08 => get_vhdl_08_keyword(ident),
		LanguageStd::Vh11 => get_vhdl_11_keyword(ident),
		LanguageStd::Vh19 => get_vhdl_19_keyword(ident),
		LanguageStd::Vh23 => get_vhdl_23_keyword(ident),
		_ => None,
	}
}

/// Get the IEEE 1076.1-1999 (VHDL-AMS 1999) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_ams_99_keyword(ident: &str) -> Option<Keyword> {
	VHDL_AMS_99_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076.1-1999 (VHDL-AMS 1999) keyword
///
/// This is used rather than [`get_vhdl_ams_99_keyword`] to test if the
/// found identifier for a lower VHDL-AMS standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_ams_99_keyword(ident: &str) -> bool {
	VHDL_AMS_99_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076.1-2007 (VHDL-AMS 2007) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_ams_07_keyword(ident: &str) -> Option<Keyword> {
	VHDL_AMS_07_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076.1-2007 (VHDL-AMS 2007) keyword
///
/// This is used rather than [`get_vhdl_ams_07_keyword`] to test if the
/// found identifier for a lower VHDL-AMS standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_ams_07_keyword(ident: &str) -> bool {
	VHDL_AMS_07_KEYWORD_SET.contains(ident)
}

/// Get the IEC 61691-6:2009 (VHDL-AMS 2009) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_ams_09_keyword(ident: &str) -> Option<Keyword> {
	VHDL_AMS_09_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEC 61691-6:2009 (VHDL-AMS 2009) keyword
///
/// This is used rather than [`get_vhdl_ams_09_keyword`] to test if the
/// found identifier for a lower VHDL-AMS standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_ams_09_keyword(ident: &str) -> bool {
	VHDL_AMS_09_KEYWORD_SET.contains(ident)
}

/// Get the IEEE 1076.1-2017 (VHDL-AMS 2017) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_ams_17_keyword(ident: &str) -> Option<Keyword> {
	VHDL_AMS_17_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEEE 1076.1-2017 (VHDL-AMS 2017) keyword
///
/// This is used rather than [`get_vhdl_ams_17_keyword`] to test if the
/// found identifier for a lower VHDL-AMS standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_ams_17_keyword(ident: &str) -> bool {
	VHDL_AMS_17_KEYWORD_SET.contains(ident)
}

/// Get the IEC 61691-6:2021 (VHDL-AMS 2021) keyword for the given identifier if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_ams_21_keyword(ident: &str) -> Option<Keyword> {
	VHDL_AMS_21_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a IEC 61691-6:2021 (VHDL-AMS 2021) keyword
///
/// This is used rather than [`get_vhdl_ams_21_keyword`] to test if the
/// found identifier for a lower VHDL-AMS standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[allow(unused)]
#[inline]
pub fn is_vhdl_ams_21_keyword(ident: &str) -> bool {
	VHDL_AMS_21_KEYWORD_SET.contains(ident)
}

/// Get the VHDL-AMS keyword for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_vhdl_ams_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Vhams99 => get_vhdl_ams_99_keyword(ident),
		LanguageStd::Vhams07 => get_vhdl_ams_07_keyword(ident),
		LanguageStd::Vhams09 => get_vhdl_ams_09_keyword(ident),
		LanguageStd::Vhams17 => get_vhdl_ams_17_keyword(ident),
		LanguageStd::Vhams21 => get_vhdl_ams_21_keyword(ident),
		_ => None,
	}
}

/// Get the given VHDL/VHDL-AMS keyword for the given standard if it exists
#[allow(unused)]
#[inline]
pub fn get_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Vh87 => get_vhdl_87_keyword(ident),
		LanguageStd::Vh93 => get_vhdl_93_keyword(ident),
		LanguageStd::Vh2k => get_vhdl_2k_keyword(ident),
		LanguageStd::Vh02 => get_vhdl_02_keyword(ident),
		LanguageStd::Vh04 => get_vhdl_04_keyword(ident),
		LanguageStd::Vh07 => get_vhdl_07_keyword(ident),
		LanguageStd::Vh08 => get_vhdl_08_keyword(ident),
		LanguageStd::Vh11 => get_vhdl_11_keyword(ident),
		LanguageStd::Vh19 => get_vhdl_19_keyword(ident),
		LanguageStd::Vh23 => get_vhdl_23_keyword(ident),
		LanguageStd::Vhams99 => get_vhdl_ams_99_keyword(ident),
		LanguageStd::Vhams07 => get_vhdl_ams_07_keyword(ident),
		LanguageStd::Vhams09 => get_vhdl_ams_09_keyword(ident),
		LanguageStd::Vhams17 => get_vhdl_ams_17_keyword(ident),
		LanguageStd::Vhams21 => get_vhdl_ams_21_keyword(ident),
		_ => None,
	}
}

/// Check to see if the given identifier is a keyword in a future VHDL standard, returning
/// the standard version if so.
#[allow(unused)]
#[inline]
pub fn keyword_in(ident: &str) -> LanguageStd {
	let mut supported = LanguageStd::none();

	if is_vhdl_87_keyword(ident) {
		supported |= LanguageStd::Vh87;
	}

	if is_vhdl_93_keyword(ident) {
		supported |= LanguageStd::Vh93;
	}

	if is_vhdl_2k_keyword(ident) {
		supported |= LanguageStd::Vh2k;
	}

	if is_vhdl_02_keyword(ident) {
		supported |= LanguageStd::Vh02;
	}

	if is_vhdl_04_keyword(ident) {
		supported |= LanguageStd::Vh04;
	}

	if is_vhdl_07_keyword(ident) {
		supported |= LanguageStd::Vh07;
	}

	if is_vhdl_08_keyword(ident) {
		supported |= LanguageStd::Vh08;
	}

	if is_vhdl_11_keyword(ident) {
		supported |= LanguageStd::Vh11;
	}

	if is_vhdl_19_keyword(ident) {
		supported |= LanguageStd::Vh19;
	}

	if is_vhdl_23_keyword(ident) {
		supported |= LanguageStd::Vh23;
	}

	if is_vhdl_ams_99_keyword(ident) {
		supported |= LanguageStd::Vhams99;
	}

	if is_vhdl_ams_07_keyword(ident) {
		supported |= LanguageStd::Vhams07;
	}

	if is_vhdl_ams_09_keyword(ident) {
		supported |= LanguageStd::Vhams09;
	}

	if is_vhdl_ams_17_keyword(ident) {
		supported |= LanguageStd::Vhams17;
	}

	if is_vhdl_ams_21_keyword(ident) {
		supported |= LanguageStd::Vhams21;
	}

	supported
}
