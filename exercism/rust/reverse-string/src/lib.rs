pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");

    // let my_input = "shiva reddy wow";

    let mut result = String::new();
    for c in input.chars().rev() {
        result.push(c);
        // println!("{}", c);
    }

    // let my_input = "Würstchenstand";
    // let my_expected = "dnatsnehctsrüW";

    /*
    Each char is 4 bytes in rust. And each char is a unicode
    ü (latin small letter u with diaeresis): special characters like this are made of 2 chars
        `u` -> UTF-8 encoding 117
        `̈`  -> UTF-8 encoding 776
    what we are looking for is `ü` -> unicode 252
    so when you actually reverse the string through going character by character, then
        before reverse: (other-chars-at-front), u-117 u-776, (other-chars-at-rear)
    the reverse would become become
        after reverse: (other-chars-at-rear), u-776 u-117, (other-chars-at-front)
    but this is not what we want, because our symbol is `ü`
    */

    // for c in my_input.chars() {
    //     println!("`{}` {} bytes {}", c, std::mem::size_of_val(&c), c as u32);
    // }

    result
}
