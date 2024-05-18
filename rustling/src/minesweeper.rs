#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::any::type_name_of_val;

fn get_item_at_coord(grid: &[&str], rc: (i8, i8)) -> Option<char> {
    // println!("get_item_at_coord:: rc:{:?}", rc);
    let rows = grid.len();
    let cols = grid[0].len();
    if rc.0 < 0 || rc.0 == rows as i8 || rc.1 < 0 || rc.1 == cols as i8 {
        return None;
    }
    // Some(grid[rc.0 as usize].chars().nth(rc.1 as usize).unwrap())
    grid[rc.0 as usize].chars().nth(rc.1 as usize)
}

fn get_count(grid: &[&str], r: usize, c: usize) -> i32 {
    // println!("get_count:: r:{},c:{}", r, c);
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
    // let main_grid: [&str; 4] = ["·*·*·", "··*··", "··*··", "·····"];
    let main_grid = ["*1 1*"];

    let grid: &[&str] = &main_grid;

    for (r, row_str) in grid.iter().enumerate() {
        let mut output_row_str = String::new();
        for c in 0..row_str.chars().count() {
            // check if this item in itself is a mine

            // if the given co-ord itself is a mine, then no need to check
            if row_str.chars().nth(c).unwrap() == '*' {
                output_row_str.push('*');
                continue;
            }

            // get the no.of mines at this co-ordinate
            let mines_around_this_item = get_count(&grid, r, c);
            if mines_around_this_item == 0 {
                output_row_str.push('.');
            } else {
                output_row_str.push_str(&mines_around_this_item.to_string());
            }
        }
        println!("{}", output_row_str);
    }
}
