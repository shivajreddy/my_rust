#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::any::type_name_of_val;

fn get_item_at_coord(grid: &[&str], rc: (i8, i8)) -> Option<char> {
    let rows = grid.len();
    let cols = grid[0].len();
    if rc.0 < 0 || rc.0 == rows as i8 || rc.1 < 0 || rc.1 == cols as i8 {
        return None;
    }
    Some(grid[rc.0 as usize].chars().nth(rc.1 as usize).unwrap())
}

fn get_count(grid: &[&str], r: usize, c: usize) -> i32 {
    let mut no_of_mines: i32 = 0;

    let r = r as i8;
    let c = c as i8;

    let rows = grid.len();
    let cols = grid[0].len();

    let all_neighbors = [
        // top-row
        get_item_at_coord(&grid, (r - 1, c - 1)),
        get_item_at_coord(&grid, (r - 1, c)),
        get_item_at_coord(&grid, (r - 1, c + 1)),
        // current-row
        get_item_at_coord(&grid, (r, c - 1)),
        get_item_at_coord(&grid, (r, c + 1)),
        // bottom-row
        get_item_at_coord(&grid, (r + 1, c - 1)),
        get_item_at_coord(&grid, (r + 1, c)),
        get_item_at_coord(&grid, (r + 1, c + 1)),
    ];

    // Start counting
    for neighbor in all_neighbors {
        match neighbor {
            Some(item) => {
                if item == '*' {
                    no_of_mines += 1;
                }
            }
            None => {}
        }
    }

    no_of_mines
}

pub fn main() {
    let main_grid: [&str; 4] = ["·*·*·", "··*··", "··*··", "·····"];

    let grid: &[&str] = &main_grid;

    for (r, row_str) in grid.iter().enumerate() {
        for c in 0..row_str.chars().count() {
            // for (c, item) in row_str.chars().enumerate() {
            if let Some(val) = get_item_at_coord(&grid, (r as i8, c as i8)) {
                let mines = get_count(&grid, r, c);
                println!("{}", mines);
            } else {
                println!("*");
            }
        }
    }
}
