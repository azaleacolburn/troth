use crate::{interpret, load, parser::Expression as Expr, type_system::ExprType};

#[test]
fn id_reduct() {
    let expect = Expr::Id("y".into());
    test("id_reduct", Some(expect));
}

#[test]
fn lambda_reduct() {
    let expect = Expr::Abstraction {
        arg: "y".into(),
        expr: Box::new(Expr::Id("y".into())),
        t: None,
    };
    test("lambda_reduct", Some(expect));
}

#[test]
fn if_else() {
    let expect = Expr::Id("a".into());
    test("if_else", Some(expect));
}

#[test]
fn bool() {
    let expect = Expr::Id("first".into());
    test("bool", Some(expect));
}

#[test]
fn arithmetic() {
    let expect = Expr::Id("a".into());
    test("arithmetic", Some(expect));
}

#[test]
fn use_statement() {
    let expect = Expr::Id("first".into());
    test("use_statement", Some(expect));
}

#[test]
fn fibonacci_y_combinator() {
    let expect = Expr::Id("a".into());
    test("fibonacci_y_combinator", Some(expect));
}

fn test(name: impl ToString, expect: Option<Expr>) {
    let reduced = interpret(load(name)).unwrap();
    assert_eq!(reduced, expect);
}
