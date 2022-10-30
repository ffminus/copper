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
    /// Creates a new decision variable with domain [`min`, `max`].
    #[must_use]
    pub fn new_var(&mut self, min: i32, max: i32) -> VarId {
        let id = VarId::new(self.vars.len());

        self.vars.push(Var { min, max });
        self.deps.vars.push(Vec::new());

        id
    }

    /// Creates a new binary decision variable.
    #[must_use]
    pub fn new_var_binary(&mut self) -> VarId {
        self.new_var(0, 1)
    }

    /// Creates a new constant decision variable that can be used in constraints.
    #[must_use]
    pub fn cst(&mut self, value: i32) -> VarId {
        self.new_var(value, value)
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

    /// Enforces constraint `x` == `y`.
    pub fn eq(&mut self, x: impl IntoVarId, y: impl IntoVarId) {
        let (x, y) = (x.into_var_id(self), y.into_var_id(self));

        let id = PropId::Eq(self.props.eq.len());

        self.props.eq.push(props::PropEq);

        self.deps.props.eq.push((x, y));
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
