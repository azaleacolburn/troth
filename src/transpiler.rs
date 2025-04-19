use crate::parser::Expression;

pub fn to_javascript_naive(ast: &Expression) -> String {
    match ast {
        Expression::Application(lambda, arg) => {
            let lambda = to_javascript_naive(lambda);
            let arg = to_javascript_naive(arg);
            format!("{lambda}({arg})")
        }
        Expression::Abstraction(id, expr) => {
            let expr = to_javascript_naive(expr);
            format!("({id} => {expr})")
        }
        Expression::Id(id) => format!("{id}"),
    }
}
