use clap::{command, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Takes a json file and load it for snippets, edit the areas specified with {} inside the text, Press Enter to move forward and Esc to move backward inside snippet areas. After you are done your complete snippet is printed out."
)]
pub struct Args {
    /// Path of snippet file to load, file must be valid json with key as the title and value as the snippet. Snippet is text put {} for areas that you want editable in the snippet. example "This text is {}"
    pub path: Vec<PathBuf>,

    /// Add snippet to a file
    #[arg(short = 'a', long = "add")]
    pub overall: bool,

    /// Edit snippet in a file
    #[arg(short = 'S', long = "search")]
    pub search: Option<String>,
}
