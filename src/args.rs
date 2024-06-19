use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"Takes a json file and load it for snippets. The file needs to be valid json, the key will be used for the title, and the body will be used for the snippet. Use '{}' to mark an area as editable this will be the placeholder for replacement.
Example: {
        "Title": "Snippet {}",
        "Another snippet", "Snippet mark new lines \nplaceholder {} can have multiple {} placeholders"
        }

Enter will move your cursor to the next placeholder if you have multiples, while Esc will move to previous ones."#
)]
pub struct Args {
    /// Path of snippet file to load, file must be valid json with string keys and string values"
    pub path: PathBuf,

    /// Add snippet to the selected json file using interactive mode. If the selected json file does not exist, a new one in the path will be created.
    #[arg(short = 'a', long = "add")]
    pub add: bool,

    /// Edit snippet in the selected json file
    #[arg(short = 'e', long = "edit")]
    pub edit: bool,
}
