use anyhow::{bail, Result};

use crate::{lexer::Token, token_handler::Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    Lambda {
        arg: Box<ExprType>,
        ret: Box<ExprType>,
    },
    Int,
    Bool,
}

impl Parser {
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

                ExprType::Lambda {
                    arg: Box::new(a),
                    ret: Box::new(b),
                }
            }
            _ => a,
        })
    }
}
