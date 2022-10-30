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
        todo!()
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
