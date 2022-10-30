/// Domain branching strategies.
pub mod branch;

/// Variable selection strategies.
pub mod pick;

pub mod backlog;

use std::collections::VecDeque;

use crate::props::{self, Failed, PropId, Props};
use crate::solution::Solution;
use crate::vars::{Var, VarId, Vars};

use self::backlog::Backlog;
use self::branch::{Branch, Mutation};
use self::pick::Pick;

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

    pub fn search<T: Backlog, P: Pick, B: Branch>(&self, v: &[Var], p: &Props) -> Option<Solution> {
        let space = Space {
            vars: Vars::new(v),
            props: p.clone(),
        };

        // Initial propagation runs all declared propagators
        match self.propagate_with_all_props(p, space).ok()? {
            Propagated::Fixed(space) => T::default().search::<P, B>(space, self),
            Propagated::Done(solution) => Some(solution),
        }
    }

    fn propagate_with_all_props(&self, props: &Props, space: Space) -> ResultProps {
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

    fn branch(&self, choice: &Choice, obj_opt: Option<i32>, mut space: Space) -> ResultProps {
        // Apply selected branch to search space
        space.vars = match choice.mutation {
            Mutation::Set(val) => space.vars.try_set(choice.pivot, val)?,
        };

        // Prune domains that cannot improve on the current best found objective value
        if let Some(obj) = obj_opt {
            space.vars = space.vars.try_set_max(self.obj, obj - 1)?;
        }

        // Only set dependent propagators as active
        let mut agenda = VecDeque::new();
        self.schedule_props_from_domain_changes(&mut space.vars, &mut agenda);

        self.propagate(space, agenda)
    }

    fn propagate(&self, mut space: Space, mut agenda: VecDeque<PropId>) -> ResultProps {
        // Apply all active propagators, until they are all at a fixed point, or the space fails
        while let Some(id) = agenda.pop_front() {
            // Branch on id type, to avoid dynamic dispatch for propagator and its dependencies
            let mut vars = match id {
                PropId::ScalePos(i) => {
                    props::PropScalePos::propagate(self.deps.props.scale_pos[i], space.vars)
                }
                PropId::ScaleNeg(i) => {
                    props::PropScaleNeg::propagate(self.deps.props.scale_neg[i], space.vars)
                }
                PropId::Plus(i) => props::PropPlus::propagate(self.deps.props.plus[i], space.vars),
                PropId::Sum(i) => {
                    let (s, xs) = &self.deps.props.sum[i];
                    props::PropSum::propagate((*s, xs), space.vars)
                }
                PropId::Eq(i) => props::PropEq::propagate(self.deps.props.eq[i], space.vars),
                PropId::Leq(i) => props::PropLeq::propagate(self.deps.props.leq[i], space.vars),
                PropId::Custom(i) => space.props.custom[i].propagate(space.vars),
            }?;

            // Mutated variable domains returned if space is not failed by propagator
            self.schedule_props_from_domain_changes(&mut vars, &mut agenda);

            // Propagator mutated the space's variable domains, pruning unfeasible assignments
            space.vars = vars;
        }

        let propagated =
            if let Some(assignment) = space.vars.get_assignment_if_all_variables_are_set() {
                // All variable domains are reduced to singletons, search is done for this space
                Propagated::Done(Solution::new(assignment))
            } else {
                // Some variable domains are not singletons, subsequent branching is required
                Propagated::Fixed(space)
            };

        Ok(propagated)
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
type ResultProps = Result<Propagated, Failed>;

/// No propagator failed the space, either search is done,
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum Propagated {
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
    pub scale_pos: Vec<props::PropScalePosDeps>,
    pub scale_neg: Vec<props::PropScaleNegDeps>,
    pub plus: Vec<props::PropPlusDeps>,
    pub sum: Vec<props::PropSumDeps>,
    pub eq: Vec<props::PropEqDeps>,
    pub leq: Vec<props::PropLeqDeps>,
}

/// Branch to be applied to mutate search space.
#[derive(Debug)]
pub struct Choice {
    pub pivot: VarId,
    pub mutation: Mutation,
}
