pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait A {
}
pub trait B {
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
impl A for NewsArticle {}
impl B for NewsArticle {}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T>(item: &T) where T: Summary + A + B {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let sp = NewsArticle {
        headline: String::from("s"),
        location: String::from("s"),
        author: String::from(""),
        content: String::from("")
    };
    notify(&sp);
}
