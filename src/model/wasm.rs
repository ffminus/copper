use wasm_bindgen::prelude::wasm_bindgen;

use crate::search::branch::SetMinToMax as Brancher;
use crate::search::pick::FirstUnset as Picker;
use crate::solution::Solution;
use crate::vars::wasm::{from_slice_of_ids, into_boxed_slice_of_ids, VarId};

use super::Model;

#[allow(non_snake_case)]
#[wasm_bindgen]
impl Model {
    /// Creates a new decision variable with domain [`min`, `max`].
    #[must_use]
    pub fn newVar(&mut self, min: i32, max: i32) -> VarId {
        self.new_var_impl(min, max).into()
    }

    /// Creates a new binary decision variable.
    #[must_use]
    pub fn newVarBinary(&mut self) -> VarId {
        self.new_var_binary_impl().into()
    }

    /// Creates a new constant decision variable that can be used in constraints.
    #[must_use]
    pub fn cst(&mut self, value: i32) -> VarId {
        self.cst_impl(value).into()
    }

    /// Creates `n` new decision variables with domain [`min`, `max`].
    #[must_use]
    pub fn newVars(&mut self, n: usize, min: i32, max: i32) -> Box<[VarId]> {
        into_boxed_slice_of_ids(self.new_vars_impl(n, min, max))
    }

    /// Creates `n` new binary decision variables.
    #[must_use]
    pub fn newVarsBinary(&mut self, n: usize) -> Box<[VarId]> {
        into_boxed_slice_of_ids(self.new_vars_binary_impl(n))
    }

    /// Creates a new expression that represents the opposite of `x`.
    #[must_use]
    pub fn opposite(&mut self, x: VarId) -> VarId {
        self.opposite_impl(x.into()).into()
    }

    /// Creates a new expression that represents `coef` * `x`.
    #[must_use]
    pub fn scale(&mut self, x: VarId, coef: i32) -> VarId {
        self.scale_impl(x.into(), coef).into()
    }

    /// Creates a new expression that represents `x` + `y`.
    #[must_use]
    pub fn plus(&mut self, x: VarId, y: VarId) -> VarId {
        self.plus_impl(x.into(), y.into()).into()
    }

    /// Creates a new expression that represents `x` - `y`.
    #[must_use]
    pub fn minus(&mut self, x: VarId, y: VarId) -> VarId {
        self.minus_impl(x.into(), y.into()).into()
    }

    /// Creates a new expression that represents the sum of the provided variables.
    ///
    /// # Panics
    ///
    /// Function will panic if provided slice is empty.
    #[must_use]
    pub fn sum(&mut self, xs: &[VarId]) -> VarId {
        self.sum_impl(&from_slice_of_ids(xs)).into()
    }

    /// Creates a new expression that represents a linear expression.
    ///
    /// # Panics
    ///
    /// Function will panic if passed slice is empty.
    pub fn linear(&mut self, xs: &[VarId], coefs: &[i32]) -> VarId {
        self.linear_impl(&from_slice_of_ids(xs), coefs).into()
    }

    /// Enforces constraint `x` == `y`.
    pub fn eq(&mut self, x: VarId, y: VarId) {
        self.eq_impl(x.into(), y.into());
    }

    /// Enforces constraint `x` <= `y`.
    pub fn leq(&mut self, x: VarId, y: VarId) {
        self.leq_impl(x.into(), y.into());
    }

    /// Performs search and returns the first assignment found that satisfies all constraints.
    #[must_use]
    pub fn solve(&mut self) -> Option<Solution> {
        self.solve_impl::<Picker, Brancher>()
    }

    /// Performs search and returns the assignment that minimizes the provided objective variable.
    #[must_use]
    pub fn minimize(&self, obj: VarId) -> Option<Solution> {
        self.minimize_impl::<Picker, Brancher>(obj.into())
    }

    /// Performs search and returns the assignment that maximizes the provided objective variable.
    #[must_use]
    pub fn maximize(self, obj: VarId) -> Option<Solution> {
        self.maximize_impl::<Picker, Brancher>(obj.into())
    }
}
