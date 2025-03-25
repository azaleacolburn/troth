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

pub fn lex(code: String) -> Vec<Token> {
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
