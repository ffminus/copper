use crate::Model;

#[test]
fn eq() {
    let mut m = Model::default();

    let x = m.new_var(0, 5);

    m.eq(x, 3);

    let solution = m.solve().unwrap();

    assert_eq!(solution[x], 3);
}

#[test]
fn solve() {
    let mut m = Model::default();

    let x = m.new_var(0, 5);
    let y = m.new_var(2, 5);

    m.eq(x, y);

    let solution = m.solve().unwrap();

    assert_eq!(solution[x], 2);
    assert_eq!(solution[y], 2);
}

#[test]
fn minimize() {
    let mut m = Model::default();

    let x = m.new_var(0, 5);

    let solution = m.minimize(x).unwrap();

    assert_eq!(solution[x], 0);
}

#[test]
fn maximize() {
    let mut m = Model::default();

    let x = m.new_var(0, 5);

    let solution = m.maximize(x).unwrap();

    assert_eq!(solution[x], 5);
}
