mod printer;
mod sections;
use std::collections::HashMap;

use inquire::{InquireError, Select};
use sections::section_manager::SectionManager;

fn main() -> Result<(), InquireError> {
    let mut map = HashMap::new();
    map.insert(
        "hello snippet",
        "This hello is a snippet {} placeholder {} lets go? {} nice",
    );
    map.insert("Another snippet", "BEHAVE YOURSELF {} Please.");

    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();
    let mut section_manager = SectionManager::new(key, snippet);

    if let Err(e) = section_manager.start() {
        println!("Error: {:?}\r", e);
    }
    Ok(())
}
