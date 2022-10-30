use crate::vars::Var;

/// Problem definition, with decision variables and constraints.
#[derive(Debug, Default)]
pub struct Model {
    vars: Vec<Var>,
}

impl Model {
    /// Creates a new decision variable with domain [`min`, `max`].
    pub fn new_var(&mut self, min: i32, max: i32) {
        self.vars.push(Var { min, max });
    }
}
