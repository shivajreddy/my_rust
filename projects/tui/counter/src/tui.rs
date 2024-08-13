#![allow(unused)]

use std::io::{stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Frame, Terminal,
};

use anyhow::Result;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> Result<Tui> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear();
    Ok(terminal)
}

/// Restore the terminal to its original state
pub fn restore() -> Result<()> {
    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode();
    Ok(())
}
