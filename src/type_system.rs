use anyhow::{bail, Result};
use colored::Colorize;

use crate::{lexer::Token, parser::Expression, token_handler::Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    Function {
        arg: Box<ExprType>,
        ret: Box<ExprType>,
    },
    Int,
    Bool,
}

impl Parser {
    /// Assumes that the next token is the first character after the `:`
    /// In other words, that the current token is the `:`
    pub fn parse_type(&mut self) -> Result<ExprType> {
        self.next();
        let a_token = self.get();
        let a = match a_token {
            Token::Bool => ExprType::Bool,
            Token::Int => ExprType::Int,
            Token::OParen => {
                self.next();
                let a = self.parse_type()?;
                if *self.get() != Token::CParen {
                    bail!("Unclosed CParen");
                }
                a
            }
            _ => bail!("Unsupported type"),
        };
        self.next();

        Ok(match self.get() {
            Token::Arrow => {
                self.next();
                let b = self.parse_type()?;

                ExprType::Function {
                    arg: Box::new(a),
                    ret: Box::new(b),
                }
            }
            _ => a,
        })
    }
}

pub enum ApplicationSide {
    Left,
    Right,
}

impl Expression {
    pub fn check_types(&self) -> bool {
        match self {
            Expression::Id(_) => true,
            Expression::Abstraction { arg: _, expr, t: _ } => expr.check_types(),
            Expression::Application(a, b) => {
                if !a.check_types() {
                    return false;
                }

                if !b.check_types() {
                    return false;
                }

                let a_type = a.eval_type(ApplicationSide::Left);
                let b_type = b.eval_type(ApplicationSide::Right);

                // NOTE
                // The result of evaluating b has to be the same type of the input to a
                a_type == b_type
            }
        }
    }

    pub fn eval_type(&self, side: ApplicationSide) -> Option<&ExprType> {
        match (self, side) {
            (Expression::Id(_), _) => None,
            (Expression::Abstraction { arg: _, expr: _, t }, _) => t.as_ref(),
            (Expression::Application(a, _), ApplicationSide::Left) => {
                let a_type = a.eval_type(ApplicationSide::Left)?;

                match a_type {
                    ExprType::Int => Some(&ExprType::Int),
                    ExprType::Bool => Some(&ExprType::Bool),
                    ExprType::Function { arg: _, ret } => Some(ret.as_ref()),
                }
            }

            (Expression::Application(_, b), ApplicationSide::Right) => {
                let b_type = b.eval_type(ApplicationSide::Right)?;

                match b_type {
                    ExprType::Int => Some(&ExprType::Int),
                    ExprType::Bool => Some(&ExprType::Bool),
                    ExprType::Function { arg: _, ret } => Some(ret.as_ref()),
                }
            }
        }
    }

    // pub fn check_abstraction_type(&self, abstraction_type: type) -> bool {
    //
    //                 match abstraction_type
    // }
}
