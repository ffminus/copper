# Copper

A constraint programming solver.

A solver is a declarative framework, where combinatorial problems are described in terms of decision variables and binding constraints. The algorithms used to find a feasible solution or minimize an expression are abstracted away from the user. Focus on problem definition, create relationships between variables, and Copper will take care of performing the actual search.


## Constraint programming or linear solvers?

Most integer programming solvers, be they [open source](https://github.com/coin-or/Clp) or [commercial](https://www.gurobi.com), are restricted to linear constraints. This unlocks performant methods like the [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm), which make them much faster than constraint programming solvers on linear problems.

However, it requires users to formulate their constraints in terms of linear equations. This restriction can lead to [awkward linearizations](https://en.wikipedia.org/wiki/Travelling_salesman_problem#Integer_linear_programming_formulations), [iffy approximations](https://www.gurobi.com/documentation/current/refman/objectives.html#subsubsection:PiecewiseObj), or [existential dread](https://en.wikipedia.org/wiki/Social_golfer_problem).

Constraint programming imposes no such restriction on the relationships between decision variables. Complex, non-linear constraints can even help the propagation engine prune the search space further, and improve overall performance.


## Why Copper?

- **Straight and strict API:** attention to developer experience makes Copper easy to pick up, and Rust's robust type system lets us expose strict APIs that are harder to misuse.
- **Permissive license:** Copper is developed under the [MIT license](https://tldrlegal.com/license/mit-license), which imposes few limitations on end users. Free as in beer *and* speech.


## Why not Copper?

- **Performance:** Copper is still quite early in its development, it cannot rival with mature solvers like [Gecode](https://www.gecode.org) or [or-tools](https://github.com/google/or-tools). Though written in Rust, it is not **⚡️ blazingly fast &trade;⚡️** (yet).
- **Features:** Copper currently. supports a limited number of variable types and constraints. You can extend its engine yourself, but other solvers will offer more out of the box.


## Disclaimer

Copper is under heavy development. Some exposed APIs are subject to change, including the ones that relate to custom propagators and branching strategies.
