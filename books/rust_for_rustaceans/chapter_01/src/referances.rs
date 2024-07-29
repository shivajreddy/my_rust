#![allow(unused)]

pub fn main() {
    let res = str_before("hello there", 'e');
    println!("{:?}", res.unwrap());
}

struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        while self.document.len() > 0 {
            if &self.document[0..1] == &self.delimiter[0..1] {
                return Some(&self.document);
            } else {
                self.document = &self.document[1..];
            }
        }
        None
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}
