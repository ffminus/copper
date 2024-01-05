use crate::vars::VarId;
use crate::views::{Context, View};

use super::{Propagate, Prune};

/// Enforce inequality between two views: `x <= y`.
#[derive(Clone, Copy, Debug)]
pub struct LessThanOrEquals<U, V> {
    x: U,
    y: V,
}

impl<U, V> LessThanOrEquals<U, V> {
    pub const fn new(x: U, y: V) -> Self {
        Self { x, y }
    }
}

impl<U: View, V: View> Prune for LessThanOrEquals<U, V> {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        let _max = self.x.try_set_max(self.y.max(ctx), ctx)?;
        let _min = self.y.try_set_min(self.x.min(ctx), ctx)?;

        Some(())
    }
}

impl<U: View, V: View> Propagate for LessThanOrEquals<U, V> {
    gen fn list_trigger_vars(&self) -> VarId {
        if let Some(v) = self.x.get_underlying_var() {
            yield v;
        }

        if let Some(v) = self.y.get_underlying_var() {
            yield v;
        }
    }
}
