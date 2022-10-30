use std::iter::Rev;
use std::ops::RangeInclusive;

use crate::vars::Var;

/// Change to apply to a variable to restrict its domain.
#[derive(Debug)]
pub enum Mutation {
    /// Assign a specific value to the variable.
    Set(i32),
}

/// Enumerate mutations on pivot variable when branching.
pub trait Branch: Iterator<Item = Mutation> {
    /// Initialize brancher from pivot's current domain.
    fn from_var(pivot: &Var) -> Self;
}

/// Set each value in current domain of pivot variable iteratively, in ascending order.
pub struct SetMinToMax(Rev<RangeInclusive<i32>>);

impl Branch for SetMinToMax {
    fn from_var(pivot: &Var) -> Self {
        Self((pivot.min..=pivot.max).rev())
    }
}

impl Iterator for SetMinToMax {
    type Item = Mutation;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Mutation::Set)
    }
}
