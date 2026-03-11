// SPDX-License-Identifier: BSD-3-Clause

pub mod component;
pub mod component_instance;
pub mod logical_wire;
#[allow(
	clippy::module_inception,
	reason = "At the whims of how the schema types are defined"
)]
pub mod power;
pub mod wire;
