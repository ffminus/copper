/// Branch enumeration strategies.
pub mod enumerate;

/// Pivot variable selection strategies.
pub mod pick;

/// Change to apply to a variable to restrict its domain.
#[derive(Debug)]
pub enum Mutation {
    /// Assign a specific value to the variable.
    Set(i32),

    /// Set a new minimum value to the variable's domain.
    Min(i32),

    /// Set a new maximum value to the variable's domain.
    Max(i32),
}
