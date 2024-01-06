use crate::props::Propagators;
use crate::search::{mode, search};
use crate::solution::Solution;
use crate::vars::{VarId, Vars};

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

    /// Enumerate all assignments that satisfy all constraints.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn enumerate(self) -> impl Iterator<Item = Solution> {
        search(self.vars, self.props, mode::Enumerate)
    }
}
