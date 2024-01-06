use crate::props::Propagators;
use crate::search::{mode, search};
use crate::solution::Solution;
use crate::vars::{VarId, VarIdBinary, Vars};
use crate::views::{View, ViewExt};

/// Library entry point used to declare decision variables and constraints, and configure search.
///
/// Optimization problems are modeled with base decision variables and derived expressions.
/// You can use constraints to restrict which values are considered valid assignments.
/// An assignment that satisfies all constraints is called "feasible".
/// Once all decision variables and constraints have been declared, call one of the following:
/// - [solve](Self::solve): get the first feasible assignment
/// - [enumerate](Self::enumerate): iterate over all feasible assignments
/// - [minimize](Self::minimize): find the assignment that minimizes the provided expression
/// - [maximize](Self::maximize): find the assignment that maximizes the provided expression
/// - [minimize_and_iterate](Self::minimize_and_iterate): iterate over feasible assignments while minimizing an expression
/// - [maximize_and_iterate](Self::maximize_and_iterate): iterate over feasible assignments while maximizing an expression
///
/// Here is an example to describe how a typical model is formulated.
/// It is a rendition of a combinatorial optimization classic:
/// the [Knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem).
///
/// Let's say we are building a brand new PC. We want to play AAA games without going bankrupt.
///
/// ```
/// // All problem formulations will start with a model object
/// let mut m = copper::Model::default();
/// ```
///
/// # Variables and expressions
///
/// Decision variables represent a decision: they are declared with a domain.
///
/// ```
/// # let mut m = copper::Model::default();
/// // How many monitors do we buy: we need at least one, but not more than three
/// let n_monitors = m.new_var(1, 3).unwrap();
///
/// // All monitors cost the same, and each additional monitor provides the same bump to our score
/// let monitor_price = 100;
/// let monitor_score = 250;
///
/// // Each GPU model has a fixed price, and an associated benchmark score
/// let gpu_prices = [150, 250, 500];
/// let gpu_scores = [100, 400, 800];
///
/// // We use binary decision variables to represent "do I pick this GPU?"
/// let gpus: Vec<_> = m.new_vars_binary(gpu_scores.len()).collect();
/// ```
///
/// Using variables as building blocks, we can create expressions to represent other quantities.
///
/// ```
/// # let mut m = copper::Model::default();
/// # let n_monitors = m.new_var(1, 3).unwrap();
/// # let monitor_price = 100;
/// # let monitor_score = 250;
/// # let gpu_prices = [150, 250, 500];
/// # let gpu_scores = [100, 400, 800];
/// # let gpus: Vec<_> = m.new_vars_binary(gpu_scores.len()).collect();
///
/// // Extension trait, used here to scale our decision variables by a constant (`times` method)
/// use copper::views::ViewExt;
///
/// // For each potential GPU, we multiply its price (and score) by whether or not it is selected.
/// // The sum of these terms gives us the price and score of the selected GPU.
/// let gpu_price = m.sum_iter(gpus.iter().zip(gpu_prices).map(|(x, price)| x.times(price)));
/// let gpu_score = m.sum_iter(gpus.iter().zip(gpu_scores).map(|(x, score)| x.times(score)));
///
/// // This expression is the overall price of our build
/// let price = m.add(gpu_price, n_monitors.times(monitor_price));
///
/// // We want to maximize this score: how much we'll value this particular build
/// let score = m.add(gpu_score, n_monitors.times(monitor_score));
/// ```
///
/// # Constraints
///
/// Constraints establish relationships between variables, and restrict feasible values.
///
/// ```
/// # use copper::views::ViewExt;
/// # let mut m = copper::Model::default();
/// # let n_monitors = m.new_var(1, 3).unwrap();
/// # let monitor_price = 100;
/// # let monitor_score = 250;
/// # let gpu_prices = [150, 250, 500];
/// # let gpu_scores = [100, 400, 800];
/// # let gpus: Vec<_> = m.new_vars_binary(gpu_scores.len()).collect();
/// # let gpu_price = m.sum_iter(gpus.iter().zip(gpu_prices).map(|(x, p)| x.times(p)));
/// # let gpu_score = m.sum_iter(gpus.iter().zip(gpu_scores).map(|(x, s)| x.times(s)));
/// # let price = m.add(gpu_price, n_monitors.times(monitor_price));
/// # let score = m.add(gpu_score, n_monitors.times(monitor_score));
/// // Exactly one GPU: we want to run Crysis, but our case must fit under the desk
/// let n_gpus = m.sum(&gpus);
/// m.equals(n_gpus, 1);
///
/// // Grandma got us some money for our birthday, that will be our budget
/// m.less_than_or_equals(price, 600);
/// ```
///
/// # Search
///
/// While constraints define feasibility, objectives are soft: they only determine optimality.
///
/// ```
/// # use copper::views::ViewExt;
/// # let mut m = copper::Model::default();
/// # let n_monitors = m.new_var(1, 3).unwrap();
/// # let monitor_price = 100;
/// # let monitor_score = 250;
/// # let gpu_prices = [150, 250, 500];
/// # let gpu_scores = [100, 400, 800];
/// # let gpus: Vec<_> = m.new_vars_binary(gpu_scores.len()).collect();
/// # let gpu_price = m.sum_iter(gpus.iter().zip(gpu_prices).map(|(x, p)| x.times(p)));
/// # let gpu_score = m.sum_iter(gpus.iter().zip(gpu_scores).map(|(x, s)| x.times(s)));
/// # let price = m.add(gpu_price, n_monitors.times(monitor_price));
/// # let score = m.add(gpu_score, n_monitors.times(monitor_score));
/// # let n_gpus = m.sum(&gpus);
/// # m.equals(n_gpus, 1);
/// # m.less_than_or_equals(price, 600);
/// // Let the solver find the assignment that upholds our constraints and maximizes our score
/// let solution = m.maximize(score).unwrap();
///
/// // Our optimal build has three monitors and a mid-tier GPU. We even have some left-over cash!
/// assert_eq!(solution[n_monitors], 3);
/// assert_eq!(solution.get_values_binary(&gpus), vec![false, true, false]);
/// assert_eq!(solution[score], 1150);
/// assert_eq!(solution[price], 550);
/// ```
///
/// Find the full code in the [examples directory](https://github.com/ffmins/copper/examples/pc.rs).
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

    /// Create new integer decision variables, with the provided domain bounds.
    ///
    /// All created variables will have the same starting domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// This function will only create decision variables if `min < max`.
    pub fn new_vars(
        &mut self,
        n: usize,
        min: i32,
        max: i32,
    ) -> Option<impl Iterator<Item = VarId>> {
        if min < max {
            Some(core::iter::repeat_with(move || self.new_var_unchecked(min, max)).take(n))
        } else {
            None
        }
    }

    /// Create a new binary decision variable.
    pub fn new_var_binary(&mut self) -> VarIdBinary {
        VarIdBinary(self.new_var_unchecked(0, 1))
    }

    /// Create new binary decision variables.
    pub fn new_vars_binary(&mut self, n: usize) -> impl Iterator<Item = VarIdBinary> {
        core::iter::repeat_with(|| self.new_var_binary()).take(n)
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
