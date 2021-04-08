fn main() {
    // The return value is moved into s1.
    let s1 = gives_ownership();

    // s1 comes into scope.
    let s2 = String::from("hello");

    // s2 is moved into function.
    // The return value is moved into s3.
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
