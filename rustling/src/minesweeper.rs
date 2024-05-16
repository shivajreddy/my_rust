#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

/*
    ·*·*·
    ··*··
    ··*··
    ·····

    1*3*1
    13*31
    ·2*2·
    ·111·
*/

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
        // if get_item_at_coord(&grid, neighbor) == '*' {
        //     no_of_mines += 1;
        // }
    }

    no_of_mines

    /*
    // get neighbors
    let r_prev = if r == 0 { r } else { r - 1 };
    let r_next = if r == rows - 1 { r } else { r + 1 };
    let c_prev = if c == 0 { c } else { c - 1 };
    let c_next = if c == cols - 1 { c } else { c + 1 };

    let tl: Some<(usize, usize)> = (r_prev, c_prev); // top r-1, c-1
    let t = (r_prev, c); // top r-1, c
    let tr = (r_prev, c_next); // top-right r-1, c+1

    let l = (r, c_prev); // left r, c-1
    let r = (r, c_next); // right r, c+1

    let bl = (r_next, c_prev); // bottom-left r+1, c-1
    let b = (r_next, c); // bottom r+1, c
    let br = (r_next, c_next); // right-bottom r+1, c+1

    let all_neighbors = [tl, t, tr, l, r, bl, b, br];
    */
}

pub fn main() {
    let main_grid: [&str; 4] = ["·*·*·", "··*··", "··*··", "·····"];

    let grid: &[&str] = &main_grid;

    for (r, row_str) in grid.iter().enumerate() {
        // println!("\nROW -> {}", row_str);
        for (c, item) in row_str.chars().enumerate() {
            if let Some(char) = get_item_at_coord(&grid, (r as i8, c as i8)) {
                println!("*");
            } else {
                let mines = get_count(&grid, r, c);
                println!("{}", mines);
            }
            // println!("({},{}) {}", r, c, item);
        }
    }
}
