// SPDX-License-Identifier: BSD-3-Clause

use std::fmt::Display;

use phf::{phf_map, phf_set};

use crate::LanguageStd;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Keyword {
	AddCellsToPBlock, // Added: XDC
	AddToPowerRail,   // Added: XDC
	AllClocks,
	AllCpus,   // Added: XDC
	AllDsps,   // Added: XDC
	AllFanin,  // Added: XDC
	AllFanout, // Added: XDC
	AllFfs,    // Added: XDC
	AllHsios,  // Added: XDC
	AllInputs,
	AllLatches, // Added: XDC
	AllOutputs,
	AllRams,          // Added: XDC
	AllRegisters,     // Added: SDC 1.7
	ConnectDebugPort, // Added: XDC
	CreateClock,
	CreateDebugCore,     // Added: XDC
	CreateDebugPort,     // Added: XDC
	CreateGenerateClock, // Added: SDC 1.3 (2001.08)
	CreateMacro,         // Added: XDC
	CreateNocConnection, // Added: XDC
	CreatePBlock,        // Added: XDC
	CreatePowerRail,     // Added: XDC
	CreateProperty,      // Added: XDC
	CreateVoltageArea,   // Added: SDC 1.6
	CreateWaiver,        // Added: XDC
	CurrentDesign,
	CurrentInstance,
	DeleteMacros,     // Added: XDC
	DeletePBlock,     // Added: XDC
	DeletePowerRails, // Added: XDC
	EndGroup,         // Added: XDC
	Expr,
	Filter,     // Added: XDC
	GetBelPins, // Added: XDC
	GetBels,    // Added: XDC
	GetCells,
	GetClocks,
	GetDebugCores,         // Added: XDC
	GetDebugPorts,         // Added: XDC
	GetGeneratedClocks,    // Added: XDC
	GetHierarchySeparator, // Added: XDC
	GetIoBanks,            // Added: XDC
	GetLibCells,
	GetLibPins,
	GetLibs,
	GetMacros, // Added: XDC
	GetNets,
	GetNocInterfaces, // Added: XDC
	GetNodes,         // Added: XDC
	GetPackagePins,   // Added: XDC
	GetPathGroups,    // Added: XDC
	GetPBlocks,       // Added: XDC
	GetPins,
	GetPips,             // Added: XDC
	GetPkgPinByteGroups, // Added: XDC
	GetPkgPinNibbles,    // Added: XDC
	GetPorts,
	GetPowerRails,   // Added: XDC
	GetProperty,     // Added: XDC
	GetSitePins,     // Added: XDC
	GetSitePips,     // Added: XDC
	GetSites,        // Added: XDC
	GetSlrs,         // Added: XDC
	GetSpeedModels,  // Added: XDC
	GetTiles,        // Added: XDC
	GetTimingArcs,   // Added: XDC
	GetWires,        // Added: XDC
	GroupPath,       // Added: SDC 1.7
	LdcCreateGroup,  // Added: PDC
	LdcCreateMacro,  // Added: PDC
	LdcCreateRegion, // Added: PDC
	LdcCreateVref,   // Added: PDC
	LdcProhibit,     // Added: PDC
	LdcSetAttribute, // Added: PDC
	LdcSetLocation,  // Added: PDC
	LdcSetSysconfig, // Added: PDC
	LdcSetVcc,       // Added: PDC
	List,
	MakeDiffPairPorts,        // Added: XDC
	RemoveCellsFromPBlock,    // Added: XDC
	RemoveFromPowerRail,      // Added: XDC
	ResetOperatingConditions, // Added: XDC
	ResetSwitchingActivity,   // Added: XDC
	ResizePBlock,             // Added: XDC
	Set,
	SetBusSkew, // Added: XDC
	SetCaseAnalysis,
	SetClockGatingCheck,
	SetClockGroups, // Added: SDC 1.7
	SetClockJitter, // Added: SDC 2.2
	SetClockLatency,
	SetClockSense, // Added: SDC 1.7-SDC 2.1
	SetClockTransition,
	SetClockUncertainty,
	SetDataCheck, // Added: SDC 1.4 (2003.03)
	SetDisableTiming,
	SetDrive,
	SetDrivingCell,
	SetExternalDelay, // Added: XDC
	SetFalsePath,
	SetFanoutLoad,
	SetHierarchySeparator,
	SetIdealLatency,    // Added: SDC 1.7
	SetIdealNetwork,    // Added: SDC 1.7
	SetIdealTransition, // Added: SDC 1.7
	SetInputDelay,
	SetInputJitter, // Added: XDC
	SetInputTransition,
	SetLevelShifterStrategy,  // Added: SDC 1.6
	SetLevelShifterThreshold, // Added: SDC 1.6
	SetLoad,
	SetLogicDc,
	SetLogicOne,
	SetLogicUnconnected, // Added: XDC
	SetLogicZero,
	SetMaxArea,
	SetMaxCapacitance,
	SetMaxDelay,
	SetMaxDynamicPower, // Added: SDC 1.4 (2003.03)
	SetMaxFanout,
	SetMaxLeakagePower, // Added: SDC 1.4 (2003.03)
	SetMaxTimeBorrow,
	SetMaxTransition,
	SetMinCapacitance,
	SetMinDelay,
	SetMinPorosity,   // Added: SDC 1.4 (2003.03) Removed SDC 1.7
	SetMinPulseWidth, // Added: SDC 2.0
	SetMulticyclePath,
	SetOperatingConditions,
	SetOutputDelay,
	SetPackagePinVal, // Added: XDC
	SetPortFanoutNumber,
	SetPowerOpt, // Added: XDC
	SetPropagatedClock,
	SetProperty, // Added: XDC
	SetResistance,
	SetSense,             // Added: SDC 2.1
	SetSwitchingActivity, // Added: XDC
	SetSystemJitter,      // Added: XDC
	SetTimingDerate,      // Added: SDC 1.5 (2005.09)
	SetUnits,             // Added: SDC 1.7
	SetVoltage,           // Added: SDC 1.9
	SetWireLoadMinBlockSize,
	SetWireLoadMode,
	SetWireLoadModel,
	SetWireLoadSelectionGroup,
	StartGroup,  // Added: XDC
	UpdateMacro, // Added: XDC
}

/// SDC keyword to [`Keyword`] token map
pub static SDC_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"create_clock" => Keyword::CreateClock,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC keyword set
pub static SDC_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",        "set_clock_gating_check",  "set_max_delay",
	"all_inputs",        "set_clock_latency",       "set_max_fanout",
	"all_outputs",       "set_clock_transition",    "set_max_time_borrow",
	"create_clock",      "set_clock_uncertainty",   "set_max_transition",
	"current_design",    "set_disable_timing",      "set_min_capacitance",
	"current_instance",  "set_drive",               "set_min_delay",
	"expr",              "set_driving_cell",        "set_multicycle_path",
	"get_cells",         "set_false_path",          "set_operating_conditions",
	"get_clocks",        "set_fanout_load",         "set_output_delay",
	"get_lib_cells",     "set_hierarchy_separator", "set_port_fanout_number",
	"get_lib_pins",      "set_input_delay",         "set_propagate_clock",
	"get_libs",          "set_input_transition",    "set_resistance",
	"get_nets",          "set_load",                "set_wire_load_min_block_size",
	"get_pins",          "set_logic_dc",            "set_wire_load_mode",
	"get_ports",         "set_logic_one",           "set_wire_load_model",
	"list",              "set_logic_zero",          "set_wire_load_selection_group",
	"set",               "set_max_area",
	"set_case_analysis", "set_max_capacitance",
};

