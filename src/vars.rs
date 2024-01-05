use core::ops::{Index, IndexMut};

use crate::props::PropId;

/// Domain for a decision variable, tracked as an interval of integers.
#[derive(Clone, Debug)]
pub struct Var {
    pub min: i32,
    pub max: i32,
}

/// Store decision variables and expose a limited interface to operate on them.
#[derive(Clone, Debug, Default)]
pub struct Vars(Vec<Var>);

impl Vars {
    /// Create a new decision variable.
    pub fn new_var_with_bounds(&mut self, min: i32, max: i32) -> VarId {
        let v = VarId(self.0.len());

        self.0.push(Var { min, max });

        v
    }
}

/// Decision variable handle that is not bound to a specific memory location.
#[derive(Clone, Copy, Debug)]
pub struct VarId(usize);

impl Index<VarId> for Vars {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<VarId> for Vars {
    fn index_mut(&mut self, index: VarId) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}
