use std::iter::Peekable;

use crate::vars::{VarId, VarIds, Vars};

/// Select variable to branch on during search.
pub trait Pick: Clone {
    /// Initialize picker from model variables.
    fn from_vars(vars: &Vars) -> Self;

    /// Pick the id of an unset variable to branch on, fail space by returning `None`.
    fn pick(&mut self, vars: &Vars) -> Option<VarId>;
}

/// Pick first variable in list with an unset domain.
#[derive(Clone)]
pub struct FirstUnset(Peekable<VarIds>);

impl Pick for FirstUnset {
    fn from_vars(vars: &Vars) -> Self {
        Self(vars.get_var_ids().peekable())
    }

    fn pick(&mut self, vars: &Vars) -> Option<VarId> {
        while let Some(x) = self.0.peek().copied() {
            if !vars[x].is_set() {
                return Some(x);
            }

            self.0.next();
        }

        None
    }
}
