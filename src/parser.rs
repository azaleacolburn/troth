use crate::{lexer::Token, token_handler::Parser, type_system::ExprType};
use anyhow::{bail, Result};
use colored::Colorize;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Id(String),
    Abstraction {
        arg: String,
        expr: Box<Expression>,
        t: Option<ExprType>,
    },
    Application(Box<Expression>, Box<Expression>),
}

impl Parser {
    pub fn parse(&mut self) -> Result<Option<Expression>> {
        match self.get() {
            Token::Define => self.definition(),
            Token::Use(path) => self.include(path.clone()),
            _ => Ok(Some(self.application()?)),
        }
    }

    fn expression(&mut self) -> Result<Expression> {
        Ok(match self.get() {
            Token::OParen => {
                self.next();
                let expr = match self.get() {
                    Token::Lambda => self.abstraction()?,
                    _ => self.application()?,
                };

                self.next();
                let cparen = self.get();
                if *cparen != Token::CParen {
                    bail!(
                        "{} {}",
                        "Expected CParen found".red(),
                        cparen.to_string().red()
                    );
                }

                expr
            }
            Token::Lambda => self.abstraction()?,
            Token::Id(id) => Expression::Id(id.clone()),
            Token::Alias(id) => alpha_conversion(Box::new(self.get_def(&id)), &id),
            c => bail!(
                "{} {}",
                "Token in Illegal Position: ".red(),
                c.to_string().red()
            ),
        })
    }

    fn abstraction(&mut self) -> Result<Expression> {
        self.next();
        if let Token::Id(id) = self.get().clone() {
            self.next();
            let abstraction_type = match &self.get() {
                Token::Colon => {
                    let t = self.parse_type()?;
                    self.next();
                    Some(t)
                }
                _ => None,
            };

            if *self.get() != Token::Dot {
                bail!("{}", "Found abstraction without dot".red());
            }

            self.next();
            return Ok(Expression::Abstraction {
                arg: id.clone(),
                expr: Box::new(self.application()?),
                t: abstraction_type,
            });
        }

        bail!(
            "{} {}",
            "Expected id after abstraction, found".red(),
            self.get().to_string().red()
        );
    }

    fn application(&mut self) -> Result<Expression> {
        let mut a = Box::new(self.expression()?);
        loop {
            if self.is_done() {
                return Ok(*a);
            }
            self.next();

            if *self.get() == Token::CParen || *self.get() == Token::Semi {
                self.prev();
                return Ok(*a);
            }

            let b = Box::new(self.expression()?);
            *a = Expression::Application(a.clone(), b);
        }
    }

    fn definition(&mut self) -> Result<Option<Expression>> {
        self.next();
        if let Token::Alias(id) = self.get().clone() {
            self.next();
            let expr = self.application()?;
            self.next();

            if *self.get() != Token::Semi {
                bail!(
                    "{} {}",
                    "Expected Semi after definition, found".red(),
                    self.get().to_string().red()
                );
            }

            self.new_def(id.clone(), expr);

            if self.is_done() {
                return Ok(None);
            }

            self.next();
            let r = self.parse();

            return r;
        }

        bail!("{}", "Found definition without name".red());
    }

    // fn type_signature(&mut self) -> Result<Option<Expression>> {}

    fn include(&mut self, path: PathBuf) -> Result<Option<Expression>> {
        assert!(path.is_file());

        let file = fs::read_to_string(path)?;
        let mut parser = Parser::from_source(file);
        parser.parse()?;

        self.next();
        if *self.get() != Token::Semi {
            bail!(
                "{} {}",
                "Expected Semi after use statement, found".red(),
                self.get().to_string().red()
            )
        }

        self.merge_definitions(&parser);

        self.next();
        self.parse()
    }
}

// TODO Change to mutate `expr` instead of cloning it
fn alpha_conversion(expr: Box<Expression>, postfix: &str) -> Expression {
    match *expr {
        Expression::Id(id) => {
            let new_id = format!("{id}_{postfix}");
            Expression::Id(new_id)
        }
        Expression::Abstraction {
            arg,
            expr: abstraction_expr,
            t,
        } => {
            let new_expr = Box::new(alpha_conversion(abstraction_expr, postfix));
            let new_id = format!("{arg}_{postfix}");
            Expression::Abstraction {
                arg: new_id,
                expr: new_expr,
                t,
            }
        }
        Expression::Application(expr1, expr2) => {
            let new_expr1 = Box::new(alpha_conversion(expr1, postfix));
            let new_expr2 = Box::new(alpha_conversion(expr2, postfix));
            Expression::Application(new_expr1, new_expr2)
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Id(id) => String::from(id),
            Expression::Application(expr1, expr2) => {
                let string1 = expr1.to_string();
                let string2 = expr2.to_string();

                format!("({string1} {string2})")
            }
            Expression::Abstraction { arg, expr, t: _ } => {
                let string_expr = expr.to_string();

                format!("l{arg}.{string_expr}")
            }
        }
    }
}
