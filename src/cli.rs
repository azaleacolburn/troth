use anyhow::Result;
use clap::{arg, command, Parser};
// use serde::Serialize;
use std::{
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(version = "0.0.1", about = "A Simple Lambda Calculus Interpreter", long_about = None)]
pub struct Cli {
    // Default is stdin
    input_file: Option<PathBuf>,

    #[arg(short, long, value_enum)]
    backend: Option<BackendOption>,

    // Default is stdout
    #[arg(short, long, value_name = "path")]
    output_file: Option<PathBuf>,

    #[arg(short, long)]
    pub debug: bool,
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
#[clap(rename_all = "lowercase")]
pub enum BackendOption {
    #[default]
    Reduce,
    Transpile, // To JS
    Compile,
}

impl Cli {
    pub fn read_input(&self) -> Result<String> {
        match &self.input_file {
            Some(name) => Ok(std::fs::read_to_string(name)?),
            None => {
                let mut handle = stdin().lock();
                let mut buff = String::new();
                handle.read_to_string(&mut buff)?;

                Ok(buff)
            }
        }
    }

    pub fn get_backend(&self) -> BackendOption {
        self.backend.clone().unwrap_or_default()
    }

    pub fn write_output(&self, output: &str) -> Result<()> {
        match &self.output_file {
            Some(name) => Ok(std::fs::write(name, output)?),
            None => {
                let mut handle = stdout().lock();
                Ok(handle.write_all(output.as_bytes())?)
            }
        }
    }
}
