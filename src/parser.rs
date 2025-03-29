use crate::{lexer::Token, token_handler::TokenHandler};
use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Id(String),
    Lambda(String, Box<Expression>),
    Call(Box<Expression>, Box<Expression>),
}

impl TokenHandler {
    // TODO Find better way to return null
    /// Returns Ok(None) if there are only aliases
    pub fn parse(&mut self) -> Result<Option<Expression>> {
        match self.get() {
            Token::Define => {
                self.next();
                if let Token::Alias(id) = self.get().clone() {
                    self.next();
                    let expr = self.expression()?;

                    self.next();
                    if *self.get() != Token::Semi {
                        bail!("Expected Semi after definition found {:?}", self.get());
                    }

                    self.new_def(id.clone(), expr);

                    if self.is_done() {
                        return Ok(None);
                    }

                    self.next();
                    let r = self.parse();

                    return r;
                }
                bail!("Found definition without name");
            }
            _ => Ok(Some(self.expression()?)),
        }
    }

    fn expression(&mut self) -> Result<Expression> {
        Ok(match self.get() {
            Token::OParen => {
                self.next();
                let expr = match self.get() {
                    Token::Lambda => self.lambda()?,
                    _ => self.call()?,
                };

                self.next();
                if *self.get() != Token::CParen {
                    bail!("Expected CParen found {:?}", self.get());
                }

                expr
            }
            Token::Lambda => self.lambda()?,
            Token::Id(id) => Expression::Id(id.clone()),
            Token::Alias(id) => alpha_conversion(Box::new(self.get_def(&id)), &id),
            c => bail!("Unsupported Token: {:?}", c),
        })
    }

    fn lambda(&mut self) -> Result<Expression> {
        self.next();
        if let Token::Id(id) = self.get().clone() {
            self.next();
            if *self.get() == Token::Dot {
                self.next();
                return Ok(Expression::Lambda(id.clone(), Box::new(self.expression()?)));
            }
            bail!("Found lambda without dot");
        }

        bail!("Found lambda without id");
    }

    fn call(&mut self) -> Result<Expression> {
        let a = Box::new(self.expression()?);
        self.next();
        let b = Box::new(self.expression()?);

        Ok(Expression::Call(a, b))
    }
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
