#![allow(dead_code)]
#![allow(unused_variables)]

/*
If A = [3, 4] and B = [1, 2, 3, 4, 5], then A is a sublist of B
If A = [1, 2, 3] and B = [1, 2, 3], then A and B are equal
If A = [1, 2, 3, 4, 5] and B = [2, 3, 4], then A is a superlist of B
*/

pub fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = [3, 4];

    let a = [1, 2, 3, 3, 3, 4, 5];
    let b = [3, 4];

    let result: bool;

    // check if the same sequence of numbers exist
    let mut possible_starting_indices: Vec<usize> = vec![];
    let target_starting_number: i32 = b[0];
    // get all locations of the starting number
    for (idx, num) in a.iter().enumerate() {
        if *num == target_starting_number {
            possible_starting_indices.push(idx);
        }
        println!("{} {}", idx, num);
    }
    println!("possible_starting_indices:{:?}", possible_starting_indices);

    a.

    let a = [1, 2, 3];
    let b = [1, 2, 3];

    let a = [1, 2, 3, 4, 5];
    let b = [2, 3, 4];

}
