use crate::vars::Vars;

/// Enforce a specific constraint by removing assignments that break it from variable domains.
pub trait Propagate {
    /// Dependent variables stored externally and injected during propagation.
    type Deps;

    /// Prunes unfeasible assignments from domain, signal failed nodes with `None` value.
    fn propagate(&mut self, deps: &Self::Deps, vars: Vars) -> Option<Vars>;
}

/// Discriminate propagator type with enum to enable static dispatch and dependency injection.
#[derive(Clone, Copy, Debug)]
pub enum PropId {}

/// Helper type to group propagators by type.
#[derive(Clone, Debug, Default)]
pub struct Props {}
