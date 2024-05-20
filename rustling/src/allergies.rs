#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

pub fn main() {
    let num: u32 = 4_106_098_957;

    let mut n = num;

    // get the list of digits
    let mut all_digits: Vec<u32> = vec![];

    if n == 0 {
        all_digits.push(0)
    }

    while n > 0 {
        let ones_place = n % 10;
        n = n / 10;
        all_digits.push(ones_place);

        // println!("{}", ones_place);
    }
    println!("all_digits: {:?}", all_digits);

    let no_of_digits = all_digits.len();

    let mut new_number: u32 = 0;

    for digit in all_digits {
        new_number += digit.pow(no_of_digits as u32);
    }
    println!("old={} new={}", num, new_number);

    let result = num == new_number;
    // println!("{}", result);
}
