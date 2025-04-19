use anyhow::Result;
use clap::Parser;
use cli::{BackendOption, Cli};
use parser::Expression;
use reducer::reduce;
use std::fs::read_to_string;

mod cli;
mod lexer;
mod parser;
mod reducer;
mod stdlib;
#[cfg(test)]
mod test;
mod token_handler;
mod transpiler;

fn main() -> Result<()> {
    let stdlib_definitions = stdlib::stdlib_definitions()?;

    let cli = Cli::parse();
    let code = cli.read_input()?;

    let tokens = lexer::lex(code);
    let mut parser = token_handler::Parser::new(tokens);

    parser.set_map(stdlib_definitions);
    let mut ast = parser.parse()?.unwrap();

    let output: String = match cli.get_backend() {
        BackendOption::Reduce => handle_reduction(&mut ast),
        BackendOption::Compile => panic!("Unsupported Backend Option"),
        BackendOption::Transpile => transpiler::to_javascript_naive(&ast),
    };

    cli.write_output(&output)
}

fn handle_reduction(ast: &mut Expression) -> String {
    reduce(ast);
    format!("{}\n", ast.to_string())
}

pub fn load(name: impl ToString) -> String {
    let name = format!("./tests/{}.lc", name.to_string());
    read_to_string(name).expect("Test function missing test file")
}

pub fn interpret(code: String) -> Option<Expression> {
    let tokens = lexer::lex(code);
    println!("{:?}", tokens);
    let mut parser = token_handler::Parser::new(tokens);
    let ast = match parser.parse().unwrap() {
        Some(ast) => ast,
        None => return None,
    };
    println!("{:?}", ast);
    let reduced = reducer::reduce(&ast);
    println!("{:?}", reduced);
    Some(reduced)
}
