mod agenda;
mod branch;

use crate::props::Propagators;
use crate::solution::Solution;
use crate::vars::Vars;
use crate::views::Context;

use self::agenda::Agenda;

/// Data required to perform search, copied on branch and discarded on failure.
#[derive(Clone, Debug)]
pub struct Space {
    pub vars: Vars,
    pub props: Propagators,
}

/// Perform search, iterating over assignments that satisfy all constraints.
pub gen fn search(vars: Vars, props: Propagators) -> Solution {
    // Schedule all propagators during initial propagation step
    let agenda = Agenda::with_props(props.get_prop_ids_iter());

    // Propagate constraints until search is stalled or a solution is found
    if let Some((is_stalled, space)) = propagate(Space { vars, props }, agenda) {
        if is_stalled {
            // Explore space by alternating branching and propagation
            for solution in search_with_branching(space) {
                yield solution;
            }
        } else {
            // Extract solution assignment for all decision variables
            yield space.vars.into_solution();
        }
    }
}

/// Explore search tree, leveraging propagators to prune domains.
gen fn search_with_branching(space: Space) -> Solution {
    // Branching strategy when search is stalled
    let get_branch_iter = branch::split_on_unassigned;

    // Preserve a trail of copies to allow backtracking on failed spaces
    let mut stack = vec![get_branch_iter(space)];

    while let Some(mut branch_iter) = stack.pop() {
        while let Some((space, p)) = branch_iter.next() {
            // Schedule propagator triggered by the branch
            let agenda = Agenda::with_props(core::iter::once(p));

            // Failed spaces are discarded, fixed points get explored further (depth-first search)
            if let Some((is_stalled, space)) = propagate(space, agenda) {
                if is_stalled {
                    // Save where search will resume if sub-space gets failed
                    stack.push(branch_iter);

                    // Branch on new space, to explore it further
                    branch_iter = get_branch_iter(space);
                } else {
                    // Extract solution assignment for all decision variables
                    yield space.vars.into_solution();
                }
            }
        }
    }
}

/// Apply scheduled propagators, pruning domains until space is failed, stalled, or assigned.
fn propagate(mut space: Space, mut agenda: Agenda) -> Option<(bool, Space)> {
    // Track which domains got updated, to schedule next propagators in batch
    let mut events = Vec::new();

    // Agenda establishes the order in which scheduled propagators get run
    while let Some(p) = agenda.pop() {
        // Acquire trait object for propagator, which points to both code and inner state
        let prop = space.props.get_state_mut(p);

        // Wrap engine objects before passing them to user-controlled propagation logic
        let mut ctx = Context::new(&mut space.vars, &mut events);

        // Prune decision variable domains to enforce constraints
        prop.prune(&mut ctx)?;

        // Schedule propagators that depend on changed variables
        #[allow(clippy::iter_with_drain)]
        for v in events.drain(..) {
            for p in space.props.on_bound_change(v) {
                agenda.schedule(p);
            }
        }

        // Search is over once all decision variables have been assigned
        if space.vars.is_assigned_all() {
            return Some((false, space));
        }
    }

    Some((true, space))
}
