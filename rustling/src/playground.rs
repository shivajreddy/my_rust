#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
    // let mut char_indices = yes.char_indices();
    // assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
    // assert_eq!(Some((1, '\u{0306}')), char_indices.next());
    // // note the 3 here - the previous character took up two bytes
    // assert_eq!(Some((3, 'e')), char_indices.next());
    // assert_eq!(Some((4, 's')), char_indices.next());
    // assert_eq!(None, char_indices.next());

    let input = "y̆es";

    println!("{} length={}", &input, input.len());
    for (byte_index, c) in input.char_indices() {
        let c_len = c.len_utf8();
        let next_index = byte_index + c_len;
        let c_str = &input[byte_index..next_index];

        println!("{}  | {} len_utf8  | UTF8-{}", c_str, c_len, c as u32);
    }

    println!("-----");

    for (byte_index, c) in input.char_indices().rev() {
        let c_len = c.len_utf8();
        let next_index = byte_index + c_len;
        let c_str = &input[byte_index..next_index];

        println!("{}  | {} len_utf8  | UTF8-{}", c_str, c_len, c as u32);
    }

    println!("{:?}", input.char_indices());

    let x = input.graphemes(true);
    println!("{:?}", &x);
}
