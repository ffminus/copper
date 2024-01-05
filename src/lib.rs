// Use `README.md` as documentation home page, to reduce duplication
#![doc = include_str!("../README.md")]

mod model;
mod props;
mod vars;

pub use crate::model::Model;
pub use crate::vars::VarId;
