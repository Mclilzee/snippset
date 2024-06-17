mod args;
mod sections;
use args::Args;
use clap::Parser;
use inquire::{InquireError, Select, Text};
use sections::snippet_engine::SnippetEngine;
use std::fmt::Display;
use std::io::Error;
use std::process::exit;
use std::{collections::HashMap, fs::File, io::BufReader};

type Snippets = HashMap<String, String>;

fn main() -> Result<(), InquireError> {
    let config = Args::parse();
    let file = get_file(&config).unwrap_or_else(handle_error);
    let reader = BufReader::new(file);
    let map: Snippets = serde_json::from_reader(reader).unwrap_or_else(handle_error);

    if config.add {
        let title = Text::new("Title: ").prompt().unwrap_or_else(handle_error);
        let snippet = Text::new("Snippet: ").prompt().unwrap_or_else(handle_error);
        let mut map = map;
        map.insert(title, snippet);
        exit(0);
    }

    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();

    let mut snippet_engine = SnippetEngine::new(key, snippet);
    if let Err(e) = snippet_engine.start() {
        println!("Error: {:?}\r", e);
    }

    Ok(())
}

fn get_file(config: &Args) -> Result<File, Error> {
    let file = File::open(&config.file);
    if file.is_err() && config.add {
        File::create_new(&config.file)
    } else {
        file
    }
}

fn handle_error<E: Display, T>(e: E) -> T {
    eprintln!("{e}");
    exit(1);
}
