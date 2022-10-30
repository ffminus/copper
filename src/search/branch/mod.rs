/// Branch enumeration strategies.
pub mod enumerate;

/// Pivot variable selection strategies.
pub mod pick;

use crate::vars::VarId;

/// Branch to be applied to mutate search space.
#[derive(Debug)]
pub struct Choice {
    pub pivot: VarId,
    pub mutation: Mutation,
}

impl Choice {
    const fn new(pivot: VarId, mutation: Mutation) -> Self {
        Self { pivot, mutation }
    }
}

/// Change to apply to a variable to restrict its domain.
#[derive(Debug)]
pub enum Mutation {
    /// Assign a specific value to the variable.
    Set(i32),

    /// Set a new minimum value to the variable's domain.
    Min(i32),

    /// Set a new maximum value to the variable's domain.
    Max(i32),
}
