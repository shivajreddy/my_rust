use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let mut hm: HashMap<char, u32> = HashMap::new();

    let result: HashSet<&'a str> = HashSet::new();

    // iterate over the chars in the given string
    for c in word.chars() {
        if let Some(val) = hm.get(&c) {
            hm.insert(c, val + 1);
        } else {
            hm.insert(c, 1);
        }
    }

    println!("hm =================================");
    println!("{:?}", hm);

    result
}
/*
example, "PoTS" is an anagram of "sTOp",
but StoP is not an anagram of sTOp
*/
