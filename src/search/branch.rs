use crate::vars::{VarId, Vars};

/// Branch to be applied to mutate search space.
#[derive(Debug)]
pub struct Choice {
    pub pivot: VarId,
    pub mutation: Mutation,
}

impl Choice {
    const fn new(pivot: VarId, mutation: Mutation) -> Self {
        Self { pivot, mutation }
    }
}

/// Change to apply to a variable to restrict its domain.
#[derive(Debug)]
pub enum Mutation {
    /// Assign a specific value to the variable.
    Set(i32),
}

pub fn branch(vars: &Vars) -> impl Iterator<Item = Choice> {
    // Branch on the first unset variable
    let (pivot, pivot_var) = vars.iter().find(|(_id, var)| !var.is_set()).unwrap();

    // Iterate over all possible values within domain
    (pivot_var.min..=pivot_var.max)
        .rev()
        .map(move |value| Choice::new(pivot, Mutation::Set(value)))
}
