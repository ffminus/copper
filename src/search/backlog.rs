use crate::solution::Solution;

use super::{Searcher, Space};

/// Engine to schedule spaces to be explored during search.
pub trait Backlog {
    /// Perform search, keeping track of spaces to explore on branching.
    fn search(space: Space, searcher: &Searcher) -> Option<Solution>;
}

/// Single-threaded LIFO list of nodes to explore.
#[derive(Default)]
pub struct Stack;

impl Backlog for Stack {
    fn search(space: Space, searcher: &Searcher) -> Option<Solution> {
        todo!()
    }
}
