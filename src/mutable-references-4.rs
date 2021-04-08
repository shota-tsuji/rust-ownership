fn main() {
    let mut s = String::from("hello");

    let _r1 = &s;
    let _r2 = &s;
    // This causes the error "cannot borrow `s` as mutable
    // because it is also borrowed as immutable".
    //let r3 = &mut s; // BIG PROBLEM

    //println!("{}, {}, and {}", _r1, _r2, r3);
}
