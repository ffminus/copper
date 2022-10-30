use std::collections::VecDeque;

use crate::props::{self, PropId, Propagate, Props};
use crate::solution::Solution;
use crate::vars::{Var, Vars};

/// Store immutable model variables referenced during search.
pub struct Searcher<'s> {
    deps: &'s Deps,
}

impl<'s> Searcher<'s> {
    pub const fn new(deps: &'s Deps) -> Self {
        Self { deps }
    }

    pub fn search(&self, vars: &[Var], props: &Props) -> Option<Solution> {
        let space = Space {
            vars: Vars::new(vars),
            props: props.clone(),
        };

        // Initial propagation runs all declared propagators
        match self.propagate_with_all_props(props, space) {
            Propagated::Failed => None,
            Propagated::Fixed(_) => todo!(),
            Propagated::Done(solution) => Some(solution),
        }
    }

    fn propagate_with_all_props(&self, props: &Props, space: Space) -> Propagated {
        self.propagate(space, (0..props.eq.len()).map(PropId::Eq).collect())
    }

    fn propagate(&self, mut space: Space, mut agenda: VecDeque<PropId>) -> Propagated {
        // Apply all active propagators, until they are all at a fixed point, or the space fails
        while let Some(id) = agenda.pop_front() {
            // Branch on id type, to avoid dynamic dispatch for propagator and its dependencies
            let vars_opt = match id {
                PropId::Eq(i) => space.props.eq[i].propagate(&self.deps.props.eq[i], space.vars),
            };

            // Mutated variable domains returned if space is not failed by propagator
            if let Some(mut vars) = vars_opt {
                self.schedule_props_from_domain_changes(&mut vars, &mut agenda);

                // Propagator mutated the space's variable domains, pruning unfeasible assignments
                space.vars = vars;
            } else {
                // Search space is failed if it contains no feasible assignment
                return Propagated::Failed;
            }
        }

        // All variable domains are reduced to singletons, search is done for this space
        if let Some(assignment) = space.vars.get_assignment_if_all_variables_are_set() {
            Propagated::Done(Solution::new(assignment))
        } else {
            // Some variable domains are not singletons, subsequent branching is required
            Propagated::Fixed(space)
        }
    }

    /// Schedule all dependent propagators of changed variables
    fn schedule_props_from_domain_changes(&self, vars: &mut Vars, agenda: &mut VecDeque<PropId>) {
        for id in vars.drain_events() {
            for propagator_id in &self.deps.vars[*id] {
                agenda.push_back(*propagator_id);
            }
        }
    }
}

/// State required for exploring a search tree.
#[derive(Debug)]
struct Space {
    vars: Vars,
    props: Props,
}

/// Result of applying all propagators on agenda to variable domains.
#[derive(Debug)]
enum Propagated {
    /// No feasible solution exist in this search space.
    Failed,

    /// All propagators are at a fixed point, and domain is not reduced to a single assignment.
    Fixed(Space),

    /// All propagators are at a fixed point, and domain is reduced to a single assignment.
    Done(Solution),
}

/// Subscription mappings, from variables to propagators and vice versa.
#[derive(Debug, Default)]
pub struct Deps {
    pub vars: Vec<Vec<PropId>>,

    pub props: DepsProps,
}

/// Helper struct to group dependencies for each propagator type.
#[derive(Debug, Default)]
pub struct DepsProps {
    pub eq: Vec<<props::PropEq as Propagate>::Deps>,
}
