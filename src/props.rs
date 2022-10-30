use std::cmp::{max as max_of, min as min_of};

use crate::utils::NumExt;
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
    ScalePos(usize),
    ScaleNeg(usize),
    Plus(usize),
    Sum(usize),
    Eq(usize),
}

/// Helper type to group propagators by type.
#[derive(Clone, Debug, Default)]
pub struct Props {
    pub scale_pos: Vec<PropScalePos>,
    pub scale_neg: Vec<PropScaleNeg>,
    pub plus: Vec<PropPlus>,
    pub sum: Vec<PropSum>,
    pub eq: Vec<PropEq>,
}

/// Scaling factor constraint with a positive coefficient.
#[derive(Clone, Debug)]
pub struct PropScalePos;

impl Propagate for PropScalePos {
    type Deps = (VarId, VarId, i32);

    fn propagate(&mut self, deps: &Self::Deps, mut vars: Vars) -> Option<Vars> {
        let (x, y, coef) = *deps;

        let (var_x, var_y) = (&vars[x], &vars[y]);

        let min = max_of(var_x.min, var_y.min.next_multiple_of_tmp(coef) / coef);
        let max = min_of(var_x.max, (var_y.max - var_y.max.rem_euclid(coef)) / coef);

        if min > max {
            None
        } else {
            vars.set_min_and_max(x, min, max);
            vars.set_min_and_max(y, min * coef, max * coef);

            Some(vars)
        }
    }
}

/// Scaling factor constraint with a negative coefficient.
#[derive(Clone, Debug)]
pub struct PropScaleNeg;

impl Propagate for PropScaleNeg {
    type Deps = (VarId, VarId, i32);

    fn propagate(&mut self, deps: &Self::Deps, mut vars: Vars) -> Option<Vars> {
        let (x, y, coef) = *deps;

        let (var_x, var_y) = (&vars[x], &vars[y]);

        let min = max_of(var_x.min, (var_y.max - var_y.max.rem_euclid(-coef)) / coef);
        let max = min_of(var_x.max, var_y.min.next_multiple_of_tmp(-coef) / coef);

        if min > max {
            None
        } else {
            vars.set_min_and_max(x, min, max);
            vars.set_min_and_max(y, max * coef, min * coef);

            Some(vars)
        }
    }
}

/// Add two variables together.
#[derive(Clone, Debug)]
pub struct PropPlus;

impl Propagate for PropPlus {
    type Deps = (VarId, (VarId, VarId));

    fn propagate(&mut self, deps: &Self::Deps, mut vars: Vars) -> Option<Vars> {
        let (p, (x, y)) = *deps;

        let (var_x, var_y, var_p) = (&vars[x], &vars[y], &vars[p]);

        let min = max_of(var_x.min + var_y.min, var_p.min);
        let max = min_of(var_x.max + var_y.max, var_p.max);

        if min > max {
            return None;
        }

        let (x_min_new, x_max_new) = (min - var_y.max, max - var_y.min);
        let (y_min_new, y_max_new) = (min - var_x.max, max - var_x.min);

        if x_min_new > x_max_new || y_min_new > y_max_new {
            return None;
        }

        vars.set_min_and_max(x, x_min_new, x_max_new);
        vars.set_min_and_max(y, y_min_new, y_max_new);
        vars.set_min_and_max(p, min, max);

        Some(vars)
    }
}

/// Sum of arbitrary number of variables.
#[derive(Clone, Debug)]
pub struct PropSum;

impl Propagate for PropSum {
    type Deps = (VarId, Vec<VarId>);

    fn propagate(&mut self, (s, xs): &Self::Deps, mut vars: Vars) -> Option<Vars> {
        let (sum_of_mins, sum_of_maxs) = xs
            .iter()
            .copied()
            .map(|id| &vars[id])
            .fold((0, 0), |(min, max), x| (min + x.min, max + x.max));

        let var = &vars[*s];

        let min = max_of(sum_of_mins, var.min);
        let max = min_of(sum_of_maxs, var.max);

        if min > max {
            return None;
        }

        vars.set_min_and_max(*s, min, max);

        for &x in xs {
            let x_min_new = min - (sum_of_maxs - vars[x].max);
            let x_max_new = max - (sum_of_mins - vars[x].min);

            if x_min_new > x_max_new {
                return None;
            }

            vars.set_min_and_max(x, x_min_new, x_max_new);
        }

        Some(vars)
    }
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
