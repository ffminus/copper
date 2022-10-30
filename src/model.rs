use crate::props::{self, PropId, Props};
use crate::search::{Deps, Searcher};
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

    /// Creates a new constant decision variable that can be used in constraints.
    #[must_use]
    pub fn cst(&mut self, value: i32) -> VarId {
        self.new_var(value, value)
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
    pub fn solve(&self) -> Option<Solution> {
        Searcher::new(&self.deps).search(&self.vars, &self.props)
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
