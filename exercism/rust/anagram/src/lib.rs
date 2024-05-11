use std::collections::{HashMap, HashSet};

/*
 Anagram definition:
  - a replacement of the word, case-insesitive
  - if (case-insesitive) all chars are at same location, then NO
    Eg: DOg and dOg are not anagrams
*/

fn create_char_map(word: &str) -> HashMap<char, u32> {
    let mut hm: HashMap<char, u32> = HashMap::new();
    // Create HashMap
    for c in word.chars() {
        if let Some(val) = hm.get(&c) {
            hm.insert(c, val + 1);
        } else {
            hm.insert(c, 1);
        }
    }
    hm
}

fn same_frequencey(map1: &HashMap<char, u32>, map2: &HashMap<char, u32>) -> bool {
    if map1.len() != map2.len() {
        return false;
    }
    for character in map1.keys() {
        if !map2.contains_key(&character) {
            return false;
        }
        if map1.get_key_value(&character) != map2.get_key_value(&character) {
            return false;
        }
    }
    true
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let word_freq_map = create_char_map(word);

    println!("Given word >> {}", word);
    for possible_anagram in possible_anagrams {
        println!("possible_anagram::{}", possible_anagram);
        if word.to_lowercase() == possible_anagram.to_lowercase() {
            break;
        }
        let freq_map = create_char_map(possible_anagram);
        if !same_frequencey(&word_freq_map, &freq_map) {
            break;
        }
        result.insert(&possible_anagram);
    }

    result
}
