#![allow(unused)]

use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        return format!("(Read more from {}...)", self.summarize_author());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        return format!("{}", self.author);
    }

    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
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
        return format!("@{}", self.username);
    }

    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

pub struct Book {
    pub author: String,
    pub title: String,
}

impl Summary for Book {
    fn summarize_author(&self) -> String {
        return format!("{}", self.author);
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {
    unimplemented!()
}

fn some_other_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    unimplemented!()
}

fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    }
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        return Self { first, second };
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is x = {}", self.first);
        } else {
            println!("The largest member is y = {}", self.second);
        }
    }
}
