use crate::snippet_engine::SnippetEngine;
use crate::Snippets;
use arboard::Clipboard;
use inquire::{Select, Text};
use std::path::{Path, PathBuf};
use std::{fs::File, io::BufReader};
use anyhow::{bail, Context, Result};

pub fn start_editing_engine(path: PathBuf) -> Result<()> {
    let map: Snippets = get_snippets_from_file(&path)?;
    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;

    let snippet = match map.get(key) {
        Some(sn) => sn,
        None => bail!("Couldn't find snippet for key {key}")
    };

    let mut snippet_engine = SnippetEngine::new(key, snippet);
    let text = snippet_engine.start()?;

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    println!("Copied result to clipboard");
    Ok(())
}

pub fn add_to_file(path: PathBuf) -> Result<()> {
    let mut map: Snippets = get_snippets_from_file(&path).unwrap_or_default();
    let title = Text::new("Title: ").prompt()?;
    let snippet = Text::new("Snippet: ")
        .prompt()?
        .replace("\\n", "\n");

    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path)?, &map)?;
    Ok(())
}

pub fn edit_file(path: PathBuf) -> Result<()> {
    let mut map: Snippets = get_snippets_from_file(&path)?;

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

fn get_snippets_from_file(path: &Path) -> Result<Snippets> {
    let file = File::open(path).with_context(|| format!("{path:?} could not be found"))?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).with_context(|| format!("{path:?} is not a valid snippet JSON format"))
}
