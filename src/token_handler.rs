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

    pub fn prev(&mut self) {
        self.curr -= 1;
    }

    pub fn is_done(&self) -> bool {
        self.curr == self.tokens.len() - 1
    }

    pub fn get_def(&self, id: &str) -> Expression {
        self.definitions.get(id).unwrap().clone()
    }

    pub fn new_def(&mut self, id: impl ToString, expr: Expression) {
        self.definitions.insert(id.to_string(), expr);
    }

    pub fn merge_definitions(&mut self, other: &Parser) {
        other
            .definitions
            .iter()
            .for_each(|(id, expr)| self.new_def(id.clone(), expr.clone()));
    }

    pub fn all_defs(&self) -> &HashMap<String, Expression> {
        &self.definitions
    }

    pub fn set_map(&mut self, defs: HashMap<String, Expression>) {
        self.definitions = defs;
    }
}
