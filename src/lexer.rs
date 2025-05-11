use crate::token_handler::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OParen,
    CParen,
    Lambda,
    Dot,
    Id(String),
    Alias(String),
    Define,
    Use(PathBuf),
    Semi,
    Colon,
    Arrow,
    Int,
    Bool,
}

impl Parser {
    pub fn from_source(code: String) -> Self {
        let mut alias = String::new();
        let mut tokens: Vec<Token> = vec![];
        let code: Vec<char> = code.chars().collect();
        let mut i = 0;
        while i < code.len() {
            match code[i] {
                '.' => tokens.push(Token::Dot),
                'l' => tokens.push(Token::Lambda),
                '(' => tokens.push(Token::OParen),
                ')' => tokens.push(Token::CParen),
                'f' if code[i + 1] == 'n' && code[i + 2] == ' ' => {
                    i += 2;
                    tokens.push(Token::Define);
                }
                'u' if code[i + 1] == 's'
                    && code[i + 2] == 'e'
                    && (code[i + 3] == ' ' || code[i + 3] == '"') =>
                {
                    i += 5;
                    let mut path_str = String::new();
                    while code[i] != '"' {
                        path_str.push(code[i]);
                        i += 1;
                    }

                    tokens.push(Token::Use(PathBuf::from(path_str)));
                }
                '/' if code[i + 1] == '/' => {
                    i += 1;
                    while code[i] != '\n' {
                        i += 1;
                    }
                }
                ' ' => {}
                '\n' => {}
                ';' => tokens.push(Token::Semi),
                'b' if code[i + 1] == 'o'
                    && code[i + 2] == 'o'
                    && code[i + 3] == 'l'
                    && !(code[i + 4].is_alphabetic() || code[i + 4] == '_') =>
                {
                    i += 3;
                    tokens.push(Token::Bool)
                }
                'i' if code[i + 1] == 'n'
                    && code[i + 2] == 't'
                    && !(code[i + 3].is_alphabetic() || code[i + 3] == '_') =>
                {
                    i += 2;
                    tokens.push(Token::Int);
                }
                '-' if code[i + 1] == '>'
                    && !(code[i + 2].is_alphabetic() || code[i + 2] == '_') =>
                {
                    i += 1;
                    tokens.push(Token::Arrow);
                }
                ':' => tokens.push(Token::Colon),
                mut c if c.is_uppercase() || is_valid_symbol(c) || c.is_numeric() => {
                    alias.push(c);
                    while i + 1 < code.len()
                        && (code[i + 1].is_uppercase()
                            || code[i + 1].is_numeric()
                            || is_valid_symbol(code[i + 1]))
                    {
                        i += 1;
                        c = code[i];
                        alias.push(c);
                    }
                    tokens.push(Token::Alias(alias.clone()));
                    alias = "".to_string();
                }
                mut c if c.is_alphabetic() || c == '_' => {
                    alias.push(c);
                    while i + 1 < code.len() && (code[i + 1].is_alphanumeric() || c == '_') {
                        i += 1;
                        c = code[i];
                        alias.push(c);
                    }
                    tokens.push(Token::Id(alias.clone()));
                    alias = "".to_string();
                }
                c => {
                    panic!("Unexpected character: {c}");
                }
            };
            i += 1;
        }

        Parser::new(tokens)
    }
}

fn is_valid_symbol(c: char) -> bool {
    let symbols = "$&+,:=?@#|'<>-^*%!_";
    symbols.contains(c)
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::OParen => ")".into(),
            Token::CParen => "(".into(),
            Token::Dot => ".".into(),
            Token::Semi => ";".into(),
            Token::Use(path) => format!("use \"{}\"", path.to_str().unwrap()),
            Token::Lambda => "l".into(),
            Token::Define => "fn".into(),
            Token::Id(id) => id.into(),
            Token::Alias(id) => id.into(),
            Token::Colon => ":".into(),
            Token::Arrow => "->".into(),
            Token::Int => "int".into(),
            Token::Bool => "bool".into(),
        }
    }
}
