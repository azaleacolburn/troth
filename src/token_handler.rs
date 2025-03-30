use std::collections::HashMap;

use crate::{lexer::Token, parser::Expression};

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
    definitions: HashMap<String, Expression>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            curr: 0,
            definitions: HashMap::new(),
        }
    }

    pub fn get(&self) -> &Token {
        &self.tokens[self.curr]
    }

    pub fn next(&mut self) {
        self.curr += 1;
    }

    pub fn is_done(&self) -> bool {
        self.curr == self.tokens.len() - 1
    }

    pub fn get_def(&self, id: &str) -> Expression {
        println!("{id}");
        self.definitions.get(id).unwrap().clone()
    }

    pub fn new_def(&mut self, id: String, expr: Expression) {
        self.definitions.insert(id, expr);
    }
}
