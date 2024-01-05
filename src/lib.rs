// Use `README.md` as documentation home page, to reduce duplication
#![doc = include_str!("../README.md")]

/// Simple domain transformations to make propagators more generic.
pub mod views;

mod model;
mod props;
mod vars;

#[cfg(test)]
mod tests;

pub use crate::model::Model;
pub use crate::vars::VarId;
