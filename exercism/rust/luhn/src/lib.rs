/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let all_digits = get_all_digits(code);
    // println!("all-digits: {:?}", all_digits);

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
    // println!("doubled={:?}", doubled_digits);

    let mut sum_digits = 0;
    for digit in &doubled_digits {
        sum_digits += digit;
    }
    // println!("sum_digits={}", sum_digits);

    let result = sum_digits % 10 == 0;
    // println!("{}", result);
    result
}

fn get_all_digits(code: &str) -> Vec<u8> {
    let mut result = vec![];
    for c in code.chars() {
        let num = c.to_digit(10).unwrap() as u8;
        result.push(num);
    }
    result
}
