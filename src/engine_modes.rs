use crate::sections::snippet_engine::SnippetEngine;
use crate::Snippets;
use inquire::{Select, Text};
use std::path::PathBuf;
use std::{collections::HashMap, fs::File, io::BufReader};

pub fn start_editing_engine(path: PathBuf) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let map: Snippets = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    let key = Select::new("Choose snippet", map.keys().collect())
        .prompt()
        .map_err(|e| e.to_string())?;
    let snippet = map.get(key).unwrap();

    let mut snippet_engine = SnippetEngine::new(key, snippet).map_err(|e| e.to_string())?;
    let result = snippet_engine.start().map_err(|e| e.to_string())?;
    print!("{esc}[2J{esc}[1;1H{result}", esc = 27 as char);

    Ok(())
}

pub fn add_to_file(path: PathBuf) -> Result<(), String> {
    let mut map: Snippets = match File::open(&path) {
        Ok(f) => serde_json::from_reader(BufReader::new(f)).map_err(|e| e.to_string())?,
        Err(_) => HashMap::new(),
    };

    let title = Text::new("Title: ").prompt().map_err(|e| e.to_string())?;
    let snippet = Text::new("Snippet: ")
        .prompt()
        .map_err(|e| e.to_string())?
        .replace("\\n", "\n");

    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path).map_err(|e| e.to_string())?, &map)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn edit_file(path: PathBuf) -> Result<(), String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut map: Snippets = serde_json::from_reader(reader).map_err(|e| e.to_string())?;

    let key = Select::new("Choose snippet to edit", map.keys().collect())
        .prompt()
        .map_err(|e| e.to_string())?
        .to_owned();

    let snippet = map.get(&key).unwrap();

    let title = Text::new("Title: ")
        .with_initial_value(&key)
        .prompt()
        .map_err(|e| e.to_string())?;
    let snippet = Text::new("Snippet: ")
        .with_initial_value(snippet)
        .prompt()
        .map_err(|e| e.to_string())?
        .replace("\\n", "\n");

    map.remove(&key);
    map.insert(title, snippet);
    serde_json::to_writer(File::create(&path).map_err(|e| e.to_string())?, &map)
        .map_err(|e| e.to_string())?;

    Ok(())
}
