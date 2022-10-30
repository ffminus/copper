use criterion::{black_box as bb, criterion_group, criterion_main, Criterion};

use copper::Model;

fn run(weights: &[i32], values: &[i32], weight_max: i32) -> i32 {
    // Model object, used to declare variables and constraints
    let mut m = Model::new();

    // Binary decision variables: for each item, do I put it in the bag?
    let xs = m.new_vars_binary(weights.len());

    // Sum the weight of the selected items using a linear expression
    let weight = m.linear(&xs, weights);

    // Ensure the bag's weight does not exceed the maximum
    m.leq(weight, weight_max);

    // Sum the value of the selected items
    let value = m.linear(&xs, values);

    // Find the selection of items that maximizes the bag's value
    let solution = m.maximize(value).unwrap();

    // Total value contained in the bag
    solution[value]
}

#[derive(serde::Deserialize)]
struct Item {
    weight: i32,
    value: i32,
}

fn declare_benchmarks(c: &mut Criterion) {
    let items: Vec<Item> = serde_json::from_str(include_str!("items.json")).unwrap();

    let weights: Vec<_> = items.iter().map(|item| item.weight).collect();
    let values: Vec<_> = items.iter().map(|item| item.value).collect();

    c.bench_function("knapsack", |b| b.iter(|| run(&weights, &values, bb(100))));
}

criterion_group!(benches, declare_benchmarks);
criterion_main!(benches);
