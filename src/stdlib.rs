use std::collections::HashMap;

use crate::{parser::Expression as E, token_handler::Parser};
use anyhow::Result;

// This should be handled at compile-time
pub fn stdlib_definitions() -> Result<HashMap<String, E>> {
    let mut defs = HashMap::new();

    let files = std::fs::read_dir("./stdlib/")?;
    files.filter_map(|file| file.ok()).for_each(|file| {
        let code = std::fs::read_to_string(file.path()).unwrap();
        let mut parser = Parser::from_source(code);
        let _ = parser.parse();

        parser.all_defs().iter().for_each(|(key, value)| {
            defs.insert(key.clone(), value.clone());
        });
    });

    Ok(defs)
}
