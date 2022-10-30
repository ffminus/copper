/// Branch enumeration strategies.
pub mod enumerate;

/// Pivot variable selection strategies.
pub mod pick;

use crate::vars::{VarId, Vars};

use self::enumerate::Enumerate;
use self::pick::Pick;

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

/// Branch by mutating space and exploring resulting sub-trees.
pub trait Branch: Clone {
    /// Iterator over mutations to apply to generate branches to explore.
    type Iter: Iterator<Item = Choice>;

    /// Create brancher instance from model variables.
    fn from_vars(vars: &Vars) -> Self;

    /// Enumerate choices to apply to create branches, fail space by returning `None`.
    fn branch(&mut self, vars: &Vars) -> Option<Self::Iter>;
}

/// Common branching strategy that picks a pivot variable to branch on then enumerates mutations.
#[derive(Clone)]
pub struct Brancher<P: Pick, E: Enumerate> {
    picker: P,
    enumerator: E,
}

impl<P: Pick, E: Enumerate> Branch for Brancher<P, E> {
    type Iter = BrancherIter<E>;

    fn from_vars(vars: &Vars) -> Self {
        Self {
            picker: P::from_vars(vars),
            enumerator: E::new_enumerator(),
        }
    }

    fn branch(&mut self, vars: &Vars) -> Option<Self::Iter> {
        self.picker
            .pick(vars)
            .map(|pivot| BrancherIter::new(pivot, self.enumerator.branch_on(&vars[pivot])))
    }
}

pub struct BrancherIter<E: Enumerate> {
    pivot: VarId,
    mutations: E::Iter,
}

impl<E: Enumerate> BrancherIter<E> {
    const fn new(pivot: VarId, mutations: E::Iter) -> Self {
        Self { pivot, mutations }
    }
}

impl<E: Enumerate> Iterator for BrancherIter<E> {
    type Item = Choice;

    fn next(&mut self) -> Option<Self::Item> {
        self.mutations
            .next()
            .map(|mutation| Choice::new(self.pivot, mutation))
    }
}
