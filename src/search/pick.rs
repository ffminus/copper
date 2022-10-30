use crate::vars::{VarId, Vars};

/// Select variable to branch on during search.
pub trait Pick {
    /// Pick the id of an unset variable to branch on.
    fn pick(vars: &Vars) -> VarId;
}

/// Pick first variable in list with an unset domain.
pub struct FirstUnset;

impl Pick for FirstUnset {
    fn pick(vars: &Vars) -> VarId {
        vars.iter().find(|(_id, var)| !var.is_set()).unwrap().0
    }
}
