use crate::{lexer::Token, parser::Expression};

pub enum ExprType {
    Lambda {
        arg: Box<ExprType>,
        expr: Box<ExprType>,
    },
    Int,
    Bool,
}

impl ExprType {
    pub fn parse(tokens: Vec<Token>) -> Self {
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i] {
                Token::Colon
            }
        }
    }
}

impl Expression {}