/// SDC 1.3 keyword to [`Keyword`] token map
pub static SDC_1_3_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 1.3 keyword set
pub static SDC_1_3_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "set_case_analysis",       "set_max_capacitance",
	"all_inputs",             "set_clock_gating_check",  "set_max_delay",
	"all_outputs",            "set_clock_latency",       "set_max_fanout",
	"create_clock",           "set_clock_transition",    "set_max_time_borrow",
	"create_generated_clock", "set_clock_uncertainty",   "set_max_transition",
	"current_design",         "set_disable_timing",      "set_min_capacitance",
	"current_instance",       "set_drive",               "set_min_delay",
	"expr",                   "set_driving_cell",        "set_multicycle_path",
	"get_cells",              "set_false_path",          "set_operating_conditions",
	"get_clocks",             "set_fanout_load",         "set_output_delay",
	"get_lib_cells",          "set_hierarchy_separator", "set_port_fanout_number",
	"get_lib_pins",           "set_input_delay",         "set_propagate_clock",
	"get_libs",               "set_input_transition",    "set_resistance",
	"get_nets",               "set_load",                "set_wire_load_min_block_size",
	"get_pins",               "set_logic_dc",            "set_wire_load_mode",
	"get_ports",              "set_logic_one",           "set_wire_load_model",
	"list",                   "set_logic_zero",          "set_wire_load_selection_group",
	"set",                    "set_max_area",
};

/// SDC 1.4 keyword to [`Keyword`] token map
pub static SDC_1_4_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_min_porosity" => Keyword::SetMinPorosity,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 1.4 keyword set
pub static SDC_1_4_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "set_clock_gating_check",  "set_max_delay",
	"all_inputs",             "set_clock_latency",       "set_max_dynamic_power",
	"all_outputs",            "set_clock_transition",    "set_max_fanout",
	"create_clock",           "set_clock_uncertainty",   "set_max_leakage_power",
	"create_generated_clock", "set_data_check",          "set_max_time_borrow",
	"current_design",         "set_disable_timing",      "set_max_transition",
	"current_instance",       "set_drive",               "set_min_capacitance",
	"expr",                   "set_driving_cell",        "set_min_delay",
	"get_cells",              "set_false_path",          "set_min_porosity",
	"get_clocks",             "set_fanout_load",         "set_multicycle_path",
	"get_lib_cells",          "set_hierarchy_separator", "set_operating_conditions",
	"get_lib_pins",           "set_input_delay",         "set_output_delay",
	"get_libs",               "set_input_transition",    "set_port_fanout_number",
	"get_nets",               "set_load",                "set_propagate_clock",
	"get_pins",               "set_logic_dc",            "set_resistance",
	"get_ports",              "set_logic_one",           "set_wire_load_min_block_size",
	"list",                   "set_logic_zero",          "set_wire_load_mode",
	"set",                    "set_max_area",            "set_wire_load_model",
	"set_case_analysis",      "set_max_capacitance",     "set_wire_load_selection_group",
};

/// SDC 1.5 keyword to [`Keyword`] token map
pub static SDC_1_5_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_min_porosity" => Keyword::SetMinPorosity,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_timing_derate" => Keyword::SetTimingDerate,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 1.5 keyword set
pub static SDC_1_5_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "set_clock_latency",       "set_max_fanout",
	"all_inputs",             "set_clock_transition",    "set_max_leakage_power",
	"all_outputs",            "set_clock_uncertainty",   "set_max_time_borrow",
	"create_clock",           "set_data_check",          "set_max_transition",
	"create_generated_clock", "set_disable_timing",      "set_min_capacitance",
	"current_design",         "set_drive",               "set_min_delay",
	"current_instance",       "set_driving_cell",        "set_min_porosity",
	"expr",                   "set_false_path",          "set_multicycle_path",
	"get_cells",              "set_fanout_load",         "set_operating_conditions",
	"get_clocks",             "set_hierarchy_separator", "set_output_delay",
	"get_lib_cells",          "set_input_delay",         "set_port_fanout_number",
	"get_lib_pins",           "set_input_transition",    "set_propagate_clock",
	"get_libs",               "set_load",                "set_resistance",
	"get_nets",               "set_logic_dc",            "set_timing_derate",
	"get_pins",               "set_logic_one",           "set_wire_load_min_block_size",
	"get_ports",              "set_logic_zero",          "set_wire_load_mode",
	"list",                   "set_max_area",            "set_wire_load_model",
	"set",                    "set_max_capacitance",     "set_wire_load_selection_group",
	"set_case_analysis",      "set_max_delay",
	"set_clock_gating_check", "set_max_dynamic_power",
};

/// SDC 1.6 keyword to [`Keyword`] token map
pub static SDC_1_6_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"create_voltage_area" => Keyword::CreateVoltageArea,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_level_shifter_strategy" => Keyword::SetLevelShifterStrategy,
	"set_level_shifter_threshold" => Keyword::SetLevelShifterThreshold,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_min_porosity" => Keyword::SetMinPorosity,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_timing_derate" => Keyword::SetTimingDerate,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 1.6 keyword set
pub static SDC_1_6_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "set_clock_latency",           "set_max_dynamic_power",
	"all_inputs",             "set_clock_transition",        "set_max_fanout",
	"all_outputs",            "set_clock_uncertainty",       "set_max_leakage_power",
	"create_clock",           "set_data_check",              "set_max_time_borrow",
	"create_generated_clock", "set_disable_timing",          "set_max_transition",
	"create_voltage_area",    "set_drive",                   "set_min_capacitance",
	"current_design",         "set_driving_cell",            "set_min_delay",
	"current_instance",       "set_false_path",              "set_min_porosity",
	"expr",                   "set_fanout_load",             "set_multicycle_path",
	"get_cells",              "set_hierarchy_separator",     "set_operating_conditions",
	"get_clocks",             "set_input_delay",             "set_output_delay",
	"get_lib_cells",          "set_input_transition",        "set_port_fanout_number",
	"get_lib_pins",           "set_level_shifter_strategy",  "set_propagate_clock",
	"get_libs",               "set_level_shifter_threshold", "set_resistance",
	"get_nets",               "set_load",                    "set_timing_derate",
	"get_pins",               "set_logic_dc",                "set_wire_load_min_block_size",
	"get_ports",              "set_logic_one",               "set_wire_load_mode",
	"list",                   "set_logic_zero",              "set_wire_load_model",
	"set",                    "set_max_area",                "set_wire_load_selection_group",
	"set_case_analysis",      "set_max_capacitance",
	"set_clock_gating_check", "set_max_delay",
};

