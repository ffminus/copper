use std::cmp::{max as max_of, min as min_of};

use crate::vars::{VarId, Vars};

/// Enforce a specific constraint by removing assignments that break it from variable domains.
pub trait Propagate {
    /// Dependent variables stored externally and injected during propagation.
    type Deps;

    /// Prunes unfeasible assignments from domain, signal failed nodes with `None` value.
    fn propagate(&mut self, deps: &Self::Deps, vars: Vars) -> Option<Vars>;
}

/// Discriminate propagator type with enum to enable static dispatch and dependency injection.
#[derive(Clone, Copy, Debug)]
pub enum PropId {
    Eq(usize),
}

/// Helper type to group propagators by type.
#[derive(Clone, Debug, Default)]
pub struct Props {
    pub eq: Vec<PropEq>,
}

#[derive(Clone, Debug)]
pub struct PropEq;

impl Propagate for PropEq {
    type Deps = (VarId, VarId);

    fn propagate(&mut self, deps: &Self::Deps, mut vars: Vars) -> Option<Vars> {
        let (x, y) = *deps;

        let (var_x, var_y) = (&vars[x], &vars[y]);

        let min = max_of(var_x.min, var_y.min);
        let max = min_of(var_x.max, var_y.max);

        if min > max {
            None
        } else {
            vars.set_min_and_max(x, min, max);
            vars.set_min_and_max(y, min, max);

            Some(vars)
        }
    }
}
