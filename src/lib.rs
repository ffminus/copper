// Prevent all unsafe code usage in crate
#![forbid(unsafe_code)]
// Enable stricter lints
#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications
)]
// Disable lints with false positives
#![allow(clippy::option_if_let_else)]

mod model;
mod props;
mod search;
mod vars;

pub use crate::model::{IntoVarId, Model};
pub use crate::vars::VarId;
