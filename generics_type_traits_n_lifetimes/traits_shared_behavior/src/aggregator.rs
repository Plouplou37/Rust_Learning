use std::cmp::PartialOrd;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

#[derive(Clone)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // Concatenate String attibutes --> (Summarize)
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("author is {}", self.author)
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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Tell to the compiler that notify must implement object with Summary Trait in its sigature.
pub fn notify_v1<T: Summary>(item1: &T) {
    println!("Breaking news ! {}", item1.summarize());
}

pub fn notify_v2<T, U>(item1: &T, item2: &U)
where
    T: Summary,
    U: Summary + Clone,
{
    let clone_item2 = item2.clone();
    println!(
        "Breaking news ! {} and also {} and {}",
        item1.summarize(),
        item2.summarize(),
        clone_item2.summarize(),
    );
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

// This implementation is used only if the inner type <T> implement PartialOrd and Display Traits.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_Display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
