use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let string1 = "abcd".to_owned();
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("longest: {}", result);

    let novel = "Call me Ishmael. Some year ago...".to_string();
    let first_sentence = novel.split('.').next().expect("could not find .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
