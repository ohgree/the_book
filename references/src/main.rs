fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("length of {:?} is {}", s1, len);

    let mut s = s1.clone();
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        //r1: goes out of scope.
    }
    let r2 = &mut s;
    //new ref is ok.
    //multiple imm ref is ok. imm & mut ref is NOT ok.

    //scope is terminated after it is used last.
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}
fn dangle() -> String/*&String*/ { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s
    //&s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
fn change(s: &mut String) {
    s.push_str(", World!");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