/// SDC 1.7 keyword to [`Keyword`] token map
pub static SDC_1_7_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"all_registers" => Keyword::AllRegisters,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"create_voltage_area" => Keyword::CreateVoltageArea,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"group_path" => Keyword::GroupPath,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"get_clock_groups" => Keyword::SetClockGroups,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_sense" => Keyword::SetClockSense,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_ideal_latency" => Keyword::SetIdealLatency,
	"set_ideal_network" => Keyword::SetIdealNetwork,
	"set_ideal_transition" => Keyword::SetIdealTransition,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_level_shifter_strategy" => Keyword::SetLevelShifterStrategy,
	"set_level_shifter_threshold" => Keyword::SetLevelShifterThreshold,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_min_porosity" => Keyword::SetMinPorosity,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_timing_derate" => Keyword::SetTimingDerate,
	"set_units" => Keyword::SetUnits,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 1.7 keyword set
pub static SDC_1_7_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "get_clock_groups",            "set_max_area",
	"all_inputs",             "set_clock_latency",           "set_max_capacitance",
	"all_outputs",            "set_clock_sense",             "set_max_delay",
	"all_registers",          "set_clock_transition",        "set_max_dynamic_power",
	"create_clock",           "set_clock_uncertainty",       "set_max_fanout",
	"create_generated_clock", "set_data_check",              "set_max_leakage_power",
	"create_voltage_area",    "set_disable_timing",          "set_max_time_borrow",
	"current_design",         "set_drive",                   "set_max_transition",
	"current_instance",       "set_driving_cell",            "set_min_capacitance",
	"expr",                   "set_false_path",              "set_min_delay",
	"get_cells",              "set_fanout_load",             "set_min_porosity",
	"get_clocks",             "set_hierarchy_separator",     "set_multicycle_path",
	"get_lib_cells",          "set_ideal_latency",           "set_operating_conditions",
	"get_lib_pins",           "set_ideal_network",           "set_output_delay",
	"get_libs",               "set_ideal_transition",        "set_port_fanout_number",
	"get_nets",               "set_input_delay",             "set_propagate_clock",
	"get_pins",               "set_input_transition",        "set_resistance",
	"get_ports",              "set_level_shifter_strategy",  "set_timing_derate",
	"group_path",             "set_level_shifter_threshold", "set_units",
	"list",                   "set_load",                    "set_wire_load_min_block_size",
	"set",                    "set_logic_dc",                "set_wire_load_mode",
	"set_case_analysis",      "set_logic_one",               "set_wire_load_model",
	"set_clock_gating_check", "set_logic_zero",              "set_wire_load_selection_group",
};

/// SDC 1.9 keyword to [`Keyword`] token map
pub static SDC_1_9_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"all_registers" => Keyword::AllRegisters,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"create_voltage_area" => Keyword::CreateVoltageArea,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"group_path" => Keyword::GroupPath,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"get_clock_groups" => Keyword::SetClockGroups,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_sense" => Keyword::SetClockSense,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_ideal_latency" => Keyword::SetIdealLatency,
	"set_ideal_network" => Keyword::SetIdealNetwork,
	"set_ideal_transition" => Keyword::SetIdealTransition,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_level_shifter_strategy" => Keyword::SetLevelShifterStrategy,
	"set_level_shifter_threshold" => Keyword::SetLevelShifterThreshold,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_timing_derate" => Keyword::SetTimingDerate,
	"set_units" => Keyword::SetUnits,
	"set_voltage" => Keyword::SetVoltage,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 1.9 keyword set
pub static SDC_1_9_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "get_clock_groups",            "set_max_area",
	"all_inputs",             "set_clock_latency",           "set_max_capacitance",
	"all_outputs",            "set_clock_sense",             "set_max_delay",
	"all_registers",          "set_clock_transition",        "set_max_dynamic_power",
	"create_clock",           "set_clock_uncertainty",       "set_max_fanout",
	"create_generated_clock", "set_data_check",              "set_max_leakage_power",
	"create_voltage_area",    "set_disable_timing",          "set_max_time_borrow",
	"current_design",         "set_drive",                   "set_max_transition",
	"current_instance",       "set_driving_cell",            "set_min_capacitance",
	"expr",                   "set_false_path",              "set_min_delay",
	"get_cells",              "set_fanout_load",             "set_multicycle_path",
	"get_clocks",             "set_hierarchy_separator",     "set_operating_conditions",
	"get_lib_cells",          "set_ideal_latency",           "set_output_delay",
	"get_lib_pins",           "set_ideal_network",           "set_port_fanout_number",
	"get_libs",               "set_ideal_transition",        "set_propagate_clock",
	"get_nets",               "set_input_delay",             "set_resistance",
	"get_pins",               "set_input_transition",        "set_timing_derate",
	"get_ports",              "set_level_shifter_strategy",  "set_units",
	"group_path",             "set_level_shifter_threshold", "set_voltage",
	"list",                   "set_load",                    "set_wire_load_min_block_size",
	"set",                    "set_logic_dc",                "set_wire_load_mode",
	"set_case_analysis",      "set_logic_one",               "set_wire_load_model",
	"set_clock_gating_check", "set_logic_zero",              "set_wire_load_selection_group",
};

/// SDC 2.0 keyword to [`Keyword`] token map
pub static SDC_2_0_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"all_registers" => Keyword::AllRegisters,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"create_voltage_area" => Keyword::CreateVoltageArea,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"group_path" => Keyword::GroupPath,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"get_clock_groups" => Keyword::SetClockGroups,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_sense" => Keyword::SetClockSense,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_ideal_latency" => Keyword::SetIdealLatency,
	"set_ideal_network" => Keyword::SetIdealNetwork,
	"set_ideal_transition" => Keyword::SetIdealTransition,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_level_shifter_strategy" => Keyword::SetLevelShifterStrategy,
	"set_level_shifter_threshold" => Keyword::SetLevelShifterThreshold,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_min_pulse_width" => Keyword::SetMinPulseWidth,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_timing_derate" => Keyword::SetTimingDerate,
	"set_units" => Keyword::SetUnits,
	"set_voltage" => Keyword::SetVoltage,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 2.0 keyword set
