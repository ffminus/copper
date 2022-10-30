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

pub fn branch(vars: &Vars) -> impl Iterator<Item = Branch> {
    // Branch on the first unset variable
    let (pivot, pivot_var) = vars.iter().find(|(_id, var)| !var.is_set()).unwrap();

    // Iterate over all possible values within domain
    (pivot_var.min..=pivot_var.max)
        .rev()
        .map(move |value| Branch::new(pivot, Choice::Set(value)))
}
