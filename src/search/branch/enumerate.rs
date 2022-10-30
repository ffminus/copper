use std::iter::Rev;
use std::ops::RangeInclusive;

use crate::vars::Var;

use super::Mutation;

/// Enumerate mutations on pivot variable when branching.
pub trait Enumerate: Clone {
    /// Iterator over mutations to apply to generate branches to explore.
    type Iter: Iterator<Item = Mutation>;

    /// Initialize brancher on search start.
    fn new_enumerator() -> Self;

    /// Initialize brancher from pivot's current domain.
    fn branch_on(&mut self, pivot: &Var) -> Self::Iter;
}

/// Set each value in current domain of pivot variable iteratively, in ascending order.
#[derive(Clone)]
pub struct SetMinToMax;

impl Enumerate for SetMinToMax {
    type Iter = SetMinToMaxIter;

    fn new_enumerator() -> Self {
        Self
    }

    fn branch_on(&mut self, pivot: &Var) -> Self::Iter {
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
#[derive(Clone)]
pub struct SetMaxToMin;

impl Enumerate for SetMaxToMin {
    type Iter = SetMaxToMinIter;

    fn new_enumerator() -> Self {
        Self
    }

    fn branch_on(&mut self, pivot: &Var) -> Self::Iter {
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