pub static SDC_2_0_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "set_clock_latency",           "set_max_delay",
	"all_inputs",             "set_clock_sense",             "set_max_dynamic_power",
	"all_outputs",            "set_clock_transition",        "set_max_fanout",
	"all_registers",          "set_clock_uncertainty",       "set_max_leakage_power",
	"create_clock",           "set_data_check",              "set_max_time_borrow",
	"create_generated_clock", "set_disable_timing",          "set_max_transition",
	"create_voltage_area",    "set_drive",                   "set_min_capacitance",
	"current_design",         "set_driving_cell",            "set_min_delay",
	"current_instance",       "set_false_path",              "set_min_pulse_width",
	"expr",                   "set_fanout_load",             "set_multicycle_path",
	"get_cells",              "set_hierarchy_separator",     "set_operating_conditions",
	"get_clocks",             "set_ideal_latency",           "set_output_delay",
	"get_lib_cells",          "set_ideal_network",           "set_port_fanout_number",
	"get_lib_pins",           "set_ideal_transition",        "set_propagate_clock",
	"get_libs",               "set_input_delay",             "set_resistance",
	"get_nets",               "set_input_transition",        "set_timing_derate",
	"get_pins",               "set_level_shifter_strategy",  "set_units",
	"get_ports",              "set_level_shifter_threshold", "set_voltage",
	"group_path",             "set_load",                    "set_wire_load_min_block_size",
	"list",                   "set_logic_dc",                "set_wire_load_mode",
	"set",                    "set_logic_one",               "set_wire_load_model",
	"set_case_analysis",      "set_logic_zero",              "set_wire_load_selection_group",
	"set_clock_gating_check", "set_max_area",
	"get_clock_groups",       "set_max_capacitance",
};

/// SDC 2.1 keyword to [`Keyword`] token map
pub static SDC_2_1_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"all_registers" => Keyword::AllRegisters,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"create_voltage_area" => Keyword::CreateVoltageArea,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"group_path" => Keyword::GroupPath,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"get_clock_groups" => Keyword::SetClockGroups,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_ideal_latency" => Keyword::SetIdealLatency,
	"set_ideal_network" => Keyword::SetIdealNetwork,
	"set_ideal_transition" => Keyword::SetIdealTransition,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_level_shifter_strategy" => Keyword::SetLevelShifterStrategy,
	"set_level_shifter_threshold" => Keyword::SetLevelShifterThreshold,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_min_pulse_width" => Keyword::SetMinPulseWidth,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_sense" => Keyword::SetSense,
	"set_timing_derate" => Keyword::SetTimingDerate,
	"set_units" => Keyword::SetUnits,
	"set_voltage" => Keyword::SetVoltage,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 2.1 keyword set
pub static SDC_2_1_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "set_clock_latency",           "set_max_dynamic_power",
	"all_inputs",             "set_clock_transition",        "set_max_fanout",
	"all_outputs",            "set_clock_uncertainty",       "set_max_leakage_power",
	"all_registers",          "set_data_check",              "set_max_time_borrow",
	"create_clock",           "set_disable_timing",          "set_max_transition",
	"create_generated_clock", "set_drive",                   "set_min_capacitance",
	"create_voltage_area",    "set_driving_cell",            "set_min_delay",
	"current_design",         "set_false_path",              "set_min_pulse_width",
	"current_instance",       "set_fanout_load",             "set_multicycle_path",
	"expr",                   "set_hierarchy_separator",     "set_operating_conditions",
	"get_cells",              "set_ideal_latency",           "set_output_delay",
	"get_clocks",             "set_ideal_network",           "set_port_fanout_number",
	"get_lib_cells",          "set_ideal_transition",        "set_propagate_clock",
	"get_lib_pins",           "set_input_delay",             "set_resistance",
	"get_libs",               "set_input_transition",        "set_sense",
	"get_nets",               "set_level_shifter_strategy",  "set_timing_derate",
	"get_pins",               "set_level_shifter_threshold", "set_units",
	"get_ports",              "set_load",                    "set_voltage",
	"group_path",             "set_logic_dc",                "set_wire_load_min_block_size",
	"list",                   "set_logic_one",               "set_wire_load_mode",
	"set",                    "set_logic_zero",              "set_wire_load_model",
	"set_case_analysis",      "set_max_area",                "set_wire_load_selection_group",
	"set_clock_gating_check", "set_max_capacitance",
	"get_clock_groups",       "set_max_delay",
};

/// SDC 2.2 keyword to [`Keyword`] token map
pub static SDC_2_2_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"all_clocks" => Keyword::AllClocks,
	"all_inputs" => Keyword::AllInputs,
	"all_outputs" => Keyword::AllOutputs,
	"all_registers" => Keyword::AllRegisters,
	"create_clock" => Keyword::CreateClock,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"create_voltage_area" => Keyword::CreateVoltageArea,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"expr" => Keyword::Expr,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_lib_cells" => Keyword::GetLibCells,
	"get_lib_pins" => Keyword::GetLibPins,
	"get_libs" => Keyword::GetLibs,
	"get_nets" => Keyword::GetNets,
	"get_pins" => Keyword::GetPins,
	"get_ports" => Keyword::GetPorts,
	"group_path" => Keyword::GroupPath,
	"list" => Keyword::List,
	"set" => Keyword::Set,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_gating_check" => Keyword::SetClockGatingCheck,
	"get_clock_groups" => Keyword::SetClockGroups,
	"set_clock_jitter" => Keyword::SetClockJitter,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_transition" => Keyword::SetClockTransition,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_drive" => Keyword::SetDrive,
	"set_driving_cell" => Keyword::SetDrivingCell,
	"set_false_path" => Keyword::SetFalsePath,
	"set_fanout_load" => Keyword::SetFanoutLoad,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_ideal_latency" => Keyword::SetIdealLatency,
	"set_ideal_network" => Keyword::SetIdealNetwork,
	"set_ideal_transition" => Keyword::SetIdealTransition,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_transition" => Keyword::SetInputTransition,
	"set_level_shifter_strategy" => Keyword::SetLevelShifterStrategy,
	"set_level_shifter_threshold" => Keyword::SetLevelShifterThreshold,
	"set_load" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_area" => Keyword::SetMaxArea,
	"set_max_capacitance" => Keyword::SetMaxCapacitance,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_dynamic_power" => Keyword::SetMaxDynamicPower,
	"set_max_fanout" => Keyword::SetMaxFanout,
	"set_max_leakage_power" => Keyword::SetMaxLeakagePower,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_max_transition" => Keyword::SetMaxTransition,
	"set_min_capacitance" => Keyword::SetMinCapacitance,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_min_pulse_width" => Keyword::SetMinPulseWidth,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_port_fanout_number" => Keyword::SetPortFanoutNumber,
	"set_propagate_clock" => Keyword::SetPropagatedClock,
	"set_resistance" => Keyword::SetResistance,
	"set_sense" => Keyword::SetSense,
	"set_timing_derate" => Keyword::SetTimingDerate,
	"set_units" => Keyword::SetUnits,
	"set_voltage" => Keyword::SetVoltage,
	"set_wire_load_min_block_size" => Keyword::SetWireLoadMinBlockSize,
	"set_wire_load_mode" => Keyword::SetWireLoadMode,
	"set_wire_load_model" => Keyword::SetWireLoadModel,
	"set_wire_load_selection_group" => Keyword::SetWireLoadSelectionGroup,
};

