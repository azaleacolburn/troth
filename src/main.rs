use parser::Expression;

mod lexer;
mod parser;
mod reducer;
#[cfg(test)]
mod test;
mod token_handler;

fn main() {
    let code: String = "fn D ((lx.(ly.(y x))) ((lz.z) x)); fn G (D ((lx.x) D)); (D G)".into();
    interpret(code);
}

pub fn interpret(code: String) -> Expression {
    let tokens = lexer::lex(code);
    println!("{:?}", tokens);
    let mut token_handler = token_handler::TokenHandler::new(tokens);
    let ast = parser::parse(&mut token_handler);
    println!("{:?}", ast);
    let reduced = reducer::reduce(&ast[0]);
    println!("{:?}", reduced);
    reduced
}
