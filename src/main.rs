mod args;
mod sections;
use args::Args;
use clap::Parser;
use sections::snippet_engine::SnippetEngine;
use std::collections::HashMap;
use std::path::PathBuf;

use inquire::{InquireError, Select};

fn main() -> Result<(), InquireError> {
    let config = Args::parse();
    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();
    let mut snippet_engine = SnippetEngine::new(key, snippet);

    if let Err(e) = snippet_engine.start() {
        println!("Error: {:?}\r", e);
    }

    Ok(())
}
