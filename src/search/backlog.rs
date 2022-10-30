use std::rc::Rc;

use crate::solution::Solution;

use super::branch::{branch, Branch};
use super::{Propagated, Searcher, Space};

/// Engine to schedule spaces to be explored during search.
pub trait Backlog {
    /// Perform search, keeping track of spaces to explore on branching.
    fn search(space: Space, searcher: &Searcher) -> Option<Solution>;
}

/// Single-threaded LIFO list of nodes to explore.
#[derive(Default)]
pub struct Stack {
    solution: Option<Solution>,

    tasks: Vec<(Branch, Rc<Space>)>,
}

impl Backlog for Stack {
    fn search(space: Space, searcher: &Searcher) -> Option<Solution> {
        let mut backlog = Self::default();

        backlog.push_branches(space);

        while let Some((branch, space)) = backlog.tasks.pop() {
            let space = (*space).clone();

            // No additional searching required for failed spaces
            match searcher.branch(&branch, space) {
                Propagated::Failed => {}
                Propagated::Fixed(space) => backlog.push_branches(space),
                Propagated::Done(candidate) => {
                    // End search early if user is only looking for feasibility
                    if searcher.stop_on_feasibility {
                        return Some(candidate);
                    }

                    if let Some(solution) = backlog.solution.as_mut() {
                        // Only store new solution if it improves on current best
                        if candidate[searcher.obj] < solution[searcher.obj] {
                            backlog.solution = Some(candidate);
                        }
                    } else {
                        backlog.solution = Some(candidate);
                    }
                }
            }
        }

        backlog.solution
    }
}

impl Stack {
    fn push_branches(&mut self, space: Space) {
        // Store a single copy of search space, drops when all related choices have been explored
        let space = Rc::new(space);

        // Queue branches to be explored
        for branch in branch(&space.vars) {
            self.tasks.push((branch, Rc::clone(&space)));
        }
    }
}
