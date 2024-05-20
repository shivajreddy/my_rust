#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::{borrow::Borrow, thread::current};

#[derive(Debug, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    // hey man, use u8 instead
    pub fn value(&self) -> u32 {
        match self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }
}

const ALL_ALLERGENS: &[Allergen] = &[
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

pub fn main() {
    // let mut result: Vec<Allergen> = vec![];
    //
    // let mut given_value = 80;
    // for allergen in ALL_ALLERGENS.iter().rev() {
    //     // println!(" {:?} {}", allergen, allergen.value());
    //
    //     if allergen.value() <= given_value {
    //         given_value -= allergen.value();
    //         result.push(*allergen);
    //     }
    // }
    //
    // println!("{:?}", result);
    let mut given_score = 509;
    let mut given_score = 257;
    while given_score > 256 {
        given_score -= 256
    }
    println!("{}", given_score);
}
