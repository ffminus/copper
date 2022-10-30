use std::collections::HashSet;
use std::ops::{Deref, Index};

/// New-type wrapper to identify decision variables and expressions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VarId(usize);

impl VarId {
    pub(crate) const fn new(i: usize) -> Self {
        Self(i)
    }
}

impl Deref for VarId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Decision variable or expression, with its associated domain bounds.
#[derive(Clone, Debug)]
pub struct Var {
    pub min: i32,
    pub max: i32,
}

/// Decision variable domains, encapsulated to track changes used to schedule propagators.
#[derive(Clone, Debug)]
pub struct Vars {
    vars: Vec<Var>,

    events: HashSet<VarId>,
}

impl Vars {
    pub fn new(vars: &[Var]) -> Self {
        Self {
            vars: vars.to_vec(),
            events: HashSet::new(),
        }
    }

    pub fn set_min_and_max(&mut self, id: VarId, min: i32, max: i32) {
        self.set_min(id, min);
        self.set_max(id, max);
    }

    pub fn set_min(&mut self, id: VarId, min: i32) {
        let var = &mut self.vars[*id];

        if min > var.min {
            var.min = min;
            self.events.insert(id);
        }
    }

    pub fn set_max(&mut self, id: VarId, max: i32) {
        let var = &mut self.vars[*id];

        if max < var.max {
            var.max = max;
            self.events.insert(id);
        }
    }
}

impl Index<VarId> for Vars {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.vars[*index]
    }
}
