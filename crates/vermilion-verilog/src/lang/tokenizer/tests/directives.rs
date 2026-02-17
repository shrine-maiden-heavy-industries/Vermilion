// SPDX-License-Identifier: BSD-3-Clause

#[test]
fn test_tokenize_directive_is_future_verilog_95() {
	use super::directives::{is_future_verilog_directive, is_verilog_95_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_95_directive("nya"));
	assert_eq!(
		is_future_verilog_directive("nya", VerilogVariant::Verilog(VerilogStd::Vl95)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_verilog_95_directive("define"));
	assert_eq!(
		is_future_verilog_directive("define", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl95))
	);

	// One version up
	assert!(!is_verilog_95_directive("elsif"));
	assert_eq!(
		is_future_verilog_directive("elsif", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl01))
	);

	// Two versions up
	assert!(!is_verilog_95_directive("pragma"));
	assert_eq!(
		is_future_verilog_directive("pragma", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl05))
	);

	// Four versions up
	assert!(!is_verilog_95_directive("undefineall"));
	assert_eq!(
		is_future_verilog_directive("undefineall", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);
}

#[test]
fn test_tokenize_directive_is_future_verilog_01() {
	use super::directives::{is_future_verilog_directive, is_verilog_01_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_01_directive("awoo"));
	assert_eq!(
		is_future_verilog_directive("awoo", VerilogVariant::Verilog(VerilogStd::Vl01)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_verilog_01_directive("line"));
	assert_eq!(
		is_future_verilog_directive("line", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl01))
	);

	// One version up
	assert!(!is_verilog_01_directive("pragma"));
	assert_eq!(
		is_future_verilog_directive("pragma", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl05))
	);

	// Three versions up
	assert!(!is_verilog_01_directive("undefineall"));
	assert_eq!(
		is_future_verilog_directive("undefineall", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);
}

#[test]
fn test_tokenize_directive_is_future_verilog_05() {
	use super::directives::{is_future_verilog_directive, is_verilog_05_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_05_directive("kon"));
	assert_eq!(
		is_future_verilog_directive("kon", VerilogVariant::Verilog(VerilogStd::Vl05)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_verilog_05_directive("end_keywords"));
	assert_eq!(
		is_future_verilog_directive("end_keywords", VerilogVariant::Verilog(VerilogStd::Vl05)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl05))
	);

	// Two versions up
	assert!(!is_verilog_05_directive("undefineall"));
	assert_eq!(
		is_future_verilog_directive("undefineall", VerilogVariant::Verilog(VerilogStd::Vl05)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);
}

#[test]
fn test_tokenize_directive_is_future_system_verilog_05() {
	use super::directives::{is_future_verilog_directive, is_system_verilog_05_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_05_directive("meow"));
	assert_eq!(
		is_future_verilog_directive(
			"meow",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05)
		),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_system_verilog_05_directive("begin_keywords"));
	assert_eq!(
		is_future_verilog_directive(
			"begin_keywords",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
	);

	// One version up
	assert!(!is_system_verilog_05_directive("undefineall"));
	assert_eq!(
		is_future_verilog_directive(
			"undefineall",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);
}

#[test]
fn test_tokenize_directive_is_future_system_verilog_09() {
	use super::directives::{is_future_verilog_directive, is_system_verilog_09_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_09_directive("wah"));
	assert_eq!(
		is_future_verilog_directive("wah", VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_system_verilog_09_directive("undefineall"));
	assert_eq!(
		is_future_verilog_directive(
			"undefineall",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);
}

#[test]
fn test_tokenize_directive_is_future_system_verilog_12() {
	use super::directives::{is_future_verilog_directive, is_system_verilog_12_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_12_directive("nya"));
	assert_eq!(
		is_future_verilog_directive("nya", VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_system_verilog_12_directive("delay_mode_distributed"));
	assert_eq!(
		is_future_verilog_directive(
			"delay_mode_distributed",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
	);

	// SystemVerilog 2009, 2012, 2017, and 2023 all have the same directives
}

#[test]
fn test_tokenize_directive_is_future_system_verilog_17() {
	use super::directives::{is_future_verilog_directive, is_system_verilog_17_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_17_directive("awoo"));
	assert_eq!(
		is_future_verilog_directive(
			"awoo",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17)
		),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_system_verilog_17_directive("line"));
	assert_eq!(
		is_future_verilog_directive(
			"line",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
	);

	// SystemVerilog 2009, 2012, 2017, and 2023 all have the same directives
}

#[test]
fn test_tokenize_directive_is_future_system_verilog_23() {
	use super::directives::{is_future_verilog_directive, is_system_verilog_23_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_23_directive("kon"));
	assert_eq!(
		is_future_verilog_directive("kon", VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_system_verilog_23_directive("undefineall"));
	assert_eq!(
		is_future_verilog_directive(
			"undefineall",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
	);

	// SystemVerilog 2009, 2012, 2017, and 2023 all have the same directives
}

#[test]
fn test_tokenize_directive_is_future_verilog_ams_09() {
	use super::directives::{is_future_verilog_directive, is_verilog_ams_09_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_ams_09_directive("meow"));
	assert_eq!(
		is_future_verilog_directive("meow", VerilogVariant::VerilogAms(VerilogAmsStd::Vams09)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_verilog_ams_09_directive("default_discipline"));
	assert_eq!(
		is_future_verilog_directive(
			"default_discipline",
			VerilogVariant::VerilogAms(VerilogAmsStd::Vams09)
		),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams09))
	);

	// Verilog-AMS 2009, 2014, and 2023 all have the same directives
}
#[test]
fn test_tokenize_directive_is_future_verilog_ams_14() {
	use super::directives::{is_future_verilog_directive, is_verilog_ams_14_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_ams_14_directive("wah"));
	assert_eq!(
		is_future_verilog_directive("wah", VerilogVariant::VerilogAms(VerilogAmsStd::Vams14)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_verilog_ams_14_directive("default_transition"));
	assert_eq!(
		is_future_verilog_directive(
			"default_transition",
			VerilogVariant::VerilogAms(VerilogAmsStd::Vams14)
		),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams14))
	);

	// Verilog-AMS 2009, 2014, and 2023 all have the same directives
}

#[test]
fn test_tokenize_directive_is_future_verilog_ams_23() {
	use super::directives::{is_future_verilog_directive, is_verilog_ams_23_directive};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_ams_23_directive("nya"));
	assert_eq!(
		is_future_verilog_directive("nya", VerilogVariant::VerilogAms(VerilogAmsStd::Vams23)),
		None
	);

	// If it's a directive for the current version, it should return the current version
	assert!(is_verilog_ams_23_directive("default_discipline"));
	assert_eq!(
		is_future_verilog_directive(
			"default_discipline",
			VerilogVariant::VerilogAms(VerilogAmsStd::Vams23)
		),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams23))
	);

	// Verilog-AMS 2009, 2014, and 2023 all have the same directives
}
