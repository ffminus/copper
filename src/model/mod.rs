// ? Generic trampoline methods that cannot be exposed to WASM
#[cfg(not(feature = "wasm"))]
pub mod generic;

// ? Wrapper methods that can be exposed to WASM
#[cfg(feature = "wasm")]
mod wasm;

use std::cmp::Ordering;
use std::marker::PhantomData;

use crate::props::{self, PropId, Propagate, Props};
use crate::search::branch::{enumerate, pick, Branch, Brancher};
use crate::search::{engine, Deps, Searcher};
use crate::solution::Solution;
use crate::vars::{Var, VarId};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Problem definition, with decision variables and constraints.
///
/// Optimization problems are modeled with base decision variables and derived expressions.
/// Use constraints to restrict which values are considered valid assignments.
/// Start the search with either [solve](Self::solve) to return the first solution
/// that satisfies all constraints, or [minimize](Self::minimize) (or [maximize](Self::maximize))
/// to optimize an objective value with exhaustive exploration.
///
/// We'll use an example to describe how a typical model is formulated.
/// Let's say we are building a new PC, and want to be able to play AAA games without going bankrupt.
///
/// ```
/// // All problem formulations will start with a model object
/// let mut m = copper::Model::new();
/// ```
///
/// # Variables and expressions
///
/// Decision variables represent a decision, or an assignment; they are declared with a domain.
///
/// ```
/// # let mut m = copper::Model::new();
/// // How many monitors do we buy: we need at least one, but not more than three
/// let n_monitors = m.new_var(1, 3);
///
/// // Each GPU model has a fixed price, and an associated benchmark score
/// let gpu_prices = [150, 250, 500];
/// let gpu_scores = [100, 400, 800];
///
/// // We use binary decision variables to represent "do I pick this GPU?"
/// let gpus = m.new_vars_binary(gpu_scores.len());
/// ```
///
/// Using variables as building blocks, we can create expressions to represent other quantities.
///
///
/// ```
/// # let mut m = copper::Model::new();
/// # let n_monitors = m.new_var(1, 3);
/// # let gpu_prices = [150, 250, 500];
/// # let gpu_scores = [100, 400, 800];
/// # let gpus = m.new_vars_binary(gpu_scores.len());
/// // Each monitor costs $100, and brings a score bump of 250
/// let price_monitors = m.scale(n_monitors, 100);
/// let score_monitors = m.scale(n_monitors, 250);
///
/// // Linear expressions let us consider a GPU's data only if it is selected
/// let price_gpu = m.linear(&gpus, &gpu_prices);
/// let score_gpu = m.linear(&gpus, &gpu_scores);
///
/// // Let's say we have already set our mind on a specific keyboard, mouse and case
/// let price_others = m.cst(200);
///
/// // The overall price of our build
/// let price = m.sum(&[price_monitors, price_gpu, price_others]);
///
/// // What we want to maximize, how much we'll value this particular build
/// let score = m.plus(score_monitors, score_gpu);
/// ```
///
/// # Constraints
///
/// Establish relationships between variables, and restrictions on feasible values using constraints.
///
/// ```
/// # let mut m = copper::Model::new();
/// # let n_monitors = m.new_var(1, 3);
/// # let gpu_prices = [150, 250, 500];
/// # let gpu_scores = [100, 400, 800];
/// # let gpus = m.new_vars_binary(gpu_scores.len());
/// # let price_monitors = m.scale(n_monitors, 100);
/// # let score_monitors = m.scale(n_monitors, 240);
/// # let price_gpu = m.linear(&gpus, &gpu_prices);
/// # let score_gpu = m.linear(&gpus, &gpu_scores);
/// # let price_others = m.cst(200);
/// # let price = m.sum(&[price_monitors, price_gpu, price_others]);
/// # let score = m.plus(score_monitors, score_gpu);
/// // Exactly one GPU: we want to run Crysis, but our case must fit under the desk
/// let n_gpus = m.sum(&gpus);
/// m.eq(n_gpus, 1);
///
/// // We got $800 for our birthday, that will be our budget
/// m.leq(price, 800);
///```
///
/// # Search
///
/// Contrary to hard constraints, objectives are soft: they do not determine feasibility but optimality.
///
///
/// ```
/// # let mut m = copper::Model::new();
/// # let n_monitors = m.new_var(1, 3);
/// # let gpu_prices = [150, 250, 500];
/// # let gpu_scores = [100, 400, 800];
/// # let gpus = m.new_vars_binary(gpu_scores.len());
/// # let price_monitors = m.scale(n_monitors, 100);
/// # let score_monitors = m.scale(n_monitors, 250);
/// # let price_gpu = m.linear(&gpus, &gpu_prices);
/// # let score_gpu = m.linear(&gpus, &gpu_scores);
/// # let price_others = m.cst(200);
/// # let price = m.sum(&[price_monitors, price_gpu, price_others]);
/// # let score = m.plus(score_monitors, score_gpu);
/// # let n_gpus = m.sum(&gpus);
/// # m.eq(n_gpus, 1);
/// # m.leq(price, 800);
/// // Let the solver find the assignment that upholds our constraints and maximizes our score
/// let solution = m.maximize(score).unwrap();
///
/// // Our optimal build has three monitors and a mid-tier GPU, we even have some left-over cash!
/// assert_eq!(solution[n_monitors], 3);
/// assert_eq!(solution.get_values_binary(&gpus), vec![false, true, false]);
/// assert_eq!(solution[score], 1150);
/// assert_eq!(solution[price],  750);
///```
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

    fn propagator_impl(&mut self, prop: Box<dyn Propagate>, deps: &[VarId]) {
        let id = PropId::Custom(self.props.custom.len());

        for x in deps {
            self.deps.vars[**x].push(id);
        }

        self.props.custom.push(prop);
    }

    fn solve_impl<B: Branch>(&mut self) -> Option<Solution> {
        // ? Dummy decision variable to use generic search logic
        let obj = self.cst_impl(0);

        self.search::<B>(obj, false)
    }

    fn minimize_impl<B: Branch>(&self, obj: VarId) -> Option<Solution> {
        self.search::<B>(obj, true)
    }

    fn maximize_impl<B: Branch>(mut self, obj: VarId) -> Option<Solution> {
        let obj_opposite = self.scale_impl(obj, -1);

        self.minimize_impl::<B>(obj_opposite)
    }

    fn search<B: Branch>(&self, obj: VarId, is_exhaustive: bool) -> Option<Solution> {
        Searcher::new(&self.deps, obj, is_exhaustive)
            .search::<B, engine::Stack<_>>(&self.vars, &self.props)
    }
}

/// Model and branching strategy to be applied during search.
pub struct Strategy<B: Branch> {
    model: Model,

    _b: PhantomData<B>,
}

/// Default branching strategy used if user does not specify one.
pub type StrategyDefault = Strategy<Brancher<pick::FirstUnset, enumerate::SetMinToMax>>;

impl<B: Branch> Strategy<B> {
    const fn new(model: Model) -> Self {
        Self {
            model,
            _b: PhantomData,
        }
    }

    /// Performs search and returns the first assignment found that satisfies all constraints.
    #[must_use]
    pub fn solve(&mut self) -> Option<Solution> {
        self.model.solve_impl::<B>()
    }

    fn minimize_impl(self, obj: VarId) -> Option<Solution> {
        self.model.minimize_impl::<B>(obj)
    }

    fn maximize_impl(self, obj: VarId) -> Option<Solution> {
        self.model.maximize_impl::<B>(obj)
    }
}
