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
