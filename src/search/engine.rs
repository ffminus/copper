use std::rc::Rc;

use crate::solution::Solution;

use super::branch::pick::Pick;
use super::branch::Enumerate;
use super::{Choice, Propagated, Searcher, Space};

/// Engine to schedule spaces to be explored during search.
pub trait Engine<P: Pick, E: Enumerate> {
    /// Initialize engine without requiring a `Default` bound on its generic parameter.
    fn new_engine() -> Self;

    /// Perform search, keeping track of spaces to explore on branching.
    fn search(self, space: Space<P, E>, searcher: &Searcher) -> Option<Solution>;
}

/// Single-threaded LIFO list of nodes to explore.
pub struct Stack<P: Pick, E: Enumerate> {
    solution: Option<Solution>,

    tasks: Vec<(Choice, Rc<Space<P, E>>)>,
}

impl<P: Pick, E: Enumerate> Engine<P, E> for Stack<P, E> {
    fn new_engine() -> Self {
        Self {
            solution: None,
            tasks: Vec::new(),
        }
    }

    fn search(mut self, space: Space<P, E>, searcher: &Searcher) -> Option<Solution> {
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

impl<P: Pick, E: Enumerate> Stack<P, E> {
    fn push_tasks(&mut self, mut space: Space<P, E>) {
        // Select pivot variable to branch on
        if let Some(pivot) = space.picker.pick(&space.vars) {
            let mutations = space.enumerator.branch_on(&space.vars[pivot]);

            // Store a single copy of search space, drops when all choices have been explored
            let space = Rc::new(space);

            // Queue branches to be explored
            for mutation in mutations {
                self.tasks
                    .push((Choice { pivot, mutation }, Rc::clone(&space)));
            }
        }
    }
}
