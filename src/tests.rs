use crate::{Model, Solution};

#[test]
fn new_var() {
    let mut m = Model::default();

    assert!(m.new_var(1, 1).is_none());
    assert!(m.new_var(1, 0).is_none());
    assert!(m.new_var(0, 1).is_some());
}

#[test]
fn enumerate() {
    let mut m = Model::default();

    let (min, max) = (-7, 9);

    let _x = m.new_var(min, max);

    let mut solutions: Vec<_> = m.enumerate().collect();
    solutions.sort();

    let expected: Vec<_> = (min..=max).map(|v| Solution::from(vec![v])).collect();

    assert_eq!(solutions, expected);
}
