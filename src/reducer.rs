use crate::parser::Expression;

pub fn reduce(expr: &Expression) -> Expression {
    match expr {
        Expression::Call(lambda, arg) => {
            let reduced_lambda = reduce(lambda);
            let reduced_arg = reduce(arg);

            if let Expression::Lambda(id, mut expr) = reduced_lambda {
                replace(&mut expr, &id, reduced_arg);
                return reduce(&*expr);
            }
            return Expression::Call(Box::new(reduced_lambda), Box::new(reduced_arg));
        }
        Expression::Lambda(id, expr) => Expression::Lambda(id.clone(), expr.clone()),
        Expression::Id(id) => Expression::Id(id.clone()),
    }
}

fn replace(lambda_expr: &mut Expression, id: &str, argument: Expression) {
    match lambda_expr {
        Expression::Id(sub_id) if sub_id == id => {
            *lambda_expr = argument;
        }
        Expression::Call(lambda, expr) => {
            replace(lambda, id, argument.clone());
            replace(expr, id, argument);
        }
        Expression::Lambda(_sub_id, expr) => {
            replace(expr, id, argument);
        }
        Expression::Id(_) => {}
    }
}
