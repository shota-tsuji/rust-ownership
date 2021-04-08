fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(_some_string: &String) {
    // This causes error "cannot borrow '...' as mutable,
    // as it is behind a '&' reference".
    //_some_string.push_str(", world");
}
