pub mod backlog;
pub mod branch;

use std::collections::VecDeque;

use crate::props::{self, PropId, Propagate, Props};
use crate::solution::Solution;
use crate::vars::{Var, VarId, Vars};

use self::backlog::Backlog;
use self::branch::{Branch, Choice};

/// Store immutable model variables referenced during search.
pub struct Searcher<'s> {
    deps: &'s Deps,
    obj: VarId,
    stop_on_feasibility: bool,
}

impl<'s> Searcher<'s> {
    pub const fn new(deps: &'s Deps, obj: VarId, stop_on_feasibility: bool) -> Self {
        Self {
            deps,
            obj,
            stop_on_feasibility,
        }
    }

    pub fn search<B: Backlog>(&self, vars: &[Var], props: &Props) -> Option<Solution> {
        let space = Space {
            vars: Vars::new(vars),
            props: props.clone(),
        };

        // Initial propagation runs all declared propagators
        match self.propagate_with_all_props(props, space) {
            Propagated::Failed => None,
            Propagated::Fixed(space) => B::search(space, self),
            Propagated::Done(solution) => Some(solution),
        }
    }

    fn propagate_with_all_props(&self, props: &Props, space: Space) -> Propagated {
        let agenda = (0..props.scale_pos.len())
            .map(PropId::ScalePos)
            .chain((0..props.scale_neg.len()).map(PropId::ScaleNeg))
            .chain((0..props.plus.len()).map(PropId::Plus))
            .chain((0..props.sum.len()).map(PropId::Sum))
            .chain((0..props.eq.len()).map(PropId::Eq))
            .chain((0..props.leq.len()).map(PropId::Leq))
            .collect();

        self.propagate(space, agenda)
    }

    fn branch(&self, branch: &Branch, mut space: Space) -> Propagated {
        // Apply selected branch to search space
        match branch.choice {
            Choice::Set(val) => space.vars.set_unchecked(branch.pivot, val),
        }

        // Only set dependent propagators as active
        let mut agenda = VecDeque::new();
        self.schedule_props_from_domain_changes(&mut space.vars, &mut agenda);

        self.propagate(space, agenda)
    }

    fn propagate(&self, mut space: Space, mut agenda: VecDeque<PropId>) -> Propagated {
        // Apply all active propagators, until they are all at a fixed point, or the space fails
        while let Some(id) = agenda.pop_front() {
            // Branch on id type, to avoid dynamic dispatch for propagator and its dependencies
            let vars_opt = match id {
                PropId::ScalePos(i) => {
                    space.props.scale_pos[i].propagate(&self.deps.props.scale_pos[i], space.vars)
                }
                PropId::ScaleNeg(i) => {
                    space.props.scale_neg[i].propagate(&self.deps.props.scale_neg[i], space.vars)
                }
                PropId::Plus(i) => {
                    space.props.plus[i].propagate(&self.deps.props.plus[i], space.vars)
                }
                PropId::Sum(i) => space.props.sum[i].propagate(&self.deps.props.sum[i], space.vars),
                PropId::Eq(i) => space.props.eq[i].propagate(&self.deps.props.eq[i], space.vars),
                PropId::Leq(i) => space.props.leq[i].propagate(&self.deps.props.leq[i], space.vars),
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
#[derive(Clone, Debug)]
pub struct Space {
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
    pub scale_pos: Vec<<props::PropScalePos as Propagate>::Deps>,
    pub scale_neg: Vec<<props::PropScaleNeg as Propagate>::Deps>,
    pub plus: Vec<<props::PropPlus as Propagate>::Deps>,
    pub sum: Vec<<props::PropSum as Propagate>::Deps>,
    pub eq: Vec<<props::PropEq as Propagate>::Deps>,
    pub leq: Vec<<props::PropLeq as Propagate>::Deps>,
}
