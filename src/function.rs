fn main() {
    let s = String::from("hello");

    // S's value moves into the function.
    takes_ownership(s);

    // So s is no longer valid here.

    let x = 5;

    // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward.
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
