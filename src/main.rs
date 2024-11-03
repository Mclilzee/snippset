mod args;
mod engine_modes;
mod sections;

use std::{collections::HashMap, io::stdout};
use args::Args;
use clap::Parser;
use crossterm::{
    cursor, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use engine_modes::*;

use ratatui::prelude::*;

type Snippets = HashMap<String, String>;

fn main() -> Result<(), String> {
    let config = Args::parse();
    execute!(stdout(), EnterAlternateScreen, cursor::MoveTo(0, 0)).map_err(|_| "Failed to start new terminal buffer".to_string())?;

    let engine = if config.add {
        add_to_file
    } else if config.edit {
        edit_file
    } else {
        start_editing_engine
    };

    let result = engine(config.path);
    execute!(stdout(), LeaveAlternateScreen).map_err(|e| e.to_string())?;

    result
}
