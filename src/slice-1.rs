fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    // word still has the value 5 here.
    // Because s became empty, word is totally invalid.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
