mod args;
mod sections;
use args::Args;
use clap::Parser;
use inquire::{InquireError, Select, Text};
use sections::snippet_engine::SnippetEngine;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::exit;
use std::{collections::HashMap, fs::File, io::BufReader};

type Snippets = HashMap<String, String>;

fn main() -> Result<(), InquireError> {
    let config = Args::parse();

    if config.add {
        add_to_file(config.file)?;
    } else if config.edit {
        edit_file(config.file)?;
    } else {
        start_editing_engine(config.file)?;
    }

    Ok(())
}

fn start_editing_engine(path: PathBuf) -> Result<(), InquireError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map: Snippets = serde_json::from_reader(reader).unwrap_or_else(print_error);
    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();

    let mut snippet_engine = SnippetEngine::new(key, snippet);
    let result = snippet_engine.start()?;
    println!("{result}");

    Ok(())
}

fn add_to_file(path: PathBuf) -> Result<(), InquireError> {
    let mut map: Snippets = match File::open(&path) {
        Ok(f) => serde_json::from_reader(BufReader::new(f)).unwrap_or_else(print_error),
        Err(_) => HashMap::new(),
    };

    let title = Text::new("Title: ").prompt()?;
    let snippet = Text::new("Snippet: ").prompt()?.replace("\\n", "\n");

    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path)?, &map).unwrap_or_else(print_error);
    exit(0);
}

fn edit_file(path: PathBuf) -> Result<(), InquireError> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut map: Snippets = serde_json::from_reader(reader).unwrap_or_else(print_error);

    let key = Select::new("Choose snippet to edit", map.keys().collect())
        .prompt()?
        .to_owned();

    let snippet = map.get(&key).unwrap();

    let title = Text::new("Title: ").with_initial_value(&key).prompt()?;
    let snippet = Text::new("Snippet: ")
        .with_initial_value(snippet)
        .prompt()?
        .replace("\\n", "\n");

    map.remove(&key);
    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path)?, &map).unwrap_or_else(print_error);
    exit(0);
}

fn print_error<E: Display, T>(e: E) -> T {
    eprintln!("{e}");
    exit(1);
}
