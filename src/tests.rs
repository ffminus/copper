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
