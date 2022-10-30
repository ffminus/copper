use crate::vars::{Var, VarId};

/// Problem definition, with decision variables and constraints.
#[derive(Debug, Default)]
pub struct Model {
    vars: Vec<Var>,
}

impl Model {
    /// Creates a new decision variable with domain [`min`, `max`].
    #[must_use]
    pub fn new_var(&mut self, min: i32, max: i32) -> VarId {
        let id = VarId::new(self.vars.len());

        self.vars.push(Var { min, max });

        id
    }
}
