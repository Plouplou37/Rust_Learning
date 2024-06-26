mod aggregator;
use aggregator::{notify_v1, notify_v2, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("test_username"),
        content: String::from("test_content"),
        reply: false,
        retweet: false,
    };

    println!("tweet summary: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    println!("Article: {}", article.summarize());
    println!("author is {}", article.summarize_author());

    notify_v1(&tweet);
    notify_v2(&tweet, &article);
}
