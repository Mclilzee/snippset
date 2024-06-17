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
    if config.add {
        let file = File::open(&config.file)
            .or_else(|_| File::create_new(&config.file))
            .unwrap_or_else(handle_error);

        let mut map: HashMap<String, String> =
            serde_json::from_reader(BufReader::new(&file)).unwrap_or_else(handle_error);
        let title = Text::new("Title: ").prompt().unwrap_or_else(handle_error);
        let snippet = Text::new("Snippet: ").prompt().unwrap_or_else(handle_error);
        map.insert(title, snippet);
        serde_json::to_writer(file, &map).unwrap_or_else(handle_error);
        exit(0);
    }

    // let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    // let snippet = map.get(key).unwrap();

    // let mut snippet_engine = SnippetEngine::new(key, snippet);
    // if let Err(e) = snippet_engine.start() {
    //     println!("Error: {:?}\r", e);
    // }

    Ok(())
}

fn handle_error<E: Display, T>(e: E) -> T {
    eprintln!("{e}");
    exit(1);
}
