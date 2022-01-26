mod basic_trait;
mod conditionally_implement_methods;
mod finding_largest_using_trait_bounds;
mod trait_bounds;

use basic_trait::*;
use conditionally_implement_methods::*;
use finding_largest_using_trait_bounds::*;
use trait_bounds::*;

fn main() {
    println!("basic_trait in rust!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

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

    notify(&article);

    println!("{}", returns_summarizable().summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
