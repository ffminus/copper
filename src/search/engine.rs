use std::rc::Rc;

use crate::solution::Solution;

use super::branch::Branch;
use super::{Choice, Propagated, Searcher, Space};

/// Engine to schedule spaces to be explored during search.
pub trait Engine<B: Branch> {
    /// Initialize engine without requiring a `Default` bound on its generic parameter.
    fn new_engine() -> Self;

    /// Perform search, keeping track of spaces to explore on branching.
    fn search(self, space: Space<B>, searcher: &Searcher) -> Option<Solution>;
}

/// Single-threaded LIFO list of nodes to explore.
pub struct Stack<B: Branch> {
    solution: Option<Solution>,

    tasks: Vec<(Choice, Rc<Space<B>>)>,
}

impl<B: Branch> Engine<B> for Stack<B> {
    fn new_engine() -> Self {
        Self {
            solution: None,
            tasks: Vec::new(),
        }
    }

    fn search(mut self, space: Space<B>, searcher: &Searcher) -> Option<Solution> {
        self.push_tasks(space);

        while let Some((choice, space)) = self.tasks.pop() {
            let space = (*space).clone();

            // Provide current best objective value to allow for additional pruning
            let obj_opt = self
                .solution
                .as_ref()
                .map(|solution| solution[searcher.obj]);

            // No additional searching required for failed spaces
            if let Ok(propagated) = searcher.mutate_then_propagate(&choice, obj_opt, space) {
                match propagated {
                    Propagated::Fixed(space) => self.push_tasks(space),
                    Propagated::Done(candidate) => {
                        // End search early if user is only looking for feasibility
                        if !searcher.is_exhaustive {
                            return Some(candidate);
                        }

                        if let Some(solution) = self.solution.as_mut() {
                            // Only store new solution if it improves on current best
                            if candidate[searcher.obj] < solution[searcher.obj] {
                                self.solution = Some(candidate);
                            }
                        } else {
                            self.solution = Some(candidate);
                        }
                    }
                }
            }
        }

        self.solution
    }
}

impl<B: Branch> Stack<B> {
    fn push_tasks(&mut self, mut space: Space<B>) {
        // Select pivot variable to branch on
        if let Some(choices) = space.brancher.branch(&space.vars) {
            // Store a single copy of search space, drops when all choices have been explored
            let space = Rc::new(space);

            // Queue branches to be explored
            for choice in choices {
                self.tasks.push((choice, Rc::clone(&space)));
            }
        }
    }
}
