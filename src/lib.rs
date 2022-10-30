// Prevent all unsafe code usage in crate
#![forbid(unsafe_code)]
// Enable stricter lints
#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    missing_docs,
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

//! A constraint programming solver.
//!
//! # How to use Copper
//!
//! Declare your variables and constraints using the [`Model`] struct.
//! Calling its [solve][`Model::solve`] method will perform the search
//! and find a feasible solution. You can also minimize (or maximize)
//! an arbitrary objective expression to explore the search space
//! exhaustively.
//!
//! Below is an example formulation of the
//! [Knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem):
//!
//!```
//! use copper::Model;
//!
//! // Problem parameters
//! let item_weights = [10, 60, 30, 40, 30, 20, 20, 2];
//! let item_values = [1, 10, 15, 40, 60, 90, 100, 15];
//! let weight_max = 102;
//!
//! // Model object, used to declare variables and constraints
//! let mut m = Model::new();
//!
//! // Binary decision variables: for each item, do I put it in the bag?
//! let xs = m.new_vars_binary(item_weights.len());
//!
//! // Sum the weight of the selected items using a linear expression
//! let weight = m.linear(&xs, &item_weights);
//!
//! // Ensure the bag's weight does not exceed the maximum
//! m.leq(weight, weight_max);
//!
//! // Sum the value of the selected items
//! let value = m.linear(&xs, &item_values);
//!
//! // Find the selection of items that maximizes the bag's value
//! let solution = m.maximize(value).unwrap();
//!
//! assert_eq!(solution.get_values_binary(&xs), vec![false, false, true, false, true, true, true, true]);
//! assert_eq!(solution[weight], 102);
//! assert_eq!(solution[value], 280);
//! ```

mod model;
mod props;
mod search;
mod solution;
mod utils;
mod vars;

#[cfg(test)]
mod tests;

pub use crate::model::{Model, Strategy};
pub use crate::solution::Solution;

// ? Avoid exporting symbols not part of the WASM API if feature is enabled
#[cfg(not(feature = "wasm"))]
mod not_wasm {
    pub use crate::model::generic::IntoVarId;
    pub use crate::props::{Failed, Propagate, ResultProp};
    pub use crate::search::branch::{enumerate, pick};
    pub use crate::vars::{VarId, Vars};
}
#[cfg(not(feature = "wasm"))]
pub use not_wasm::*;
