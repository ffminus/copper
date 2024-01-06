use crate::props::Propagators;
use crate::search::{mode, search};
use crate::solution::Solution;
use crate::vars::{VarId, Vars};
use crate::views::{View, ViewExt};

/// Library entry point used to declare decision variables and constraints, and configure search.
#[derive(Debug, Default)]
pub struct Model {
    vars: Vars,
    props: Propagators,
}

impl Model {
    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// This function will only create a decision variable if `min < max`.
    pub fn new_var(&mut self, min: i32, max: i32) -> Option<VarId> {
        if min < max {
            Some(self.new_var_unchecked(min, max))
        } else {
            None
        }
    }

    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    ///
    /// This function assumes that `min < max`.
    fn new_var_unchecked(&mut self, min: i32, max: i32) -> VarId {
        self.props.on_new_var();
        self.vars.new_var_with_bounds(min, max)
    }

    /// Create an expression of two views added together.
    pub fn add(&mut self, x: impl View, y: impl View) -> VarId {
        let min = x.min_raw(&self.vars) + y.min_raw(&self.vars);
        let max = x.max_raw(&self.vars) + y.max_raw(&self.vars);
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.add(x, y, s);

        s
    }

    /// Create an expression of the sum of a slice of views.
    pub fn sum(&mut self, xs: &[impl View]) -> VarId {
        self.sum_iter(xs.iter().copied())
    }

    /// Create an expression of the sum of an iterator of views.
    pub fn sum_iter(&mut self, xs: impl IntoIterator<Item = impl View>) -> VarId {
        let xs: Vec<_> = xs.into_iter().collect();

        let min: i32 = xs.iter().map(|x| x.min_raw(&self.vars)).sum();
        let max: i32 = xs.iter().map(|x| x.max_raw(&self.vars)).sum();
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.sum(xs, s);

        s
    }

    /// Declare two expressions to be equal.
    pub fn equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.equals(x, y);
    }

    /// Declare constraint `x <= y`.
    pub fn less_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.less_than_or_equals(x, y);
    }

    /// Declare constraint `x < y`.
    pub fn less_than(&mut self, x: impl View, y: impl View) {
        let _p = self.props.less_than(x, y);
    }

    /// Declare constraint `x >= y`.
    pub fn greater_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.greater_than_or_equals(x, y);
    }

    /// Declare constraint `x > y`.
    pub fn greater_than(&mut self, x: impl View, y: impl View) {
        let _p = self.props.greater_than(x, y);
    }

    /// Find assignment that minimizes objective expression while satisfying all constraints.
    #[must_use]
    pub fn minimize(self, objective: impl View) -> Option<Solution> {
        self.minimize_and_iterate(objective).last()
    }

    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn minimize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        search(self.vars, self.props, mode::Minimize::new(objective))
    }

    /// Find assignment that maximizes objective expression while satisfying all constraints.
    #[must_use]
    pub fn maximize(self, objective: impl View) -> Option<Solution> {
        self.minimize(objective.opposite())
    }

    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn maximize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        self.minimize_and_iterate(objective.opposite())
    }

    /// Search for assignment that satisfies all constraints within bounds of decision variables.
    #[must_use]
    pub fn solve(self) -> Option<Solution> {
        self.enumerate().next()
    }

    /// Enumerate all assignments that satisfy all constraints.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn enumerate(self) -> impl Iterator<Item = Solution> {
        search(self.vars, self.props, mode::Enumerate)
    }
}