/// SDC 2.2 keyword set
pub static SDC_2_2_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"all_clocks",             "set_clock_jitter",            "set_max_delay",
	"all_inputs",             "set_clock_latency",           "set_max_dynamic_power",
	"all_outputs",            "set_clock_transition",        "set_max_fanout",
	"all_registers",          "set_clock_uncertainty",       "set_max_leakage_power",
	"create_clock",           "set_data_check",              "set_max_time_borrow",
	"create_generated_clock", "set_disable_timing",          "set_max_transition",
	"create_voltage_area",    "set_drive",                   "set_min_capacitance",
	"current_design",         "set_driving_cell",            "set_min_delay",
	"current_instance",       "set_false_path",              "set_min_pulse_width",
	"expr",                   "set_fanout_load",             "set_multicycle_path",
	"get_cells",              "set_hierarchy_separator",     "set_operating_conditions",
	"get_clocks",             "set_ideal_latency",           "set_output_delay",
	"get_lib_cells",          "set_ideal_network",           "set_port_fanout_number",
	"get_lib_pins",           "set_ideal_transition",        "set_propagate_clock",
	"get_libs",               "set_input_delay",             "set_resistance",
	"get_nets",               "set_input_transition",        "set_sense",
	"get_pins",               "set_level_shifter_strategy",  "set_timing_derate",
	"get_ports",              "set_level_shifter_threshold", "set_units",
	"group_path",             "set_load",                    "set_voltage",
	"list",                   "set_logic_dc",                "set_wire_load_min_block_size",
	"set",                    "set_logic_one",               "set_wire_load_mode",
	"set_case_analysis",      "set_logic_zero",              "set_wire_load_model",
	"set_clock_gating_check", "set_max_area",                "set_wire_load_selection_group",
	"get_clock_groups",       "set_max_capacitance",
};

/// XDC keyword to [`Keyword`] token map
pub static XDC_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"add_cells_to_pblock" => Keyword::AddCellsToPBlock,
	"add_to_power_rail" => Keyword::AddToPowerRail,
	"all_clocks" => Keyword::AllClocks,
	"all_cpus" => Keyword::AllCpus,
	"all_dsps" => Keyword::AllDsps,
	"all_fanin" => Keyword::AllFanin,
	"all_fanout" => Keyword::AllFanout,
	"all_ffs" => Keyword::AllFfs,
	"all_hsios" => Keyword::AllHsios,
	"all_inputs" => Keyword::AllInputs,
	"all_latches" => Keyword::AllLatches,
	"all_outputs" => Keyword::AllOutputs,
	"all_rams" => Keyword::AllRams,
	"all_registers" => Keyword::AllRegisters,
	"connect_debug_port" => Keyword::ConnectDebugPort,
	"create_clock" => Keyword::CreateClock,
	"create_debug_core" => Keyword::CreateDebugCore,
	"create_debug_port" => Keyword::CreateDebugPort,
	"create_generated_clock" => Keyword::CreateGenerateClock,
	"crate_macro" => Keyword::CreateMacro,
	"create_noc_connection" => Keyword::CreateNocConnection,
	"create_pblock" => Keyword::CreatePBlock,
	"create_power_rail" => Keyword::CreatePowerRail,
	"create_property" => Keyword::CreateProperty,
	"create_waiver" => Keyword::CreateWaiver,
	"current_design" => Keyword::CurrentDesign,
	"current_instance" => Keyword::CurrentInstance,
	"delete_macros" => Keyword::DeleteMacros,
	"delete_pblock" => Keyword::DeletePBlock,
	"delete_power_rails" => Keyword::DeletePowerRails,
	"endgroup" => Keyword::EndGroup,
	"expr" => Keyword::Expr,
	"filter" => Keyword::Filter,
	"get_bel_pins" => Keyword::GetBelPins,
	"get_bels" => Keyword::GetBels,
	"get_cells" => Keyword::GetCells,
	"get_clocks" => Keyword::GetClocks,
	"get_debug_cores" => Keyword::GetDebugCores,
	"get_debug_ports" => Keyword::GetDebugPorts,
	"get_generated_clocks" => Keyword::GetGeneratedClocks,
	"get_hierarchy_separator" => Keyword::GetHierarchySeparator,
	"get_iobanks" => Keyword::GetIoBanks,
	"get_macros" => Keyword::GetMacros,
	"get_nets" => Keyword::GetNets,
	"get_noc_interfaces" => Keyword::GetNocInterfaces,
	"get_nodes" => Keyword::GetNodes,
	"get_package_pins" => Keyword::GetPackagePins,
	"get_path_groups" => Keyword::GetPathGroups,
	"get_pblocks" => Keyword::GetPBlocks,
	"get_pins" => Keyword::GetPins,
	"get_pips" => Keyword::GetPips,
	"get_pkgpin_bytegroups" => Keyword::GetPkgPinByteGroups,
	"get_pkgpin_nibbles" => Keyword::GetPkgPinNibbles,
	"get_ports" => Keyword::GetPorts,
	"get_power_rails" => Keyword::GetPowerRails,
	"get_property" => Keyword::GetProperty,
	"get_site_pins" => Keyword::GetSitePins,
	"get_site_pips" => Keyword::GetSitePips,
	"get_sites" => Keyword::GetSites,
	"get_slrs" => Keyword::GetSlrs,
	"get_speed_models" => Keyword::GetSpeedModels,
	"get_tiles" => Keyword::GetTiles,
	"get_timing_arcs" => Keyword::GetTimingArcs,
	"get_wires" => Keyword::GetWires,
	"group_path" => Keyword::GroupPath,
	"list" => Keyword::List,
	"make_diff_pair_ports" => Keyword::MakeDiffPairPorts,
	"remove_cells_from_pblock" => Keyword::RemoveCellsFromPBlock,
	"remove_from_power_rail" => Keyword::RemoveFromPowerRail,
	"reset_operating_conditions" => Keyword::ResetOperatingConditions,
	"reset_switching_activity" => Keyword::ResetSwitchingActivity,
	"resize_pblock" => Keyword::ResizePBlock,
	"set" => Keyword::Set,
	"set_bus_skew" => Keyword::SetBusSkew,
	"set_case_analysis" => Keyword::SetCaseAnalysis,
	"set_clock_groups" => Keyword::SetClockGroups,
	"set_clock_latency" => Keyword::SetClockLatency,
	"set_clock_sense" => Keyword::SetClockSense,
	"set_clock_uncertainty" => Keyword::SetClockUncertainty,
	"set_data_check" => Keyword::SetDataCheck,
	"set_disable_timing" => Keyword::SetDisableTiming,
	"set_external_delay" => Keyword::SetExternalDelay,
	"set_false_path" => Keyword::SetFalsePath,
	"set_hierarchy_separator" => Keyword::SetHierarchySeparator,
	"set_input_delay" => Keyword::SetInputDelay,
	"set_input_jitter" => Keyword::SetInputJitter,
	"set_loa" => Keyword::SetLoad,
	"set_logic_dc" => Keyword::SetLogicDc,
	"set_logic_one" => Keyword::SetLogicOne,
	"set_logic_unconnected" => Keyword::SetLogicUnconnected,
	"set_logic_zero" => Keyword::SetLogicZero,
	"set_max_delay" => Keyword::SetMaxDelay,
	"set_max_time_borrow" => Keyword::SetMaxTimeBorrow,
	"set_min_delay" => Keyword::SetMinDelay,
	"set_multicycle_path" => Keyword::SetMulticyclePath,
	"set_operating_conditions" => Keyword::SetOperatingConditions,
	"set_output_delay" => Keyword::SetOutputDelay,
	"set_package_pin_val" => Keyword::SetPackagePinVal,
	"set_power_opt" => Keyword::SetPowerOpt,
	"set_propagated_clock" => Keyword::SetPropagatedClock,
	"set_property" => Keyword::SetProperty,
	"set_switching_activity" => Keyword::SetSwitchingActivity,
	"set_system_jitter" => Keyword::SetSystemJitter,
	"set_units" => Keyword::SetUnits,
	"startgroup" => Keyword::StartGroup,
	"update_macro" => Keyword::UpdateMacro,
};

