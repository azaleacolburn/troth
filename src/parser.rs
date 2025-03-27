use crate::{lexer::Token, token_handler::TokenHandler};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Id(String),
    Lambda(String, Box<Expression>),
    Call(Box<Expression>, Box<Expression>),
}

// TODO Find better way to return null
pub fn parse(handler: &mut TokenHandler) -> Vec<Expression> {
    match handler.get() {
        Token::Define => {
            handler.next();
            if let Token::Alias(id) = handler.get().clone() {
                handler.next();
                let expr = expression(handler);

                handler.next();
                if *handler.get() != Token::Semi {
                    panic!("Expected Semi after definition");
                }

                handler.new_def(id.clone(), expr);

                if handler.is_done() {
                    return vec![];
                }

                handler.next();
                let r = parse(handler);

                return r;
            }
            panic!("Definition without name");
        }
        _ => vec![expression(handler)],
    }
}

fn expression(handler: &mut TokenHandler) -> Expression {
    match handler.get() {
        Token::OParen => {
            handler.next();
            let expr = match handler.get() {
                Token::Lambda => lambda(handler),
                _ => call(handler),
            };

            handler.next();
            if *handler.get() != Token::CParen {
                panic!("Expected CParen");
            }

            expr
        }
        Token::Lambda => lambda(handler),
        Token::Id(id) => Expression::Id(id.clone()),
        Token::Alias(id) => alpha_conversion(Box::new(handler.get_def(&id)), id),
        c => panic!("Unsupported Token: {:?}", c),
    }
}

fn lambda(handler: &mut TokenHandler) -> Expression {
    handler.next();
    if let Token::Id(id) = handler.get().clone() {
        handler.next();
        if *handler.get() == Token::Dot {
            handler.next();
            return Expression::Lambda(id.clone(), Box::new(expression(handler)));
        }
        panic!("Found lambda without dot");
    }

    panic!("Found lambda without id");
}

fn call(handler: &mut TokenHandler) -> Expression {
    let a = Box::new(expression(handler));
    handler.next();
    let b = Box::new(expression(handler));

    Expression::Call(a, b)
}

// TODO Change to mutate `expr` instead of cloning it
fn alpha_conversion(expr: Box<Expression>, postfix: &str) -> Expression {
    match *expr {
        Expression::Id(id) => {
            let new_id = format!("{id}_{postfix}");
            Expression::Id(new_id)
        }
        Expression::Lambda(id, lambda_expr) => {
            let new_expr = Box::new(alpha_conversion(lambda_expr, postfix));
            let new_id = format!("{id}_{postfix}");
            Expression::Lambda(new_id, new_expr)
        }
        Expression::Call(expr1, expr2) => {
            let new_expr1 = Box::new(alpha_conversion(expr1, postfix));
            let new_expr2 = Box::new(alpha_conversion(expr2, postfix));
            Expression::Call(new_expr1, new_expr2)
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Id(id) => String::from(id),
            Expression::Call(expr1, expr2) => {
                let string1 = expr1.to_string();
                let string2 = expr2.to_string();

                format!("({string1} {string2})")
            }
            Expression::Lambda(id, expr) => {
                let string_expr = expr.to_string();

                format!("l{id}.{string_expr}")
            }
        }
    }
}
