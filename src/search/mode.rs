use crate::props::PropId;
use crate::search::Space;
use crate::vars::Vars;
use crate::views::View;

/// Control search behavior when a solution is found.
pub trait Mode: core::fmt::Debug {
    /// List propagators to be scheduled on after branch.
    gen fn on_branch(&self, _: &mut Space) -> PropId {}

    /// Update internal state when new solution is found.
    fn on_solution(&mut self, _vars: &Vars) {}
}

/// Enumerate assignments that satisfy all constraints.
#[derive(Debug)]
pub struct Enumerate;

impl Mode for Enumerate {}

/// Enumerate assignments that satisfy all constraints, and gradually lower objective expression.
#[derive(Debug)]
pub struct Minimize<V> {
    objective: V,
    minimum_opt: Option<i32>,
}

impl<V: View> Minimize<V> {
    pub const fn new(objective: V) -> Self {
        Self {
            objective,
            minimum_opt: None,
        }
    }
}

impl<V: View> Mode for Minimize<V> {
    gen fn on_branch(&self, space: &mut Space) -> PropId {
        // Prune assignments that cannot lower objective expression
        if let Some(minimum) = self.minimum_opt {
            yield space.props.less_than(self.objective, minimum);
        };
    }

    fn on_solution(&mut self, vars: &Vars) {
        // New objective value is necessarily lower than previous lowest
        self.minimum_opt = Some(self.objective.min_raw(vars));
    }
}
