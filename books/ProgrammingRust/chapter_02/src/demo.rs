/*
#![allow(unused)]

use std::cmp::min;
use std::collections::HashMap;

fn main() {
    assert_eq!(20, Solution::min_coins(vec![1, 2, 5], 100));
}

struct Solution {}

impl Solution {
    fn min_coins(coins: Vec<i32>, amount: i32) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let res = Self::rec(amount, &coins, &mut hm);
        if (res != i32::MAX) {
            return res;
        }
        -1
    }

    fn rec(val: i32, coins: &Vec<i32>, hm: &mut HashMap<i32, i32>) -> i32 {
        if val == 0 {
            return 0;
        }
        if val < 0 {
            return i32::MAX;
        }
        if hm.contains_key(&val) {
            return *hm.get(&val).unwrap();
        }
        let mut min_coins = i32::MAX;
        for coin in coins {
            let num_coins = Self::rec(val - coin, coins, hm);
            if num_coins != i32::MAX {
                min_coins = min(min_coins, num_coins + 1);
            }
        }
        hm.insert(val, min_coins);
        min_coins
    }
}
*/
