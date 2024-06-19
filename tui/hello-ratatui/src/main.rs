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


use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{self, Read};
use std::mem::size_of_val;

// fn main() -> Result<String, io::Error> { 
fn main() {
    // create a file with text
    // let mut new_file = File::create("bar.txt");

    // create a file in target directory

    // println!("{}", size_of_val(&new_file));

    // read a file, and print content to console, line by line

    // copy data from one file to another
}


enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    // Method to calculate area of the shape
    fn area(&self) -> f64 {
        // Encapsulating the logic specific to each variant
        match self {
            Shape::Circle(radius) => {
                std::f64::consts::PI * radius * radius
            }
            Shape::Rectangle(length, width) => {
                length * width
            }
        }
    }
    // Method to calculate the perimeter of a shape
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
        }
    }
}


use std::fmt;

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "<Circle r:{r}>"),
            Shape::Rectangle(l, w) => write!(f, "<Rectangle l:{l} w:{w}>")
        }
    }
}


enum Result<T, E>
    where E: Display
{
    Ok(T),
    Err(E),
}


