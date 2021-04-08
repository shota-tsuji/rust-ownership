fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    // An error ocure because "there is no value in &s points to".
    // S is dropped when it is goes out of scope (out of this function).
    &s
}
