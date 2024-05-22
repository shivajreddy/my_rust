#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::any::type_name_of_val;

pub fn make_first_character_capital() -> String {
    let name = "pRedatoR".to_string();
    let name_in_lowercase = name.to_lowercase();
    let (first, rest) = name_in_lowercase.split_at(1);
    format!("{}{}", first.to_uppercase(), rest)
}

pub fn main() {
    // let arr = [10, 20, 30];

    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[0, 1, 2];

    let arr = [10, 20, 30, 40, 50];
    println!("arr:   {:?}", arr);

    let target = [40, 50];

    for x in 0..arr.len() {
        let slice = &arr[x..];
        println!("slice: {:?}", slice);
        if slice.starts_with(&target) {
            println!("yes");
        }
    }

    // let slice = &arr[..3];
    // println!("slice: {:?}", slice);

    // for (item, idx) in slice.iter().enumerate() {
    //     println!("{}:{}", item, idx);
    // }

    // for (_, idx) in list_one.iter().enumerate() {
    //     println!("{:?}", list_one[..]);
    // }
}

fn some_compare<T>(a: &[T], b: &[T]) {}
