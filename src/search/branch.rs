use crate::vars::{VarId, Vars};

/// Branch to be applied to mutate search space.
#[derive(Debug)]
pub struct Branch {
    pub pivot: VarId,
    pub choice: Choice,
}

impl Branch {
    const fn new(pivot: VarId, choice: Choice) -> Self {
        Self { pivot, choice }
    }
}

/// Change to apply to a variable to restrict its domain.
#[derive(Debug)]
pub enum Choice {
    /// Assign a specific value to the variable.
    Set(i32),
}
