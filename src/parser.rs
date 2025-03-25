use crate::{lexer::Token, token_handler::TokenHandler};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Definition(String, Box<Expression>),
    Expression(Box<Expression>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Id(String),
    Lambda(String, Box<Expression>),
    Call(Box<Expression>, Box<Expression>),
}

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
            println!("next : {:?}", handler.get());
            if *handler.get() != Token::CParen {
                panic!("Expected CParen");
            }

            expr
        }
        Token::Id(id) => Expression::Id(id.clone()),
        Token::Alias(id) => handler.get_def(&id),
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
    println!("call1: {:?}", handler.get());
    let a = Box::new(expression(handler));
    handler.next();
    println!("call2: {:?}", handler.get());
    let b = Box::new(expression(handler));
    println!("call3: {:?}", handler.get());

    Expression::Call(a, b)
}
