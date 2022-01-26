use crate::basic_trait::*;


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