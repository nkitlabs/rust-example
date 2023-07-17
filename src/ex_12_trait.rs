use std::fmt::{Debug, Display};

trait Summary {
    // fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    // default implementation.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle2 {
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    return 0;
}

fn some_function_2<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 0;
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

pub fn example_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // use default implementation.
    let article = NewsArticle2 {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article2 available! {}", article.summarize());

    // combined traits & trait bounds
    let str1 = String::from("test1");
    let str2 = String::from("test2");
    let _x = some_function(&str1, &str2);
    let _x = some_function_2(&str1, &str2);

    // return object that align with the trait.
    // the following example is a simple one, it can return multiple type of objects
    // but will show later.
    let example_object = returns_summarizable();
    println!("example_object: {}", example_object.summarize());
}
