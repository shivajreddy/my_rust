#![allow(unused)]
/// Based on the following tutorial
/// https://www.youtube.com/watch?v=s5S2Ed5T-dc&t=703s

fn main() {
    let hm = HashMap::new(10, 20);
    {
        let r = hm.get(&10);
        println!("{}", r);
    }
    // output type: Option<&V>
    {
        let r1 = hm.get_with_option_some(&10);
        println!("{:?}", r1);
        let r2 = hm.get_with_option_none(&10);
        println!("{:?}", r2);
    }
    // output type: Result<&V, KeyNotFound>
    {
        let r1 = hm.get_with_result_no_error(&10);
        println!("{:?}", r1);
        let r2 = hm.get_with_result_error(&10);
        println!("{:?}", r2);
    }
}

struct HashMap<K, V>
where
    V: Copy + Clone,
{
    key: K,
    val: V,
}

#[derive(Debug)]
struct KeyNotFound;

impl<T1, T2> HashMap<T1, T2>
where
    T2: Copy + Clone,
{
    fn new(key: T1, val: T2) -> Self {
        HashMap { key, val }
    }

    fn get(&self, key: &T1) -> &T2 {
        &self.val
    }

    // ---- the way its done in std
    fn get_with_option_some(&self, key: &T1) -> Option<&T2> {
        Some(&self.val)
    }
    fn get_with_option_none(&self, key: &T1) -> Option<&T2> {
        None
    }

    // ---- we could also output a better output using Result
    fn get_with_result_no_error(&self, key: &T1) -> Result<&T2, KeyNotFound> {
        Ok(&self.val)
    }
    fn get_with_result_error(&self, key: &T1) -> Result<&T2, KeyNotFound> {
        Err(KeyNotFound)
    }
}
