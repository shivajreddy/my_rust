use std::usize;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;

    // get the list of digits
    let mut all_digits: Vec<usize> = vec![];

    if n == 0 {
        all_digits.push(0)
    }

    while n > 0 {
        let ones_place = n % 10;
        n = n / 10;
        all_digits.push(ones_place as usize);

        // println!("{}", ones_place);
    }
    // println!("all_digits: {:?}", all_digits);

    let no_of_digits = all_digits.len();

    let mut new_number: usize = 0;

    for digit in all_digits {
        let powered = digit.pow(no_of_digits as u32);
        new_number += powered;
    }

    let result = num as usize == new_number;

    result
}
