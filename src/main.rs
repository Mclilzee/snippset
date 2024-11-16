mod args;
mod engine_modes;
mod snippet_engine;
mod sections;

use anyhow::Result;
use std::{collections::HashMap, io::stdout};
use args::Args;
use clap::Parser;
use crossterm::{
    cursor, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use engine_modes::*;

type Snippets = HashMap<String, String>;

fn main() -> Result<()> {
    let config = Args::parse();
    execute!(stdout(), EnterAlternateScreen, cursor::MoveTo(0, 0))?;

    let engine = if config.add {
        add_to_file
    } else if config.edit {
        edit_file
    } else {
        start_editing_engine
    };

    let result = engine(config.path);
    execute!(stdout(), LeaveAlternateScreen)?;
    result?;
    Ok(())
}
