#![allow(unused)]

pub fn main() {
    println!("Chapter 11");
    println!("Chapter 11");
}

fn addition(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        assert_eq!(10, addition(5, 5))
    }
}
