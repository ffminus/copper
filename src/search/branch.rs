use crate::vars::Vars;

use super::Choice;

/// Change to apply to a variable to restrict its domain.
#[derive(Debug)]
pub enum Mutation {
    /// Assign a specific value to the variable.
    Set(i32),
}

pub fn branch(pivot: &Var) -> impl Iterator<Item = Mutation> {
    // Iterate over all possible values within domain
    (pivot.min..=pivot.max).rev().map(Mutation::Set)
}
