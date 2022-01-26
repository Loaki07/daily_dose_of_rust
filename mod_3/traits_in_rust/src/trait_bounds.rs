use crate::basic_trait::*;
use std::fmt::{Debug, Display};

// the impl syntax here is actually
// a syntax sugar for a trait bound
// as shown below
// impl can be used for more consise code
// in normal cases
// pub fn notify(item: &impl Summary) {
//     println!("Breking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breking news! {}", item.summarize());
}

// multiple trait bound implementation using impl
// pub fn notify_items(item1: &(impl Summary + Display), item2: &impl Summary) {
//     // ...
// }

// multiple trait bound implementation using generics
pub fn notify_items<T: Summary + Display>(item1: &T, item2: &T) {
    // ...
}

// declaring multiple generics with additional traits
// pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     // ...
//     1
// }

// declaring multiple generics with additional traits
// using the where clause
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
    1
}

// returning types that implement a certain trait instead of concrete types
// it is very useful inside of closures and iterators
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// this will throw an error as you cannot return different types
// error: if and else have incompatiable types
// this is solved in the module conditionally implement methods
// pub fn returns_summarizable_conditions(switch: bool) -> impl Summary {
//     if switch {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     } else {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//             ),
//         }
//     }
// }
