use crate::Model;

#[test]
fn eq() {
    let mut m = Model::default();

    let x = m.new_var(0, 5);

    m.eq(x, 3);

    let solution = m.solve().unwrap();

    assert_eq!(solution[x], 3);
}
