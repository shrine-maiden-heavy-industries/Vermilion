// SPDX-License-Identifier: BSD-3-Clause
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod commands;
mod env;

vermilion_exec::new!(Cinnabar, "CINNABAR_LOG_LEVEL");
