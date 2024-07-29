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

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for (r, row_str) in minefield.iter().enumerate() {
        let mut output_row_str = String::new();
        for c in 0..row_str.chars().count() {
            // if the given co-ord itself is a mine, then no need to check
            let curr_item = row_str.chars().nth(c).unwrap();
            if curr_item == '*' {
                output_row_str.push('*');
                continue;
            }

            // get the no.of mines at this co-ordinate
            let mines_around_this_item = get_count(&minefield, r, c);
            if mines_around_this_item == 0 {
                output_row_str.push(curr_item);
            } else {
                output_row_str.push_str(&mines_around_this_item.to_string());
            }
        }
        // println!("{}", output_row_str);
        result.push(output_row_str)
    }
    result
}
