use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OParen,
    CParen,
    Lambda,
    Dot,
    Id(String),
    Alias(String),
    Define,
    Semi,
}

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

fn main() {
    let code: String = "fn D ((lx.(ly.(y x))) ((lz.z) x)); fn G (D ((lx.x) D)); (D G)".into();
    let tokens = lex(code);
    println!("{:?}", tokens);
    let mut token_handler = TokenHandler::new(tokens);
    let ast = parse(&mut token_handler);
    println!("{:?}", ast)
}

fn lex(code: String) -> Vec<Token> {
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
            ' ' => {
                i += 1;
                continue;
            }
            ';' => tokens.push(Token::Semi),
            mut c if c.is_uppercase() => {
                alias.push(c);
                while i + 1 < code.len() && code[i + 1].is_uppercase() {
                    i += 1;
                    c = code[i];
                    alias.push(c);
                }
                tokens.push(Token::Alias(alias.clone()));
                alias = "".to_string();
            }
            c => tokens.push(Token::Id(c.to_string())),
        };
        i += 1;
    }

    tokens
}

pub struct TokenHandler {
    tokens: Vec<Token>,
    curr: usize,
    definitions: HashMap<String, Expression>,
}

impl TokenHandler {
    fn new(tokens: Vec<Token>) -> Self {
        TokenHandler {
            tokens,
            curr: 0,
            definitions: HashMap::new(),
        }
    }

    fn get(&self) -> &Token {
        &self.tokens[self.curr]
    }

    fn next(&mut self) {
        self.curr += 1;
    }

    fn is_done(&self) -> bool {
        self.curr == self.tokens.len() - 1
    }

    fn get_def(&self, id: &str) -> Expression {
        self.definitions.get(id).unwrap().clone()
    }

    fn new_def(&mut self, id: String, expr: Expression) {
        self.definitions.insert(id, expr);
    }
}

fn parse(handler: &mut TokenHandler) -> Vec<Expression> {
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
