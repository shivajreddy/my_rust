#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use unicode_segmentation::UnicodeSegmentation;

// pub fn main() {
//     let input = "y̆es";
//
//     println!("{} length={}", &input, input.len());
//     for (byte_index, c) in input.char_indices() {
//         let c_len = c.len_utf8();
//         let next_index = byte_index + c_len;
//         let c_str = &input[byte_index..next_index];
//
//         println!("{}  | {} len_utf8  | UTF8-{}", c_str, c_len, c as u32);
//     }
//     println!("{:?}", input.char_indices());
//
//     let x = input.graphemes(true);
//     println!("{:?}", &x);
// }

use std::collections::{HashMap, HashSet};

pub fn main<'a>() -> HashSet<&'a str> {
    let word = String::from("dog");
    let possiblel_anagrams: &[&str] = &["dgo, dogo, god"];

    let mut hm: HashMap<char, u32> = HashMap::new();

    for c in word.chars() {
        if let Some(val) = hm.get(&c) {
            hm.insert(c, val + 1);
        } else {
            hm.insert(c, 1);
        }
    }

    let result: HashSet<&'a str> = HashSet::new();

    println!("{:?}", hm);

    result
}

fn create_map(word: &str) -> HashMap<char, u32> {
    let mut result: HashMap<char, u32> = HashMap::new();

    for c in word.chars() {
        if let Some(val) = result.get(&c) {
            result.insert(c, val + 1);
        } else {
            result.insert(c, 1);
        }
    }

    result
}
