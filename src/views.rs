use crate::vars::{VarId, Vars};

/// Apply simple domain transformations on the fly to make propagators more generic.
#[allow(private_bounds)]
pub trait View: ViewRaw {
    /// Get the handle of the variable this view depends on.
    fn get_underlying_var(self) -> Option<VarId> {
        self.get_underlying_var_raw()
    }

    /// Access domain minimum.
    fn min(self, ctx: &Context) -> i32 {
        self.min_raw(ctx.vars)
    }

    /// Access domain maximum.
    fn max(self, ctx: &Context) -> i32 {
        self.max_raw(ctx.vars)
    }

    /// Try to set the provided value as domain minimum, failing the search space on infeasibility.
    ///
    /// The `None` case signals failure, otherwise the new minimum is returned.
    fn try_set_min(self, min: i32, ctx: &mut Context) -> Option<i32>;

    /// Try to the set provided value as domain maximum, failing the search space on infeasibility.
    ///
    /// The `None` case signals failure, otherwise the new maximum is returned.
    fn try_set_max(self, max: i32, ctx: &mut Context) -> Option<i32>;
}

/// Wrapper around search space object to restrict exposed interface and track changes.
#[derive(Debug)]
pub struct Context<'s> {
    vars: &'s mut Vars,
    events: &'s mut Vec<VarId>,
}

impl Context<'_> {
    /// Try to set provided value as domain maximum, failing the space on infeasibility.
    pub fn try_set_min(&mut self, v: VarId, min: i32) -> Option<i32> {
        // Access domain of variable using the provided handle
        let var = &mut self.vars[v];

        // Infeasible, fail space
        if min > var.max {
            return None;
        }

        if min > var.min {
            // Set new minimum
            var.min = min;

            // Record modification event
            self.events.push(v);
        }

        Some(var.min)
    }

    /// Try to set provided value as domain maximum, failing the space on infeasibility.
    pub fn try_set_max(&mut self, v: VarId, max: i32) -> Option<i32> {
        let var = &mut self.vars[v];

        // Infeasible, fail space
        if max < var.min {
            return None;
        }

        if max < var.max {
            // Set new maximum
            var.max = max;

            // Record modification event
            self.events.push(v);
        }

        Some(var.max)
    }
}

// Trait kept internal, to prevent users from declaring their own views.
pub(crate) trait ViewRaw: Copy + core::fmt::Debug + 'static {
    /// Get the handle of the variable this view depends on.
    fn get_underlying_var_raw(self) -> Option<VarId>;

    /// Access domain minimum.
    fn min_raw(self, vars: &Vars) -> i32;

    /// Access domain maximum.
    fn max_raw(self, vars: &Vars) -> i32;
}
