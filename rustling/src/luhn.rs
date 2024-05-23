#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

pub fn main() {
    println!("luhn");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Create a new test module
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add2() {
        assert_eq!(add(2, -3), 1);
    }
}
