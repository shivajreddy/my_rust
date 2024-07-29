#[allow(unused_variables)]
pub fn main() {
    let string_slice: &str = "Howdy";
    let mut heap_string: String = "partner".to_string();

    // How to translate b/w the two types
    // Transfer: &str to String
    let converted_type: String = string_slice.to_string();
    let converted_type1: String = "askdfj alskjdf".to_string();
    let s1 = String::from("aslkdjf");
    let s2 = String::from(string_slice);

    // Transfer: String to &str
    let borrower_of_same_type = &heap_string;
    let converted_slice = &heap_string[..];

    // Adding strings
    // let s3 = string_slice + heap_string; // cant add String to &str
    let s3 = heap_string + string_slice; // can add &str to String
                                         // let s4 = heap_string.push_str("hello");

    // Getting chars from string
    // grabbing chars is same for both satck(&str) and heap(Stack)
    for c in string_slice.chars() {
        println!("{}", c);
    }
    let optional_c1 = string_slice.chars().nth(0);
    let c1 = optional_c1.unwrap(); // panicks if optional value returns none
    if let Some(c1) = string_slice.chars().nth(200) {
        println!("{}", c1);
    }

    let optional_c1 = s3.chars().nth(201);
    match optional_c1 {
        Some(c1) => {
            println!("{}", c1);
        }
        None => {}
    }

    // same for heap string
    for c in s3.chars() {
        println!("{}", c);
    }
    let optional_c1 = s3.chars().nth(10);
    let c1 = optional_c1.unwrap(); // panicks if optional value returns none
    if let Some(c1) = optional_c1 {
        println!("{}", c1);
    }
}

/*
String:
- stored on heap
- mutable

&str:
- immutable reference
- reference to the string on heap, or stack, or embedded in the code itself.
Hardcoded strings are called string literals, these are of type &str (string slice) that are
hold in programs binary or static memory.

common:
- Both are grouping of characters, where each char is a U8 encoded
- They both can translate between into the otehr

*/
