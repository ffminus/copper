use core::borrow::Borrow;

use crate::vars::{VarId, VarIdBinary};

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
    pub fn get_values_iter<'a, I>(&'a self, vs: I) -> impl Iterator<Item = i32> + 'a
    where
        I: IntoIterator + 'a,
        I::Item: Borrow<VarId>,
    {
        vs.into_iter().map(|v| self[*v.borrow()])
    }

    /// Get binary assignment for the provided decision variable.
    #[must_use]
    pub fn get_value_binary(&self, v: impl Borrow<VarIdBinary>) -> bool {
        self.0[v.borrow().0] == 1
    }

    /// Get binary assignments for the decision variables provided as a slice.
    #[must_use]
    pub fn get_values_binary(&self, vs: &[VarIdBinary]) -> Vec<bool> {
        self.get_values_binary_iter(vs.iter().copied()).collect()
    }

    /// Get binary assignments for the decision variables provided as a reference to an array.
    #[must_use]
    pub fn get_values_binary_array<const N: usize>(&self, vs: &[VarIdBinary; N]) -> [bool; N] {
        vs.map(|v| self.get_value_binary(v))
    }

    /// Get binary assignments for the provided decision variables.
    pub fn get_values_binary_iter<'a, I>(&'a self, vs: I) -> impl Iterator<Item = bool> + 'a
    where
        I: IntoIterator + 'a,
        I::Item: Borrow<VarIdBinary>,
    {
        vs.into_iter().map(|v| self.get_value_binary(v))
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
