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
            '/' if code[i + 1] == '/' => {
                i += 1;
                while code[i] != '\n' {
                    i += 1;
                }
            }
            ' ' => {}
            '\n' => {}
            ';' => tokens.push(Token::Semi),
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
                panic!("Unexpected character: ${c}");
            }
        };
        i += 1;
    }

    tokens
}

fn is_valid_symbol(c: char) -> bool {
    let symbols = "$&+,:=?@#|'<>-^*%!_";
    symbols.contains(c)
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Dot => ".",
            Token::Semi => ";",
            Token::OParen => "(",
            Token::CParen => ")",
            Token::Lambda => "l",
            Token::Id(id) => id,
            Token::Alias(id) => id,
            Token::Define => "fn",
        }
        .into()
    }
}
