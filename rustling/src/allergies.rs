#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

pub fn main() {
    // let num = "1789372997";
    // let num = "4539319503436467";
    // let num = "8273123273520569";
    let num = "0";

    let all_digits = get_all_digits(num);
    println!("all-digits: {:?}", all_digits);

    let mut doubled_digits: Vec<u8> = vec![];

    for (idx, digit) in all_digits.iter().enumerate() {
        if idx % 2 == 0 {
            // println!("{}:{} idx%2={}", idx, digit, idx % 2);
            let doubled = if digit * 2 > 9 {
                (digit * 2) - 9
            } else {
                digit * 2
            };
            doubled_digits.push(doubled);
        } else {
            doubled_digits.push(*digit);
        }
    }
    println!("doubled={:?}", doubled_digits);

    let mut sum_digits = 0;
    for digit in &doubled_digits {
        sum_digits += digit;
    }
    println!("sum_digits={}", sum_digits);

    let result = sum_digits % 10 == 0;
    println!("{}", result);

    // println!("sum_digits={}", sum_digits);

    // let formula_result = 10 - (sum_digits % 10);
    // println!("formula_result={}", formula_result);
}

fn get_all_digits(code: &str) -> Vec<u8> {
    let mut result = vec![];
    for c in code.chars() {
        let num = c.to_digit(10).unwrap() as u8;
        result.push(num);
        // println!("{}", num);
    }
    result
}

fn get_all_digits_of_number(num: u32) -> Vec<u8> {
    let mut result = vec![];
    let mut n = num;
    if n == 0 {
        result.push(0)
    };
    while n > 0 {
        let digit = n % 10;
        n /= 10;
        result.push(digit as u8);
    }
    result
}

fn sum_digits_in_number(num: u32) -> u32 {
    let mut result: u32 = 0;
    let all_digits = get_all_digits_of_number(num);

    // println!("{} {:?}", num, all_digits);

    for digit in all_digits {
        result += digit as u32;
    }

    result
}
