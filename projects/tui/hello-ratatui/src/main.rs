#![allow(unused)]

use std::io::stdout;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

use anyhow::Result;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut count = 1;

    /// Main loop
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let text = format!("Count: {}", count);
            frame.render_widget(Paragraph::new(text).white().on_blue(), area)
        });

        // Events
        let fps = std::time::Duration::from_millis(16);
        if event::poll(fps)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('i') {
                    count += 1;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode();
    Ok(())
}
