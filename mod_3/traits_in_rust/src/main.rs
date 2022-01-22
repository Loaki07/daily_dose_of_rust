mod basic_trait;

use basic_trait::*;

fn main() {
    println!("basic_trait in rust!");

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
