use anyhow::Result;
use clap::Parser;
use cli::{BackendOption, Cli};
use parser::Expression;
use reducer::reduce;

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
