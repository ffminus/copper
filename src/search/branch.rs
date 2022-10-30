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
pub trait Branch {
    /// Iterator over mutations to apply to generate branches to explore.
    type Iter: Iterator<Item = Mutation>;

    /// Initialize brancher from pivot's current domain.
    fn branch_on(pivot: &Var) -> Self::Iter;
}

/// Set each value in current domain of pivot variable iteratively, in ascending order.
pub struct SetMinToMax;

impl Branch for SetMinToMax {
    type Iter = SetMinToMaxIter;

    fn branch_on(pivot: &Var) -> Self::Iter {
        SetMinToMaxIter((pivot.min..=pivot.max).rev())
    }
}

/// Iterator over current domain of pivot variable, in ascending order.
pub struct SetMinToMaxIter(Rev<RangeInclusive<i32>>);

impl Iterator for SetMinToMaxIter {
    type Item = Mutation;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Mutation::Set)
    }
}

/// Set each value in current domain of pivot variable iteratively, in descending order.
pub struct SetMaxToMin;

impl Branch for SetMaxToMin {
    type Iter = SetMaxToMinIter;

    fn branch_on(pivot: &Var) -> Self::Iter {
        SetMaxToMinIter(pivot.min..=pivot.max)
    }
}

/// Iterator over current domain of pivot variable, in descending order.
pub struct SetMaxToMinIter(RangeInclusive<i32>);

impl Iterator for SetMaxToMinIter {
    type Item = Mutation;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Mutation::Set)
    }
}
