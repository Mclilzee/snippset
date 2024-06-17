mod args;
mod sections;
use args::Args;
use clap::Parser;
use inquire::{InquireError, Select};
use sections::snippet_engine::SnippetEngine;
use std::io::Error;
use std::process::exit;
use std::{collections::HashMap, fs::File, io::BufReader};

type Snippets = HashMap<String, String>;

fn main() -> Result<(), InquireError> {
    let config = Args::parse();
    let map: Snippets = parse_config(&config).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(1);
    });

    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();

    let mut snippet_engine = SnippetEngine::new(key, snippet);
    if let Err(e) = snippet_engine.start() {
        println!("Error: {:?}\r", e);
    }

    Ok(())
}

fn parse_config(config: &Args) -> Result<Snippets, Error> {
    let file = File::open(&config.file);

    if let Ok(file) = file {
        let reader = BufReader::new(file);
        return serde_json::from_reader(reader).map_err(Error::from);
    }

    if config.add {
        Ok(HashMap::new())
    } else {
        Err(file.err().unwrap())
    }
}
