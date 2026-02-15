// SPDX-License-Identifier: BSD-3-Clause

#[test]
fn test_tokenize_keywords_is_future_verilog_95() {
	use super::keywords::{is_future_verilog_keyword, is_verilog_95_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_95_keyword("nya"));
	assert_eq!(
		is_future_verilog_keyword("nya", VerilogVariant::Verilog(VerilogStd::Vl95)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_verilog_95_keyword("always"));
	assert_eq!(
		is_future_verilog_keyword("always", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl95))
	);

	// One version up
	assert!(!is_verilog_95_keyword("endconfig"));
	assert_eq!(
		is_future_verilog_keyword("endconfig", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl01))
	);

	// Two versions up
	assert!(!is_verilog_95_keyword("uwire"));
	assert_eq!(
		is_future_verilog_keyword("uwire", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl05))
	);

	// Three versions up
	assert!(!is_verilog_95_keyword("shortint"));
	assert_eq!(
		is_future_verilog_keyword("shortint", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
	);

	// Four versions up
	assert!(!is_verilog_95_keyword("accept_on"));
	assert_eq!(
		is_future_verilog_keyword("accept_on", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);

	// Five versions up
	assert!(!is_verilog_95_keyword("soft"));
	assert_eq!(
		is_future_verilog_keyword("soft", VerilogVariant::Verilog(VerilogStd::Vl95)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_verilog_01() {
	use super::keywords::{is_future_verilog_keyword, is_verilog_01_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_01_keyword("meow"));
	assert_eq!(
		is_future_verilog_keyword("meow", VerilogVariant::Verilog(VerilogStd::Vl01)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_verilog_01_keyword("include"));
	assert_eq!(
		is_future_verilog_keyword("include", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl01))
	);

	// One version up
	assert!(!is_verilog_01_keyword("uwire"));
	assert_eq!(
		is_future_verilog_keyword("uwire", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl05))
	);

	// Two versions up
	assert!(!is_verilog_01_keyword("wait_order"));
	assert_eq!(
		is_future_verilog_keyword("wait_order", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
	);

	// Three versions up
	assert!(!is_verilog_01_keyword("s_nexttime"));
	assert_eq!(
		is_future_verilog_keyword("s_nexttime", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);

	// Four versions up
	assert!(!is_verilog_01_keyword("interconnect"));
	assert_eq!(
		is_future_verilog_keyword("interconnect", VerilogVariant::Verilog(VerilogStd::Vl01)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_verilog_05() {
	use super::keywords::{is_future_verilog_keyword, is_verilog_05_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_05_keyword("awoo"));
	assert_eq!(
		is_future_verilog_keyword("awoo", VerilogVariant::Verilog(VerilogStd::Vl05)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_verilog_05_keyword("rtran"));
	assert_eq!(
		is_future_verilog_keyword("rtran", VerilogVariant::Verilog(VerilogStd::Vl05)),
		Some(VerilogVariant::Verilog(VerilogStd::Vl05))
	);

	// One version up
	assert!(!is_verilog_05_keyword("throughout"));
	assert_eq!(
		is_future_verilog_keyword("throughout", VerilogVariant::Verilog(VerilogStd::Vl05)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
	);

	// Two versions up
	assert!(!is_verilog_05_keyword("untyped"));
	assert_eq!(
		is_future_verilog_keyword("untyped", VerilogVariant::Verilog(VerilogStd::Vl05)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);

	// Three versions up
	assert!(!is_verilog_05_keyword("implements"));
	assert_eq!(
		is_future_verilog_keyword("implements", VerilogVariant::Verilog(VerilogStd::Vl05)),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_system_verilog_05() {
	use super::keywords::{is_future_verilog_keyword, is_system_verilog_05_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_05_keyword("kon"));
	assert_eq!(
		is_future_verilog_keyword("kon", VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_system_verilog_05_keyword("property"));
	assert_eq!(
		is_future_verilog_keyword(
			"property",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05))
	);

	// One version up
	assert!(!is_system_verilog_05_keyword("unique0"));
	assert_eq!(
		is_future_verilog_keyword(
			"unique0",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);

	// Two versions up
	assert!(!is_system_verilog_05_keyword("nettype"));
	assert_eq!(
		is_future_verilog_keyword(
			"nettype",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv05)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_system_verilog_09() {
	use super::keywords::{is_future_verilog_keyword, is_system_verilog_09_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_09_keyword("wah"));
	assert_eq!(
		is_future_verilog_keyword("wah", VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_system_verilog_09_keyword("ignore_bins"));
	assert_eq!(
		is_future_verilog_keyword(
			"ignore_bins",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09))
	);

	// One version up
	assert!(!is_system_verilog_09_keyword("implements"));
	assert_eq!(
		is_future_verilog_keyword(
			"implements",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv09)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_system_verilog_12() {
	use super::keywords::{is_future_verilog_keyword, is_system_verilog_12_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_12_keyword("meow"));
	assert_eq!(
		is_future_verilog_keyword(
			"meow",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12)
		),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_system_verilog_12_keyword("soft"));
	assert_eq!(
		is_future_verilog_keyword(
			"soft",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv12))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_system_verilog_17() {
	use super::keywords::{is_future_verilog_keyword, is_system_verilog_17_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_17_keyword("nya"));
	assert_eq!(
		is_future_verilog_keyword("nya", VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_system_verilog_17_keyword("soft"));
	assert_eq!(
		is_future_verilog_keyword(
			"soft",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv17))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_system_verilog_23() {
	use super::keywords::{is_future_verilog_keyword, is_system_verilog_23_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_system_verilog_23_keyword("kon"));
	assert_eq!(
		is_future_verilog_keyword("kon", VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_system_verilog_23_keyword("nettype"));
	assert_eq!(
		is_future_verilog_keyword(
			"nettype",
			VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23)
		),
		Some(VerilogVariant::SystemVerilog(SystemVerilogStd::Sv23))
	);

	// SystemVerilog 2012, 2017, and 2023 all have the same keywords
}

#[test]
fn test_tokenize_keywords_is_future_verilog_ams_09() {
	use super::keywords::{is_future_verilog_keyword, is_verilog_ams_09_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_ams_09_keyword("wah"));
	assert_eq!(
		is_future_verilog_keyword("wah", VerilogVariant::VerilogAms(VerilogAmsStd::Vams09)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_verilog_ams_09_keyword("laplace_zp"));
	assert_eq!(
		is_future_verilog_keyword(
			"laplace_zp",
			VerilogVariant::VerilogAms(VerilogAmsStd::Vams09)
		),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams09))
	);

	// One version up
	assert!(!is_verilog_ams_09_keyword("noise_table_log"));
	assert_eq!(
		is_future_verilog_keyword(
			"noise_table_log",
			VerilogVariant::VerilogAms(VerilogAmsStd::Vams09)
		),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams14))
	);

	// Two versions up
	assert!(!is_verilog_ams_09_keyword("ln1p"));
	assert_eq!(
		is_future_verilog_keyword("ln1p", VerilogVariant::VerilogAms(VerilogAmsStd::Vams09)),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams23))
	);
}

#[test]
fn test_tokenize_keywords_is_future_verilog_ams_14() {
	use super::keywords::{is_future_verilog_keyword, is_verilog_ams_14_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_ams_14_keyword("nya"));
	assert_eq!(
		is_future_verilog_keyword("nya", VerilogVariant::VerilogAms(VerilogAmsStd::Vams14)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_verilog_ams_14_keyword("absdelta"));
	assert_eq!(
		is_future_verilog_keyword(
			"absdelta",
			VerilogVariant::VerilogAms(VerilogAmsStd::Vams14)
		),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams14))
	);

	// One version up
	assert!(!is_verilog_ams_14_keyword("break"));
	assert_eq!(
		is_future_verilog_keyword("break", VerilogVariant::VerilogAms(VerilogAmsStd::Vams14)),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams23))
	);
}

#[test]
fn test_tokenize_keywords_is_future_verilog_ams_23() {
	use super::keywords::{is_future_verilog_keyword, is_verilog_ams_23_keyword};

	// If it's a normal identifier, then we shouldn't get anything back
	assert!(!is_verilog_ams_23_keyword("meow"));
	assert_eq!(
		is_future_verilog_keyword("meow", VerilogVariant::VerilogAms(VerilogAmsStd::Vams23)),
		None
	);

	// If it's a keyword for the current version, it should return the current version
	assert!(is_verilog_ams_23_keyword("expm1"));
	assert_eq!(
		is_future_verilog_keyword("expm1", VerilogVariant::VerilogAms(VerilogAmsStd::Vams23)),
		Some(VerilogVariant::VerilogAms(VerilogAmsStd::Vams23))
	);
}
