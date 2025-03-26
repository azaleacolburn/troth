use std::fs::read_to_string;

use crate::{interpret, parser::Expression as Expr};

#[test]
fn id_reduct() {
    let expect = Expr::Id("y".into());
    test("id_reduct", expect);
}

#[test]
fn lambda_reduct() {
    let expect = Expr::Lambda("y".into(), Box::new(Expr::Id("y".into())));
    test("lambda_reduct", expect);
}

#[test]
fn if_else() {
    let expect = Expr::Id("a".into());
    test("if_else", expect);
}

#[test]
fn bool_logic() {
    let expect = Expr::Id("b".into());
    test("bool_logic", expect);
}

fn test(name: impl ToString, expect: Expr) {
    let reduced = interpret(load(name));
    assert_eq!(reduced, expect);
}
fn load(name: impl ToString) -> String {
    let name = format!("./tests/{}.lc", name.to_string());
    read_to_string(name).expect("Test function missing test file")
}
