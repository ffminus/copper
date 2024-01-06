use core::borrow::Borrow;

use crate::vars::VarId;

/// Assignment for decision variables that satisfies all constraints.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Solution(Vec<i32>);

impl Solution {
    /// Get assignments for the decision variables provided as a slice.
    #[must_use]
    pub fn get_values(&self, vs: &[VarId]) -> Vec<i32> {
        self.get_values_iter(vs.iter().copied()).collect()
    }

    /// Get assignments for the decision variables provided as a reference to an array.
    #[must_use]
    pub fn get_values_array<const N: usize>(&self, vs: &[VarId; N]) -> [i32; N] {
        vs.map(|v| self[v])
    }

    /// Get assignments for the provided decision variables.
    pub fn get_values_iter<I: IntoIterator>(&self, vs: I) -> impl Iterator<Item = i32>
    where
        I::Item: Borrow<VarId>,
    {
        vs.into_iter().map(|v| self[*v.borrow()])
    }
}

impl From<Vec<i32>> for Solution {
    fn from(value: Vec<i32>) -> Self {
        Self(value)
    }
}

impl core::ops::Index<VarId> for Solution {
    type Output = i32;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.0[index]
    }
}
