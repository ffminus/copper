use std::ops::Index;

use crate::vars::VarId;

/// Assignment that satisfies all model constraints, returned to caller
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug)]
pub struct Solution(Vec<i32>);

#[cfg(not(feature = "wasm"))]
impl Solution {
    /// Access the solution value of a slice of variables.
    #[must_use]
    pub fn get_values(&self, xs: &[VarId]) -> Vec<i32> {
        self.get_values_impl(xs)
    }

    /// Access the solution value of a slice of binary variables.
    #[must_use]
    pub fn get_values_binary(&self, xs: &[VarId]) -> Vec<bool> {
        self.get_values_binary_impl(xs)
    }
}

impl Solution {
    pub(crate) const fn new(assignment: Vec<i32>) -> Self {
        Self(assignment)
    }

    fn get_values_impl(&self, xs: &[VarId]) -> Vec<i32> {
        xs.iter().map(|id| self[*id]).collect()
    }

    fn get_values_binary_impl(&self, xs: &[VarId]) -> Vec<bool> {
        xs.iter().map(|id| self[*id] != 0).collect()
    }
}

impl Index<VarId> for Solution {
    type Output = i32;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.0[*index]
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    use super::Solution;

    use crate::vars::wasm::{from_slice_of_ids, VarId};

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    impl Solution {
        /// Access the solution value of a variable.
        #[must_use]
        pub fn getValue(&self, x: VarId) -> i32 {
            self[x.into()]
        }

        /// Access the solution value of a slice of variables.
        #[must_use]
        pub fn getValues(&self, xs: &[VarId]) -> Box<[i32]> {
            self.get_values_impl(&from_slice_of_ids(xs))
                .into_boxed_slice()
        }
    }
}
