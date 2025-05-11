use crate::parser::Expression;

pub fn to_javascript_naive(ast: &Expression) -> String {
    match ast {
        Expression::Application(lambda, arg) => {
            let lambda = to_javascript_naive(lambda);
            let arg = to_javascript_naive(arg);
            format!("{lambda}({arg})")
        }
        Expression::Abstraction { arg, expr, t: _ } => {
            let expr = to_javascript_naive(expr);
            format!("({arg} => {expr})")
        }
        Expression::Id(id) => format!("{id}"),
    }
}
