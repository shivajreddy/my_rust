#![allow(unused)]

/*
use std::fs;
// crossterm necessary to run the application
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode,
               EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};

// ratatui necessary to run the application
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph
};


use std::io::{stdout, Result};


/// Add code to set up and restore the terminal
/// 1. App enters the 'alternate screen'
/// - allows app to render, without disturbing normal term.
/// 2. App enables the 'raw mode'
/// - this turns off input and output process by terminal
/// 3. App creates a 'backend' and 'terminal'
///  - and then clears the screen
/// Note: If disabling raw mode doesn't work
///  - windows: close terminal tab
///  - unix: `reset`
fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    Ok(())
}
// */

use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{self, stdout, Read, Result};
use std::mem::size_of_val;

// fn main() -> Result<String, io::Error> {
fn main() -> Result<()> {
    // create a file with text
    let file = File::create("foo.txt");
    println!("{}", size_of_val(&file));
    Ok(())
    //Result::Ok("done".to_string())
    // let mut new_file = File::create("bar.txt");

    // create a file in target directory

    // println!("{}", size_of_val(&new_file));

    // read a file, and print content to console, line by line

    // copy data from one file to another
}

fn something(x: i32) -> Result<i32, String> {
    if x > 0 {
        Result::Ok(x)
    } else {
        Result::Err("wrong".to_string())
    }
}
