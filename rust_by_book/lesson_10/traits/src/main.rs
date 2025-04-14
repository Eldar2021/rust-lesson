use traits::collect_two_summaries;
use traits::notify;
use traits::{Article, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Eldiiar Almazbek"),
        content: String::from("Hello world"),
    };

    println!("1-tweet summarize: {}", tweet.summarize());
    println!("1-tweet min: {}", tweet.min_summary());

    notify(&tweet);

    let article = Article {
        title: String::from("Article title"),
        content: String::from("Article content"),
    };

    println!("2-article summarize: {}", article.summarize());
    println!("2-article min: {}", article.min_summary());

    collect_two_summaries(&tweet, &tweet);
}
