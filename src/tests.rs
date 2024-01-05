use crate::Model;

#[test]
fn new_var() {
    let mut m = Model::default();

    assert!(m.new_var(1, 1).is_none());
    assert!(m.new_var(1, 0).is_none());
    assert!(m.new_var(0, 1).is_some());
}
