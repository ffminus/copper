//! Here is an example to describe how a typical model is formulated.
//! It is a rendition of a combinatorial optimization classic:
//! the [Knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem).
//!
//! Let's say we are building a brand new PC. We want to play AAA games without going bankrupt.

use copper::views::ViewExt;

fn main() {
    // All problem formulations will start with a model object
    let mut m = copper::Model::default();

    // How many monitors do we buy: we need at least one, but not more than three
    let n_monitors = m.new_var(1, 3).unwrap();

    // All monitors cost the same, and each additional monitor provides the same bump to our score
    let monitor_price = 100;
    let monitor_score = 250;

    // Each GPU model has a fixed price, and an associated benchmark score
    let gpu_prices = [150, 250, 500];
    let gpu_scores = [100, 400, 800];

    // We use binary decision variables to represent "do I pick this GPU?"
    let gpus: Vec<_> = m.new_vars_binary(gpu_scores.len()).collect();

    // For each potential GPU, we multiply its price (and score) by whether or not it is selected.
    // The sum of these terms gives us the price and score of the selected GPU.
    let gpu_price = m.sum_iter(gpus.iter().zip(gpu_prices).map(|(x, price)| x.times(price)));
    let gpu_score = m.sum_iter(gpus.iter().zip(gpu_scores).map(|(x, score)| x.times(score)));

    // This expression is the overall price of our build
    let price = m.add(gpu_price, n_monitors.times(monitor_price));

    // We want to maximize this score: how much we'll value this particular build
    let score = m.add(gpu_score, n_monitors.times(monitor_score));

    // Exactly one GPU: we want to run Crysis, but our case must fit under the desk
    let n_gpus = m.sum(&gpus);
    m.equals(n_gpus, 1);

    // Grandma got us some money for our birthday, that will be our budget
    m.less_than_or_equals(price, 600);

    // Let the solver find the assignment that upholds our constraints and maximizes our score
    let solution = m.maximize(score).unwrap();

    // Our optimal build has three monitors and a mid-tier GPU. We even have some left-over cash!
    assert_eq!(solution[n_monitors], 3);
    assert_eq!(solution.get_values_binary(&gpus), vec![false, true, false]);
    assert_eq!(solution[score], 1150);
    assert_eq!(solution[price], 550);
}
