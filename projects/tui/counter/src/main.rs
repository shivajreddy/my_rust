#![allow(unused)]

use std::io::{stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEvent, KeyEventKind},
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
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

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
                self.event_increase(key);
                self.event_decrease(key);
                self.event_exit(key);
            }
        }
        Ok(())
    }

    // Events
    fn event_exit(&mut self, key: KeyEvent) {
        // verify event
        let target_key = 'q';
        if key.kind != KeyEventKind::Press || key.code != KeyCode::Char(target_key) {
            return;
        }
        // event logic
        self.exit = true;
    }

    fn event_increase(&mut self, key: KeyEvent) {
        // verify event
        let target_key = 'k';
        if key.kind != KeyEventKind::Press || key.code != KeyCode::Char(target_key) {
            return;
        }
        // event logic
        self.counter += 1;
    }

    fn event_decrease(&mut self, key: KeyEvent) {
        // verify event
        let target_key = 'j';
        if key.kind != KeyEventKind::Press || key.code != KeyCode::Char(target_key) {
            return;
        }
        // event logic
        self.counter -= 1;
    }
}
