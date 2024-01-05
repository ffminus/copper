use core::ops::{Index, IndexMut};

use crate::props::PropId;
use crate::solution::Solution;

/// Domain for a decision variable, tracked as an interval of integers.
#[derive(Clone, Debug)]
pub struct Var {
    pub min: i32,
    pub max: i32,
}

impl Var {
    /// Assigned variables have a domain reduced to a singleton.
    pub const fn is_assigned(&self) -> bool {
        self.min == self.max
    }

    /// Midpoint of domain for easier binary splits.
    pub const fn mid(&self) -> i32 {
        self.min + (self.max - self.min) / 2
    }

    /// Extract assignment for decision variable.
    ///
    /// # Panics
    ///
    /// This function will panic if the decision variable is not assigned.
    pub const fn get_assignment(&self) -> i32 {
        assert!(self.is_assigned());

        self.min
    }
}

/// Store decision variables and expose a limited interface to operate on them.
#[derive(Clone, Debug, Default)]
pub struct Vars(Vec<Var>);

impl Vars {
    /// Create a new decision variable.
    pub fn new_var_with_bounds(&mut self, min: i32, max: i32) -> VarId {
        let v = VarId(self.0.len());

        self.0.push(Var { min, max });

        v
    }

    /// Get handle to an unassigned decision variable.
    pub fn get_unassigned_var(&self) -> Option<VarId> {
        self.0.iter().position(|var| !var.is_assigned()).map(VarId)
    }

    /// Determine if all decision variables are assigned.
    pub fn is_assigned_all(&self) -> bool {
        self.get_unassigned_var().is_none()
    }

    /// Extract assignment for all decision variables.
    ///
    /// # Panics
    ///
    /// This function will panic if any decision variables are not assigned.
    pub fn into_solution(self) -> Solution {
        // Extract values for each decision variable
        let values: Vec<_> = self.0.into_iter().map(|v| v.get_assignment()).collect();

        Solution::from(values)
    }
}

/// Decision variable handle that is not bound to a specific memory location.
#[derive(Clone, Copy, Debug)]
pub struct VarId(usize);

impl Index<VarId> for Vars {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<VarId> for Vars {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl Index<VarId> for Vec<i32> {
    type Output = i32;

    fn index(&self, index: VarId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<VarId> for Vec<i32> {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl Index<VarId> for Vec<Vec<PropId>> {
    type Output = Vec<PropId>;

    fn index(&self, index: VarId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<VarId> for Vec<Vec<PropId>> {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self[index.0]
    }
}
