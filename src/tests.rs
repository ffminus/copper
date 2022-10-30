use crate::Model;

#[test]
fn plus() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);
    let y = m.new_var(3, 6);

    let plus = m.plus(x, y);

    let solution = m.minimize(plus).unwrap();

    assert_eq!(solution[x], 0);
    assert_eq!(solution[y], 3);
}

#[test]
fn sum() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);
    let y = m.new_var(3, 6);

    let sum = m.sum(&[x, y]);

    let solution = m.minimize(sum).unwrap();

    assert_eq!(solution[x], 0);
    assert_eq!(solution[y], 3);
}

#[test]
fn eq() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);

    m.eq(x, 3);

    let solution = m.solve().unwrap();

    assert_eq!(solution[x], 3);
}

#[test]
fn leq() {
    let mut m = Model::new();

    let x = m.new_var(0, 9);

    m.leq(x, 5);

    let solution = m.maximize(x).unwrap();

    assert_eq!(solution[x], 5);
}

#[test]
fn solve() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);
    let y = m.new_var(2, 5);

    m.eq(x, y);

    let solution = m.solve().unwrap();

    assert_eq!(solution[x], 2);
    assert_eq!(solution[y], 2);
}

#[test]
fn minimize() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);

    let solution = m.minimize(x).unwrap();

    assert_eq!(solution[x], 0);
}

#[test]
fn maximize() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);

    let solution = m.maximize(x).unwrap();

    assert_eq!(solution[x], 5);
}
