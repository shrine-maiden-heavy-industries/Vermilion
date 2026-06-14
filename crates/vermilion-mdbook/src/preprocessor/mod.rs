// SPDX-License-Identifier: BSD-3-Clause

pub(crate) mod cinnabar;
mod cli;
mod schema;
pub(crate) mod vermilion;

pub(crate) use crate::preprocessor::{
	cinnabar::CinnabarPreprocessor, vermilion::VermilionPreprocessor,
};
