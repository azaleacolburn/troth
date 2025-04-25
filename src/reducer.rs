use crate::parser::Expression;

impl Expression {
    pub fn reduce(&self) -> Expression {
        match self {
            Expression::Application(lambda, arg) => {
                let reduced_lambda = lambda.reduce();
                let reduced_arg = arg.reduce();

                if let Expression::Abstraction(id, mut expr) = reduced_lambda {
                    expr.replace(&id, reduced_arg);
                    return expr.reduce();
                }
                return Expression::Application(Box::new(reduced_lambda), Box::new(reduced_arg));
            }
            Expression::Abstraction(id, expr) => Expression::Abstraction(id.clone(), expr.clone()),
            Expression::Id(id) => Expression::Id(id.clone()),
        }
    }

    fn replace(&mut self, id: &str, argument: Expression) {
        match self {
            Expression::Id(sub_id) if sub_id == id => {
                *self = argument;
            }
            Expression::Application(lambda, expr) => {
                lambda.replace(id, argument.clone());
                expr.replace(id, argument);
            }
            Expression::Abstraction(_sub_id, expr) => {
                expr.replace(id, argument);
            }
            Expression::Id(_) => {}
        }
    }
}
