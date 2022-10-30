use std::cmp::{max as max_of, min as min_of};

use crate::utils::NumExt;
use crate::vars::{VarId, Vars};

/// A propagator either fails a space, or returns updated variable domains.
pub type ResultProp = Result<Vars, Failed>;

/// Error type to signal a space has been failed by a propagator.
pub struct Failed;

/// Enforce a specific constraint by removing assignments that break it from variable domains.
pub trait Propagate: std::fmt::Debug + CloneBoxed {
    /// Prunes unfeasible assignments from domain.
    ///
    /// # Errors
    ///
    /// Failed spaces are signaled by returning a `Failed` error value.
    fn propagate(&mut self, vars: Vars) -> ResultProp;
}

/// Place `Clone` requirement on trait implementers without violating object safety rules.
pub trait CloneBoxed {
    fn clone_boxed(&self) -> Box<dyn Propagate>;
}

impl<T: 'static + Propagate + Clone> CloneBoxed for T {
    fn clone_boxed(&self) -> Box<dyn Propagate> {
        Box::new(self.clone())
    }
}

// ? Call to `clone` is forwarded to boxed version
impl Clone for Box<dyn Propagate> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}

/// Discriminate propagator type with enum to enable static dispatch and dependency injection.
#[derive(Clone, Copy, Debug)]
pub enum PropId {
    ScalePos(usize),
    ScaleNeg(usize),
    Plus(usize),
    Sum(usize),
    Eq(usize),
    Leq(usize),
    Custom(usize),
}

/// Helper type to group propagators by type.
#[derive(Clone, Debug, Default)]
pub struct Props {
    pub scale_pos: Vec<PropScalePos>,
    pub scale_neg: Vec<PropScaleNeg>,
    pub plus: Vec<PropPlus>,
    pub sum: Vec<PropSum>,
    pub eq: Vec<PropEq>,
    pub leq: Vec<PropLeq>,
    pub custom: Vec<Box<dyn Propagate>>,
}

/// Scaling factor constraint with a positive coefficient.
#[derive(Clone, Debug)]
pub struct PropScalePos;

pub type PropScalePosDeps = (VarId, VarId, i32);

impl PropScalePos {
    pub fn propagate((x, y, coef): PropScalePosDeps, vars: Vars) -> ResultProp {
        let (var_x, var_y) = (&vars[x], &vars[y]);

        let min = max_of(var_x.min, var_y.min.next_multiple_of_tmp(coef) / coef);
        let max = min_of(var_x.max, (var_y.max - var_y.max.rem_euclid(coef)) / coef);

        let vars = vars.set_min_and_max(x, min, max)?;
        let vars = vars.set_min_and_max(y, min * coef, max * coef)?;

        Ok(vars)
    }
}

/// Scaling factor constraint with a negative coefficient.
#[derive(Clone, Debug)]
pub struct PropScaleNeg;

pub type PropScaleNegDeps = (VarId, VarId, i32);

impl PropScaleNeg {
    pub fn propagate((x, y, coef): PropScaleNegDeps, vars: Vars) -> ResultProp {
        let (var_x, var_y) = (&vars[x], &vars[y]);

        let min = max_of(var_x.min, (var_y.max - var_y.max.rem_euclid(-coef)) / coef);
        let max = min_of(var_x.max, var_y.min.next_multiple_of_tmp(-coef) / coef);

        let vars = vars.set_min_and_max(x, min, max)?;
        let vars = vars.set_min_and_max(y, max * coef, min * coef)?;

        Ok(vars)
    }
}

/// Add two variables together.
#[derive(Clone, Debug)]
pub struct PropPlus;

pub type PropPlusDeps = (VarId, (VarId, VarId));

impl PropPlus {
    pub fn propagate((p, (x, y)): PropPlusDeps, vars: Vars) -> ResultProp {
        let (var_x, var_y, var_p) = (&vars[x], &vars[y], &vars[p]);

        let min = max_of(var_x.min + var_y.min, var_p.min);
        let max = min_of(var_x.max + var_y.max, var_p.max);

        let (x_min_new, x_max_new) = (min - var_y.max, max - var_y.min);
        let (y_min_new, y_max_new) = (min - var_x.max, max - var_x.min);

        let vars = vars.set_min_and_max(x, x_min_new, x_max_new)?;
        let vars = vars.set_min_and_max(y, y_min_new, y_max_new)?;
        let vars = vars.set_min_and_max(p, min, max)?;

        Ok(vars)
    }
}

/// Sum of arbitrary number of variables.
#[derive(Clone, Debug)]
pub struct PropSum;

pub type PropSumDepsRef<'xs> = (VarId, &'xs [VarId]);
pub type PropSumDeps = (VarId, Vec<VarId>);

impl PropSum {
    pub fn propagate((s, xs): PropSumDepsRef, mut vars: Vars) -> ResultProp {
        let (sum_of_mins, sum_of_maxs) = xs
            .iter()
            .copied()
            .map(|id| &vars[id])
            .fold((0, 0), |(min, max), x| (min + x.min, max + x.max));

        let var = &vars[s];

        let min = max_of(sum_of_mins, var.min);
        let max = min_of(sum_of_maxs, var.max);

        vars = vars.set_min_and_max(s, min, max)?;

        for &x in xs {
            let x_min_new = min - (sum_of_maxs - vars[x].max);
            let x_max_new = max - (sum_of_mins - vars[x].min);

            vars = vars.set_min_and_max(x, x_min_new, x_max_new)?;
        }

        Ok(vars)
    }
}

#[derive(Clone, Debug)]
pub struct PropEq;

pub type PropEqDeps = (VarId, VarId);

impl PropEq {
    pub fn propagate((x, y): PropEqDeps, vars: Vars) -> ResultProp {
        let (var_x, var_y) = (&vars[x], &vars[y]);

        let min = max_of(var_x.min, var_y.min);
        let max = min_of(var_x.max, var_y.max);

        let vars = vars.set_min_and_max(x, min, max)?;
        let vars = vars.set_min_and_max(y, min, max)?;

        Ok(vars)
    }
}

/// Equality constraint between two variables.
#[derive(Clone, Debug)]
pub struct PropLeq;

pub type PropLeqDeps = (VarId, VarId);

impl PropLeq {
    pub fn propagate((x, y): PropLeqDeps, vars: Vars) -> ResultProp {
        let max = min_of(vars[x].max, vars[y].max);

        vars.set_max(x, max)
    }
}
