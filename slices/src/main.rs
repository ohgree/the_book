fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();
    //still, word == 5

    //string slices - refs to portion of String
    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[..];

    let mut s = String::from("hello world");
    let word = first_word(&s);

    //s.clear();

    println!("{}", word);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
// Compatible w/ &String and &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn demo_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
