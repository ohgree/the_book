fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // Stack-Only Data - Copy
    // Known Size & stored entirely in stack: Deep & Shallow have no diffs.
    // trait: Copy
    let x = 5;
    let y = x;
    if x == y { println!("equal!") }

    // Move - Rust never create deep copy automatically
    // Drop trait - states what to do when a var goes out of scope
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1);

    // Clone - Deep copy
    let s1 = s2.clone();
    println!("{}, {}", s1, s2);

    let s = String::from("hello");
    take_ownership(s);
    //s is invalid here

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello!");
    let s3 = takes_and_gives_back(s2);
    // after scope - dropped: s3, s1. nothing: s2(moved)

    let (s2, len) = calculate_length(s1);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
fn gives_ownership() -> String {
    let some = String::from("hello");
    some
}
fn takes_and_gives_back(some: String) -> String {
    some
}

fn take_ownership(some: String) {
    println!("{} is taken", some);
}
fn makes_copy(some: i32) {
    println!("{} is copied", some);
}
