mod agenda;
mod branch;

use crate::props::Propagators;
use crate::vars::Vars;

/// Data required to perform search, copied on branch and discarded on failure.
#[derive(Clone, Debug)]
pub struct Space {
    pub vars: Vars,
    pub props: Propagators,
}
