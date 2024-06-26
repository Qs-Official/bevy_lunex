#![doc = include_str!("../README.md")]

// #======================#
// #=== PRELUDE EXPORT ===#

pub mod common;
pub use common::*;

pub mod core;
pub use core::*;

pub mod layout;
pub use layout::*;

pub mod nodes;
pub use nodes::*;


pub mod prelude {
    pub use super::common::prelude::*;
    pub use super::core::prelude::*;
    pub use super::layout::prelude::*;
}

// #=========================#
// #=== CRATE ONLY EXPORT ===#

pub(crate) mod import {
    pub(crate) use std::borrow::Borrow;

    pub(crate) use indexmap::IndexMap as HashMap;
    pub(crate) use colored::Colorize;

    pub(crate) use bevy::math::{Vec2, Vec3, Vec4};
    pub(crate) use bevy::utils::thiserror::Error;
}