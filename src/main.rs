mod printer;
mod sections;
use sections::snippet_engine::SnippetEngine;
use std::collections::HashMap;

use inquire::{InquireError, Select};

fn main() -> Result<(), InquireError> {
    let mut map = HashMap::new();
    map.insert(
        "hello snippet",
        "This hello is a snippet {} placeholder {} lets go? {} nice",
    );
    map.insert("Another snippet", "BEHAVE YOURSELF {} Please.");

    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();
    let mut snippet_engine = SnippetEngine::new(key, snippet);

    if let Err(e) = snippet_engine.start() {
        println!("Error: {:?}\r", e);
    }
    Ok(())
}
