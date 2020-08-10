pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl NewsArticle {
    pub fn new(headline: String, location: String, author: String, content: String) -> NewsArticle {
        NewsArticle {
            headline,
            location,
            author,
            content,
        }
    }
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("user @{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
use std::fmt::Debug;
use std::fmt::Display;
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    42
}
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_owned(),
        content: "lorem ipsum".to_owned(),
        reply: false,
        retweet: false,
    }
}
fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_owned(),
        content: "lorem ipsum".to_owned(),
        reply: false,
        retweet: false,
    };
    println!("tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest: {}", self.x);
        } else {
            println!("The largest: {}", self.y);
        }
    }
}
