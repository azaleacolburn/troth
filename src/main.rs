use anyhow::Result;
use clap::Parser;
use cli::{BackendOption, Cli};
use parser::Expression;
use reducer::reduce;

mod cli;
mod lexer;
mod parser;
mod reducer;
#[cfg(test)]
mod test;
mod token_handler;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let code = cli.read_input()?;
    let tokens = lexer::lex(code);
    let mut parser = token_handler::Parser::new(tokens);
    let ast = parser.parse()?.unwrap();

    let output: String = match cli.get_backend() {
        BackendOption::Reduce => handle_reduction(&ast),
        BackendOption::Compile => panic!("Unsupported Backend Option"),
        BackendOption::Transpile => panic!("Unsupported Backend Option"),
    };

    cli.write_output(&output)
}

fn handle_reduction(ast: &Expression) -> String {
    format!("{}\n", reduce(ast).to_string())
}
