use std::fs::read_to_string;

use crate::{lexer, parser::Expression as Expr, reducer, token_handler};

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
    let expect = Expr::Id("first".into());
    test("bool_logic", expect);
}

#[test]
fn arithmetic() {
    let expect = Expr::Id("a".into());
    test("arithmetic", expect);
}

fn test(name: impl ToString, expect: Expr) {
    let reduced = interpret(load(name));
    assert_eq!(reduced, expect);
}
fn load(name: impl ToString) -> String {
    let name = format!("./tests/{}.lc", name.to_string());
    read_to_string(name).expect("Test function missing test file")
}

fn interpret(code: String) -> Expr {
    let tokens = lexer::lex(code);
    println!("{:?}", tokens);
    let mut parser = token_handler::Parser::new(tokens);
    let ast = parser.parse().unwrap().unwrap();
    println!("{:?}", ast);
    let reduced = reducer::reduce(&ast);
    println!("{:?}", reduced);
    reduced
}
