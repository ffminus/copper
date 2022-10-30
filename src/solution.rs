use std::ops::Index;

use crate::vars::VarId;

/// Assignment that satisfies all model constraints, returned to caller
#[derive(Debug)]
pub struct Solution(Vec<i32>);

impl Solution {
    pub(crate) const fn new(assignment: Vec<i32>) -> Self {
        Self(assignment)
    }

    /// Access the solution value of a slice of variables.
    #[must_use]
    pub fn get_values(&self, xs: &[VarId]) -> Vec<i32> {
        xs.iter().map(|id| self[*id]).collect()
    }
}

impl Index<VarId> for Solution {
    type Output = i32;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.0[*index]
    }
}
