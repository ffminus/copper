use crate::vars::Vars;

use super::Choice;

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
        .map(move |value| Choice {
            pivot,
            mutation: Mutation::Set(value),
        })
}
