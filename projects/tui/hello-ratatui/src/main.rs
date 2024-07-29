#![allow(unused)]

// /*
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::Backend,
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::fs;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new("Hi wow there woaw"), area)
        });
    }

    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode();
    Ok(())
}
// */
/*
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{self, stdout, Error, Read, Result, Write};
use std::mem::size_of_val;
use std::ops::Deref;

fn do_some_fs() -> Result<()> {
    let result_file = std::fs::File::open("foo.txt");
    let mut file = result_file.unwrap(); // panics if file isn't found
                                         // create a file with text
                                         // let mut file = File::create("foo.txt")?;
                                         // file.write_all(b"wow\n")?;
                                         // file.write_all(b"wow")?;

    let mut buffer = vec![];

    // check the current operating system
    let x = file.read_to_end(&mut buffer);

    let r1 = cfg!(target_os = "windows");
    let r2 = cfg!(target_os = "macos");
    let r3 = cfg!(target_os = "linux");
    println!("{r1}{r2}{r3}");

    if cfg!(target_os = "windows") {
        println!("you are running a windows")
    };

    println!("{:?}", buffer);

    std::io::Result::Ok(())
}
// */
