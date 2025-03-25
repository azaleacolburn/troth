mod lexer;
mod parser;
mod token_handler;

fn main() {
    let code: String = "fn D ((lx.(ly.(y x))) ((lz.z) x)); fn G (D ((lx.x) D)); (D G)".into();
    let tokens = lexer::lex(code);
    println!("{:?}", tokens);
    let mut token_handler = token_handler::TokenHandler::new(tokens);
    let ast = parser::parse(&mut token_handler);
    println!("{:?}", ast)
}
