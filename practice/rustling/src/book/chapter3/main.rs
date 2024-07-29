#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

/*
Notes
Chapter-3 : Contsants

variable :: - by default they are immutable
            - can make them mutable by using `mut`
            - can't create outside functions
                - to create a variable outside the functions,
                  use `static` instead of `let`
                - static variables can also be declared as mut
                  but needs declration of type
                ex: static mut SOME_VAR2:i32 = 10;

constants:: - they are immutable
            - can't use `mut` infront of the name
            - CAN be created outside the function
            - can have expressions that will evaluate to a value,
              known at compile time
            - can directly use thim inside print statements {}
              ex: print!("{ANY_CONST_VARIABLE}")

Data Types:
    - scalar Types
    - Integer Types

*/

const HI: i32 = 10 + 10;
static SOME_STATIC_VARIABLE: i32 = 10;

pub fn main() {
    println!("{HI}");
}
