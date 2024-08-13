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

mod tui;

/// Application entry point
fn main() -> Result<()> {
    // Create a terminal
    let mut terminal = tui::init()?;

    App::default().run(&mut terminal);

    // Restore the terminal
    tui::restore();
    // exit the application
    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    counter: i32,
    exit: bool,
}

impl App {
    fn render_frame(&self, frame: &mut Frame) {
        let area = frame.area();
        let text = format!("Count: {}", self.counter);
        frame.render_widget(Paragraph::new(text).white().on_blue(), area);
    }

    fn handle_events(&mut self) -> Result<()> {
        // Events
        let fps = std::time::Duration::from_millis(16);
        if event::poll(fps)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // 'j' key to decrease count
                    if key.code == KeyCode::Char('j') {
                        self.decrease_counter();
                    }
                    // 'k' key to increase count
                    if key.code == KeyCode::Char('k') {
                        self.increase_counter();
                    }
                    // 'q' key to exit application
                    if key.code == KeyCode::Char('q') {
                        self.exit();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn increase_counter(&mut self) {
        self.counter += 1;
    }
    fn decrease_counter(&mut self) {
        self.counter -= 1;
    }
}
