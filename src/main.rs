mod args;
mod sections;
use args::Args;
use clap::Parser;
use sections::snippet_engine::SnippetEngine;
use std::{collections::HashMap, fs::File, io::BufReader};

use inquire::{InquireError, Select};

fn main() -> Result<(), InquireError> {
    let config = Args::parse();
    let file = File::open(config.file).unwrap();
    let reader = BufReader::new(file);
    let map: HashMap<String, String> = serde_json::from_reader(reader).unwrap();

    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();
    let mut snippet_engine = SnippetEngine::new(key, snippet);

    if let Err(e) = snippet_engine.start() {
        println!("Error: {:?}\r", e);
    }

    Ok(())
}
