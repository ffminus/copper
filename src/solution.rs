use crate::vars::VarId;

/// Assignment for decision variables that satisfies all constraints.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Solution(Vec<i32>);

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
