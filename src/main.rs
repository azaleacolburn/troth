use anyhow::{bail, Result};
use clap::Parser;
use cli::{BackendOption, Cli};
use colored::Colorize;
use parser::Expression;
use std::fs::read_to_string;

mod cli;
mod error;
mod lexer;
mod parser;
mod reducer;
mod stdlib;
#[cfg(test)]
mod test;
mod token_handler;
mod transpiler;
mod type_system;
mod visualizer;

fn main() -> Result<()> {
    let stdlib_definitions = stdlib::stdlib_definitions()?;

    let cli = Cli::parse();
    let code = match cli.read_input() {
        Ok(code) => code,
        Err(err) => bail!("{}", err.to_string().red()),
    };

    let mut parser = token_handler::Parser::from_source(code);
    parser.set_map(stdlib_definitions);
    let mut ast = match parser.parse()? {
        Some(ast) => ast,
        None => {
            println!(
                "{}",
                "Warning: Parsed Independent Library File (Produced No Output)".yellow()
            );
            return Ok(());
        }
    };

    let output: String = match cli.get_backend() {
        BackendOption::Reduce => handle_reduction(&mut ast),
        BackendOption::Compile => bail!("{}", "Unsupported Backend Option".red()),
        BackendOption::Transpile => transpiler::to_javascript_naive(&ast),
    };

    cli.write_output(&output)
}

fn handle_reduction(ast: &mut Expression) -> String {
    let mut expr = ast.reduce().to_string();
    expr.push('\n');

    expr
}

pub fn load(name: impl ToString) -> String {
    let name = format!("./tests/{}.lc", name.to_string());
    read_to_string(name).expect("Test function missing test file")
}

pub fn interpret(code: String) -> Result<Option<Expression>> {
    let mut parser = token_handler::Parser::from_source(code);
    let ast = match parser.parse()? {
        Some(ast) => ast,
        None => return Ok(None),
    };
    println!("{:?}", ast);
    let reduced = ast.reduce();
    println!("{:?}", reduced);
    Ok(Some(reduced))
}
