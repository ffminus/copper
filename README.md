# Copper

A constraint programming solver for Rust.

A solver is a declarative framework, where combinatorial problems are described in terms of variables to assign and constraints to uphold. The algorithms used to find a solution or minimize an expression are abstracted away from the user. You can focus on problem definition, create relationships between variables, and Copper will take care of performing the actual search.


## Usage

Below is a formulation of the [Knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem): it maximizes the value of a bag with a fixed maximum weight.

```rust
// Problem parameters
let item_weights = [10, 60, 30, 40, 30, 20,  20,  2];
let item_values  = [ 1, 10, 15, 40, 60, 90, 100, 15];
let weight_max = 102;

// Model object, used to declare variables, expressions, and constraints
let mut m = copper::Model::new();

// Binary decision variables: for each item, do I put it in the bag?
let xs = m.new_vars_binary(item_weights.len());

// Sum the weight of the selected items using a linear expression
let weight = m.linear(&xs, &item_weights);

// Ensure the bag's weight does not exceed the maximum
m.leq(weight, weight_max);

// Sum the value of the selected items
let value = m.linear(&xs, &item_values);

// Find the selection of items that maximizes the bag's value
let solution = m.maximize(value).unwrap();

// For each item, does the bag with maximal value contain it?
let is_item_in_bag = solution.get_values_binary(&xs);

assert_eq!(is_item_in_bag, vec![false, false, true, false, true, true, true, true]);
assert_eq!(solution[weight], 102);
assert_eq!(solution[value ], 280);
```


## Constraint programming or linear solvers?

Most well-known solvers, be they [open source](https://github.com/coin-or/Clp) or [commercial](https://www.gurobi.com/), are limited to linear modeling. This unlocks performant methods like the [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm), but forces users to formulate their constraints in terms of linear equations.

Constraint programming imposes no such restriction on the relationships between variables. This allows expressions that would be approximated, computationally expensive, or even impossible to formulate in linear solvers.

Give linear solvers a try first if your problem can easily be expressed within their limitations, especially if your decision variables are floating points.


## Why Copper?

- **Straight and strict API**: A reduced list of features makes Copper easy to pick up, and Rust's robust type system lets the solver expose strict APIs that are hard to misuse.
- **WASM bindings**: Ever wanted to deploy an optimization model to the web without maintaining dedicated servers? Copper's WASM package weighs 20kB and lets the solver run directly in the browser, with close to native performance.
- **Permissive license**: Copper is developed under the [MIT license](https://tldrlegal.com/license/mit-license), which imposes few limitations on end users. Free as in beer <ins>and</ins> speech.


## Why not Copper?

- **Performance**: Copper is still quite early in its development, it cannot yet compare to mature solvers like [Gecode](https://www.gecode.org/) or [or-tools](https://github.com/google/or-tools). This can quickly become a deal-breaker as combinatorial problems grow in size.
- **Features**: Copper supports a limited number of variable types and constraints to reduce implementation complexity. Though its design allows extensibility, other solvers will offer more out-of-the-box.


## Disclaimer

Copper is still under heavy development. Some APIs are subject to change, especially the ones that relate to extending the solver with custom branching strategies and propagators. Use at your own risk!
