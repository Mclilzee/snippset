use crate::snippet_engine::SnippetEngine;
use crate::Snippets;
use inquire::{Select, Text};
use std::path::PathBuf;
use std::{collections::HashMap, fs::File, io::BufReader};
use anyhow::{bail, Result};

pub fn start_editing_engine(path: PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map: Snippets = serde_json::from_reader(reader)?;
    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;

    let snippet = match map.get(key) {
        Some(sn) => sn,
        None => bail!("Couldn't find snippet for key {key}")
    };

    let mut snippet_engine = SnippetEngine::new(key, snippet);
    let _result = snippet_engine.start()?;
    println!("Copied result to clipboard");
    Ok(())
}

pub fn add_to_file(path: PathBuf) -> Result<()> {
    let mut map: Snippets = match File::open(&path) {
        Ok(f) => serde_json::from_reader(BufReader::new(f))?,
        Err(_) => HashMap::new(),
    };

    let title = Text::new("Title: ").prompt()?;
    let snippet = Text::new("Snippet: ")
        .prompt()?
        .replace("\\n", "\n");

    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path)?, &map)?;

    Ok(())
}

pub fn edit_file(path: PathBuf) -> Result<()> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut map: Snippets = serde_json::from_reader(reader)?;

    let key = Select::new("Choose snippet to edit", map.keys().collect())
        .prompt()?
        .to_owned();

    let snippet = map.get(&key).unwrap();

    let title = Text::new("Title: ")
        .with_initial_value(&key)
        .prompt()?;

    let snippet = Text::new("Snippet: ")
        .with_initial_value(snippet)
        .prompt()?
        .replace("\\n", "\n");

    map.remove(&key);
    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path)?, &map)?;

    Ok(())
}
