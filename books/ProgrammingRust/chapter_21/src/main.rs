#![allow(unused)]
/**
- Macros are the first things that get expanded during compilation
- They are expanded to actual rust code.
- You CAN'T call a macro before defining it

-  All macros are defined using `macro_rules!` which in itself is a macro
- There are few macros that are built into the compiler such as
    - file!, line!, macro_rules!

- macros that are defined using macro_rules! work by pattern matching

    (pattern1) => (template1);
    (pattern2) => (template2);

- when calling the macro, you don't have to only use (), can also use [], {}
- only difference is that if using {}, then ; is optional


- Rust macros are like a small language within
- just like how regular expressions match for patterns, macros match for tokens
- tokens: numbers, names, punctuation marks, etc.
- comments & whitespace are NOT tokens, so we can use them in macros
for readability purposes


Types with in Macros are called `Framgment Types`. Table 21-2(pg 613) in Oreily book.

*/

// My own version of assert_eq! macro
macro_rules! my_assert_eq {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "ASSERTION ERROR: `left == right` failed
     left: `{:?}`
    right: `{:?}`)",
                        left_val, right_val
                    )
                }
            }
        }
    };
}

fn main() {
    // my_assert_eq! {"hi", "hell"};
    // println!("hello there");
    print!("hello\n");
}