/// XDC keyword set
pub static XDC_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"add_cells_to_pblock",    "get_clocks",                 "set",
	"add_to_power_rail",      "get_debug_cores",            "set_bus_skew",
	"all_clocks",             "get_debug_ports",            "set_case_analysis",
	"all_cpus",               "get_generated_clocks",       "set_clock_groups",
	"all_dsps",               "get_hierarchy_separator",    "set_clock_latency",
	"all_fanin",              "get_iobanks",                "set_clock_sense",
	"all_fanout",             "get_macros",                 "set_clock_uncertainty",
	"all_ffs",                "get_nets",                   "set_data_check",
	"all_hsios",              "get_noc_interfaces",         "set_disable_timing",
	"all_inputs",             "get_nodes",                  "set_external_delay",
	"all_latches",            "get_package_pins",           "set_false_path",
	"all_outputs",            "get_path_groups",            "set_hierarchy_separator",
	"all_rams",               "get_pblocks",                "set_input_delay",
	"all_registers",          "get_pins",                   "set_input_jitter",
	"connect_debug_port",     "get_pips",                   "set_loa",
	"create_clock",           "get_pkgpin_bytegroups",      "set_logic_dc",
	"create_debug_core",      "get_pkgpin_nibbles",         "set_logic_one",
	"create_debug_port",      "get_ports",                  "set_logic_unconnected",
	"create_generated_clock", "get_power_rails",            "set_logic_zero",
	"crate_macro",            "get_property",               "set_max_delay",
	"create_noc_connection",  "get_site_pins",              "set_max_time_borrow",
	"create_pblock",          "get_site_pips",              "set_min_delay",
	"create_power_rail",      "get_sites",                  "set_multicycle_path",
	"create_property",        "get_slrs",                   "set_operating_conditions",
	"create_waiver",          "get_speed_models",           "set_output_delay",
	"current_design",         "get_tiles",                  "set_package_pin_val",
	"current_instance",       "get_timing_arcs",            "set_power_opt",
	"delete_macros",          "get_wires",                  "set_propagated_clock",
	"delete_pblock",          "group_path",                 "set_property",
	"delete_power_rails",     "list",                       "set_switching_activity",
	"endgroup",               "make_diff_pair_ports",       "set_system_jitter",
	"expr",                   "remove_cells_from_pblock",   "set_units",
	"filter",                 "remove_from_power_rail",     "startgroup",
	"get_bel_pins",           "reset_operating_conditions", "update_macro",
	"get_bels",               "reset_switching_activity",
	"get_cells",              "resize_pblock",
};

/// PDC keyword to [`Keyword`] token map
pub static PDC_KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {
	"ldc_create_group" => Keyword::LdcCreateGroup,
	"ldc_create_macro" => Keyword::LdcCreateMacro,
	"ldc_create_region" => Keyword::LdcCreateRegion,
	"ldc_create_vref" => Keyword::LdcCreateVref,
	"ldc_prohibit" => Keyword::LdcProhibit,
	"ldc_set_attribute" => Keyword::LdcSetAttribute,
	"ldc_set_location" => Keyword::LdcSetLocation,
	"ldc_set_sysconfig" => Keyword::LdcSetSysconfig,
	"ldc_set_vcc" => Keyword::LdcSetVcc,
};

