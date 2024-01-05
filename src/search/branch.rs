use crate::props::PropId;
use crate::search::Space;

/// Perform a binary split on the first unassigned decision variable.
pub gen fn split_on_unassigned(space: Space) -> (Space, PropId) {
    if let Some(pivot) = space.vars.get_unassigned_var() {
        // Split domain at mid-point of domain
        let mid = space.vars[pivot].mid();

        // Split the provided space using a new propagator, to explore a specific branch.
        let mut space_branch_left = space.clone();
        let p = space_branch_left.props.less_than_or_equals(pivot, mid);
        yield (space_branch_left, p);

        // Right branch
        let mut space_branch_right = space;
        let p = space_branch_right.props.greater_than(pivot, mid);
        yield (space_branch_right, p);
    }
}
