// ? Generic trampoline methods that cannot be exposed to WASM
#[cfg(not(feature = "wasm"))]
pub mod generic;

// ? Wrapper methods that can be exposed to WASM
#[cfg(feature = "wasm")]
mod wasm;

use std::cmp::Ordering;

use crate::props::{self, PropId, Props};
use crate::search::{backlog, Deps, Searcher};
use crate::solution::Solution;
use crate::vars::{Var, VarId};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Problem definition, with decision variables and constraints.
#[derive(Debug, Default)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Model {
    vars: Vec<Var>,
    props: Props,
    deps: Deps,
}

// ? Methods that are exported as-is to both Rust and WASM interfaces
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Model {
    /// Creates a new model, used to declare decision variables and constraints.
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Performs search and returns the first assignment found that satisfies all constraints.
    #[must_use]
    pub fn solve(&mut self) -> Option<Solution> {
        // ? Dummy decision variable to use generic search logic
        let obj = self.cst_impl(0);

        self.search(obj, true)
    }
}

// ? Internal method implementations that wrappers can call
impl Model {
    fn new_var_impl(&mut self, min: i32, max: i32) -> VarId {
        let id = VarId::new(self.vars.len());

        self.vars.push(Var { min, max });
        self.deps.vars.push(Vec::new());

        id
    }

    fn new_var_binary_impl(&mut self) -> VarId {
        self.new_var_impl(0, 1)
    }

    fn cst_impl(&mut self, value: i32) -> VarId {
        self.new_var_impl(value, value)
    }

    fn new_vars_impl(&mut self, n: usize, min: i32, max: i32) -> Vec<VarId> {
        (0..n).map(|_| self.new_var_impl(min, max)).collect()
    }

    fn new_vars_binary_impl(&mut self, n: usize) -> Vec<VarId> {
        (0..n).map(|_| self.new_var_binary_impl()).collect()
    }

    fn opposite_impl(&mut self, x: VarId) -> VarId {
        self.scale_neg(x, -1)
    }

    fn scale_impl(&mut self, x: VarId, coef: i32) -> VarId {
        match coef.cmp(&0) {
            Ordering::Less => self.scale_neg(x, coef),
            Ordering::Equal => self.cst_impl(0),
            Ordering::Greater => self.scale_pos(x, coef),
        }
    }

    fn scale_pos(&mut self, x: VarId, coef: i32) -> VarId {
        let var = &self.vars[*x];

        let s = self.new_var_impl(var.min * coef, var.max * coef);

        let id = PropId::ScalePos(self.props.scale_pos.len());

        self.props.scale_pos.push(props::PropScalePos);

        self.deps.props.scale_pos.push((x, s, coef));
        self.deps.vars[*x].push(id);
        self.deps.vars[*s].push(id);

        s
    }

    fn scale_neg(&mut self, x: VarId, coef: i32) -> VarId {
        let var = &self.vars[*x];

        let s = self.new_var_impl(var.max * coef, var.min * coef);

        let id = PropId::ScaleNeg(self.props.scale_neg.len());

        self.props.scale_neg.push(props::PropScaleNeg);

        self.deps.props.scale_neg.push((x, s, coef));
        self.deps.vars[*x].push(id);
        self.deps.vars[*s].push(id);

        s
    }

    fn plus_impl(&mut self, x: VarId, y: VarId) -> VarId {
        let var_x = &self.vars[*x];
        let var_y = &self.vars[*y];

        let plus = self.new_var_impl(var_x.min + var_y.min, var_x.max + var_y.max);

        let id = PropId::Plus(self.props.plus.len());

        self.props.plus.push(props::PropPlus);

        self.deps.props.plus.push((plus, (x, y)));
        self.deps.vars[*x].push(id);
        self.deps.vars[*y].push(id);

        plus
    }

    fn minus_impl(&mut self, x: VarId, y: VarId) -> VarId {
        let y_opposite = self.opposite_impl(y);

        self.plus_impl(x, y_opposite)
    }

    fn sum_impl(&mut self, xs: &[VarId]) -> VarId {
        assert!(!xs.is_empty());

        let min = xs.iter().copied().map(|id| self.vars[*id].min).sum();
        let max = xs.iter().copied().map(|id| self.vars[*id].max).sum();

        let sum = self.new_var_impl(min, max);

        let id = PropId::Sum(self.props.sum.len());

        self.props.sum.push(props::PropSum);

        self.deps.props.sum.push((sum, xs.to_vec()));

        for &x in xs {
            self.deps.vars[*x].push(id);
        }

        sum
    }

    fn linear_impl(&mut self, xs: &[VarId], coefs: &[i32]) -> VarId {
        let terms: Vec<_> = xs
            .iter()
            .copied()
            .zip(coefs.iter().copied())
            .map(|(x, coef)| self.scale_impl(x, coef))
            .collect();

        self.sum_impl(&terms)
    }

    fn eq_impl(&mut self, x: VarId, y: VarId) {
        let id = PropId::Eq(self.props.eq.len());

        self.props.eq.push(props::PropEq);

        self.deps.props.eq.push((x, y));
        self.deps.vars[*x].push(id);
        self.deps.vars[*y].push(id);
    }

    fn leq_impl(&mut self, x: VarId, y: VarId) {
        let id = PropId::Leq(self.props.leq.len());

        self.props.leq.push(props::PropLeq);

        self.deps.props.leq.push((x, y));
        self.deps.vars[*x].push(id);
        self.deps.vars[*y].push(id);
    }

    fn minimize_impl(&self, obj: VarId) -> Option<Solution> {
        self.search(obj, false)
    }

    fn maximize_impl(mut self, obj: VarId) -> Option<Solution> {
        let obj_opposite = self.scale_impl(obj, -1);

        self.minimize_impl(obj_opposite)
    }

    fn search(&self, obj: VarId, stop_on_feasibility: bool) -> Option<Solution> {
        Searcher::new(&self.deps, obj, stop_on_feasibility)
            .search::<backlog::Stack>(&self.vars, &self.props)
    }
}
