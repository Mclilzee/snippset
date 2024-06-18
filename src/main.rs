mod args;
mod sections;
use args::Args;
use clap::Parser;
use inquire::{InquireError, Select, Text};
use sections::snippet_engine::SnippetEngine;
use std::fmt::Display;
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::process::exit;
use std::{collections::HashMap, fs::File, io::BufReader};

type Snippets = HashMap<String, String>;

fn main() -> Result<(), InquireError> {
    let config = Args::parse();

    if config.add {
        add_to_file(config.file)?;
    }

    // let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    // let snippet = map.get(key).unwrap();
    //
    // let mut snippet_engine = SnippetEngine::new(key, snippet);
    // if let Err(e) = snippet_engine.start() {
    //     println!("Error: {:?}\r", e);
    // }
    //
    Ok(())
}

fn add_to_file(path: PathBuf) -> Result<(), InquireError> {
    let mut map: Snippets = match File::open(&path) {
        Ok(f) => serde_json::from_reader(BufReader::new(f)).unwrap_or_else(handle_error),
        Err(_) => HashMap::new(),
    };

    let title = Text::new("Title: ").prompt()?;
    let snippet = Text::new("Snippet: ").prompt()?;

    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path)?, &map).unwrap();
    exit(0);
}

fn handle_error<E: Display, T>(e: E) -> T {
    eprintln!("{e}");
    exit(1);
}
