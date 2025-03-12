use std::fmt::Display;

// trait == interface
pub trait Summary {
    fn summarize(&self) -> String; // virtual method
    fn summarize_default(&self) -> String { // default implementations
        String::from("(Read more...)")
    }
}

pub fn notify<T: Summary>(item: &T) { // reference to anything that implements Summary
    println!("Breaking news! {}", item.summarize());
}
fn some_function<T, U>(t: &T, u: &U) -> impl Summary
where
    T: Summary + Clone, // Implements both Summary and Clone
    U: Clone + Display,
{
    NewsArticle {
        headline: String::from(""),
        location: String::from(""),
        author: String::from(""),
        content: String::from("")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

trait ToString {}

impl<T: Display> ToString for T {
    // --snip--
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
// Rule - more specific wins
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}