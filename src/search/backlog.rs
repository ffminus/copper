use std::rc::Rc;

use crate::solution::Solution;

use super::branch::{branch, Choice};
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

    tasks: Vec<(Choice, Rc<Space>)>,
}

impl Backlog for Stack {
    fn search(space: Space, searcher: &Searcher) -> Option<Solution> {
        let mut backlog = Self::default();

        backlog.push_tasks(space);

        while let Some((choice, space)) = backlog.tasks.pop() {
            let space = (*space).clone();

            // Provide current best objective value to allow for additional pruning
            let obj_opt = backlog
                .solution
                .as_ref()
                .map(|solution| solution[searcher.obj]);

            // No additional searching required for failed spaces
            if let Ok(propagated) = searcher.branch(&choice, obj_opt, space) {
                match propagated {
                    Propagated::Fixed(space) => backlog.push_tasks(space),
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
        }

        backlog.solution
    }
}

impl Stack {
    fn push_tasks(&mut self, space: Space) {
        // Store a single copy of search space, drops when all related choices have been explored
        let space = Rc::new(space);

        // Queue branches to be explored
        for choice in branch(&space.vars) {
            self.tasks.push((choice, Rc::clone(&space)));
        }
    }
}
