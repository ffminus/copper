use dyn_clone::DynClone;

use crate::vars::VarId;
use crate::views::Context;

/// Enforce a specific constraint by pruning domain of decision variables.
pub trait Prune: core::fmt::Debug + DynClone {
    /// Perform pruning based on variable domains and internal state.
    fn prune(&mut self, ctx: &mut Context) -> Option<()>;
}

/// Isolate methods that prevent propagator from being used as a trait-object.
pub trait Propagate: Prune + 'static {
    /// List variables that schedule the propagator when their domain changes.
    gen fn list_trigger_vars(&self) -> VarId;
}

/// Store internal state for each propagators, along with dependencies for when to schedule each.
#[derive(Clone, Debug, Default)]
pub struct Propagators {
    dependencies: Vec<Vec<PropId>>,
}

impl Propagators {
    /// Extend dependencies matrix with a row for the new decision variable.
    pub fn on_new_var(&mut self) {
        self.dependencies.push(Vec::new());
    }
}

/// Propagator handle that is not bound to a specific memory location.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PropId(usize);
