use crate::vars::VarId;
use crate::views::{Context, View};

use super::{Propagate, Prune};

/// Enforce equality between two views: `x == y`.
#[derive(Clone, Copy, Debug)]
pub struct Equals<U, V> {
    x: U,
    y: V,
}

impl<U, V> Equals<U, V> {
    pub const fn new(x: U, y: V) -> Self {
        Self { x, y }
    }
}

impl<U: View, V: View> Prune for Equals<U, V> {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        let _min = self.x.try_set_min(self.y.min(ctx), ctx)?;
        let _max = self.x.try_set_max(self.y.max(ctx), ctx)?;

        let _min = self.y.try_set_min(self.x.min(ctx), ctx)?;
        let _max = self.y.try_set_max(self.x.max(ctx), ctx)?;

        Some(())
    }
}

impl<U: View, V: View> Propagate for Equals<U, V> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.x
            .get_underlying_var()
            .into_iter()
            .chain(self.y.get_underlying_var())
    }
}
