use std::cmp::Ordering;

use crate::props::{self, PropId, Props};
use crate::search::{backlog, Deps, Searcher};
use crate::solution::Solution;
use crate::vars::{Var, VarId};

/// Problem definition, with decision variables and constraints.
#[derive(Debug, Default)]
pub struct Model {
    vars: Vec<Var>,
    props: Props,
    deps: Deps,
}

impl Model {
    /// Creates a new model, used to declare decision variables and constraints.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new decision variable with domain [`min`, `max`].
    #[must_use]
    pub fn new_var(&mut self, min: i32, max: i32) -> VarId {
        let id = VarId::new(self.vars.len());

        self.vars.push(Var { min, max });
        self.deps.vars.push(Vec::new());

        id
    }

    /// Creates `n` new decision variables with domain [`min`, `max`].
    #[must_use]
    pub fn new_vars(&mut self, n: usize, min: i32, max: i32) -> Vec<VarId> {
        (0..n).map(|_| self.new_var(min, max)).collect()
    }

    /// Creates a new binary decision variable.
    #[must_use]
    pub fn new_var_binary(&mut self) -> VarId {
        self.new_var(0, 1)
    }

    /// Creates `n` new binary decision variables.
    #[must_use]
    pub fn new_vars_binary(&mut self, n: usize) -> Vec<VarId> {
        (0..n).map(|_| self.new_var_binary()).collect()
    }

    /// Creates a new constant decision variable that can be used in constraints.
    #[must_use]
    pub fn cst(&mut self, value: i32) -> VarId {
        self.new_var(value, value)
    }

    /// Creates a new expression that represents the opposite of `x`.
    #[must_use]
    pub fn opposite(&mut self, x: impl IntoVarId) -> VarId {
        let x_opposite = x.into_var_id(self);
        self.scale_neg(x_opposite, -1)
    }

    /// Creates a new expression that represents `coef` * `x`.
    #[must_use]
    pub fn scale(&mut self, x: impl IntoVarId, coef: i32) -> VarId {
        match coef.cmp(&0) {
            Ordering::Less => self.scale_neg(x, coef),
            Ordering::Equal => self.cst(0),
            Ordering::Greater => self.scale_pos(x, coef),
        }
    }

    fn scale_pos(&mut self, x: impl IntoVarId, coef: i32) -> VarId {
        let x = x.into_var_id(self);

        let var = &self.vars[*x];

        let s = self.new_var(var.min * coef, var.max * coef);

        let id = PropId::ScalePos(self.props.scale_pos.len());

        self.props.scale_pos.push(props::PropScalePos);

        self.deps.props.scale_pos.push((x, s, coef));
        self.deps.vars[*x].push(id);
        self.deps.vars[*s].push(id);

        s
    }

    fn scale_neg(&mut self, x: impl IntoVarId, coef: i32) -> VarId {
        let x = x.into_var_id(self);

        let var = &self.vars[*x];

        let s = self.new_var(var.max * coef, var.min * coef);

        let id = PropId::ScaleNeg(self.props.scale_neg.len());

        self.props.scale_neg.push(props::PropScaleNeg);

        self.deps.props.scale_neg.push((x, s, coef));
        self.deps.vars[*x].push(id);
        self.deps.vars[*s].push(id);

        s
    }

    /// Creates a new expression that represents `x` + `y`.
    #[must_use]
    pub fn plus(&mut self, x: impl IntoVarId, y: impl IntoVarId) -> VarId {
        let (x, y) = (x.into_var_id(self), y.into_var_id(self));

        let var_x = &self.vars[*x];
        let var_y = &self.vars[*y];

        let plus = self.new_var(var_x.min + var_y.min, var_x.max + var_y.max);

        let id = PropId::Plus(self.props.plus.len());

        self.props.plus.push(props::PropPlus);

        self.deps.props.plus.push((plus, (x, y)));
        self.deps.vars[*x].push(id);
        self.deps.vars[*y].push(id);

        plus
    }

    /// Creates a new expression that represents `x` - `y`.
    #[must_use]
    pub fn minus(&mut self, x: impl IntoVarId, y: impl IntoVarId) -> VarId {
        let (x, y_opposite) = (x.into_var_id(self), y.into_var_id(self));

        let y = self.opposite(y_opposite);

        self.plus(x, y)
    }

    /// Creates a new expression that represents the sum of the provided variables.
    ///
    /// # Panics
    ///
    /// Function will panic if provided slice is empty.
    #[must_use]
    pub fn sum(&mut self, xs: &[VarId]) -> VarId {
        assert!(!xs.is_empty());

        let min = xs.iter().copied().map(|id| self.vars[*id].min).sum();
        let max = xs.iter().copied().map(|id| self.vars[*id].max).sum();

        let sum = self.new_var(min, max);

        let id = PropId::Sum(self.props.sum.len());

        self.props.sum.push(props::PropSum);

        self.deps.props.sum.push((sum, xs.to_vec()));

        for &x in xs {
            self.deps.vars[*x].push(id);
        }

        sum
    }

    /// Creates a new expression that represents a linear expression.
    ///
    /// # Panics
    ///
    /// Function will panic if passed slice is empty.
    pub fn linear(&mut self, xs: &[VarId], coefs: &[i32]) -> VarId {
        let terms: Vec<_> = xs
            .iter()
            .copied()
            .zip(coefs.iter().copied())
            .map(|(x, coef)| self.scale(x, coef))
            .collect();

        self.sum(&terms)
    }

    /// Enforces constraint `x` == `y`.
    pub fn eq(&mut self, x: impl IntoVarId, y: impl IntoVarId) {
        let (x, y) = (x.into_var_id(self), y.into_var_id(self));

        let id = PropId::Eq(self.props.eq.len());

        self.props.eq.push(props::PropEq);

        self.deps.props.eq.push((x, y));
        self.deps.vars[*x].push(id);
        self.deps.vars[*y].push(id);
    }

    /// Enforces constraint `x` <= `y`.
    pub fn leq(&mut self, x: impl IntoVarId, y: impl IntoVarId) {
        let (x, y) = (x.into_var_id(self), y.into_var_id(self));

        let id = PropId::Leq(self.props.leq.len());

        self.props.leq.push(props::PropLeq);

        self.deps.props.leq.push((x, y));
        self.deps.vars[*x].push(id);
        self.deps.vars[*y].push(id);
    }

    /// Performs search and returns the first assignment found that satisfies all constraints.
    #[must_use]
    pub fn solve(&mut self) -> Option<Solution> {
        // ? Dummy decision variable to use generic search logic
        let obj = self.cst(0);

        self.search(obj, true)
    }

    /// Performs search and returns the assignment that minimizes the provided objective variable.
    #[must_use]
    pub fn minimize(mut self, obj: impl IntoVarId) -> Option<Solution> {
        let obj = obj.into_var_id(&mut self);

        self.search(obj, false)
    }

    /// Performs search and returns the assignment that maximizes the provided objective variable.
    #[must_use]
    pub fn maximize(mut self, obj: impl IntoVarId) -> Option<Solution> {
        let obj_opposite = obj.into_var_id(&mut self);
        let obj = self.scale(obj_opposite, -1);

        self.search(obj, false)
    }

    fn search(&self, obj: VarId, stop_on_feasibility: bool) -> Option<Solution> {
        Searcher::new(&self.deps, obj, stop_on_feasibility)
            .search::<backlog::Stack>(&self.vars, &self.props)
    }
}

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
