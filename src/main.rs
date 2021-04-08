fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // This cause error 'borrow of moved value'
    //println!("{}, world!", s1);
}
