use crate::{branch, Model, Propagate, ResultProp, VarId, Vars};

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

#[derive(Clone, Debug)]
struct PropEqCustom {
    x: VarId,
    y: VarId,
}

impl Propagate for PropEqCustom {
    fn propagate(&mut self, vars: Vars) -> ResultProp {
        let (var_x, var_y) = (&vars[self.x], &vars[self.y]);

        let min = std::cmp::max(var_x.min, var_y.min);
        let max = std::cmp::min(var_x.max, var_y.max);

        let vars = vars.try_set_min_and_max(self.x, min, max)?;
        let vars = vars.try_set_min_and_max(self.y, min, max)?;

        Ok(vars)
    }
}

#[test]
fn propagate() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);
    let y = m.cst(4);

    m.propagator(PropEqCustom { x, y }, &[x, y]);

    let solution = m.solve().unwrap();

    assert_eq!(solution[x], 4);
}

#[test]
fn max_to_min() {
    let mut m = Model::new();

    let x = m.new_var(0, 5);

    let solution = m.with_brancher::<branch::SetMaxToMin>().solve().unwrap();

    assert_eq!(solution[x], 5);
}
