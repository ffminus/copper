use crate::views::ViewExt;
use crate::{Model, Solution};

#[test]
fn new_var() {
    let mut m = Model::default();

    assert!(m.new_var(1, 1).is_none());
    assert!(m.new_var(1, 0).is_none());
    assert!(m.new_var(0, 1).is_some());
}

#[test]
fn new_vars() {
    let mut m = Model::default();

    assert!(m.new_vars(5, 1, 1).is_none());
    assert!(m.new_vars(5, 1, 0).is_none());
    assert!(m.new_vars(5, 0, 1).is_some());
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

#[test]
fn minimize() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    assert_eq!(m.minimize(x).unwrap()[x], -7);
}

#[test]
fn maximize() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    assert_eq!(m.maximize(x).unwrap()[x], 9);
}

#[test]
fn opposite() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.opposite(), 5);

    assert_eq!(m.solve().unwrap()[x], -5);
}

#[test]
fn opposite_of_opposite() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.opposite().opposite(), 6);

    assert_eq!(m.solve().unwrap()[x], 6);
}

#[test]
fn plus() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.plus(5), 7);

    assert_eq!(m.solve().unwrap()[x], 2);
}

#[test]
fn plus_unfeasible() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.plus(10), 1);

    assert!(m.solve().is_none());
}

#[test]
fn times_with_neg_scale() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times(-2), 4);

    assert_eq!(m.solve().unwrap()[x], -2);
}

#[test]
fn times_with_neg_scale_unfeasible() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times(-2), 3);

    assert!(m.solve().is_none());
}

#[test]
fn times_with_zero_scale() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times(0), 0);

    assert_eq!(m.maximize(x).unwrap()[x], 9);
}

#[test]
fn times_with_zero_scale_unfeasible() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times(0), 4);

    assert!(m.solve().is_none());
}

#[test]
fn times_with_pos_scale() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times(2), 4);

    assert_eq!(m.solve().unwrap()[x], 2);
}

#[test]
fn times_with_pos_scale_unfeasible() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times(2), 3);

    assert!(m.solve().is_none());
}

#[test]
fn times_pos() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times_pos(2), 4);

    assert_eq!(m.solve().unwrap()[x], 2);
}

#[test]
fn times_pos_unfeasible() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times_pos(2), 3);

    assert!(m.solve().is_none());
}

#[test]
fn times_neg() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times_neg(-2), 4);

    assert_eq!(m.solve().unwrap()[x], -2);
}

#[test]
fn times_neg_unfeasible() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x.times_neg(-2), 3);

    assert!(m.solve().is_none());
}

#[test]
fn add() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();
    let y = m.new_var(-7, 9).unwrap();
    let p = m.add(x, y);

    m.equals(p, 18);

    let solution = m.solve().unwrap();

    assert_eq!(solution[x], 9);
    assert_eq!(solution[y], 9);
    assert_eq!(solution[p], 18);
}

#[test]
fn sum() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();
    let y = m.new_var(-7, 9).unwrap();
    let s = m.sum(&[x, y]);

    let solution = m.maximize(s).unwrap();

    assert_eq!(solution[x], 9);
    assert_eq!(solution[y], 9);
    assert_eq!(solution[s], 18);
}

#[test]
fn equals() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();
    let y = m.new_var(4, 8).unwrap();

    m.equals(x, y);

    let solution = m.minimize(x).unwrap();

    assert_eq!(solution[x], 4);
    assert_eq!(solution[y], 4);
}

#[test]
fn equals_with_constant() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.equals(x, 4);

    assert_eq!(m.solve().unwrap()[x], 4);
}

#[test]
fn less_than_or_equals() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();
    let y = m.new_var(1, 3).unwrap();

    m.less_than_or_equals(x, y);

    let solution = m.maximize(x).unwrap();

    assert_eq!(solution[x], 3);
    assert_eq!(solution[y], 3);
}

#[test]
fn less_than_or_equals_with_constant() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();

    m.less_than_or_equals(x, 1);

    assert_eq!(m.maximize(x).unwrap()[x], 1);
}

#[test]
fn less_than() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();
    let y = m.new_var(1, 3).unwrap();

    m.less_than(x, y);

    let solution = m.maximize(x).unwrap();

    assert_eq!(solution[x], 2);
    assert_eq!(solution[y], 3);
}

#[test]
fn greater_than_or_equals() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();
    let y = m.new_var(1, 3).unwrap();

    m.greater_than_or_equals(x, y);

    let solution = m.minimize(x).unwrap();

    assert_eq!(solution[x], 1);
    assert_eq!(solution[y], 1);
}

#[test]
fn greater_than() {
    let mut m = Model::default();

    let x = m.new_var(-7, 9).unwrap();
    let y = m.new_var(1, 3).unwrap();

    m.greater_than(x, y);

    let solution = m.minimize(x).unwrap();

    assert_eq!(solution[x], 2);
    assert_eq!(solution[y], 1);
}
