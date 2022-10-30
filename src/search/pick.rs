use crate::vars::{VarId, Vars};

/// Select variable to branch on during search.
pub trait Pick {
    /// Pick the id of an unset variable to branch on, fail space by returning `None`.
    fn pick(vars: &Vars) -> Option<VarId>;
}

/// Pick first variable in list with an unset domain.
pub struct FirstUnset;

impl Pick for FirstUnset {
    fn pick(vars: &Vars) -> Option<VarId> {
        vars.iter()
            .find(|(_id, var)| !var.is_set())
            .map(|(id, _var)| id)
    }
}
