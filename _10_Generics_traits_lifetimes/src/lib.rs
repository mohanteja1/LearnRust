// defining trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// implementing trait on a type

pub struct NewsArticle {
    pub headline: String, 
    pub author: String,
    pub content: String,
    pub location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}, {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}