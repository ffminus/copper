use crate::props::Propagate;
use crate::search::branch::SetMinToMax;
use crate::search::pick::FirstUnset;
use crate::solution::Solution;
use crate::vars::VarId;

use super::Model;

/// Convenience trait for values that can be converted to a variable id.
pub trait IntoVarId {
    /// Convert value to a variable id.
    fn into_var_id(self, m: &mut Model) -> VarId;
}

impl IntoVarId for VarId {
    fn into_var_id(self, _m: &mut Model) -> VarId {
        self
    }
}

impl IntoVarId for i32 {
    fn into_var_id(self, m: &mut Model) -> VarId {
        m.cst(self)
    }
}

impl Model {
    /// Creates a new constant decision variable that can be used in constraints.
    #[must_use]
    pub fn cst(&mut self, value: i32) -> VarId {
        self.cst_impl(value)
    }

    /// Creates a new decision variable with domain [`min`, `max`].
    #[must_use]
    pub fn new_var(&mut self, min: i32, max: i32) -> VarId {
        self.new_var_impl(min, max)
    }

    /// Creates a new binary decision variable.
    #[must_use]
    pub fn new_var_binary(&mut self) -> VarId {
        self.new_var_binary_impl()
    }

    /// Creates `n` new decision variables with domain [`min`, `max`].
    #[must_use]
    pub fn new_vars(&mut self, n: usize, min: i32, max: i32) -> Vec<VarId> {
        self.new_vars_impl(n, min, max)
    }

    /// Creates `n` new binary decision variables.
    #[must_use]
    pub fn new_vars_binary(&mut self, n: usize) -> Vec<VarId> {
        self.new_vars_binary_impl(n)
    }

    /// Creates a new expression that represents the opposite of `x`.
    #[must_use]
    pub fn opposite(&mut self, x: impl IntoVarId) -> VarId {
        let x = x.into_var_id(self);
        self.opposite_impl(x)
    }

    /// Creates a new expression that represents `coef` * `x`.
    #[must_use]
    pub fn scale(&mut self, x: impl IntoVarId, coef: i32) -> VarId {
        let x = x.into_var_id(self);
        self.scale_impl(x, coef)
    }

    /// Creates a new expression that represents `x` + `y`.
    #[must_use]
    pub fn plus(&mut self, x: impl IntoVarId, y: impl IntoVarId) -> VarId {
        let x = x.into_var_id(self);
        let y = y.into_var_id(self);
        self.plus_impl(x, y)
    }

    /// Creates a new expression that represents `x` - `y`.
    #[must_use]
    pub fn minus(&mut self, x: impl IntoVarId, y: impl IntoVarId) -> VarId {
        let x = x.into_var_id(self);
        let y = y.into_var_id(self);
        self.minus_impl(x, y)
    }

    /// Creates a new expression that represents the sum of the provided variables.
    ///
    /// # Panics
    ///
    /// Function will panic if provided slice is empty.
    #[must_use]
    pub fn sum(&mut self, xs: &[VarId]) -> VarId {
        self.sum_impl(xs)
    }

    /// Creates a new expression that represents a linear expression.
    ///
    /// # Panics
    ///
    /// Function will panic if passed slice is empty.
    pub fn linear(&mut self, xs: &[VarId], coefs: &[i32]) -> VarId {
        self.linear_impl(xs, coefs)
    }

    /// Enforces constraint `x` == `y`.
    pub fn eq(&mut self, x: impl IntoVarId, y: impl IntoVarId) {
        let x = x.into_var_id(self);
        let y = y.into_var_id(self);
        self.eq_impl(x, y);
    }

    /// Enforces constraint `x` <= `y`.
    pub fn leq(&mut self, x: impl IntoVarId, y: impl IntoVarId) {
        let x = x.into_var_id(self);
        let y = y.into_var_id(self);
        self.leq_impl(x, y);
    }

    /// Declare custom propagator, with its associated dependencies.
    pub fn propagator(&mut self, prop: impl Propagate + 'static, deps: &[VarId]) {
        self.propagator_impl(Box::new(prop), deps);
    }

    /// Performs search and returns the first assignment found that satisfies all constraints.
    #[must_use]
    pub fn solve(&mut self) -> Option<Solution> {
        self.solve_impl::<FirstUnset, SetMinToMax>()
    }

    /// Performs search and returns the assignment that minimizes the provided objective variable.
    #[must_use]
    pub fn minimize(mut self, obj: impl IntoVarId) -> Option<Solution> {
        let obj = obj.into_var_id(&mut self);
        self.minimize_impl::<FirstUnset, SetMinToMax>(obj)
    }

    /// Performs search and returns the assignment that maximizes the provided objective variable.
    #[must_use]
    pub fn maximize(mut self, obj: impl IntoVarId) -> Option<Solution> {
        let obj = obj.into_var_id(&mut self);
        self.maximize_impl::<FirstUnset, SetMinToMax>(obj)
    }
}
