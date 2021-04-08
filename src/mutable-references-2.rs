fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // This causes an error "cannot borrow `s` as mutable
    // more than once at a time".
    //let r2 = &mut s;

    //println!("{}, {}", r1, r2);
}
