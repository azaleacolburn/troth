use crate::parser::Expression;

impl Expression {
    pub fn reduce(&self) -> Expression {
        match self {
            Expression::Application(lambda, arg) => {
                let reduced_lambda = lambda.reduce();
                let reduced_arg = arg.reduce();

                match reduced_lambda {
                    Expression::Abstraction {
                        arg,
                        mut expr,
                        t: _,
                    } => {
                        expr.replace(&arg, &reduced_arg);
                        return expr.reduce();
                    }
                    _ => Expression::Application(Box::new(reduced_lambda), Box::new(reduced_arg)),
                }
            }
            _ => self.clone(),
        }
    }

    fn replace(&mut self, id: &str, argument: &Expression) {
        match self {
            Expression::Id(sub_id) if sub_id == id => {
                *self = argument.clone();
            }
            Expression::Application(lambda, expr) => {
                lambda.replace(id, argument);
                expr.replace(id, argument);
            }
            Expression::Abstraction { arg: _, expr, t: _ } => {
                expr.replace(id, argument);
            }
            Expression::Id(_) => {}
        }
    }
}
