use std::collections::HashSet;
use std::ops::{Deref, Index, Range};

use crate::props::{Failed, ResultProp};

/// New-type wrapper to identify decision variables and expressions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
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

/// Iterate over variable ids in model.
#[derive(Clone)]
pub struct VarIds(Range<usize>);

impl Iterator for VarIds {
    type Item = VarId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(VarId::new)
    }
}

#[cfg(feature = "wasm")]
pub mod wasm {
    /// Identify decision variables and expressions with their ids
    pub type VarId = usize;

    impl From<super::VarId> for VarId {
        fn from(x: super::VarId) -> Self {
            *x
        }
    }

    impl From<VarId> for super::VarId {
        fn from(x: VarId) -> Self {
            Self::new(x)
        }
    }

    pub fn into_boxed_slice_of_ids(xs: Vec<super::VarId>) -> Box<[VarId]> {
        xs.into_iter().map(Into::into).collect()
    }

    pub fn from_slice_of_ids(xs: &[VarId]) -> Vec<super::VarId> {
        xs.iter().copied().map(Into::into).collect()
    }
}

/// Decision variable or expression, with its associated domain bounds.
#[derive(Clone, Debug)]
pub struct Var {
    pub min: i32,
    pub max: i32,
}

impl Var {
    /// Variable domain is reduced to a singleton.
    pub const fn is_set(&self) -> bool {
        self.min == self.max
    }

    /// Assign value to variable, if it is contained in its domain.
    fn try_set(&mut self, value: i32) -> Result<(), Failed> {
        if value < self.min || value > self.max {
            return Err(Failed);
        }

        self.min = value;
        self.max = value;

        Ok(())
    }
}

/// Decision variable domains, encapsulated to track changes used to schedule propagators.
#[derive(Clone, Debug)]
pub struct Vars {
    vars: Vec<Var>,

    events: HashSet<VarId>,
}

impl Vars {
    /// Try to assign a value to a variable.
    ///
    /// # Errors
    ///
    /// Function will fail if `value` falls outside of the domain's current bounds.
    pub fn try_set(mut self, id: VarId, value: i32) -> ResultProp {
        let var = &mut self.vars[*id];

        let was_variable_already_set = var.is_set();

        var.try_set(value)?;

        if !was_variable_already_set {
            self.events.insert(id);
        }

        Ok(self)
    }

    /// Try to bound a variable's domain with a minimum and maximum value.
    ///
    /// # Errors
    ///
    /// Function will fail if either:
    ///     - `min` is greater than `max`
    ///     - `min` is greater than the domain's current maximum
    ///     - `max` is smaller than the domain's current minimum
    pub fn try_set_min_and_max(self, id: VarId, min: i32, max: i32) -> ResultProp {
        ensure_min_leq_max(min, max)?;

        let vars = self.try_set_min(id, min)?;
        let vars = vars.try_set_max(id, max)?;

        Ok(vars)
    }

    /// Try to bound a variable's domain with a minimum value.
    ///
    /// # Errors
    ///
    /// Function will fail if `min` is greater than the domain's current maximum.
    pub fn try_set_min(mut self, id: VarId, min: i32) -> ResultProp {
        let var = &mut self.vars[*id];

        ensure_min_leq_max(min, var.max)?;

        if min > var.min {
            var.min = min;
            self.events.insert(id);
        }

        Ok(self)
    }

    /// Try to bound a variable's domain with a maximum value.
    ///
    /// # Errors
    ///
    /// Function will fail if `max` is smaller than the domain's current minimum.
    pub fn try_set_max(mut self, id: VarId, max: i32) -> ResultProp {
        let var = &mut self.vars[*id];

        ensure_min_leq_max(var.min, max)?;

        if max < var.max {
            var.max = max;
            self.events.insert(id);
        }

        Ok(self)
    }

    /// Iterate over variable ids.
    #[must_use]
    pub fn get_var_ids(&self) -> VarIds {
        VarIds(0..self.vars.len())
    }

    pub(crate) fn new(vars: &[Var]) -> Self {
        Self {
            vars: vars.to_vec(),
            events: HashSet::new(),
        }
    }

    /// Extract assignments if all domains are singletons.
    pub(crate) fn get_assignment_if_all_variables_are_set(&self) -> Option<Vec<i32>> {
        if self.vars.iter().all(Var::is_set) {
            Some(self.vars.iter().map(|var| var.min).collect())
        } else {
            None
        }
    }

    /// Iterate over change events triggered by propagation, while preserving allocated capacity.
    pub(crate) fn drain_events(&mut self) -> impl Iterator<Item = VarId> + '_ {
        self.events.drain()
    }
}

impl Index<VarId> for Vars {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.vars[*index]
    }
}

/// Validate provided minimum is below maximum
const fn ensure_min_leq_max(min: i32, max: i32) -> Result<(), Failed> {
    if min > max {
        Err(Failed)
    } else {
        Ok(())
    }
}