/// PDC keyword set
pub static PDC_KEYWORD_SET: phf::Set<&'static str> = phf_set! {
	"ldc_create_group",  "ldc_create_vref",   "ldc_set_location",
	"ldc_create_macro",  "ldc_prohibit",      "ldc_set_sysconfig",
	"ldc_create_region", "ldc_set_attribute", "ldc_set_vcc",
};

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_keyword(ident: &str) -> Option<Keyword> {
	SDC_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_keyword(ident: &str) -> bool {
	SDC_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_1_3_keyword(ident: &str) -> Option<Keyword> {
	SDC_1_3_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_1_3_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_1_3_keyword(ident: &str) -> bool {
	SDC_1_3_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_1_4_keyword(ident: &str) -> Option<Keyword> {
	SDC_1_4_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_1_4_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_1_4_keyword(ident: &str) -> bool {
	SDC_1_4_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_1_5_keyword(ident: &str) -> Option<Keyword> {
	SDC_1_5_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_1_5_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_1_5_keyword(ident: &str) -> bool {
	SDC_1_5_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_1_6_keyword(ident: &str) -> Option<Keyword> {
	SDC_1_6_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_1_6_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_1_6_keyword(ident: &str) -> bool {
	SDC_1_6_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_1_7_keyword(ident: &str) -> Option<Keyword> {
	SDC_1_7_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_1_7_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_1_7_keyword(ident: &str) -> bool {
	SDC_1_7_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_1_9_keyword(ident: &str) -> Option<Keyword> {
	SDC_1_9_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_1_9_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_1_9_keyword(ident: &str) -> bool {
	SDC_1_9_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_2_0_keyword(ident: &str) -> Option<Keyword> {
	SDC_2_0_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_2_0_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_2_0_keyword(ident: &str) -> bool {
	SDC_2_0_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_2_1_keyword(ident: &str) -> Option<Keyword> {
	SDC_2_1_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_2_1_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_2_1_keyword(ident: &str) -> bool {
	SDC_2_1_KEYWORD_SET.contains(ident)
}

/// Get the SDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_sdc_2_2_keyword(ident: &str) -> Option<Keyword> {
	SDC_2_2_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a SDC keyword
///
/// This is used rather than [`get_sdc_2_2_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_sdc_2_2_keyword(ident: &str) -> bool {
	SDC_2_2_KEYWORD_SET.contains(ident)
}

/// Get the XDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_xdc_keyword(ident: &str) -> Option<Keyword> {
	XDC_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a XDC keyword
///
/// This is used rather than [`get_xdc_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_xdc_keyword(ident: &str) -> bool {
	XDC_KEYWORD_SET.contains(ident)
}

/// Get the PDC keyword for the given identifier if it exists
#[inline(always)]
pub fn get_pdc_keyword(ident: &str) -> Option<Keyword> {
	PDC_KEYWORD_MAP.get(ident).copied()
}

/// Check to see if the given identifier is a PDC keyword
///
/// This is used rather than [`get_pdc_keyword`] to test if the
/// found identifier for a lower standard is a keyword in this
/// standard.
///
/// It prevents the copy into an [`Option`] if it is a keyword.
#[inline(always)]
pub fn is_pdc_keyword(ident: &str) -> bool {
	PDC_KEYWORD_SET.contains(ident)
}

/// Get the given SDC/XDC/PDC keyword for the given standard if it exists
#[inline(always)]
pub fn get_keyword(ident: &str, std: LanguageStd) -> Option<Keyword> {
	match std {
		LanguageStd::Pdc => get_pdc_keyword(ident),
		LanguageStd::Sdc => get_sdc_keyword(ident),
		LanguageStd::Sdc_1_3 => get_sdc_1_3_keyword(ident),
		LanguageStd::Sdc_1_4 => get_sdc_1_4_keyword(ident),
		LanguageStd::Sdc_1_5 => get_sdc_1_5_keyword(ident),
		LanguageStd::Sdc_1_6 => get_sdc_1_6_keyword(ident),
		LanguageStd::Sdc_1_7 => get_sdc_1_7_keyword(ident),
		LanguageStd::Sdc_1_9 => get_sdc_1_9_keyword(ident),
		LanguageStd::Sdc_2_0 => get_sdc_2_0_keyword(ident),
		LanguageStd::Sdc_2_1 => get_sdc_2_1_keyword(ident),
		LanguageStd::Sdc_2_2 => get_sdc_2_2_keyword(ident),
		LanguageStd::Xdc => get_xdc_keyword(ident),
		_ => None,
	}
}

/// Check to see if the given identifier is an SDC/PDC/XDC keyword returning, the standard version
/// if so.
#[inline(always)]
pub fn is_keyword(ident: &str) -> LanguageStd {
	let mut supported = LanguageStd::none();

	if is_sdc_keyword(ident) {
		supported |= LanguageStd::Sdc;
	}

	if is_sdc_1_3_keyword(ident) {
		supported |= LanguageStd::Sdc_1_3;
	}

	if is_sdc_1_4_keyword(ident) {
		supported |= LanguageStd::Sdc_1_4;
	}

	if is_sdc_1_5_keyword(ident) {
		supported |= LanguageStd::Sdc_1_5;
	}

	if is_sdc_1_6_keyword(ident) {
		supported |= LanguageStd::Sdc_1_6;
	}

	if is_sdc_1_7_keyword(ident) {
		supported |= LanguageStd::Sdc_1_7;
	}

	if is_sdc_1_9_keyword(ident) {
		supported |= LanguageStd::Sdc_1_9;
	}

	if is_sdc_2_0_keyword(ident) {
		supported |= LanguageStd::Sdc_2_0;
	}

	if is_sdc_2_1_keyword(ident) {
		supported |= LanguageStd::Sdc_2_1;
	}

	if is_sdc_2_2_keyword(ident) {
		supported |= LanguageStd::Sdc_2_2;
	}

	if is_xdc_keyword(ident) {
		supported |= LanguageStd::Xdc;
	}

	if is_pdc_keyword(ident) {
		supported |= LanguageStd::Pdc;
	}

	supported
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::AddCellsToPBlock => "add_cells_to_pblock", // Added: XDC
				Self::AddToPowerRail => "add_to_power_rail",     // Added: XDC
				Self::AllClocks => "all_clocks",
				Self::AllCpus => "all_cpus",     // Added: XDC
				Self::AllDsps => "all_dsps",     // Added: XDC
				Self::AllFanin => "all_fanin",   // Added: XDC
				Self::AllFanout => "all_fanout", // Added: XDC
				Self::AllFfs => "all_ffs",       // Added: XDC
				Self::AllHsios => "all_hsios",   // Added: XDC
				Self::AllInputs => "all_inputs",
				Self::AllLatches => "all_latches", // Added: XDC
				Self::AllOutputs => "all_outputs",
				Self::AllRams => "all_rams",                    // Added: XDC
				Self::AllRegisters => "all_registers",          // Added: SDC 1.7
				Self::ConnectDebugPort => "connect_debug_port", // Added: XDC
				Self::CreateClock => "create_clock",
				Self::CreateDebugCore => "create_debug_core", // Added: XDC
				Self::CreateDebugPort => "create_debug_port", // Added: XDC
				Self::CreateGenerateClock => "create_generated_clock", // Added: SDC 1.3 (2001.08)
				Self::CreateMacro => "create_macro",          // Added: XDC
				Self::CreateNocConnection => "create_noc_connection", // Added: XDC
				Self::CreatePBlock => "crate_pblock",         // Added: XDC
				Self::CreatePowerRail => "create_power_rail", // Added: XDC
				Self::CreateProperty => "create_property",    // Added: XDC
				Self::CreateVoltageArea => "create_voltage_area", // Added: SDC 1.6
				Self::CreateWaiver => "create_waiver",        // Added: XDC
				Self::CurrentDesign => "current_design",
				Self::CurrentInstance => "current_instance",
				Self::DeleteMacros => "delete_macros", // Added: XDC
				Self::DeletePBlock => "delete_pblock", // Added: XDC
				Self::DeletePowerRails => "delete_power_rails", // Added: XDC
				Self::EndGroup => "endgroup",          // Added: XDC
				Self::Expr => "expr",
				Self::Filter => "filter",           // Added: XDC
				Self::GetBelPins => "get_bel_pins", // Added: XDC
				Self::GetBels => "get_bels",        // Added: XDC
				Self::GetCells => "get_cells",
				Self::GetClocks => "get_clocks",
				Self::GetDebugCores => "get_debug_cores", // Added: XDC
				Self::GetDebugPorts => "get_debug_ports", // Added: XDC
				Self::GetGeneratedClocks => "get_generated_clocks", // Added: XDC
				Self::GetHierarchySeparator => "get_hierarchy_separator", // Added: XDC
				Self::GetIoBanks => "get_iobanks",        // Added: XDC
				Self::GetLibCells => "get_lib_cells",
				Self::GetLibPins => "get_lib_pins",
				Self::GetLibs => "get_libs",
				Self::GetMacros => "get_macros", // Added: XDC
				Self::GetNets => "get_nets",
				Self::GetNocInterfaces => "get_noc_interfaces", // Added: XDC
				Self::GetNodes => "get_nodes",                  // Added: XDC
				Self::GetPackagePins => "get_package_pins",     // Added: XDC
				Self::GetPathGroups => "get_path_groups",       // Added: XDC
				Self::GetPBlocks => "get_pblocks",              // Added: XDC
				Self::GetPins => "get_pins",
				Self::GetPips => "get_pips",                          // Added: XDC
				Self::GetPkgPinByteGroups => "get_pkgpin_bytegroups", // Added: XDC
				Self::GetPkgPinNibbles => "get_pkgpin_nibbles",       // Added: XDC
				Self::GetPorts => "get_ports",
				Self::GetPowerRails => "get_power_rails", // Added: XDC
				Self::GetProperty => "get_property",      // Added: XDC
				Self::GetSitePins => "get_site_pins",     // Added: XDC
				Self::GetSitePips => "get_site_pips",     // Added: XDC
				Self::GetSites => "get_sites",            // Added: XDC
				Self::GetSlrs => "get_slrs",              // Added: XDC
				Self::GetSpeedModels => "get_speed_models", // Added: XDC
				Self::GetTiles => "get_tiles",            // Added: XDC
				Self::GetTimingArcs => "get_timing_arcs", // Added: XDC
				Self::GetWires => "get_wires",            // Added: XDC
				Self::GroupPath => "group_path",          // Added: SDC 1.7
				Self::LdcCreateGroup => "ldc_create_group", // Added: PDC
				Self::LdcCreateMacro => "ldc_create_macro", // Added: PDC
				Self::LdcCreateRegion => "ldc_create_region", // Added: PDC
				Self::LdcCreateVref => "ldc_create_vref", // Added: PDC
				Self::LdcProhibit => "ldc_prohibit",      // Added: PDC
				Self::LdcSetAttribute => "ldc_set_attribute", // Added: PDC
				Self::LdcSetLocation => "ldc_set_location", // Added: PDC
				Self::LdcSetSysconfig => "ldc_set_sysconfig", // Added: PDC
				Self::LdcSetVcc => "ldc_set_vcc",         // Added: PDC
				Self::List => "list",
				Self::MakeDiffPairPorts => "make_diff_pair_ports", // Added: XDC
				Self::RemoveCellsFromPBlock => "remove_cells_from_pblock", // Added: XDC
				Self::RemoveFromPowerRail => "remove_from_power_rail", // Added: XDC
				Self::ResetOperatingConditions => "reset_operating_conditions", // Added: XDC
				Self::ResetSwitchingActivity => "reset_switching_activity", // Added: XDC
				Self::ResizePBlock => "resize_pblock",             // Added: XDC
				Self::Set => "set",
				Self::SetBusSkew => "set_bus_skew", // Added: XDC
				Self::SetCaseAnalysis => "set_case_analysis",
				Self::SetClockGatingCheck => "set_clock_gating_check",
				Self::SetClockGroups => "get_clock_groups", // Added: SDC 1.7
				Self::SetClockJitter => "set_clock_jitter", // Added: SDC 2.2
				Self::SetClockLatency => "set_clock_latency",
				Self::SetClockSense => "set_clock_sense", // Added: SDC 1.7-SDC 2.1
				Self::SetClockTransition => "set_clock_transition",
				Self::SetClockUncertainty => "set_clock_uncertainty",
				Self::SetDataCheck => "set_data_check", // Added: SDC 1.4 (2003.03)
				Self::SetDisableTiming => "set_disable_timing",
				Self::SetDrive => "set_drive",
				Self::SetDrivingCell => "set_driving_cell",
				Self::SetExternalDelay => "set_external_delay", // Added: XDC
				Self::SetFalsePath => "set_false_path",
				Self::SetFanoutLoad => "set_fanout_load",
				Self::SetHierarchySeparator => "set_hierarchy_separator",
				Self::SetIdealLatency => "set_ideal_latency", // Added: SDC 1.7
				Self::SetIdealNetwork => "set_ideal_network", // Added: SDC 1.7
				Self::SetIdealTransition => "set_ideal_transition", // Added: SDC 1.7
				Self::SetInputDelay => "set_input_delay",
				Self::SetInputJitter => "set_input_jitter", // Added: XDC
				Self::SetInputTransition => "set_input_transition",
				Self::SetLevelShifterStrategy => "set_level_shifter_strategy", // Added: SDC 1.6
				Self::SetLevelShifterThreshold => "set_level_shifter_threshold", // Added: SDC 1.6
				Self::SetLoad => "set_load",
				Self::SetLogicDc => "set_logic_dc",
				Self::SetLogicOne => "set_logic_one",
				Self::SetLogicUnconnected => "set_logic_unconnected", // Added: XDC
				Self::SetLogicZero => "set_logic_zero",
				Self::SetMaxArea => "set_max_area",
				Self::SetMaxCapacitance => "set_max_capacitance",
				Self::SetMaxDelay => "set_max_delay",
				Self::SetMaxDynamicPower => "set_max_dynamic_power", // Added: SDC 1.4 (2003.03)
				Self::SetMaxFanout => "set_max_fanout",
				Self::SetMaxLeakagePower => "set_max_leakage_power", // Added: SDC 1.4 (2003.03)
				Self::SetMaxTimeBorrow => "set_max_time_borrow",
				Self::SetMaxTransition => "set_max_transition",
				Self::SetMinCapacitance => "set_min_capacitance",
				Self::SetMinDelay => "set_min_delay",
				Self::SetMinPorosity => "set_min_porosity", // Added: SDC 1.4 (2003.03) - SDC 1.7
				Self::SetMinPulseWidth => "set_min_pulse_width", // Added: SDC 2.0
				Self::SetMulticyclePath => "set_multicycle_path",
				Self::SetOperatingConditions => "set_operating_conditions",
				Self::SetOutputDelay => "set_output_delay",
				Self::SetPackagePinVal => "set_package_pin_val", // Added: XDC
				Self::SetPortFanoutNumber => "set_port_fanout_number",
				Self::SetPowerOpt => "set_power_opt", // Added: XDC
				Self::SetPropagatedClock => "set_propagate_clock",
				Self::SetProperty => "set_property", // Added: XDC
				Self::SetResistance => "set_resistance",
				Self::SetSense => "set_sense", // Added: SDC 2.1
				Self::SetSwitchingActivity => "set_switching_activity", // Added: XDC
				Self::SetSystemJitter => "set_system_jitter", // Added: XDC
				Self::SetTimingDerate => "set_timing_derate", // Added: SDC 1.5 (2005.09)
				Self::SetUnits => "set_units", // Added: SDC 1.7
				Self::SetVoltage => "set_voltage", // Added: SDC 1.9
				Self::SetWireLoadMinBlockSize => "set_wire_load_min_block_size",
				Self::SetWireLoadMode => "set_wire_load_mode",
				Self::SetWireLoadModel => "set_wire_load_model",
				Self::SetWireLoadSelectionGroup => "set_wire_load_selection_group",
				Self::StartGroup => "startgroup",    // Added: XDC
				Self::UpdateMacro => "update_macro", // Added: XDC
			}
		)
	}
}

#[cfg(test)]
mod tests {
	// TODO(aki): Write tests
	#[allow(unused, reason = "Currently unused")]
	use super::*;
}
